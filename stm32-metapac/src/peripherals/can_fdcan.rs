#[doc = "FDCAN"]
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
    pub const fn crel(self) -> crate::common::Reg<regs::Crel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::Endn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "For details about setting and resetting of single bits see Software initialization."]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "FDCAN_NBTP"]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    #[inline(always)]
    pub const fn rxgfc(self) -> crate::common::Reg<regs::Rxgfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[inline(always)]
    pub const fn rxfs(self, n: usize) -> crate::common::Reg<regs::Rxfs, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn rxfa(self, n: usize) -> crate::common::Reg<regs::Rxfa, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(220usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn ckdiv(self) -> crate::common::Reg<regs::Ckdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
}
pub mod regs {
    #[doc = "For details about setting and resetting of single bits see Software initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "INIT"]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "INIT"]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CCE"]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CCE"]
        #[inline(always)]
        pub fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ASM"]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ASM"]
        #[inline(always)]
        pub fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CSA"]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CSA"]
        #[inline(always)]
        pub fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSR"]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSR"]
        #[inline(always)]
        pub fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MON"]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MON"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAR"]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAR"]
        #[inline(always)]
        pub fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TEST"]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TEST"]
        #[inline(always)]
        pub fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDOE"]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDOE"]
        #[inline(always)]
        pub fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BRSE"]
        #[inline(always)]
        pub const fn brse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BRSE"]
        #[inline(always)]
        pub fn set_brse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PXHD"]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PXHD"]
        #[inline(always)]
        pub fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "EFBI"]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "EFBI"]
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
        #[doc = "NISO"]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "NISO"]
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
    #[doc = "FDCAN CFG clock divider register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckdiv(pub u32);
    impl Ckdiv {
        #[doc = "input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
        #[inline(always)]
        pub const fn pdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
        #[inline(always)]
        pub fn set_pdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ckdiv {
        #[inline(always)]
        fn default() -> Ckdiv {
            Ckdiv(0)
        }
    }
    #[doc = "FDCAN Core Release Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crel(pub u32);
    impl Crel {
        #[doc = "DAY"]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DAY"]
        #[inline(always)]
        pub fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "MON"]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "MON"]
        #[inline(always)]
        pub fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "YEAR"]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "YEAR"]
        #[inline(always)]
        pub fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SUBSTEP"]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "SUBSTEP"]
        #[inline(always)]
        pub fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "STEP"]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "STEP"]
        #[inline(always)]
        pub fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "REL"]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "REL"]
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
    #[doc = "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) \\[DTSEG1 + DTSEG2 + 3\\]
