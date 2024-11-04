#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bank {
    ptr: *mut u8,
}
unsafe impl Send for Bank {}
unsafe impl Sync for Bank {}
impl Bank {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLASH key register for bank 1"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH control register for bank 1"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH status register for bank 1"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH clear control register for bank 1"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_cur(self) -> crate::common::Reg<regs::PrarCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_prg(self) -> crate::common::Reg<regs::PrarPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_cur(self) -> crate::common::Reg<regs::ScarCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_prg(self) -> crate::common::Reg<regs::ScarPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_curr(self) -> crate::common::Reg<regs::WpsnCurr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_prgr(self) -> crate::common::Reg<regs::WpsnPrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "FLASH CRC control register for bank 1"]
    #[inline(always)]
    pub const fn crccr(self) -> crate::common::Reg<regs::Crccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "FLASH CRC start address register for bank 1"]
    #[inline(always)]
    pub const fn crcsaddr(self) -> crate::common::Reg<regs::Crcsaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH CRC end address register for bank 1"]
    #[inline(always)]
    pub const fn crceaddr(self) -> crate::common::Reg<regs::Crceaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH ECC fail address for bank 1"]
    #[inline(always)]
    pub const fn far(self) -> crate::common::Reg<regs::Far, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
}
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
    #[doc = "Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R"]
    #[inline(always)]
    pub const fn bank(self, n: usize) -> Bank {
        assert!(n < 2usize);
        unsafe { Bank::from_ptr(self.ptr.add(0x04usize + n * 256usize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::OptsrCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::OptsrPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH option clear control register"]
    #[inline(always)]
    pub const fn optccr(self) -> crate::common::Reg<regs::Optccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH register with boot address"]
    #[inline(always)]
    pub const fn boot_curr(self) -> crate::common::Reg<regs::BootCurr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH register with boot address"]
    #[inline(always)]
    pub const fn boot_prgr(self) -> crate::common::Reg<regs::BootPrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH CRC data register"]
    #[inline(always)]
    pub const fn crcdatar(self) -> crate::common::Reg<regs::Crcdatar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency"]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Read latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash signal delay"]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Flash signal delay"]
        #[inline(always)]
        pub fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[doc = "FLASH register with boot address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootCurr(pub u32);
    impl BootCurr {
        #[doc = "Boot address 0"]
        #[inline(always)]
        pub const fn boot_add0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot address 0"]
        #[inline(always)]
        pub fn set_boot_add0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Boot address 1"]
        #[inline(always)]
        pub const fn boot_add1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot address 1"]
        #[inline(always)]
        pub fn set_boot_add1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BootCurr {
        #[inline(always)]
        fn default() -> BootCurr {
            BootCurr(0)
        }
    }
    #[doc = "FLASH register with boot address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootPrgr(pub u32);
    impl BootPrgr {
        #[doc = "Boot address 0"]
        #[inline(always)]
        pub const fn boot_add0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot address 0"]
        #[inline(always)]
        pub fn set_boot_add0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Boot address 1"]
        #[inline(always)]
        pub const fn boot_add1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Boot address 1"]
        #[inline(always)]
        pub fn set_boot_add1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BootPrgr {
        #[inline(always)]
        fn default() -> BootPrgr {
            BootPrgr(0)
        }
    }
    #[doc = "FLASH clear control register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Bank 1 EOP1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 EOP1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Bank 1 WRPERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 WRPERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Bank 1 PGSERR1 flag clear bi"]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 PGSERR1 flag clear bi"]
        #[inline(always)]
        pub fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bank 1 STRBERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 STRBERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 INCERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 INCERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Bank 1 OPERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_operr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 OPERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Bank 1 RDPERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_rdperr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 RDPERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_rdperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Bank 1 RDSERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_rdserr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 RDSERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_rdserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bank 1 SNECCERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_sneccerr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 SNECCERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_sneccerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bank 1 DBECCERR1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_dbeccerr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 DBECCERR1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_dbeccerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Bank 1 CRCEND1 flag clear bit"]
        #[inline(always)]
        pub const fn clr_crcend(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRCEND1 flag clear bit"]
        #[inline(always)]
        pub fn set_clr_crcend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Bank 1 CRC read error clear bit"]
        #[inline(always)]
        pub const fn clr_crcrderr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC read error clear bit"]
        #[inline(always)]
        pub fn set_clr_crcrderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "FLASH control register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Bank 1 configuration lock bit"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 configuration lock bit"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bank 1 program enable bit"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 program enable bit"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank 1 sector erase request"]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 sector erase request"]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank 1 erase request"]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 erase request"]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bank 1 write forcing control bit"]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write forcing control bit"]
        #[inline(always)]
        pub fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bank 1 bank or sector erase start control bit"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 bank or sector erase start control bit"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Bank 1 sector erase selection number"]
        #[inline(always)]
        pub const fn ssn(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 sector erase selection number"]
        #[inline(always)]
        pub fn set_ssn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 6usize)) | (((val as u32) & 0x7f) << 6usize);
        }
        #[doc = "Bank 1 CRC control bit"]
        #[inline(always)]
        pub const fn crc_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC control bit"]
        #[inline(always)]
        pub fn set_crc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Bank 1 end-of-program interrupt control bit"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 end-of-program interrupt control bit"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Bank 1 write protection error interrupt enable bit"]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write protection error interrupt enable bit"]
        #[inline(always)]
        pub fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Bank 1 programming sequence error interrupt enable bit"]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 programming sequence error interrupt enable bit"]
        #[inline(always)]
        pub fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bank 1 strobe error interrupt enable bit"]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 strobe error interrupt enable bit"]
        #[inline(always)]
        pub fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 inconsistency error interrupt enable bit"]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 inconsistency error interrupt enable bit"]
        #[inline(always)]
        pub fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Bank 1 write/erase error interrupt enable bit"]
        #[inline(always)]
        pub const fn operrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write/erase error interrupt enable bit"]
        #[inline(always)]
        pub fn set_operrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Bank 1 read protection error interrupt enable bit"]
        #[inline(always)]
        pub const fn rdperrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 read protection error interrupt enable bit"]
        #[inline(always)]
        pub fn set_rdperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Bank 1 secure error interrupt enable bit"]
        #[inline(always)]
        pub const fn rdserrie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 secure error interrupt enable bit"]
        #[inline(always)]
        pub fn set_rdserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bank 1 ECC single correction error interrupt enable bit"]
        #[inline(always)]
        pub const fn sneccerrie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 ECC single correction error interrupt enable bit"]
        #[inline(always)]
        pub fn set_sneccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bank 1 ECC double detection error interrupt enable bit"]
        #[inline(always)]
        pub const fn dbeccerrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 ECC double detection error interrupt enable bit"]
        #[inline(always)]
        pub fn set_dbeccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Bank 1 end of CRC calculation interrupt enable bit"]
        #[inline(always)]
        pub const fn crcendie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 end of CRC calculation interrupt enable bit"]
        #[inline(always)]
        pub fn set_crcendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Bank 1 CRC read error interrupt enable bit"]
        #[inline(always)]
        pub const fn crcrderrie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC read error interrupt enable bit"]
        #[inline(always)]
        pub fn set_crcrderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "FLASH CRC control register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crccr(pub u32);
    impl Crccr {
        #[doc = "Bank 1 CRC sector number"]
        #[inline(always)]
        pub const fn crc_sect(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Bank 1 CRC sector number"]
        #[inline(always)]
        pub fn set_crc_sect(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Bank 1 CRC select bit"]
        #[inline(always)]
        pub const fn all_bank(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC select bit"]
        #[inline(always)]
        pub fn set_all_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bank 1 CRC sector mode select bit"]
        #[inline(always)]
        pub const fn crc_by_sect(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC sector mode select bit"]
        #[inline(always)]
        pub fn set_crc_by_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Bank 1 CRC sector select bit"]
        #[inline(always)]
        pub const fn add_sect(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC sector select bit"]
        #[inline(always)]
        pub fn set_add_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Bank 1 CRC sector list clear bit"]
        #[inline(always)]
        pub const fn clean_sect(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC sector list clear bit"]
        #[inline(always)]
        pub fn set_clean_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Bank 1 CRC start bit"]
        #[inline(always)]
        pub const fn start_crc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC start bit"]
        #[inline(always)]
        pub fn set_start_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Bank 1 CRC clear bit"]
        #[inline(always)]
        pub const fn clean_crc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC clear bit"]
        #[inline(always)]
        pub fn set_clean_crc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Bank 1 CRC burst size"]
        #[inline(always)]
        pub const fn crc_burst(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Bank 1 CRC burst size"]
        #[inline(always)]
        pub fn set_crc_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Crccr {
        #[inline(always)]
        fn default() -> Crccr {
            Crccr(0)
        }
    }
    #[doc = "FLASH CRC data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcdatar(pub u32);
    impl Crcdatar {
        #[doc = "CRC result"]
        #[inline(always)]
        pub const fn crc_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC result"]
        #[inline(always)]
        pub fn set_crc_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcdatar {
        #[inline(always)]
        fn default() -> Crcdatar {
            Crcdatar(0)
        }
    }
    #[doc = "FLASH CRC end address register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crceaddr(pub u32);
    impl Crceaddr {
        #[doc = "CRC end address on bank 1"]
        #[inline(always)]
        pub const fn crc_end_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC end address on bank 1"]
        #[inline(always)]
        pub fn set_crc_end_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crceaddr {
        #[inline(always)]
        fn default() -> Crceaddr {
            Crceaddr(0)
        }
    }
    #[doc = "FLASH CRC start address register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcsaddr(pub u32);
    impl Crcsaddr {
        #[doc = "CRC start address on bank 1"]
        #[inline(always)]
        pub const fn crc_start_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC start address on bank 1"]
        #[inline(always)]
        pub fn set_crc_start_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcsaddr {
        #[inline(always)]
        fn default() -> Crcsaddr {
            Crcsaddr(0)
        }
    }
    #[doc = "FLASH ECC fail address for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Far(pub u32);
    impl Far {
        #[doc = "Bank 1 ECC error address"]
        #[inline(always)]
        pub const fn fail_ecc_addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Bank 1 ECC error address"]
        #[inline(always)]
        pub fn set_fail_ecc_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Far {
        #[inline(always)]
        fn default() -> Far {
            Far(0)
        }
    }
    #[doc = "FLASH option clear control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optccr(pub u32);
    impl Optccr {
        #[doc = "OPTCHANGEERR reset bit"]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPTCHANGEERR reset bit"]
        #[inline(always)]
        pub fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Optccr {
        #[inline(always)]
        fn default() -> Optccr {
            Optccr(0)
        }
    }
    #[doc = "FLASH option control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "FLASH_OPTCR lock option configuration bit"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_OPTCR lock option configuration bit"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option byte start change option configuration bit"]
        #[inline(always)]
        pub const fn optstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte start change option configuration bit"]
        #[inline(always)]
        pub fn set_optstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash mass erase enable bit"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash mass erase enable bit"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OTP program control bit"]
        #[inline(always)]
        pub const fn pg_otp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OTP program control bit"]
        #[inline(always)]
        pub fn set_pg_otp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Option byte change error interrupt enable bit"]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error interrupt enable bit"]
        #[inline(always)]
        pub fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bank swapping configuration bit"]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank swapping configuration bit"]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    #[doc = "FLASH option status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OptsrCur(pub u32);
    impl OptsrCur {
        #[doc = "Option byte change ongoing flag"]
        #[inline(always)]
        pub const fn opt_busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change ongoing flag"]
        #[inline(always)]
        pub fn set_opt_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Brownout level option status bit"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Brownout level option status bit"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "IWDG1 control option status bit"]
        #[inline(always)]
        pub const fn iwdg1_hw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG1 control option status bit"]
        #[inline(always)]
        pub fn set_iwdg1_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "D1 DStop entry reset option status bit"]
        #[inline(always)]
        pub const fn n_rst_stop_d1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DStop entry reset option status bit"]
        #[inline(always)]
        pub fn set_n_rst_stop_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "D1 DStandby entry reset option status bit"]
        #[inline(always)]
        pub const fn n_rst_stby_d1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DStandby entry reset option status bit"]
        #[inline(always)]
        pub fn set_n_rst_stby_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Readout protection level option status byte"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Readout protection level option status byte"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IWDG Stop mode freeze option status bit"]
        #[inline(always)]
        pub const fn fz_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG Stop mode freeze option status bit"]
        #[inline(always)]
        pub fn set_fz_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG Standby mode freeze option status bit"]
        #[inline(always)]
        pub const fn fz_iwdg_sdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG Standby mode freeze option status bit"]
        #[inline(always)]
        pub fn set_fz_iwdg_sdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DTCM RAM size option status"]
        #[inline(always)]
        pub const fn st_ram_size(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "DTCM RAM size option status"]
        #[inline(always)]
        pub fn set_st_ram_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Security enable option status bit"]
        #[inline(always)]
        pub const fn security(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Security enable option status bit"]
        #[inline(always)]
        pub fn set_security(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "User option bit 1"]
        #[inline(always)]
        pub const fn rss1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "User option bit 1"]
        #[inline(always)]
        pub fn set_rss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Device personalization status bit"]
        #[inline(always)]
        pub const fn perso_ok(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Device personalization status bit"]
        #[inline(always)]
        pub fn set_perso_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
        #[inline(always)]
        pub const fn io_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
        #[inline(always)]
        pub fn set_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Option byte change error flag"]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag"]
        #[inline(always)]
        pub fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bank swapping option status bit"]
        #[inline(always)]
        pub const fn swap_bank_opt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank swapping option status bit"]
        #[inline(always)]
        pub fn set_swap_bank_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OptsrCur {
        #[inline(always)]
        fn default() -> OptsrCur {
            OptsrCur(0)
        }
    }
    #[doc = "FLASH option status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OptsrPrg(pub u32);
    impl OptsrPrg {
        #[doc = "BOR reset level option configuration bits"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "BOR reset level option configuration bits"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "IWDG1 option configuration bit"]
        #[inline(always)]
        pub const fn iwdg1_hw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG1 option configuration bit"]
        #[inline(always)]
        pub fn set_iwdg1_hw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Option byte erase after D1 DStop option configuration bit"]
        #[inline(always)]
        pub const fn n_rst_stop_d1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase after D1 DStop option configuration bit"]
        #[inline(always)]
        pub fn set_n_rst_stop_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Option byte erase after D1 DStandby option configuration bit"]
        #[inline(always)]
        pub const fn n_rst_stby_d1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase after D1 DStandby option configuration bit"]
        #[inline(always)]
        pub fn set_n_rst_stby_d1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Readout protection level option configuration byte"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Readout protection level option configuration byte"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IWDG Stop mode freeze option configuration bit"]
        #[inline(always)]
        pub const fn fz_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG Stop mode freeze option configuration bit"]
        #[inline(always)]
        pub fn set_fz_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG Standby mode freeze option configuration bit"]
        #[inline(always)]
        pub const fn fz_iwdg_sdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG Standby mode freeze option configuration bit"]
        #[inline(always)]
        pub fn set_fz_iwdg_sdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DTCM size select option configuration bits"]
        #[inline(always)]
        pub const fn st_ram_size(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "DTCM size select option configuration bits"]
        #[inline(always)]
        pub fn set_st_ram_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Security option configuration bit"]
        #[inline(always)]
        pub const fn security(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Security option configuration bit"]
        #[inline(always)]
        pub fn set_security(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "User option configuration bit 1"]
        #[inline(always)]
        pub const fn rss1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "User option configuration bit 1"]
        #[inline(always)]
        pub fn set_rss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "User option configuration bit 2"]
        #[inline(always)]
        pub const fn rss2(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "User option configuration bit 2"]
        #[inline(always)]
        pub fn set_rss2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
        #[inline(always)]
        pub const fn io_hslv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
        #[inline(always)]
        pub fn set_io_hslv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Bank swapping option configuration bit"]
        #[inline(always)]
        pub const fn swap_bank_opt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank swapping option configuration bit"]
        #[inline(always)]
        pub fn set_swap_bank_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OptsrPrg {
        #[inline(always)]
        fn default() -> OptsrPrg {
            OptsrPrg(0)
        }
    }
    #[doc = "FLASH protection address for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrarCur(pub u32);
    impl PrarCur {
        #[doc = "Bank 1 lowest PCROP protected address"]
        #[inline(always)]
        pub const fn prot_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 lowest PCROP protected address"]
        #[inline(always)]
        pub fn set_prot_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Bank 1 highest PCROP protected address"]
        #[inline(always)]
        pub const fn prot_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 highest PCROP protected address"]
        #[inline(always)]
        pub fn set_prot_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Bank 1 PCROP protected erase enable option status bit"]
        #[inline(always)]
        pub const fn dmep(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 PCROP protected erase enable option status bit"]
        #[inline(always)]
        pub fn set_dmep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PrarCur {
        #[inline(always)]
        fn default() -> PrarCur {
            PrarCur(0)
        }
    }
    #[doc = "FLASH protection address for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrarPrg(pub u32);
    impl PrarPrg {
        #[doc = "Bank 1 lowest PCROP protected address configuration"]
        #[inline(always)]
        pub const fn prot_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 lowest PCROP protected address configuration"]
        #[inline(always)]
        pub fn set_prot_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Bank 1 highest PCROP protected address configuration"]
        #[inline(always)]
        pub const fn prot_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 highest PCROP protected address configuration"]
        #[inline(always)]
        pub fn set_prot_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Bank 1 PCROP protected erase enable option configuration bit"]
        #[inline(always)]
        pub const fn dmep(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 PCROP protected erase enable option configuration bit"]
        #[inline(always)]
        pub fn set_dmep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PrarPrg {
        #[inline(always)]
        fn default() -> PrarPrg {
            PrarPrg(0)
        }
    }
    #[doc = "FLASH secure address for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScarCur(pub u32);
    impl ScarCur {
        #[doc = "Bank 1 lowest secure protected address"]
        #[inline(always)]
        pub const fn sec_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 lowest secure protected address"]
        #[inline(always)]
        pub fn set_sec_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Bank 1 highest secure protected address"]
        #[inline(always)]
        pub const fn sec_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 highest secure protected address"]
        #[inline(always)]
        pub fn set_sec_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Bank 1 secure protected erase enable option status bit"]
        #[inline(always)]
        pub const fn dmes(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 secure protected erase enable option status bit"]
        #[inline(always)]
        pub fn set_dmes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ScarCur {
        #[inline(always)]
        fn default() -> ScarCur {
            ScarCur(0)
        }
    }
    #[doc = "FLASH secure address for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScarPrg(pub u32);
    impl ScarPrg {
        #[doc = "Bank 1 lowest secure protected address configuration"]
        #[inline(always)]
        pub const fn sec_area_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 lowest secure protected address configuration"]
        #[inline(always)]
        pub fn set_sec_area_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Bank 1 highest secure protected address configuration"]
        #[inline(always)]
        pub const fn sec_area_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Bank 1 highest secure protected address configuration"]
        #[inline(always)]
        pub fn set_sec_area_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Bank 1 secure protected erase enable option configuration bit"]
        #[inline(always)]
        pub const fn dmes(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 secure protected erase enable option configuration bit"]
        #[inline(always)]
        pub fn set_dmes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ScarPrg {
        #[inline(always)]
        fn default() -> ScarPrg {
            ScarPrg(0)
        }
    }
    #[doc = "FLASH status register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Bank 1 ongoing program flag"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 ongoing program flag"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bank 1 write buffer not empty flag"]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write buffer not empty flag"]
        #[inline(always)]
        pub fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank 1 wait queue flag"]
        #[inline(always)]
        pub const fn qw(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 wait queue flag"]
        #[inline(always)]
        pub fn set_qw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank 1 CRC busy flag"]
        #[inline(always)]
        pub const fn crc_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC busy flag"]
        #[inline(always)]
        pub fn set_crc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bank 1 end-of-program flag"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 end-of-program flag"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Bank 1 write protection error flag"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write protection error flag"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Bank 1 programming sequence error flag"]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 programming sequence error flag"]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bank 1 strobe error flag"]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 strobe error flag"]
        #[inline(always)]
        pub fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 inconsistency error flag"]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 inconsistency error flag"]
        #[inline(always)]
        pub fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Bank 1 write/erase error flag"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 write/erase error flag"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Bank 1 read protection error flag"]
        #[inline(always)]
        pub const fn rdperr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 read protection error flag"]
        #[inline(always)]
        pub fn set_rdperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Bank 1 secure error flag"]
        #[inline(always)]
        pub const fn rdserr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 secure error flag"]
        #[inline(always)]
        pub fn set_rdserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bank 1 single correction error flag"]
        #[inline(always)]
        pub const fn sneccerr1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 single correction error flag"]
        #[inline(always)]
        pub fn set_sneccerr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bank 1 ECC double detection error flag"]
        #[inline(always)]
        pub const fn dbeccerr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 ECC double detection error flag"]
        #[inline(always)]
        pub fn set_dbeccerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Bank 1 CRC-complete flag"]
        #[inline(always)]
        pub const fn crcend(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC-complete flag"]
        #[inline(always)]
        pub fn set_crcend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Bank 1 CRC read error flag"]
        #[inline(always)]
        pub const fn crcrderr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 CRC read error flag"]
        #[inline(always)]
        pub fn set_crcrderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "FLASH write sector protection for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WpsnCurr(pub u32);
    impl WpsnCurr {
        #[doc = "Bank 1 sector write protection option status byte"]
        #[inline(always)]
        pub const fn wrpsn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 sector write protection option status byte"]
        #[inline(always)]
        pub fn set_wrpsn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for WpsnCurr {
        #[inline(always)]
        fn default() -> WpsnCurr {
            WpsnCurr(0)
        }
    }
    #[doc = "FLASH write sector protection for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WpsnPrgr(pub u32);
    impl WpsnPrgr {
        #[doc = "Bank 1 sector write protection configuration byte"]
        #[inline(always)]
        pub const fn wrpsn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 sector write protection configuration byte"]
        #[inline(always)]
        pub fn set_wrpsn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for WpsnPrgr {
        #[inline(always)]
        fn default() -> WpsnPrgr {
            WpsnPrgr(0)
        }
    }
}
