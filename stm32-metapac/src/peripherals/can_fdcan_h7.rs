#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Controller area network with flexible data rate (FD)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcan {
    ptr: *mut u8,
}
unsafe impl Send for Fdcan {}
unsafe impl Sync for Fdcan {}
impl Fdcan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<regs::Crel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::Endn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FDCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FDCAN Test Register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FDCAN RAM Watchdog Register"]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "FDCAN CC Control Register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FDCAN Interrupt Register"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FDCAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Enable Register"]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FDCAN Global Filter Configuration Register"]
    #[inline(always)]
    pub const fn gfc(self) -> crate::common::Reg<regs::Gfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FDCAN Standard ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn sidfc(self) -> crate::common::Reg<regs::Sidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FDCAN Extended ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn xidfc(self) -> crate::common::Reg<regs::Xidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "FDCAN New Data 1 Register"]
    #[inline(always)]
    pub const fn ndat1(self) -> crate::common::Reg<regs::Ndat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "FDCAN New Data 2 Register"]
    #[inline(always)]
    pub const fn ndat2(self) -> crate::common::Reg<regs::Ndat2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "FDCAN Rx FIFO X Configuration Register"]
    #[inline(always)]
    pub const fn rxfc(self, n: usize) -> crate::common::Reg<regs::Rxfc, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[inline(always)]
    pub const fn rxfs(self, n: usize) -> crate::common::Reg<regs::Rxfs, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "CAN Rx FIFO X Acknowledge Register"]
    #[inline(always)]
    pub const fn rxfa(self, n: usize) -> crate::common::Reg<regs::Rxfa, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize + ([0usize, 16usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Rx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn rxbc(self) -> crate::common::Reg<regs::Rxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn rxesc(self) -> crate::common::Reg<regs::Rxesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn txesc(self) -> crate::common::Reg<regs::Txesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Configuration Register"]
    #[inline(always)]
    pub const fn txefc(self) -> crate::common::Reg<regs::Txefc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn tttmc(self) -> crate::common::Reg<regs::Tttmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "FDCAN TT Reference Message Configuration Register"]
    #[inline(always)]
    pub const fn ttrmc(self) -> crate::common::Reg<regs::Ttrmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "FDCAN TT Operation Configuration Register"]
    #[inline(always)]
    pub const fn ttocf(self) -> crate::common::Reg<regs::Ttocf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "FDCAN TT Matrix Limits Register"]
    #[inline(always)]
    pub const fn ttmlm(self) -> crate::common::Reg<regs::Ttmlm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "FDCAN TUR Configuration Register"]
    #[inline(always)]
    pub const fn turcf(self) -> crate::common::Reg<regs::Turcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "FDCAN TT Operation Control Register"]
    #[inline(always)]
    pub const fn ttocn(self) -> crate::common::Reg<regs::Ttocn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "FDCAN TT Global Time Preset Register"]
    #[inline(always)]
    pub const fn ttgtp(self) -> crate::common::Reg<regs::Ttgtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "FDCAN TT Time Mark Register"]
    #[inline(always)]
    pub const fn tttmk(self) -> crate::common::Reg<regs::Tttmk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Register"]
    #[inline(always)]
    pub const fn ttir(self) -> crate::common::Reg<regs::Ttir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ttie(self) -> crate::common::Reg<regs::Ttie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "FDCAN TT Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn ttils(self) -> crate::common::Reg<regs::Ttils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "FDCAN TT Operation Status Register"]
    #[inline(always)]
    pub const fn ttost(self) -> crate::common::Reg<regs::Ttost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "FDCAN TUR Numerator Actual Register"]
    #[inline(always)]
    pub const fn turna(self) -> crate::common::Reg<regs::Turna, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "FDCAN TT Local and Global Time Register"]
    #[inline(always)]
    pub const fn ttlgt(self) -> crate::common::Reg<regs::Ttlgt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "FDCAN TT Cycle Time and Count Register"]
    #[inline(always)]
    pub const fn ttctc(self) -> crate::common::Reg<regs::Ttctc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "FDCAN TT Capture Time Register"]
    #[inline(always)]
    pub const fn ttcpt(self) -> crate::common::Reg<regs::Ttcpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "FDCAN TT Cycle Sync Mark Register"]
    #[inline(always)]
    pub const fn ttcsm(self) -> crate::common::Reg<regs::Ttcsm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "FDCAN TT Trigger Select Register"]
    #[inline(always)]
    pub const fn ttts(self) -> crate::common::Reg<regs::Ttts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
}
pub mod regs {
    #[doc = "FDCAN CC Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "Initialization"]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization"]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configuration Change Enable"]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Change Enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ASM Restricted Operation Mode"]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ASM Restricted Operation Mode"]
        #[inline(always)]
        pub fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock Stop Acknowledge"]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Acknowledge"]
        #[inline(always)]
        pub fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock Stop Request"]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Request"]
        #[inline(always)]
        pub fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bus Monitoring Mode"]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Monitoring Mode"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable Automatic Retransmission"]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Automatic Retransmission"]
        #[inline(always)]
        pub fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Test Mode Enable"]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Test Mode Enable"]
        #[inline(always)]
        pub fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FD Operation Enable"]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD Operation Enable"]
        #[inline(always)]
        pub fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FDCAN Bit Rate Switching"]
        #[inline(always)]
        pub const fn bse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Bit Rate Switching"]
        #[inline(always)]
        pub fn set_bse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Protocol Exception Handling Disable"]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Handling Disable"]
        #[inline(always)]
        pub fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Edge Filtering during Bus Integration"]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Edge Filtering during Bus Integration"]
        #[inline(always)]
        pub fn set_efbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TXP"]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TXP"]
        #[inline(always)]
        pub fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non ISO Operation"]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non ISO Operation"]
        #[inline(always)]
        pub fn set_niso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    #[doc = "FDCAN Core Release Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crel(pub u32);
    impl Crel {
        #[doc = "Timestamp Day"]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Day"]
        #[inline(always)]
        pub fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timestamp Month"]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Month"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Timestamp Year"]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Year"]
        #[inline(always)]
        pub fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Sub-step of Core release"]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Sub-step of Core release"]
        #[inline(always)]
        pub fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Step of Core release"]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Step of Core release"]
        #[inline(always)]
        pub fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Core release"]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Core release"]
        #[inline(always)]
        pub fn set_rel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Crel {
        #[inline(always)]
        fn default() -> Crel {
            Crel(0)
        }
    }
    #[doc = "FDCAN Data Bit Timing and Prescaler Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbtp(pub u32);
    impl Dbtp {
        #[doc = "Synchronization Jump Width"]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Synchronization Jump Width"]
        #[inline(always)]
        pub fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data time segment after sample point"]
        #[inline(always)]
        pub fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data BIt Rate Prescaler"]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data BIt Rate Prescaler"]
        #[inline(always)]
        pub fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Transceiver Delay Compensation"]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transceiver Delay Compensation"]
        #[inline(always)]
        pub fn set_tdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Dbtp {
        #[inline(always)]
        fn default() -> Dbtp {
            Dbtp(0)
        }
    }
    #[doc = "FDCAN Error Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Transmit Error Counter"]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit Error Counter"]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive Error Counter"]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Receive Error Counter"]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Receive Error Passive"]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Error Passive"]
        #[inline(always)]
        pub fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AN Error Logging"]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "AN Error Logging"]
        #[inline(always)]
        pub fn set_cel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    #[doc = "FDCAN Core Release Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endn(pub u32);
    impl Endn {
        #[doc = "Endiannes Test Value"]
        #[inline(always)]
        pub const fn etv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endiannes Test Value"]
        #[inline(always)]
        pub fn set_etv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Endn {
        #[inline(always)]
        fn default() -> Endn {
            Endn(0)
        }
    }
    #[doc = "FDCAN Global Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gfc(pub u32);
    impl Gfc {
        #[doc = "Reject Remote Frames Extended"]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Extended"]
        #[inline(always)]
        pub fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reject Remote Frames Standard"]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Standard"]
        #[inline(always)]
        pub fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Accept Non-matching Frames Extended"]
        #[inline(always)]
        pub const fn anfe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Extended"]
        #[inline(always)]
        pub fn set_anfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Accept Non-matching Frames Standard"]
        #[inline(always)]
        pub const fn anfs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Standard"]
        #[inline(always)]
        pub fn set_anfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Gfc {
        #[inline(always)]
        fn default() -> Gfc {
            Gfc(0)
        }
    }
    #[doc = "FDCAN High Priority Message Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpms(pub u32);
    impl Hpms {
        #[doc = "Buffer Index"]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Buffer Index"]
        #[inline(always)]
        pub fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Message Storage Indicator"]
        #[inline(always)]
        pub const fn msi(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Message Storage Indicator"]
        #[inline(always)]
        pub fn set_msi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Filter Index"]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Filter Index"]
        #[inline(always)]
        pub fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Filter List"]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter List"]
        #[inline(always)]
        pub fn set_flst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Hpms {
        #[inline(always)]
        fn default() -> Hpms {
            Hpms(0)
        }
    }
    #[doc = "FDCAN Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ie(pub u32);
    impl Ie {
        #[doc = "Rx FIFO X New Message Enable"]
        #[inline(always)]
        pub const fn rfne(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message Enable"]
        #[inline(always)]
        pub fn set_rfne(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached Enable"]
        #[inline(always)]
        pub const fn rfwe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached Enable"]
        #[inline(always)]
        pub fn set_rfwe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full Enable"]
        #[inline(always)]
        pub const fn rffe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full Enable"]
        #[inline(always)]
        pub fn set_rffe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost Enable"]
        #[inline(always)]
        pub const fn rfle(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost Enable"]
        #[inline(always)]
        pub fn set_rfle(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message Enable"]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Enable"]
        #[inline(always)]
        pub fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Enable"]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Enable"]
        #[inline(always)]
        pub fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Enable"]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Enable"]
        #[inline(always)]
        pub fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Enable"]
        #[inline(always)]
        pub const fn tefe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Enable"]
        #[inline(always)]
        pub fn set_tefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Enable"]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Enable"]
        #[inline(always)]
        pub fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Enable"]
        #[inline(always)]
        pub const fn tefwe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Enable"]
        #[inline(always)]
        pub fn set_tefwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Enable"]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Enable"]
        #[inline(always)]
        pub fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost Enable"]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost Enable"]
        #[inline(always)]
        pub fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Enable"]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Enable"]
        #[inline(always)]
        pub fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Enable"]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Enable"]
        #[inline(always)]
        pub fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Enable"]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Enable"]
        #[inline(always)]
        pub fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Enable"]
        #[inline(always)]
        pub const fn drxe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Enable"]
        #[inline(always)]
        pub fn set_drxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Enable"]
        #[inline(always)]
        pub const fn bece(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Enable"]
        #[inline(always)]
        pub fn set_bece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable"]
        #[inline(always)]
        pub const fn beue(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable"]
        #[inline(always)]
        pub fn set_beue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Enable"]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Enable"]
        #[inline(always)]
        pub fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Enable"]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Enable"]
        #[inline(always)]
        pub fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Enable"]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Enable"]
        #[inline(always)]
        pub fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status Enable"]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status Enable"]
        #[inline(always)]
        pub fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Enable"]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Enable"]
        #[inline(always)]
        pub fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Enable"]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Enable"]
        #[inline(always)]
        pub fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Enable"]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Enable"]
        #[inline(always)]
        pub fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Enable"]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Enable"]
        #[inline(always)]
        pub fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ie {
        #[inline(always)]
        fn default() -> Ie {
            Ie(0)
        }
    }
    #[doc = "FDCAN Interrupt Line Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ile(pub u32);
    impl Ile {
        #[doc = "Enable Interrupt Line 0"]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 0"]
        #[inline(always)]
        pub fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Interrupt Line 1"]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 1"]
        #[inline(always)]
        pub fn set_eint1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ile {
        #[inline(always)]
        fn default() -> Ile {
            Ile(0)
        }
    }
    #[doc = "FDCAN Interrupt Line Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ils(pub u32);
    impl Ils {
        #[doc = "Rx FIFO X New Message Interrupt Line"]
        #[inline(always)]
        pub const fn rfnl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message Interrupt Line"]
        #[inline(always)]
        pub fn set_rfnl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub const fn rfwl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub fn set_rfwl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full Interrupt Line"]
        #[inline(always)]
        pub const fn rffl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full Interrupt Line"]
        #[inline(always)]
        pub fn set_rffl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost Interrupt Line"]
        #[inline(always)]
        pub const fn rfll(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost Interrupt Line"]
        #[inline(always)]
        pub fn set_rfll(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message Interrupt Line"]
        #[inline(always)]
        pub const fn hpml(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Interrupt Line"]
        #[inline(always)]
        pub fn set_hpml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Interrupt Line"]
        #[inline(always)]
        pub const fn tcl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Interrupt Line"]
        #[inline(always)]
        pub fn set_tcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line"]
        #[inline(always)]
        pub const fn tcfl(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line"]
        #[inline(always)]
        pub fn set_tcfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Interrupt Line"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Interrupt Line"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line"]
        #[inline(always)]
        pub const fn tefnl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line"]
        #[inline(always)]
        pub fn set_tefnl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub const fn tefwl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line"]
        #[inline(always)]
        pub fn set_tefwl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Interrupt Line"]
        #[inline(always)]
        pub const fn teffl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Interrupt Line"]
        #[inline(always)]
        pub fn set_teffl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Line"]
        #[inline(always)]
        pub const fn tefll(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Line"]
        #[inline(always)]
        pub fn set_tefll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Interrupt Line"]
        #[inline(always)]
        pub const fn tswl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Interrupt Line"]
        #[inline(always)]
        pub fn set_tswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Interrupt Line"]
        #[inline(always)]
        pub const fn mrafl(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Interrupt Line"]
        #[inline(always)]
        pub fn set_mrafl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Interrupt Line"]
        #[inline(always)]
        pub const fn tool(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Interrupt Line"]
        #[inline(always)]
        pub fn set_tool(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
        #[inline(always)]
        pub const fn drxl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line"]
        #[inline(always)]
        pub fn set_drxl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Line"]
        #[inline(always)]
        pub const fn becl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Line"]
        #[inline(always)]
        pub fn set_becl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Line"]
        #[inline(always)]
        pub const fn beul(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Line"]
        #[inline(always)]
        pub fn set_beul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Interrupt Line"]
        #[inline(always)]
        pub const fn elol(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Interrupt Line"]
        #[inline(always)]
        pub fn set_elol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Interrupt Line"]
        #[inline(always)]
        pub const fn epl(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Line"]
        #[inline(always)]
        pub fn set_epl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Interrupt Line"]
        #[inline(always)]
        pub const fn ewl(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Interrupt Line"]
        #[inline(always)]
        pub fn set_ewl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bol(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Line"]
        #[inline(always)]
        pub const fn wdil(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Line"]
        #[inline(always)]
        pub fn set_wdil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Line"]
        #[inline(always)]
        pub const fn peal(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Line"]
        #[inline(always)]
        pub fn set_peal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Line"]
        #[inline(always)]
        pub const fn pedl(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Line"]
        #[inline(always)]
        pub fn set_pedl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Line"]
        #[inline(always)]
        pub const fn aral(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Line"]
        #[inline(always)]
        pub fn set_aral(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ils {
        #[inline(always)]
        fn default() -> Ils {
            Ils(0)
        }
    }
    #[doc = "FDCAN Interrupt Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Rx FIFO X New Message"]
        #[inline(always)]
        pub const fn rfn(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X New Message"]
        #[inline(always)]
        pub fn set_rfn(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Watermark Reached"]
        #[inline(always)]
        pub const fn rfw(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Watermark Reached"]
        #[inline(always)]
        pub fn set_rfw(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub const fn rff(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub fn set_rff(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub const fn rfl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub fn set_rfl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High Priority Message"]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message"]
        #[inline(always)]
        pub fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty"]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty"]
        #[inline(always)]
        pub fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry"]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry"]
        #[inline(always)]
        pub fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached"]
        #[inline(always)]
        pub const fn tefw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached"]
        #[inline(always)]
        pub fn set_tefw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full"]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full"]
        #[inline(always)]
        pub fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound"]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound"]
        #[inline(always)]
        pub fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure"]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure"]
        #[inline(always)]
        pub fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred"]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred"]
        #[inline(always)]
        pub fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer"]
        #[inline(always)]
        pub const fn drx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer"]
        #[inline(always)]
        pub fn set_drx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Error Logging Overflow"]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow"]
        #[inline(always)]
        pub fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt"]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt"]
        #[inline(always)]
        pub fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
        #[inline(always)]
        pub fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used)"]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used)"]
        #[inline(always)]
        pub fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address"]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address"]
        #[inline(always)]
        pub fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ir {
        #[inline(always)]
        fn default() -> Ir {
            Ir(0)
        }
    }
    #[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nbtp(pub u32);
    impl Nbtp {
        #[doc = "Nominal Time segment after sample point"]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal Time segment after sample point"]
        #[inline(always)]
        pub fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Nominal Time segment before sample point"]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Nominal Time segment before sample point"]
        #[inline(always)]
        pub fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bit Rate Prescaler"]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bit Rate Prescaler"]
        #[inline(always)]
        pub fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "NSJW: Nominal (Re)Synchronization Jump Width."]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "NSJW: Nominal (Re)Synchronization Jump Width."]
        #[inline(always)]
        pub fn set_nsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for Nbtp {
        #[inline(always)]
        fn default() -> Nbtp {
            Nbtp(0)
        }
    }
    #[doc = "FDCAN New Data 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat1(pub u32);
    impl Ndat1 {
        #[doc = "New data (buffers 0 - 31)"]
        #[inline(always)]
        pub const fn nd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New data (buffers 0 - 31)"]
        #[inline(always)]
        pub fn set_nd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat1 {
        #[inline(always)]
        fn default() -> Ndat1 {
            Ndat1(0)
        }
    }
    #[doc = "FDCAN New Data 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat2(pub u32);
    impl Ndat2 {
        #[doc = "New data (buffers 32 - 63)"]
        #[inline(always)]
        pub const fn nd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New data (buffers 32 - 63)"]
        #[inline(always)]
        pub fn set_nd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat2 {
        #[inline(always)]
        fn default() -> Ndat2 {
            Ndat2(0)
        }
    }
    #[doc = "FDCAN Protocol Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "Last Error Code"]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Last Error Code"]
        #[inline(always)]
        pub fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Activity"]
        #[inline(always)]
        pub const fn act(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Activity"]
        #[inline(always)]
        pub fn set_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data Last Error Code"]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Data Last Error Code"]
        #[inline(always)]
        pub fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "ESI flag of last received FDCAN Message"]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ESI flag of last received FDCAN Message"]
        #[inline(always)]
        pub fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BRS flag of last received FDCAN Message"]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BRS flag of last received FDCAN Message"]
        #[inline(always)]
        pub fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Received FDCAN Message"]
        #[inline(always)]
        pub const fn redl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Received FDCAN Message"]
        #[inline(always)]
        pub fn set_redl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Protocol Exception Event"]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Event"]
        #[inline(always)]
        pub fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitter Delay Compensation Value"]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Value"]
        #[inline(always)]
        pub fn set_tdcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Psr {
        #[inline(always)]
        fn default() -> Psr {
            Psr(0)
        }
    }
    #[doc = "FDCAN RAM Watchdog Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwd(pub u32);
    impl Rwd {
        #[doc = "Watchdog configuration"]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog configuration"]
        #[inline(always)]
        pub fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Watchdog value"]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog value"]
        #[inline(always)]
        pub fn set_wdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Rwd {
        #[inline(always)]
        fn default() -> Rwd {
            Rwd(0)
        }
    }
    #[doc = "FDCAN Rx Buffer Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxbc(pub u32);
    impl Rxbc {
        #[doc = "Rx Buffer Start Address"]
        #[inline(always)]
        pub const fn rbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx Buffer Start Address"]
        #[inline(always)]
        pub fn set_rbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
    }
    impl Default for Rxbc {
        #[inline(always)]
        fn default() -> Rxbc {
            Rxbc(0)
        }
    }
    #[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxesc(pub u32);
    impl Rxesc {
        #[doc = "Rx FIFO X Data Field Size"]
        #[inline(always)]
        pub const fn fds(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Rx FIFO X Data Field Size"]
        #[inline(always)]
        pub fn set_fds(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 4usize][n] as usize);
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
        #[doc = "Rx Buffer Data Field Size"]
        #[inline(always)]
        pub const fn rbds(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Rx Buffer Data Field Size"]
        #[inline(always)]
        pub fn set_rbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Rxesc {
        #[inline(always)]
        fn default() -> Rxesc {
            Rxesc(0)
        }
    }
    #[doc = "CAN Rx FIFO X Acknowledge Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfa(pub u32);
    impl Rxfa {
        #[doc = "Rx FIFO X Acknowledge Index"]
        #[inline(always)]
        pub const fn fai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Acknowledge Index"]
        #[inline(always)]
        pub fn set_fai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Rxfa {
        #[inline(always)]
        fn default() -> Rxfa {
            Rxfa(0)
        }
    }
    #[doc = "FDCAN Rx FIFO X Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfc(pub u32);
    impl Rxfc {
        #[doc = "Rx FIFO X Start Address"]
        #[inline(always)]
        pub const fn fsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx FIFO X Start Address"]
        #[inline(always)]
        pub fn set_fsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Rx FIFO X Size"]
        #[inline(always)]
        pub const fn fs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO X Size"]
        #[inline(always)]
        pub fn set_fs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "FIFO X Watermark"]
        #[inline(always)]
        pub const fn fwm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "FIFO X Watermark"]
        #[inline(always)]
        pub fn set_fwm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "FIFO X operation mode"]
        #[inline(always)]
        pub const fn fom(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO X operation mode"]
        #[inline(always)]
        pub fn set_fom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rxfc {
        #[inline(always)]
        fn default() -> Rxfc {
            Rxfc(0)
        }
    }
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfs(pub u32);
    impl Rxfs {
        #[doc = "Rx FIFO X Fill Level"]
        #[inline(always)]
        pub const fn ffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO X Fill Level"]
        #[inline(always)]
        pub fn set_ffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Rx FIFO X Get Index"]
        #[inline(always)]
        pub const fn fgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Get Index"]
        #[inline(always)]
        pub fn set_fgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx FIFO X Put Index"]
        #[inline(always)]
        pub const fn fpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO X Put Index"]
        #[inline(always)]
        pub fn set_fpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub const fn ff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Full"]
        #[inline(always)]
        pub fn set_ff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub const fn rfl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X Message Lost"]
        #[inline(always)]
        pub fn set_rfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Rxfs {
        #[inline(always)]
        fn default() -> Rxfs {
            Rxfs(0)
        }
    }
    #[doc = "FDCAN Standard ID Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidfc(pub u32);
    impl Sidfc {
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub const fn flssa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub fn set_flssa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Standard"]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "List Size Standard"]
        #[inline(always)]
        pub fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Sidfc {
        #[inline(always)]
        fn default() -> Sidfc {
            Sidfc(0)
        }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdcr(pub u32);
    impl Tdcr {
        #[doc = "Transmitter Delay Compensation Filter Window Length"]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Filter Window Length"]
        #[inline(always)]
        pub fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Transmitter Delay Compensation Offset"]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Offset"]
        #[inline(always)]
        pub fn set_tdco(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for Tdcr {
        #[inline(always)]
        fn default() -> Tdcr {
            Tdcr(0)
        }
    }
    #[doc = "FDCAN Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Back mode"]
        #[inline(always)]
        pub fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Control of Transmit Pin"]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Control of Transmit Pin"]
        #[inline(always)]
        pub fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Test {
        #[inline(always)]
        fn default() -> Test {
            Test(0)
        }
    }
    #[doc = "FDCAN Timeout Counter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocc(pub u32);
    impl Tocc {
        #[doc = "Enable Timeout Counter"]
        #[inline(always)]
        pub const fn etoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timeout Counter"]
        #[inline(always)]
        pub fn set_etoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout Select"]
        #[inline(always)]
        pub const fn tos(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Timeout Select"]
        #[inline(always)]
        pub fn set_tos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Timeout Period"]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Period"]
        #[inline(always)]
        pub fn set_top(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tocc {
        #[inline(always)]
        fn default() -> Tocc {
            Tocc(0)
        }
    }
    #[doc = "FDCAN Timeout Counter Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocv(pub u32);
    impl Tocv {
        #[doc = "Timeout Counter"]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Counter"]
        #[inline(always)]
        pub fn set_toc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tocv {
        #[inline(always)]
        fn default() -> Tocv {
            Tocv(0)
        }
    }
    #[doc = "FDCAN Timestamp Counter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscc(pub u32);
    impl Tscc {
        #[doc = "Timestamp Select"]
        #[inline(always)]
        pub const fn tss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Timestamp Select"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Timestamp Counter Prescaler"]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Counter Prescaler"]
        #[inline(always)]
        pub fn set_tcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Tscc {
        #[inline(always)]
        fn default() -> Tscc {
            Tscc(0)
        }
    }
    #[doc = "FDCAN Timestamp Counter Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscv(pub u32);
    impl Tscv {
        #[doc = "Timestamp Counter"]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp Counter"]
        #[inline(always)]
        pub fn set_tsc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tscv {
        #[inline(always)]
        fn default() -> Tscv {
            Tscv(0)
        }
    }
    #[doc = "FDCAN TT Capture Time Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcpt(pub u32);
    impl Ttcpt {
        #[doc = "Cycle Count Value"]
        #[inline(always)]
        pub const fn ccv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count Value"]
        #[inline(always)]
        pub fn set_ccv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Stop Watch Value"]
        #[inline(always)]
        pub const fn swv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Stop Watch Value"]
        #[inline(always)]
        pub fn set_swv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttcpt {
        #[inline(always)]
        fn default() -> Ttcpt {
            Ttcpt(0)
        }
    }
    #[doc = "FDCAN TT Cycle Sync Mark Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcsm(pub u32);
    impl Ttcsm {
        #[doc = "Cycle Sync Mark"]
        #[inline(always)]
        pub const fn csm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Sync Mark"]
        #[inline(always)]
        pub fn set_csm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ttcsm {
        #[inline(always)]
        fn default() -> Ttcsm {
            Ttcsm(0)
        }
    }
    #[doc = "FDCAN TT Cycle Time and Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttctc(pub u32);
    impl Ttctc {
        #[doc = "Cycle Time"]
        #[inline(always)]
        pub const fn ct(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Time"]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Cycle Count"]
        #[inline(always)]
        pub const fn cc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count"]
        #[inline(always)]
        pub fn set_cc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ttctc {
        #[inline(always)]
        fn default() -> Ttctc {
            Ttctc(0)
        }
    }
    #[doc = "FDCAN TT Global Time Preset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttgtp(pub u32);
    impl Ttgtp {
        #[doc = "Time Preset"]
        #[inline(always)]
        pub const fn ncl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Time Preset"]
        #[inline(always)]
        pub fn set_ncl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Cycle Time Target Phase"]
        #[inline(always)]
        pub const fn ctp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Cycle Time Target Phase"]
        #[inline(always)]
        pub fn set_ctp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttgtp {
        #[inline(always)]
        fn default() -> Ttgtp {
            Ttgtp(0)
        }
    }
    #[doc = "FDCAN TT Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttie(pub u32);
    impl Ttie {
        #[doc = "Start of Basic Cycle Interrupt Enable"]
        #[inline(always)]
        pub const fn sbce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle Interrupt Enable"]
        #[inline(always)]
        pub fn set_sbce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle Interrupt Enable"]
        #[inline(always)]
        pub const fn smce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle Interrupt Enable"]
        #[inline(always)]
        pub fn set_smce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode Interrupt Enable"]
        #[inline(always)]
        pub const fn csme(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode Interrupt Enable"]
        #[inline(always)]
        pub fn set_csme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap Interrupt Enable"]
        #[inline(always)]
        pub const fn soge(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap Interrupt Enable"]
        #[inline(always)]
        pub fn set_soge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Enable"]
        #[inline(always)]
        pub const fn rtmie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Enable"]
        #[inline(always)]
        pub fn set_rtmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Enable"]
        #[inline(always)]
        pub const fn ttmie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Enable"]
        #[inline(always)]
        pub fn set_ttmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event Interrupt Enable"]
        #[inline(always)]
        pub const fn swee(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event Interrupt Enable"]
        #[inline(always)]
        pub fn set_swee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap Interrupt Enable"]
        #[inline(always)]
        pub const fn gtwe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity Interrupt Enable"]
        #[inline(always)]
        pub const fn gtde(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error Interrupt Enable"]
        #[inline(always)]
        pub const fn gtee(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_gtee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow Interrupt Enable"]
        #[inline(always)]
        pub const fn txue(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_txue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow Interrupt Enable"]
        #[inline(always)]
        pub const fn txoe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow Interrupt Enable"]
        #[inline(always)]
        pub fn set_txoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1 Interrupt Enable"]
        #[inline(always)]
        pub const fn se1e(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1 Interrupt Enable"]
        #[inline(always)]
        pub fn set_se1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2 Interrupt Enable"]
        #[inline(always)]
        pub const fn se2e(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2 Interrupt Enable"]
        #[inline(always)]
        pub fn set_se2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Change Error Level Interrupt Enable"]
        #[inline(always)]
        pub const fn elce(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Change Error Level Interrupt Enable"]
        #[inline(always)]
        pub fn set_elce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub const fn iwtge(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub fn set_iwtge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub const fn wte(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Enable"]
        #[inline(always)]
        pub fn set_wte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog Interrupt Enable"]
        #[inline(always)]
        pub const fn awe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Interrupt Enable"]
        #[inline(always)]
        pub fn set_awe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub const fn cere(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_cere(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttie {
        #[inline(always)]
        fn default() -> Ttie {
            Ttie(0)
        }
    }
    #[doc = "FDCAN TT Interrupt Line Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttils(pub u32);
    impl Ttils {
        #[doc = "Start of Basic Cycle Interrupt Line"]
        #[inline(always)]
        pub const fn sbcl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle Interrupt Line"]
        #[inline(always)]
        pub fn set_sbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle Interrupt Line"]
        #[inline(always)]
        pub const fn smcl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle Interrupt Line"]
        #[inline(always)]
        pub fn set_smcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode Interrupt Line"]
        #[inline(always)]
        pub const fn csml(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode Interrupt Line"]
        #[inline(always)]
        pub fn set_csml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap Interrupt Line"]
        #[inline(always)]
        pub const fn sogl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap Interrupt Line"]
        #[inline(always)]
        pub fn set_sogl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Line"]
        #[inline(always)]
        pub const fn rtmil(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Line"]
        #[inline(always)]
        pub fn set_rtmil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Line"]
        #[inline(always)]
        pub const fn ttmil(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal Interrupt Line"]
        #[inline(always)]
        pub fn set_ttmil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event Interrupt Line"]
        #[inline(always)]
        pub const fn swel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event Interrupt Line"]
        #[inline(always)]
        pub fn set_swel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap Interrupt Line"]
        #[inline(always)]
        pub const fn gtwl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap Interrupt Line"]
        #[inline(always)]
        pub fn set_gtwl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity Interrupt Line"]
        #[inline(always)]
        pub const fn gtdl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity Interrupt Line"]
        #[inline(always)]
        pub fn set_gtdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error Interrupt Line"]
        #[inline(always)]
        pub const fn gtel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error Interrupt Line"]
        #[inline(always)]
        pub fn set_gtel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow Interrupt Line"]
        #[inline(always)]
        pub const fn txul(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow Interrupt Line"]
        #[inline(always)]
        pub fn set_txul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow Interrupt Line"]
        #[inline(always)]
        pub const fn txol(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow Interrupt Line"]
        #[inline(always)]
        pub fn set_txol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1 Interrupt Line"]
        #[inline(always)]
        pub const fn se1l(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1 Interrupt Line"]
        #[inline(always)]
        pub fn set_se1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2 Interrupt Line"]
        #[inline(always)]
        pub const fn se2l(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2 Interrupt Line"]
        #[inline(always)]
        pub fn set_se2l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Change Error Level Interrupt Line"]
        #[inline(always)]
        pub const fn elcl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Change Error Level Interrupt Line"]
        #[inline(always)]
        pub fn set_elcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub const fn iwtgl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub fn set_iwtgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub const fn wtl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Line"]
        #[inline(always)]
        pub fn set_wtl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog Interrupt Line"]
        #[inline(always)]
        pub const fn awl(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Interrupt Line"]
        #[inline(always)]
        pub fn set_awl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error Interrupt Line"]
        #[inline(always)]
        pub const fn cerl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error Interrupt Line"]
        #[inline(always)]
        pub fn set_cerl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttils {
        #[inline(always)]
        fn default() -> Ttils {
            Ttils(0)
        }
    }
    #[doc = "FDCAN TT Interrupt Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttir(pub u32);
    impl Ttir {
        #[doc = "Start of Basic Cycle"]
        #[inline(always)]
        pub const fn sbc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Basic Cycle"]
        #[inline(always)]
        pub fn set_sbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start of Matrix Cycle"]
        #[inline(always)]
        pub const fn smc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Matrix Cycle"]
        #[inline(always)]
        pub fn set_smc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Change of Synchronization Mode"]
        #[inline(always)]
        pub const fn csm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Change of Synchronization Mode"]
        #[inline(always)]
        pub fn set_csm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Gap"]
        #[inline(always)]
        pub const fn sog(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Gap"]
        #[inline(always)]
        pub fn set_sog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt"]
        #[inline(always)]
        pub const fn rtmi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt"]
        #[inline(always)]
        pub fn set_rtmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trigger Time Mark Event Internal"]
        #[inline(always)]
        pub const fn ttmi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Event Internal"]
        #[inline(always)]
        pub fn set_ttmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Stop Watch Event"]
        #[inline(always)]
        pub const fn swe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Event"]
        #[inline(always)]
        pub fn set_swe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Time Wrap"]
        #[inline(always)]
        pub const fn gtw(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Wrap"]
        #[inline(always)]
        pub fn set_gtw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global Time Discontinuity"]
        #[inline(always)]
        pub const fn gtd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Discontinuity"]
        #[inline(always)]
        pub fn set_gtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Global Time Error"]
        #[inline(always)]
        pub const fn gte(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Time Error"]
        #[inline(always)]
        pub fn set_gte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx Count Underflow"]
        #[inline(always)]
        pub const fn txu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Underflow"]
        #[inline(always)]
        pub fn set_txu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx Count Overflow"]
        #[inline(always)]
        pub const fn txo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Count Overflow"]
        #[inline(always)]
        pub fn set_txo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Scheduling Error 1"]
        #[inline(always)]
        pub const fn se1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 1"]
        #[inline(always)]
        pub fn set_se1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Scheduling Error 2"]
        #[inline(always)]
        pub const fn se2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Scheduling Error 2"]
        #[inline(always)]
        pub fn set_se2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Error Level Changed"]
        #[inline(always)]
        pub const fn elc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Error Level Changed"]
        #[inline(always)]
        pub fn set_elc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Initialization Watch Trigger"]
        #[inline(always)]
        pub const fn iwtg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Watch Trigger"]
        #[inline(always)]
        pub fn set_iwtg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Watch Trigger"]
        #[inline(always)]
        pub const fn wt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger"]
        #[inline(always)]
        pub fn set_wt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Application Watchdog"]
        #[inline(always)]
        pub const fn aw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog"]
        #[inline(always)]
        pub fn set_aw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Configuration Error"]
        #[inline(always)]
        pub const fn cer(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Error"]
        #[inline(always)]
        pub fn set_cer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ttir {
        #[inline(always)]
        fn default() -> Ttir {
            Ttir(0)
        }
    }
    #[doc = "FDCAN TT Local and Global Time Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttlgt(pub u32);
    impl Ttlgt {
        #[doc = "Local Time"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Local Time"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Global Time"]
        #[inline(always)]
        pub const fn gt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Global Time"]
        #[inline(always)]
        pub fn set_gt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ttlgt {
        #[inline(always)]
        fn default() -> Ttlgt {
            Ttlgt(0)
        }
    }
    #[doc = "FDCAN TT Matrix Limits Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttmlm(pub u32);
    impl Ttmlm {
        #[doc = "Cycle Count Max"]
        #[inline(always)]
        pub const fn ccm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Cycle Count Max"]
        #[inline(always)]
        pub fn set_ccm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Cycle Start Synchronization"]
        #[inline(always)]
        pub const fn css(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Cycle Start Synchronization"]
        #[inline(always)]
        pub fn set_css(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Tx Enable Window"]
        #[inline(always)]
        pub const fn txew(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Tx Enable Window"]
        #[inline(always)]
        pub fn set_txew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Expected Number of Tx Triggers"]
        #[inline(always)]
        pub const fn entt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Expected Number of Tx Triggers"]
        #[inline(always)]
        pub fn set_entt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Ttmlm {
        #[inline(always)]
        fn default() -> Ttmlm {
            Ttmlm(0)
        }
    }
    #[doc = "FDCAN TT Operation Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttocf(pub u32);
    impl Ttocf {
        #[doc = "Operation Mode"]
        #[inline(always)]
        pub const fn om(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Operation Mode"]
        #[inline(always)]
        pub fn set_om(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Gap Enable"]
        #[inline(always)]
        pub const fn gen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Enable"]
        #[inline(always)]
        pub fn set_gen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Time Master"]
        #[inline(always)]
        pub const fn tm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Time Master"]
        #[inline(always)]
        pub fn set_tm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LD of Synchronization Deviation Limit"]
        #[inline(always)]
        pub const fn ldsdl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "LD of Synchronization Deviation Limit"]
        #[inline(always)]
        pub fn set_ldsdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "Initial Reference Trigger Offset"]
        #[inline(always)]
        pub const fn irto(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Initial Reference Trigger Offset"]
        #[inline(always)]
        pub fn set_irto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Enable External Clock Synchronization"]
        #[inline(always)]
        pub const fn eecs(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable External Clock Synchronization"]
        #[inline(always)]
        pub fn set_eecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Application Watchdog Limit"]
        #[inline(always)]
        pub const fn awl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Application Watchdog Limit"]
        #[inline(always)]
        pub fn set_awl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Enable Global Time Filtering"]
        #[inline(always)]
        pub const fn egtf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Global Time Filtering"]
        #[inline(always)]
        pub fn set_egtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Enable Clock Calibration"]
        #[inline(always)]
        pub const fn ecc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Clock Calibration"]
        #[inline(always)]
        pub fn set_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Event Trigger Polarity"]
        #[inline(always)]
        pub const fn evtp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Event Trigger Polarity"]
        #[inline(always)]
        pub fn set_evtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Ttocf {
        #[inline(always)]
        fn default() -> Ttocf {
            Ttocf(0)
        }
    }
    #[doc = "FDCAN TT Operation Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttocn(pub u32);
    impl Ttocn {
        #[doc = "Set Global time"]
        #[inline(always)]
        pub const fn sgt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Set Global time"]
        #[inline(always)]
        pub fn set_sgt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Clock Synchronization"]
        #[inline(always)]
        pub const fn ecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Clock Synchronization"]
        #[inline(always)]
        pub fn set_ecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop Watch Polarity"]
        #[inline(always)]
        pub const fn swp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Watch Polarity"]
        #[inline(always)]
        pub fn set_swp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Stop Watch Source"]
        #[inline(always)]
        pub const fn sws(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Stop Watch Source"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Register Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub const fn rtie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Register Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub fn set_rtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Register Time Mark Compare"]
        #[inline(always)]
        pub const fn tmc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Register Time Mark Compare"]
        #[inline(always)]
        pub fn set_tmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Trigger Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub const fn ttie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Time Mark Interrupt Pulse Enable"]
        #[inline(always)]
        pub fn set_ttie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Gap Control Select"]
        #[inline(always)]
        pub const fn gcs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Control Select"]
        #[inline(always)]
        pub fn set_gcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Finish Gap"]
        #[inline(always)]
        pub const fn fgp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Finish Gap"]
        #[inline(always)]
        pub fn set_fgp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Time Mark Gap"]
        #[inline(always)]
        pub const fn tmg(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Time Mark Gap"]
        #[inline(always)]
        pub fn set_tmg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Next is Gap"]
        #[inline(always)]
        pub const fn nig(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Next is Gap"]
        #[inline(always)]
        pub fn set_nig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External Synchronization Control"]
        #[inline(always)]
        pub const fn escn(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "External Synchronization Control"]
        #[inline(always)]
        pub fn set_escn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TT Operation Control Register Locked"]
        #[inline(always)]
        pub const fn lckc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TT Operation Control Register Locked"]
        #[inline(always)]
        pub fn set_lckc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ttocn {
        #[inline(always)]
        fn default() -> Ttocn {
            Ttocn(0)
        }
    }
    #[doc = "FDCAN TT Operation Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttost(pub u32);
    impl Ttost {
        #[doc = "Error Level"]
        #[inline(always)]
        pub const fn el(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Error Level"]
        #[inline(always)]
        pub fn set_el(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Master State"]
        #[inline(always)]
        pub const fn ms(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Master State"]
        #[inline(always)]
        pub fn set_ms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Synchronization State"]
        #[inline(always)]
        pub const fn sys(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization State"]
        #[inline(always)]
        pub fn set_sys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Quality of Global Time Phase"]
        #[inline(always)]
        pub const fn qgtp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Quality of Global Time Phase"]
        #[inline(always)]
        pub fn set_qgtp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Quality of Clock Speed"]
        #[inline(always)]
        pub const fn qcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Quality of Clock Speed"]
        #[inline(always)]
        pub fn set_qcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Reference Trigger Offset"]
        #[inline(always)]
        pub const fn rto(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Reference Trigger Offset"]
        #[inline(always)]
        pub fn set_rto(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Wait for Global Time Discontinuity"]
        #[inline(always)]
        pub const fn wgtd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for Global Time Discontinuity"]
        #[inline(always)]
        pub fn set_wgtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Gap Finished Indicator"]
        #[inline(always)]
        pub const fn gfi(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Finished Indicator"]
        #[inline(always)]
        pub fn set_gfi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Time Master Priority"]
        #[inline(always)]
        pub const fn tmp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Time Master Priority"]
        #[inline(always)]
        pub fn set_tmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Gap Started Indicator"]
        #[inline(always)]
        pub const fn gsi(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Gap Started Indicator"]
        #[inline(always)]
        pub fn set_gsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Wait for Event"]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for Event"]
        #[inline(always)]
        pub fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Application Watchdog Event"]
        #[inline(always)]
        pub const fn awe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Application Watchdog Event"]
        #[inline(always)]
        pub fn set_awe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Wait for External Clock Synchronization"]
        #[inline(always)]
        pub const fn wecs(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for External Clock Synchronization"]
        #[inline(always)]
        pub fn set_wecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Schedule Phase Lock"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Schedule Phase Lock"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ttost {
        #[inline(always)]
        fn default() -> Ttost {
            Ttost(0)
        }
    }
    #[doc = "FDCAN TT Reference Message Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttrmc(pub u32);
    impl Ttrmc {
        #[doc = "Reference Identifier"]
        #[inline(always)]
        pub const fn rid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Reference Identifier"]
        #[inline(always)]
        pub fn set_rid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "Extended Identifier"]
        #[inline(always)]
        pub const fn xtd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Extended Identifier"]
        #[inline(always)]
        pub fn set_xtd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Reference Message Payload Select"]
        #[inline(always)]
        pub const fn rmps(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reference Message Payload Select"]
        #[inline(always)]
        pub fn set_rmps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ttrmc {
        #[inline(always)]
        fn default() -> Ttrmc {
            Ttrmc(0)
        }
    }
    #[doc = "FDCAN TT Trigger Memory Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tttmc(pub u32);
    impl Tttmc {
        #[doc = "Trigger Memory Start Address"]
        #[inline(always)]
        pub const fn tmsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Trigger Memory Start Address"]
        #[inline(always)]
        pub fn set_tmsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Trigger Memory Elements"]
        #[inline(always)]
        pub const fn tme(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Trigger Memory Elements"]
        #[inline(always)]
        pub fn set_tme(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Tttmc {
        #[inline(always)]
        fn default() -> Tttmc {
            Tttmc(0)
        }
    }
    #[doc = "FDCAN TT Time Mark Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tttmk(pub u32);
    impl Tttmk {
        #[doc = "Time Mark"]
        #[inline(always)]
        pub const fn tm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Time Mark"]
        #[inline(always)]
        pub fn set_tm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Time Mark Cycle Code"]
        #[inline(always)]
        pub const fn ticc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Time Mark Cycle Code"]
        #[inline(always)]
        pub fn set_ticc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "TT Time Mark Register Locked"]
        #[inline(always)]
        pub const fn lckm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TT Time Mark Register Locked"]
        #[inline(always)]
        pub fn set_lckm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tttmk {
        #[inline(always)]
        fn default() -> Tttmk {
            Tttmk(0)
        }
    }
    #[doc = "FDCAN TT Trigger Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttts(pub u32);
    impl Ttts {
        #[doc = "Stop watch trigger input selection"]
        #[inline(always)]
        pub const fn swtdel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Stop watch trigger input selection"]
        #[inline(always)]
        pub fn set_swtdel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Event trigger input selection"]
        #[inline(always)]
        pub const fn evtsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Event trigger input selection"]
        #[inline(always)]
        pub fn set_evtsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Ttts {
        #[inline(always)]
        fn default() -> Ttts {
            Ttts(0)
        }
    }
    #[doc = "FDCAN TUR Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Turcf(pub u32);
    impl Turcf {
        #[doc = "Numerator Configuration Low"]
        #[inline(always)]
        pub const fn ncl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Numerator Configuration Low"]
        #[inline(always)]
        pub fn set_ncl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Denominator Configuration"]
        #[inline(always)]
        pub const fn dc(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Denominator Configuration"]
        #[inline(always)]
        pub fn set_dc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
        #[doc = "Enable Local Time"]
        #[inline(always)]
        pub const fn elt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Local Time"]
        #[inline(always)]
        pub fn set_elt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Turcf {
        #[inline(always)]
        fn default() -> Turcf {
            Turcf(0)
        }
    }
    #[doc = "FDCAN TUR Numerator Actual Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Turna(pub u32);
    impl Turna {
        #[doc = "Numerator Actual Value"]
        #[inline(always)]
        pub const fn nav(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Numerator Actual Value"]
        #[inline(always)]
        pub fn set_nav(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Turna {
        #[inline(always)]
        fn default() -> Turna {
            Turna(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbar(pub u32);
    impl Txbar {
        #[doc = "Add Request"]
        #[inline(always)]
        pub const fn ar(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Add Request"]
        #[inline(always)]
        pub fn set_ar(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbar {
        #[inline(always)]
        fn default() -> Txbar {
            Txbar(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbc(pub u32);
    impl Txbc {
        #[doc = "Tx Buffers Start Address"]
        #[inline(always)]
        pub const fn tbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Tx Buffers Start Address"]
        #[inline(always)]
        pub fn set_tbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Number of Dedicated Transmit Buffers"]
        #[inline(always)]
        pub const fn ndtb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of Dedicated Transmit Buffers"]
        #[inline(always)]
        pub fn set_ndtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Transmit FIFO/Queue Size"]
        #[inline(always)]
        pub const fn tfqs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit FIFO/Queue Size"]
        #[inline(always)]
        pub fn set_tfqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Tx FIFO/Queue Mode"]
        #[inline(always)]
        pub const fn tfqm(&self) -> super::vals::Tfqm {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Tfqm::from_bits(val as u8)
        }
        #[doc = "Tx FIFO/Queue Mode"]
        #[inline(always)]
        pub fn set_tfqm(&mut self, val: super::vals::Tfqm) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Txbc {
        #[inline(always)]
        fn default() -> Txbc {
            Txbc(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcf(pub u32);
    impl Txbcf {
        #[doc = "Cancellation Finished"]
        #[inline(always)]
        pub const fn cf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Finished"]
        #[inline(always)]
        pub fn set_cf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcf {
        #[inline(always)]
        fn default() -> Txbcf {
            Txbcf(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcie(pub u32);
    impl Txbcie {
        #[doc = "Cancellation Finished Interrupt Enable"]
        #[inline(always)]
        pub const fn cf(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Finished Interrupt Enable"]
        #[inline(always)]
        pub fn set_cf(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcie {
        #[inline(always)]
        fn default() -> Txbcie {
            Txbcie(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcr(pub u32);
    impl Txbcr {
        #[doc = "Cancellation Request"]
        #[inline(always)]
        pub const fn cr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Cancellation Request"]
        #[inline(always)]
        pub fn set_cr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbcr {
        #[inline(always)]
        fn default() -> Txbcr {
            Txbcr(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Request Pending Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbrp(pub u32);
    impl Txbrp {
        #[doc = "Transmission Request Pending"]
        #[inline(always)]
        pub const fn trp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Request Pending"]
        #[inline(always)]
        pub fn set_trp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbrp {
        #[inline(always)]
        fn default() -> Txbrp {
            Txbrp(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbtie(pub u32);
    impl Txbtie {
        #[doc = "Transmission Interrupt Enable"]
        #[inline(always)]
        pub const fn tie(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Interrupt Enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbtie {
        #[inline(always)]
        fn default() -> Txbtie {
            Txbtie(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbto(pub u32);
    impl Txbto {
        #[doc = "Transmission Occurred"]
        #[inline(always)]
        pub const fn to(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Transmission Occurred"]
        #[inline(always)]
        pub fn set_to(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Txbto {
        #[inline(always)]
        fn default() -> Txbto {
            Txbto(0)
        }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefa(pub u32);
    impl Txefa {
        #[doc = "Event FIFO Acknowledge Index"]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Acknowledge Index"]
        #[inline(always)]
        pub fn set_efai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Txefa {
        #[inline(always)]
        fn default() -> Txefa {
            Txefa(0)
        }
    }
    #[doc = "FDCAN Tx Event FIFO Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefc(pub u32);
    impl Txefc {
        #[doc = "Event FIFO Start Address"]
        #[inline(always)]
        pub const fn efsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Event FIFO Start Address"]
        #[inline(always)]
        pub fn set_efsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Event FIFO Size"]
        #[inline(always)]
        pub const fn efs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Size"]
        #[inline(always)]
        pub fn set_efs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Event FIFO Watermark"]
        #[inline(always)]
        pub const fn efwm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Watermark"]
        #[inline(always)]
        pub fn set_efwm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Txefc {
        #[inline(always)]
        fn default() -> Txefc {
            Txefc(0)
        }
    }
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefs(pub u32);
    impl Txefs {
        #[doc = "Event FIFO Fill Level"]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Fill Level"]
        #[inline(always)]
        pub fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Event FIFO Get Index"]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Get Index"]
        #[inline(always)]
        pub fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Event FIFO put index"]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO put index"]
        #[inline(always)]
        pub fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Event FIFO Full"]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Event FIFO Full"]
        #[inline(always)]
        pub fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Txefs {
        #[inline(always)]
        fn default() -> Txefs {
            Txefs(0)
        }
    }
    #[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txesc(pub u32);
    impl Txesc {
        #[doc = "Tx Buffer Data Field Size"]
        #[inline(always)]
        pub const fn tbds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tx Buffer Data Field Size"]
        #[inline(always)]
        pub fn set_tbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Txesc {
        #[inline(always)]
        fn default() -> Txesc {
            Txesc(0)
        }
    }
    #[doc = "FDCAN Tx FIFO/Queue Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfqs(pub u32);
    impl Txfqs {
        #[doc = "Tx FIFO Free Level"]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Tx FIFO Free Level"]
        #[inline(always)]
        pub fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Tx FIFO/Queue Put Index"]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx FIFO/Queue Put Index"]
        #[inline(always)]
        pub fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Tx FIFO/Queue Full"]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO/Queue Full"]
        #[inline(always)]
        pub fn set_tfqf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Txfqs {
        #[inline(always)]
        fn default() -> Txfqs {
            Txfqs(0)
        }
    }
    #[doc = "FDCAN Extended ID and Mask Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidam(pub u32);
    impl Xidam {
        #[doc = "Extended ID Mask"]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Extended ID Mask"]
        #[inline(always)]
        pub fn set_eidm(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for Xidam {
        #[inline(always)]
        fn default() -> Xidam {
            Xidam(0)
        }
    }
    #[doc = "FDCAN Extended ID Filter Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidfc(pub u32);
    impl Xidfc {
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub const fn flesa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Standard Start Address"]
        #[inline(always)]
        pub fn set_flesa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Extended"]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "List Size Extended"]
        #[inline(always)]
        pub fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Xidfc {
        #[inline(always)]
        fn default() -> Xidfc {
            Xidfc(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tfqm {
        #[doc = "Tx FIFO operation"]
        FIFO = 0x0,
        #[doc = "Tx queue operation"]
        QUEUE = 0x01,
    }
    impl Tfqm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tfqm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tfqm {
        #[inline(always)]
        fn from(val: u8) -> Tfqm {
            Tfqm::from_bits(val)
        }
    }
    impl From<Tfqm> for u8 {
        #[inline(always)]
        fn from(val: Tfqm) -> u8 {
            Tfqm::to_bits(val)
        }
    }
}
