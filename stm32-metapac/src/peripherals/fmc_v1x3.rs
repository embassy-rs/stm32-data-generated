#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flexible memory controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc {
    ptr: *mut u8,
}
unsafe impl Send for Fmc {}
unsafe impl Sync for Fmc {}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bcr1(self) -> crate::common::Reg<regs::Bcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register 1-4"]
    #[inline(always)]
    pub const fn btr(self, n: usize) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 8usize) as _) }
    }
    #[doc = "PC Card/NAND Flash control register 2-4"]
    #[inline(always)]
    pub const fn pcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 32usize) as _) }
    }
    #[doc = "FIFO status and interrupt register 2-4"]
    #[inline(always)]
    pub const fn sr(self, n: usize) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize + n * 32usize) as _) }
    }
    #[doc = "Common memory space timing register 2-4"]
    #[inline(always)]
    pub const fn pmem(self, n: usize) -> crate::common::Reg<regs::Pmem, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize + n * 32usize) as _) }
    }
    #[doc = "Attribute memory space timing register 2-4"]
    #[inline(always)]
    pub const fn patt(self, n: usize) -> crate::common::Reg<regs::Patt, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize + n * 32usize) as _) }
    }
    #[doc = "ECC result register 2-3"]
    #[inline(always)]
    pub const fn eccr(self, n: usize) -> crate::common::Reg<regs::Eccr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize + n * 32usize) as _) }
    }
    #[doc = "I/O space timing register 4"]
    #[inline(always)]
    pub const fn pio4(self) -> crate::common::Reg<regs::Pio4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1-4"]
    #[inline(always)]
    pub const fn bwtr(self, n: usize) -> crate::common::Reg<regs::Bwtr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 8usize) as _) }
    }
    #[doc = "SDRAM Control Register 1-2"]
    #[inline(always)]
    pub const fn sdcr(self, n: usize) -> crate::common::Reg<regs::Sdcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM Timing register 1-2"]
    #[inline(always)]
    pub const fn sdtr(self, n: usize) -> crate::common::Reg<regs::Sdtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize + n * 4usize) as _) }
    }
    #[doc = "SDRAM Command Mode register"]
    #[inline(always)]
    pub const fn sdcmr(self) -> crate::common::Reg<regs::Sdcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "SDRAM Refresh Timer register"]
    #[inline(always)]
    pub const fn sdrtr(self) -> crate::common::Reg<regs::Sdrtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "SDRAM Status register"]
    #[inline(always)]
    pub const fn sdsr(self) -> crate::common::Reg<regs::Sdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
}
pub mod regs {
    #[doc = "SRAM/NOR-Flash chip-select control register 2-4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub const fn wrapmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub fn set_wrapmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub const fn cburstrw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr1(pub u32);
    impl Bcr1 {
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit"]
        #[inline(always)]
        pub fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit"]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable"]
        #[inline(always)]
        pub fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit"]
        #[inline(always)]
        pub fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit"]
        #[inline(always)]
        pub fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub const fn wrapmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "WRAPMOD"]
        #[inline(always)]
        pub fn set_wrapmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration"]
        #[inline(always)]
        pub fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit"]
        #[inline(always)]
        pub fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit"]
        #[inline(always)]
        pub fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable"]
        #[inline(always)]
        pub fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers"]
        #[inline(always)]
        pub fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size"]
        #[inline(always)]
        pub fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub const fn cburstrw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Write burst enable"]
        #[inline(always)]
        pub fn set_cburstrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Continuous clock enable"]
        #[inline(always)]
        pub const fn cclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous clock enable"]
        #[inline(always)]
        pub fn set_cclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Bcr1 {
        #[inline(always)]
        fn default() -> Bcr1 {
            Bcr1(0)
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr(pub u32);
    impl Btr {
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)"]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)"]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Data latency for synchronous memory"]
        #[inline(always)]
        pub const fn datlat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Data latency for synchronous memory"]
        #[inline(always)]
        pub fn set_datlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Btr {
        #[inline(always)]
        fn default() -> Btr {
            Btr(0)
        }
    }
    #[doc = "SRAM/NOR-Flash write timing registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bwtr(pub u32);
    impl Bwtr {
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration"]
        #[inline(always)]
        pub fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration"]
        #[inline(always)]
        pub fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration"]
        #[inline(always)]
        pub fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration"]
        #[inline(always)]
        pub fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode"]
        #[inline(always)]
        pub fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Bwtr {
        #[inline(always)]
        fn default() -> Bwtr {
            Bwtr(0)
        }
    }
    #[doc = "ECC result register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC computation result value"]
        #[inline(always)]
        pub const fn ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC computation result value"]
        #[inline(always)]
        pub fn set_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    #[doc = "Attribute memory space timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Patt(pub u32);
    impl Patt {
        #[doc = "Attribute memory setup time"]
        #[inline(always)]
        pub const fn attset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory setup time"]
        #[inline(always)]
        pub fn set_attset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Attribute memory wait time"]
        #[inline(always)]
        pub const fn attwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory wait time"]
        #[inline(always)]
        pub fn set_attwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Attribute memory hold time"]
        #[inline(always)]
        pub const fn atthold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory hold time"]
        #[inline(always)]
        pub fn set_atthold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Attribute memory data bus Hi-Z time"]
        #[inline(always)]
        pub const fn atthiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory data bus Hi-Z time"]
        #[inline(always)]
        pub fn set_atthiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Patt {
        #[inline(always)]
        fn default() -> Patt {
            Patt(0)
        }
    }
    #[doc = "PC Card/NAND Flash control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Wait feature enable bit"]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable bit"]
        #[inline(always)]
        pub fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NAND Flash memory bank enable bit"]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash memory bank enable bit"]
        #[inline(always)]
        pub fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub const fn ptyp(&self) -> super::vals::Ptyp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ptyp::from_bits(val as u8)
        }
        #[doc = "Memory type"]
        #[inline(always)]
        pub fn set_ptyp(&mut self, val: super::vals::Ptyp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Data bus width"]
        #[inline(always)]
        pub const fn pwid(&self) -> super::vals::Pwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Pwid::from_bits(val as u8)
        }
        #[doc = "Data bus width"]
        #[inline(always)]
        pub fn set_pwid(&mut self, val: super::vals::Pwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "ECC computation logic enable bit"]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECC computation logic enable bit"]
        #[inline(always)]
        pub fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CLE to RE delay"]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE delay"]
        #[inline(always)]
        pub fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "ALE to RE delay"]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "ALE to RE delay"]
        #[inline(always)]
        pub fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "ECC page size"]
        #[inline(always)]
        pub const fn eccps(&self) -> super::vals::Eccps {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Eccps::from_bits(val as u8)
        }
        #[doc = "ECC page size"]
        #[inline(always)]
        pub fn set_eccps(&mut self, val: super::vals::Eccps) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    #[doc = "I/O space timing register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pio4(pub u32);
    impl Pio4 {
        #[doc = "IOSETx"]
        #[inline(always)]
        pub const fn iosetx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "IOSETx"]
        #[inline(always)]
        pub fn set_iosetx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "IOWAITx"]
        #[inline(always)]
        pub const fn iowaitx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "IOWAITx"]
        #[inline(always)]
        pub fn set_iowaitx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IOHOLDx"]
        #[inline(always)]
        pub const fn ioholdx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "IOHOLDx"]
        #[inline(always)]
        pub fn set_ioholdx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "IOHIZx"]
        #[inline(always)]
        pub const fn iohizx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IOHIZx"]
        #[inline(always)]
        pub fn set_iohizx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pio4 {
        #[inline(always)]
        fn default() -> Pio4 {
            Pio4(0)
        }
    }
    #[doc = "Common memory space timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmem(pub u32);
    impl Pmem {
        #[doc = "Common memory x setup time"]
        #[inline(always)]
        pub const fn memset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x setup time"]
        #[inline(always)]
        pub fn set_memset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Common memory wait time"]
        #[inline(always)]
        pub const fn memwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory wait time"]
        #[inline(always)]
        pub fn set_memwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Common memory hold time"]
        #[inline(always)]
        pub const fn memhold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory hold time"]
        #[inline(always)]
        pub fn set_memhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Common memory x data bus Hi-Z time"]
        #[inline(always)]
        pub const fn memhiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x data bus Hi-Z time"]
        #[inline(always)]
        pub fn set_memhiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pmem {
        #[inline(always)]
        fn default() -> Pmem {
            Pmem(0)
        }
    }
    #[doc = "SDRAM Command Mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcmr(pub u32);
    impl Sdcmr {
        #[doc = "Command mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Command mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Command target bank 2"]
        #[inline(always)]
        pub const fn ctb2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Command target bank 2"]
        #[inline(always)]
        pub fn set_ctb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command target bank 1"]
        #[inline(always)]
        pub const fn ctb1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command target bank 1"]
        #[inline(always)]
        pub fn set_ctb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Number of Auto-refresh"]
        #[inline(always)]
        pub const fn nrfs(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Auto-refresh"]
        #[inline(always)]
        pub fn set_nrfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Mode Register definition"]
        #[inline(always)]
        pub const fn mrd(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x1fff;
            val as u16
        }
        #[doc = "Mode Register definition"]
        #[inline(always)]
        pub fn set_mrd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 9usize)) | (((val as u32) & 0x1fff) << 9usize);
        }
    }
    impl Default for Sdcmr {
        #[inline(always)]
        fn default() -> Sdcmr {
            Sdcmr(0)
        }
    }
    #[doc = "SDRAM Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdcr(pub u32);
    impl Sdcr {
        #[doc = "Number of column address bits"]
        #[inline(always)]
        pub const fn nc(&self) -> super::vals::Nc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Nc::from_bits(val as u8)
        }
        #[doc = "Number of column address bits"]
        #[inline(always)]
        pub fn set_nc(&mut self, val: super::vals::Nc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of row address bits"]
        #[inline(always)]
        pub const fn nr(&self) -> super::vals::Nr {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Nr::from_bits(val as u8)
        }
        #[doc = "Number of row address bits"]
        #[inline(always)]
        pub fn set_nr(&mut self, val: super::vals::Nr) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width"]
        #[inline(always)]
        pub fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of internal banks"]
        #[inline(always)]
        pub const fn nb(&self) -> super::vals::Nb {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Nb::from_bits(val as u8)
        }
        #[doc = "Number of internal banks"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: super::vals::Nb) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "CAS latency"]
        #[inline(always)]
        pub const fn cas(&self) -> super::vals::Cas {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Cas::from_bits(val as u8)
        }
        #[doc = "CAS latency"]
        #[inline(always)]
        pub fn set_cas(&mut self, val: super::vals::Cas) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SDRAM clock configuration"]
        #[inline(always)]
        pub const fn sdclk(&self) -> super::vals::Sdclk {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Sdclk::from_bits(val as u8)
        }
        #[doc = "SDRAM clock configuration"]
        #[inline(always)]
        pub fn set_sdclk(&mut self, val: super::vals::Sdclk) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Burst read"]
        #[inline(always)]
        pub const fn rburst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Burst read"]
        #[inline(always)]
        pub fn set_rburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Read pipe"]
        #[inline(always)]
        pub const fn rpipe(&self) -> super::vals::Rpipe {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Rpipe::from_bits(val as u8)
        }
        #[doc = "Read pipe"]
        #[inline(always)]
        pub fn set_rpipe(&mut self, val: super::vals::Rpipe) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
    }
    impl Default for Sdcr {
        #[inline(always)]
        fn default() -> Sdcr {
            Sdcr(0)
        }
    }
    #[doc = "SDRAM Refresh Timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrtr(pub u32);
    impl Sdrtr {
        #[doc = "Clear Refresh error flag"]
        #[inline(always)]
        pub const fn cre(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Refresh error flag"]
        #[inline(always)]
        pub fn set_cre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh Timer Count"]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x1fff;
            val as u16
        }
        #[doc = "Refresh Timer Count"]
        #[inline(always)]
        pub fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
        }
        #[doc = "RES Interrupt Enable"]
        #[inline(always)]
        pub const fn reie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RES Interrupt Enable"]
        #[inline(always)]
        pub fn set_reie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sdrtr {
        #[inline(always)]
        fn default() -> Sdrtr {
            Sdrtr(0)
        }
    }
    #[doc = "SDRAM Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdsr(pub u32);
    impl Sdsr {
        #[doc = "Refresh error flag"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh error flag"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Status Mode for Bank 1"]
        #[inline(always)]
        pub const fn modes1(&self) -> super::vals::Modes {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Modes::from_bits(val as u8)
        }
        #[doc = "Status Mode for Bank 1"]
        #[inline(always)]
        pub fn set_modes1(&mut self, val: super::vals::Modes) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Status Mode for Bank 2"]
        #[inline(always)]
        pub const fn modes2(&self) -> super::vals::Modes {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Modes::from_bits(val as u8)
        }
        #[doc = "Status Mode for Bank 2"]
        #[inline(always)]
        pub fn set_modes2(&mut self, val: super::vals::Modes) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Busy status"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Busy status"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sdsr {
        #[inline(always)]
        fn default() -> Sdsr {
            Sdsr(0)
        }
    }
    #[doc = "SDRAM Timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdtr(pub u32);
    impl Sdtr {
        #[doc = "Load Mode Register to Active"]
        #[inline(always)]
        pub const fn tmrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Load Mode Register to Active"]
        #[inline(always)]
        pub fn set_tmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Exit self-refresh delay"]
        #[inline(always)]
        pub const fn txsr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Exit self-refresh delay"]
        #[inline(always)]
        pub fn set_txsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Self refresh time"]
        #[inline(always)]
        pub const fn tras(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Self refresh time"]
        #[inline(always)]
        pub fn set_tras(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Row cycle delay"]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Row cycle delay"]
        #[inline(always)]
        pub fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Recovery delay"]
        #[inline(always)]
        pub const fn twr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Recovery delay"]
        #[inline(always)]
        pub fn set_twr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Row precharge delay"]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Row precharge delay"]
        #[inline(always)]
        pub fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Row to column delay"]
        #[inline(always)]
        pub const fn trcd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Row to column delay"]
        #[inline(always)]
        pub fn set_trcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Sdtr {
        #[inline(always)]
        fn default() -> Sdtr {
            Sdtr(0)
        }
    }
    #[doc = "FIFO status and interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Interrupt rising edge status"]
        #[inline(always)]
        pub const fn irs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge status"]
        #[inline(always)]
        pub fn set_irs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt high-level status"]
        #[inline(always)]
        pub const fn ils(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level status"]
        #[inline(always)]
        pub fn set_ils(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt falling edge status"]
        #[inline(always)]
        pub const fn ifs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge status"]
        #[inline(always)]
        pub fn set_ifs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt rising edge detection enable bit"]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt rising edge detection enable bit"]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt high-level detection enable bit"]
        #[inline(always)]
        pub const fn ilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt high-level detection enable bit"]
        #[inline(always)]
        pub fn set_ilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt falling edge detection enable bit"]
        #[inline(always)]
        pub const fn ifen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt falling edge detection enable bit"]
        #[inline(always)]
        pub fn set_ifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO empty status"]
        #[inline(always)]
        pub const fn fempt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty status"]
        #[inline(always)]
        pub fn set_fempt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Accmod {
        #[doc = "Access mode A"]
        A = 0x0,
        #[doc = "Access mode B"]
        B = 0x01,
        #[doc = "Access mode C"]
        C = 0x02,
        #[doc = "Access mode D"]
        D = 0x03,
    }
    impl Accmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Accmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Accmod {
        #[inline(always)]
        fn from(val: u8) -> Accmod {
            Accmod::from_bits(val)
        }
    }
    impl From<Accmod> for u8 {
        #[inline(always)]
        fn from(val: Accmod) -> u8 {
            Accmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cas {
        _RESERVED_0 = 0x0,
        #[doc = "1 cycle"]
        CLOCKS1 = 0x01,
        #[doc = "2 cycles"]
        CLOCKS2 = 0x02,
        #[doc = "3 cycles"]
        CLOCKS3 = 0x03,
    }
    impl Cas {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cas {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cas {
        #[inline(always)]
        fn from(val: u8) -> Cas {
            Cas::from_bits(val)
        }
    }
    impl From<Cas> for u8 {
        #[inline(always)]
        fn from(val: Cas) -> u8 {
            Cas::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cpsize {
        #[doc = "No burst split when crossing page boundary"]
        NOBURSTSPLIT = 0x0,
        #[doc = "128 bytes CRAM page size"]
        BYTES128 = 0x01,
        #[doc = "256 bytes CRAM page size"]
        BYTES256 = 0x02,
        #[doc = "512 bytes CRAM page size"]
        BYTES512 = 0x03,
        #[doc = "1024 bytes CRAM page size"]
        BYTES1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cpsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpsize {
        #[inline(always)]
        fn from(val: u8) -> Cpsize {
            Cpsize::from_bits(val)
        }
    }
    impl From<Cpsize> for u8 {
        #[inline(always)]
        fn from(val: Cpsize) -> u8 {
            Cpsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eccps {
        #[doc = "ECC page size 256 bytes"]
        BYTES256 = 0x0,
        #[doc = "ECC page size 512 bytes"]
        BYTES512 = 0x01,
        #[doc = "ECC page size 1024 bytes"]
        BYTES1024 = 0x02,
        #[doc = "ECC page size 2048 bytes"]
        BYTES2048 = 0x03,
        #[doc = "ECC page size 4096 bytes"]
        BYTES4096 = 0x04,
        #[doc = "ECC page size 8192 bytes"]
        BYTES8192 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Eccps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccps {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccps {
        #[inline(always)]
        fn from(val: u8) -> Eccps {
            Eccps::from_bits(val)
        }
    }
    impl From<Eccps> for u8 {
        #[inline(always)]
        fn from(val: Eccps) -> u8 {
            Eccps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mode {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Clock Configuration Enable"]
        CLOCKCONFIGURATIONENABLE = 0x01,
        #[doc = "PALL (All Bank Precharge) command"]
        PALL = 0x02,
        #[doc = "Auto-refresh command"]
        AUTOREFRESHCOMMAND = 0x03,
        #[doc = "Load Mode Resgier"]
        LOADMODEREGISTER = 0x04,
        #[doc = "Self-refresh command"]
        SELFREFRESHCOMMAND = 0x05,
        #[doc = "Power-down command"]
        POWERDOWNCOMMAND = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Modes {
        #[doc = "Normal Mode"]
        NORMAL = 0x0,
        #[doc = "Self-refresh mode"]
        SELFREFRESH = 0x01,
        #[doc = "Power-down mode"]
        POWERDOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Modes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Modes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Modes {
        #[inline(always)]
        fn from(val: u8) -> Modes {
            Modes::from_bits(val)
        }
    }
    impl From<Modes> for u8 {
        #[inline(always)]
        fn from(val: Modes) -> u8 {
            Modes::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mtyp {
        #[doc = "SRAM memory type"]
        SRAM = 0x0,
        #[doc = "PSRAM (CRAM) memory type"]
        PSRAM = 0x01,
        #[doc = "NOR Flash/OneNAND Flash"]
        FLASH = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mtyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtyp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtyp {
        #[inline(always)]
        fn from(val: u8) -> Mtyp {
            Mtyp::from_bits(val)
        }
    }
    impl From<Mtyp> for u8 {
        #[inline(always)]
        fn from(val: Mtyp) -> u8 {
            Mtyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mwid {
        #[doc = "Memory data bus width 8 bits"]
        BITS8 = 0x0,
        #[doc = "Memory data bus width 16 bits"]
        BITS16 = 0x01,
        #[doc = "Memory data bus width 32 bits"]
        BITS32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mwid {
        #[inline(always)]
        fn from(val: u8) -> Mwid {
            Mwid::from_bits(val)
        }
    }
    impl From<Mwid> for u8 {
        #[inline(always)]
        fn from(val: Mwid) -> u8 {
            Mwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nb {
        #[doc = "Two internal Banks"]
        NB2 = 0x0,
        #[doc = "Four internal Banks"]
        NB4 = 0x01,
    }
    impl Nb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nb {
        #[inline(always)]
        fn from(val: u8) -> Nb {
            Nb::from_bits(val)
        }
    }
    impl From<Nb> for u8 {
        #[inline(always)]
        fn from(val: Nb) -> u8 {
            Nb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nc {
        #[doc = "8 bits"]
        BITS8 = 0x0,
        #[doc = "9 bits"]
        BITS9 = 0x01,
        #[doc = "10 bits"]
        BITS10 = 0x02,
        #[doc = "11 bits"]
        BITS11 = 0x03,
    }
    impl Nc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nc {
        #[inline(always)]
        fn from(val: u8) -> Nc {
            Nc::from_bits(val)
        }
    }
    impl From<Nc> for u8 {
        #[inline(always)]
        fn from(val: Nc) -> u8 {
            Nc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nr {
        #[doc = "11 bits"]
        BITS11 = 0x0,
        #[doc = "12 bits"]
        BITS12 = 0x01,
        #[doc = "13 bits"]
        BITS13 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Nr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nr {
        #[inline(always)]
        fn from(val: u8) -> Nr {
            Nr::from_bits(val)
        }
    }
    impl From<Nr> for u8 {
        #[inline(always)]
        fn from(val: Nr) -> u8 {
            Nr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ptyp {
        _RESERVED_0 = 0x0,
        #[doc = "NAND Flash"]
        NANDFLASH = 0x01,
    }
    impl Ptyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ptyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ptyp {
        #[inline(always)]
        fn from(val: u8) -> Ptyp {
            Ptyp::from_bits(val)
        }
    }
    impl From<Ptyp> for u8 {
        #[inline(always)]
        fn from(val: Ptyp) -> u8 {
            Ptyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pwid {
        #[doc = "External memory device width 8 bits"]
        BITS8 = 0x0,
        #[doc = "External memory device width 16 bits"]
        BITS16 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwid {
        #[inline(always)]
        fn from(val: u8) -> Pwid {
            Pwid::from_bits(val)
        }
    }
    impl From<Pwid> for u8 {
        #[inline(always)]
        fn from(val: Pwid) -> u8 {
            Pwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rpipe {
        #[doc = "No clock cycle delay"]
        NODELAY = 0x0,
        #[doc = "One clock cycle delay"]
        CLOCKS1 = 0x01,
        #[doc = "Two clock cycles delay"]
        CLOCKS2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Rpipe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rpipe {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rpipe {
        #[inline(always)]
        fn from(val: u8) -> Rpipe {
            Rpipe::from_bits(val)
        }
    }
    impl From<Rpipe> for u8 {
        #[inline(always)]
        fn from(val: Rpipe) -> u8 {
            Rpipe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdclk {
        #[doc = "SDCLK clock disabled"]
        DISABLED = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "SDCLK period = 2 x HCLK period"]
        DIV2 = 0x02,
        #[doc = "SDCLK period = 3 x HCLK period"]
        DIV3 = 0x03,
    }
    impl Sdclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdclk {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdclk {
        #[inline(always)]
        fn from(val: u8) -> Sdclk {
            Sdclk::from_bits(val)
        }
    }
    impl From<Sdclk> for u8 {
        #[inline(always)]
        fn from(val: Sdclk) -> u8 {
            Sdclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Waitcfg {
        #[doc = "NWAIT signal is active one data cycle before wait state"]
        BEFOREWAITSTATE = 0x0,
        #[doc = "NWAIT signal is active during wait state"]
        DURINGWAITSTATE = 0x01,
    }
    impl Waitcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitcfg {
        #[inline(always)]
        fn from(val: u8) -> Waitcfg {
            Waitcfg::from_bits(val)
        }
    }
    impl From<Waitcfg> for u8 {
        #[inline(always)]
        fn from(val: Waitcfg) -> u8 {
            Waitcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Waitpol {
        #[doc = "NWAIT active low"]
        ACTIVELOW = 0x0,
        #[doc = "NWAIT active high"]
        ACTIVEHIGH = 0x01,
    }
    impl Waitpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitpol {
        #[inline(always)]
        fn from(val: u8) -> Waitpol {
            Waitpol::from_bits(val)
        }
    }
    impl From<Waitpol> for u8 {
        #[inline(always)]
        fn from(val: Waitpol) -> u8 {
            Waitpol::to_bits(val)
        }
    }
}