tq or (functional values) \\[Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2\\]
tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbtp(pub u32);
    impl Dbtp {
        #[doc = "DSJW"]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DSJW"]
        #[inline(always)]
        pub fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "DTSEG2"]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "DTSEG2"]
        #[inline(always)]
        pub fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "DTSEG1"]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DTSEG1"]
        #[inline(always)]
        pub fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "DBRP"]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "DBRP"]
        #[inline(always)]
        pub fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "TDC"]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TDC"]
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
        #[doc = "TEC"]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TEC"]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TREC"]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "TREC"]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "RP"]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RP"]
        #[inline(always)]
        pub fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CEL"]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "CEL"]
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
        #[doc = "ETV"]
        #[inline(always)]
        pub const fn etv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ETV"]
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
    #[doc = "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpms(pub u32);
    impl Hpms {
        #[doc = "BIDX"]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "BIDX"]
        #[inline(always)]
        pub fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "MSI"]
        #[inline(always)]
        pub const fn msi(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "MSI"]
        #[inline(always)]
        pub fn set_msi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "FIDX"]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "FIDX"]
        #[inline(always)]
        pub fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "FLST"]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FLST"]
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
    #[doc = "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ie(pub u32);
    impl Ie {
        #[doc = "Rx FIFO X new message enable"]
        #[inline(always)]
        pub const fn rfne(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X new message enable"]
        #[inline(always)]
        pub fn set_rfne(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X full enable"]
        #[inline(always)]
        pub const fn rffe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X full enable"]
        #[inline(always)]
        pub fn set_rffe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X message lost enable"]
        #[inline(always)]
        pub const fn rfle(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X message lost enable"]
        #[inline(always)]
        pub fn set_rfle(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High-priority message enable"]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "High-priority message enable"]
        #[inline(always)]
        pub fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmission completed enable"]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission completed enable"]
        #[inline(always)]
        pub fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmission cancellation finished enable"]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission cancellation finished enable"]
        #[inline(always)]
        pub fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx FIFO empty enable"]
        #[inline(always)]
        pub const fn tfee(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty enable"]
        #[inline(always)]
        pub fn set_tfee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx even FIFO new entry enable"]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx even FIFO new entry enable"]
        #[inline(always)]
        pub fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx event FIFO full enable"]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO full enable"]
        #[inline(always)]
        pub fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx event FIFO element lost enable"]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO element lost enable"]
        #[inline(always)]
        pub fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timestamp wraparound enable"]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp wraparound enable"]
        #[inline(always)]
        pub fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Message RAM access failure enable"]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM access failure enable"]
        #[inline(always)]
        pub fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timeout occurred enable"]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout occurred enable"]
        #[inline(always)]
        pub fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error logging overflow enable"]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error logging overflow enable"]
        #[inline(always)]
        pub fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Error passive enable"]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive enable"]
        #[inline(always)]
        pub fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Warning status enable"]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Warning status enable"]
        #[inline(always)]
        pub fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus_off status enable"]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_off status enable"]
        #[inline(always)]
        pub fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Watchdog interrupt enable"]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog interrupt enable"]
        #[inline(always)]
        pub fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Protocol error in arbitration phase enable"]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in arbitration phase enable"]
        #[inline(always)]
        pub fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Protocol error in data phase enable"]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in data phase enable"]
        #[inline(always)]
        pub fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Access to reserved address enable"]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Access to reserved address enable"]
        #[inline(always)]
        pub fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ie {
        #[inline(always)]
        fn default() -> Ie {
            Ie(0)
        }
    }
    #[doc = "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ile(pub u32);
    impl Ile {
        #[doc = "EINT0"]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EINT0"]
        #[inline(always)]
        pub fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EINT1"]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EINT1"]
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
    #[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ils(pub u32);
    impl Ils {
        #[doc = "RX FIFO bit grouping the following interruption"]
        #[inline(always)]
        pub const fn rxfifo(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO bit grouping the following interruption"]
        #[inline(always)]
        pub fn set_rxfifo(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Status message bit grouping the following interruption"]
        #[inline(always)]
        pub const fn smsg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Status message bit grouping the following interruption"]
        #[inline(always)]
        pub fn set_smsg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TX FIFO error grouping the following interruption"]
        #[inline(always)]
        pub const fn tferr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO error grouping the following interruption"]
        #[inline(always)]
        pub fn set_tferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt regrouping the following interruption"]
        #[inline(always)]
        pub const fn misc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt regrouping the following interruption"]
        #[inline(always)]
        pub fn set_misc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bit and line error grouping the following interruption"]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bit and line error grouping the following interruption"]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Protocol error grouping the following interruption"]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error grouping the following interruption"]
        #[inline(always)]
        pub fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ils {
        #[inline(always)]
        fn default() -> Ils {
            Ils(0)
        }
    }
    #[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Rx FIFO X new message"]
        #[inline(always)]
        pub const fn rfn(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X new message"]
        #[inline(always)]
        pub fn set_rfn(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub const fn rff(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X full"]
        #[inline(always)]
        pub fn set_rff(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Rx FIFO X message lost"]
        #[inline(always)]
        pub const fn rfl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO X message lost"]
        #[inline(always)]
        pub fn set_rfl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 3usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "High-priority message"]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "High-priority message"]
        #[inline(always)]
        pub fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmission completed"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission completed"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmission cancellation finished"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission cancellation finished"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx FIFO empty"]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty"]
        #[inline(always)]
        pub fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Tx even FIFO new entry"]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Tx even FIFO new entry"]
        #[inline(always)]
        pub fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx event FIFO full"]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO full"]
        #[inline(always)]
        pub fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx event FIFO element lost"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx event FIFO element lost"]
        #[inline(always)]
        pub fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timestamp wraparound"]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp wraparound"]
        #[inline(always)]
        pub fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Message RAM access failure"]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM access failure"]
        #[inline(always)]
        pub fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timeout occurred"]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout occurred"]
        #[inline(always)]
        pub fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error logging overflow"]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error logging overflow"]
        #[inline(always)]
        pub fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Error passive"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Warning status"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Warning status"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus_off status"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_off status"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Watchdog interrupt"]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog interrupt"]
        #[inline(always)]
        pub fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Protocol error in arbitration phase"]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in arbitration phase"]
        #[inline(always)]
        pub fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Protocol error in data phase"]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error in data phase"]
        #[inline(always)]
        pub fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Access to reserved address"]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Access to reserved address"]
        #[inline(always)]
        pub fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ir {
        #[inline(always)]
        fn default() -> Ir {
            Ir(0)
        }
    }
    #[doc = "FDCAN_NBTP"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nbtp(pub u32);
    impl Nbtp {
        #[doc = "TSEG2"]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "TSEG2"]
        #[inline(always)]
        pub fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "NTSEG1"]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "NTSEG1"]
        #[inline(always)]
        pub fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "NBRP"]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "NBRP"]
        #[inline(always)]
        pub fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "NSJW"]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "NSJW"]
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
    #[doc = "FDCAN Protocol Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "LEC"]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "LEC"]
        #[inline(always)]
        pub fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "ACT"]
        #[inline(always)]
        pub const fn act(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "ACT"]
        #[inline(always)]
        pub fn set_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "EP"]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EP"]
        #[inline(always)]
        pub fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EW"]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EW"]
        #[inline(always)]
        pub fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "BO"]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BO"]
        #[inline(always)]
        pub fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DLEC"]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "DLEC"]
        #[inline(always)]
        pub fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "RESI"]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RESI"]
        #[inline(always)]
        pub fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RBRS"]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RBRS"]
        #[inline(always)]
        pub fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "REDL"]
        #[inline(always)]
        pub const fn redl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "REDL"]
        #[inline(always)]
        pub fn set_redl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PXE"]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PXE"]
        #[inline(always)]
        pub fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TDCV"]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "TDCV"]
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
    #[doc = "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\\[WDC\\]
bits. The counter is reloaded with RWD\\[WDC\\]
bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\\[WDI\\]
bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwd(pub u32);
    impl Rwd {
        #[doc = "WDC"]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "WDC"]
        #[inline(always)]
        pub fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "WDV"]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "WDV"]
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
    #[doc = "CAN Rx FIFO X Acknowledge Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfa(pub u32);
    impl Rxfa {
        #[doc = "FAI"]
        #[inline(always)]
        pub const fn fai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "FAI"]
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
    #[doc = "FDCAN Rx FIFO X Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxfs(pub u32);
    impl Rxfs {
        #[doc = "FFL"]
        #[inline(always)]
        pub const fn ffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "FFL"]
        #[inline(always)]
        pub fn set_ffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "FGI"]
        #[inline(always)]
        pub const fn fgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "FGI"]
        #[inline(always)]
        pub fn set_fgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "FPI"]
        #[inline(always)]
        pub const fn fpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "FPI"]
        #[inline(always)]
        pub fn set_fpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "FF"]
        #[inline(always)]
        pub const fn ff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "FF"]
        #[inline(always)]
        pub fn set_ff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "RFL"]
        #[inline(always)]
        pub const fn rfl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "RFL"]
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
    #[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxgfc(pub u32);
    impl Rxgfc {
        #[doc = "RRFE"]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RRFE"]
        #[inline(always)]
        pub fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RRFS"]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RRFS"]
        #[inline(always)]
        pub fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ANFE"]
        #[inline(always)]
        pub const fn anfe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "ANFE"]
        #[inline(always)]
        pub fn set_anfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "ANFS"]
        #[inline(always)]
        pub const fn anfs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "ANFS"]
        #[inline(always)]
        pub fn set_anfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "FIFO 1 operation mode"]
        #[inline(always)]
        pub const fn f1om(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 1 operation mode"]
        #[inline(always)]
        pub fn set_f1om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO 0 operation mode"]
        #[inline(always)]
        pub const fn f0om(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 operation mode"]
        #[inline(always)]
        pub fn set_f0om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "List size standard"]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "List size standard"]
        #[inline(always)]
        pub fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "List size extended"]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "List size extended"]
        #[inline(always)]
        pub fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Rxgfc {
        #[inline(always)]
        fn default() -> Rxgfc {
            Rxgfc(0)
        }
    }
    #[doc = "FDCAN Transmitter Delay Compensation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdcr(pub u32);
    impl Tdcr {
        #[doc = "TDCF"]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "TDCF"]
        #[inline(always)]
        pub fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "TDCO"]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "TDCO"]
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
    #[doc = "Write access to the Test Register has to be enabled by setting bit CCCR\\[TEST\\]
to 1 . All Test Register functions are set to their reset values when bit CCCR\\[TEST\\]
is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "LBCK"]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LBCK"]
        #[inline(always)]
        pub fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TX"]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "TX"]
        #[inline(always)]
        pub fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "RX"]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RX"]
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
        #[doc = "ETOC"]
        #[inline(always)]
        pub const fn etoc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ETOC"]
        #[inline(always)]
        pub fn set_etoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TOS"]
        #[inline(always)]
        pub const fn tos(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "TOS"]
        #[inline(always)]
        pub fn set_tos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "TOP"]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "TOP"]
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
        #[doc = "TOC"]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "TOC"]
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
        #[doc = "TSS"]
        #[inline(always)]
        pub const fn tss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TSS"]
        #[inline(always)]
        pub fn set_tss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TCP"]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TCP"]
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
        #[doc = "TSC"]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "TSC"]
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
    #[doc = "FDCAN Tx Buffer Add Request Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbar(pub u32);
    impl Txbar {
        #[doc = "AR"]
        #[inline(always)]
        pub const fn ar(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "AR"]
        #[inline(always)]
        pub fn set_ar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "TBSA"]
        #[inline(always)]
        pub const fn tbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "TBSA"]
        #[inline(always)]
        pub fn set_tbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "NDTB"]
        #[inline(always)]
        pub const fn ndtb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "NDTB"]
        #[inline(always)]
        pub fn set_ndtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "TFQS"]
        #[inline(always)]
        pub const fn tfqs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "TFQS"]
        #[inline(always)]
        pub fn set_tfqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "TFQM"]
        #[inline(always)]
        pub const fn tfqm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TFQM"]
        #[inline(always)]
        pub fn set_tfqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
        #[doc = "CF"]
        #[inline(always)]
        pub const fn cf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "CF"]
        #[inline(always)]
        pub fn set_cf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "CFIE"]
        #[inline(always)]
        pub const fn cfie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "CFIE"]
        #[inline(always)]
        pub fn set_cfie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "CR"]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "CR"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "TRP"]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "TRP"]
        #[inline(always)]
        pub fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "TIE"]
        #[inline(always)]
        pub const fn tie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "TIE"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "TO"]
        #[inline(always)]
        pub const fn to(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "TO"]
        #[inline(always)]
        pub fn set_to(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        #[doc = "EFAI"]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "EFAI"]
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
    #[doc = "FDCAN Tx Event FIFO Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefs(pub u32);
    impl Txefs {
        #[doc = "EFFL"]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "EFFL"]
        #[inline(always)]
        pub fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "EFGI"]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "EFGI"]
        #[inline(always)]
        pub fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "EFPI"]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "EFPI"]
        #[inline(always)]
        pub fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "EFF"]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EFF"]
        #[inline(always)]
        pub fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TEFL"]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TEFL"]
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
    #[doc = "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfqs(pub u32);
    impl Txfqs {
        #[doc = "TFFL"]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "TFFL"]
        #[inline(always)]
        pub fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TFGI"]
        #[inline(always)]
        pub fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TFQPI"]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "TFQPI"]
        #[inline(always)]
        pub fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "TFQF"]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TFQF"]
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
        #[doc = "EIDM"]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "EIDM"]
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
}
