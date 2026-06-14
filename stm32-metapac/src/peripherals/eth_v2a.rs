#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eth {
    ptr: *mut u8,
}
unsafe impl Send for Eth {}
unsafe impl Sync for Eth {}
impl Eth {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ethernet_mac(self) -> EthernetMac {
        unsafe { EthernetMac::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ethernet_mtl(self) -> EthernetMtl {
        unsafe { EthernetMtl::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn ethernet_dma(self) -> EthernetDma {
        unsafe { EthernetDma::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetDma {
    ptr: *mut u8,
}
unsafe impl Send for EthernetDma {}
unsafe impl Sync for EthernetDma {}
impl EthernetDma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA mode register."]
    #[inline(always)]
    pub const fn dmamr(self) -> crate::common::Reg<regs::Dmamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "System bus mode register."]
    #[inline(always)]
    pub const fn dmasbmr(self) -> crate::common::Reg<regs::Dmasbmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn dmaisr(self) -> crate::common::Reg<regs::Dmaisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Debug status register."]
    #[inline(always)]
    pub const fn dmadsr(self) -> crate::common::Reg<regs::Dmadsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AXI4 transmit channel ACE control register."]
    #[inline(always)]
    pub const fn dmaa4txacr(self) -> crate::common::Reg<regs::Dmaa4txacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "AXI4 receive channel ACE control register."]
    #[inline(always)]
    pub const fn dmaa4rxacr(self) -> crate::common::Reg<regs::Dmaa4rxacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AXI4 descriptor ACE control register."]
    #[inline(always)]
    pub const fn dmaa4dacr(self) -> crate::common::Reg<regs::Dmaa4dacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AXI4 LPI Entry Interval register."]
    #[inline(always)]
    pub const fn dmalpiei(self) -> crate::common::Reg<regs::Dmalpiei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "DMA TBS control register 0."]
    #[inline(always)]
    pub const fn dmatbsctrl0r(self) -> crate::common::Reg<regs::Dmatbsctrl0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Channel control register."]
    #[inline(always)]
    pub const fn dmaccr(self, n: usize) -> crate::common::Reg<regs::Dmaccr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
    #[doc = "Channel transmit control register."]
    #[inline(always)]
    pub const fn dmac_tx_cr(self, n: usize) -> crate::common::Reg<regs::DmacTxCr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 128usize) as _) }
    }
    #[doc = "Channel receive control register."]
    #[inline(always)]
    pub const fn dmac_rx_cr(self, n: usize) -> crate::common::Reg<regs::DmacRxCr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize + n * 128usize) as _) }
    }
    #[doc = "Channel Tx descriptor list address register."]
    #[inline(always)]
    pub const fn dmac_tx_dlar(self, n: usize) -> crate::common::Reg<regs::DmacTxDlar, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize + n * 128usize) as _) }
    }
    #[doc = "Channel Rx descriptor list address register."]
    #[inline(always)]
    pub const fn dmac_rx_dlar(self, n: usize) -> crate::common::Reg<regs::DmacRxDlar, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize + n * 128usize) as _) }
    }
    #[doc = "Channel Tx descriptor tail pointer register."]
    #[inline(always)]
    pub const fn dmac_tx_dtpr(self, n: usize) -> crate::common::Reg<regs::DmacTxDtpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 128usize) as _) }
    }
    #[doc = "Channel Rx descriptor tail pointer register."]
    #[inline(always)]
    pub const fn dmac_rx_dtpr(self, n: usize) -> crate::common::Reg<regs::DmacRxDtpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize + n * 128usize) as _) }
    }
    #[doc = "Channel Tx descriptor ring length register."]
    #[inline(always)]
    pub const fn dmac_tx_rlr(self, n: usize) -> crate::common::Reg<regs::DmacTxRlr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize + n * 128usize) as _) }
    }
    #[doc = "Channel Rx descriptor ring length register."]
    #[inline(always)]
    pub const fn dmac_rx_rlr(self, n: usize) -> crate::common::Reg<regs::DmacRxRlr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize + n * 128usize) as _) }
    }
    #[doc = "Channel interrupt enable register."]
    #[inline(always)]
    pub const fn dmacier(self, n: usize) -> crate::common::Reg<regs::Dmacier, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize + n * 128usize) as _) }
    }
    #[doc = "Channel Rx interrupt watchdog timer register."]
    #[inline(always)]
    pub const fn dmac_rx_iwtr(self, n: usize) -> crate::common::Reg<regs::DmacRxIwtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize + n * 128usize) as _) }
    }
    #[doc = "Channel slot function control status register."]
    #[inline(always)]
    pub const fn dmacsfcsr(self, n: usize) -> crate::common::Reg<regs::Dmacsfcsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize + n * 128usize) as _) }
    }
    #[doc = "Channel current application transmit descriptor register."]
    #[inline(always)]
    pub const fn dmacca_tx_dr(self, n: usize) -> crate::common::Reg<regs::DmaccaTxDr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize + n * 128usize) as _) }
    }
    #[doc = "Channel current application receive descriptor register."]
    #[inline(always)]
    pub const fn dmacca_rx_dr(self, n: usize) -> crate::common::Reg<regs::DmaccaRxDr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize + n * 128usize) as _) }
    }
    #[doc = "Channel current application transmit buffer register."]
    #[inline(always)]
    pub const fn dmacca_tx_br(self, n: usize) -> crate::common::Reg<regs::DmaccaTxBr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize + n * 128usize) as _) }
    }
    #[doc = "Channel current application receive buffer register."]
    #[inline(always)]
    pub const fn dmacca_rx_br(self, n: usize) -> crate::common::Reg<regs::DmaccaRxBr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize + n * 128usize) as _) }
    }
    #[doc = "Channel status register."]
    #[inline(always)]
    pub const fn dmacsr(self, n: usize) -> crate::common::Reg<regs::Dmacsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize + n * 128usize) as _) }
    }
    #[doc = "Channel missed frame count register."]
    #[inline(always)]
    pub const fn dmacmfcr(self, n: usize) -> crate::common::Reg<regs::Dmacmfcr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize + n * 128usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMac {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMac {}
unsafe impl Sync for EthernetMac {}
impl EthernetMac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operating mode configuration register."]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Extended operating mode configuration register."]
    #[inline(always)]
    pub const fn macecr(self) -> crate::common::Reg<regs::Macecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Packet filtering control register."]
    #[inline(always)]
    pub const fn macpfr(self) -> crate::common::Reg<regs::Macpfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog timeout register."]
    #[inline(always)]
    pub const fn macwtr(self) -> crate::common::Reg<regs::Macwtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Hash table register."]
    #[inline(always)]
    pub const fn machtr(self, n: usize) -> crate::common::Reg<regs::Machtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "VLAN tag Control register."]
    #[inline(always)]
    pub const fn macvtcr(self) -> crate::common::Reg<regs::Macvtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "VLAN tag data register."]
    #[inline(always)]
    pub const fn macvtdr(self) -> crate::common::Reg<regs::Macvtdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "VLAN Hash table register."]
    #[inline(always)]
    pub const fn macvhtr(self) -> crate::common::Reg<regs::Macvhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "VLAN inclusion register."]
    #[inline(always)]
    pub const fn macvir(self) -> crate::common::Reg<regs::Macvir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Inner VLAN inclusion register."]
    #[inline(always)]
    pub const fn macivir(self) -> crate::common::Reg<regs::Macivir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Tx Queue 0 flow control register."]
    #[inline(always)]
    pub const fn macq_tx_fcr(self) -> crate::common::Reg<regs::MacqTxFcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Rx flow control register."]
    #[inline(always)]
    pub const fn mac_rx_fcr(self) -> crate::common::Reg<regs::MacRxFcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Rx Queue control register."]
    #[inline(always)]
    pub const fn macrxqcr(self) -> crate::common::Reg<regs::Macrxqcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Rx queue control 0 register."]
    #[inline(always)]
    pub const fn macrxqc0r(self) -> crate::common::Reg<regs::Macrxqc0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Rx queue control 1 register."]
    #[inline(always)]
    pub const fn macrxqc1r(self) -> crate::common::Reg<regs::Macrxqc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Rx queue control 2 register."]
    #[inline(always)]
    pub const fn macrxqc2r(self) -> crate::common::Reg<regs::Macrxqc2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn macisr(self) -> crate::common::Reg<regs::Macisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn macier(self) -> crate::common::Reg<regs::Macier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Rx Tx status register."]
    #[inline(always)]
    pub const fn mac_rx_tx_sr(self) -> crate::common::Reg<regs::MacRxTxSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "PMT control status register."]
    #[inline(always)]
    pub const fn macpcsr(self) -> crate::common::Reg<regs::Macpcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Remote wake-up packet filter register."]
    #[inline(always)]
    pub const fn macrwkpfr(self) -> crate::common::Reg<regs::Macrwkpfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "LPI control and status register."]
    #[inline(always)]
    pub const fn maclcsr(self) -> crate::common::Reg<regs::Maclcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "LPI timers control register."]
    #[inline(always)]
    pub const fn macltcr(self) -> crate::common::Reg<regs::Macltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "LPI entry timer register."]
    #[inline(always)]
    pub const fn macletr(self) -> crate::common::Reg<regs::Macletr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "One-microsecond-tick counter register."]
    #[inline(always)]
    pub const fn mac1ustcr(self) -> crate::common::Reg<regs::Mac1ustcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "PHYIF control status register."]
    #[inline(always)]
    pub const fn macphycsr(self) -> crate::common::Reg<regs::Macphycsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Version register."]
    #[inline(always)]
    pub const fn macvr(self) -> crate::common::Reg<regs::Macvr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Debug register."]
    #[inline(always)]
    pub const fn macdr(self) -> crate::common::Reg<regs::Macdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "HW feature 0 register."]
    #[inline(always)]
    pub const fn machwf0r(self) -> crate::common::Reg<regs::Machwf0r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "HW feature 1 register."]
    #[inline(always)]
    pub const fn machwf1r(self) -> crate::common::Reg<regs::Machwf1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "HW feature 2 register."]
    #[inline(always)]
    pub const fn machwf2r(self) -> crate::common::Reg<regs::Machwf2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "HW feature 3 register."]
    #[inline(always)]
    pub const fn machwf3r(self) -> crate::common::Reg<regs::Machwf3r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "MDIO address register."]
    #[inline(always)]
    pub const fn macmdioar(self) -> crate::common::Reg<regs::Macmdioar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MDIO data register."]
    #[inline(always)]
    pub const fn macmdiodr(self) -> crate::common::Reg<regs::Macmdiodr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "ARP address register."]
    #[inline(always)]
    pub const fn macarpar(self) -> crate::common::Reg<regs::Macarpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "CSR software control register."]
    #[inline(always)]
    pub const fn maccsrswcr(self) -> crate::common::Reg<regs::Maccsrswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "FPE control and status register."]
    #[inline(always)]
    pub const fn macfpecsr(self) -> crate::common::Reg<regs::Macfpecsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "MAC presentation time register."]
    #[inline(always)]
    pub const fn macprstimr(self) -> crate::common::Reg<regs::Macprstimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "MAC presentation time update register."]
    #[inline(always)]
    pub const fn macprstimur(self) -> crate::common::Reg<regs::Macprstimur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "MAC Address 0 high register."]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "MAC Address 0 low register."]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "MAC address high register."]
    #[inline(always)]
    pub const fn macahr(self, n: usize) -> crate::common::Reg<regs::Macahr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize + n * 8usize) as _) }
    }
    #[doc = "MAC address low register."]
    #[inline(always)]
    pub const fn macalr(self, n: usize) -> crate::common::Reg<regs::Macalr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize + n * 8usize) as _) }
    }
    #[doc = "MMC control register."]
    #[inline(always)]
    pub const fn mmc_control(self) -> crate::common::Reg<regs::MmcControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "MMC Rx interrupt register."]
    #[inline(always)]
    pub const fn mmc_rx_interrupt(self) -> crate::common::Reg<regs::MmcRxInterrupt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "MMC Tx interrupt register."]
    #[inline(always)]
    pub const fn mmc_tx_interrupt(self) -> crate::common::Reg<regs::MmcTxInterrupt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "MMC Rx interrupt mask register."]
    #[inline(always)]
    pub const fn mmc_rx_interrupt_mask(self) -> crate::common::Reg<regs::MmcRxInterruptMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "MMC Tx interrupt mask register."]
    #[inline(always)]
    pub const fn mmc_tx_interrupt_mask(self) -> crate::common::Reg<regs::MmcTxInterruptMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "Tx single collision good packets register."]
    #[inline(always)]
    pub const fn tx_single_collision_good_packets(
        self,
    ) -> crate::common::Reg<regs::TxSingleCollisionGoodPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[doc = "Tx multiple collision good packets register."]
    #[inline(always)]
    pub const fn tx_multiple_collision_good_packets(
        self,
    ) -> crate::common::Reg<regs::TxMultipleCollisionGoodPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "Tx packet count good register."]
    #[inline(always)]
    pub const fn tx_packet_count_good(self) -> crate::common::Reg<regs::TxPacketCountGood, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "Rx CRC error packets register."]
    #[inline(always)]
    pub const fn rx_crc_error_packets(self) -> crate::common::Reg<regs::RxCrcErrorPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[doc = "Rx alignment error packets register."]
    #[inline(always)]
    pub const fn rx_alignment_error_packets(
        self,
    ) -> crate::common::Reg<regs::RxAlignmentErrorPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[doc = "Rx unicast packets good register."]
    #[inline(always)]
    pub const fn rx_unicast_packets_good(self) -> crate::common::Reg<regs::RxUnicastPacketsGood, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    #[doc = "Tx LPI microsecond timer register."]
    #[inline(always)]
    pub const fn tx_lpi_usec_cntr(self) -> crate::common::Reg<regs::TxLpiUsecCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ecusize) as _) }
    }
    #[doc = "Tx LPI transition counter register."]
    #[inline(always)]
    pub const fn tx_lpi_tran_cntr(self) -> crate::common::Reg<regs::TxLpiTranCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[doc = "Rx LPI microsecond counter register."]
    #[inline(always)]
    pub const fn rx_lpi_usec_cntr(self) -> crate::common::Reg<regs::RxLpiUsecCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[doc = "Rx LPI transition counter register."]
    #[inline(always)]
    pub const fn rx_lpi_tran_cntr(self) -> crate::common::Reg<regs::RxLpiTranCntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
    #[doc = "MMC FPE Tx interrupt status register."]
    #[inline(always)]
    pub const fn mmc_fpe_tx_isr(self) -> crate::common::Reg<regs::MmcFpeTxIsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "MMC FPE Tx interrupt mask register."]
    #[inline(always)]
    pub const fn mmc_fpe_tx_imr(self) -> crate::common::Reg<regs::MmcFpeTxImr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a4usize) as _) }
    }
    #[doc = "MMC FPE Tx fragment counter register."]
    #[inline(always)]
    pub const fn mmc_fpe_tx_fcr(self) -> crate::common::Reg<regs::MmcFpeTxFcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a8usize) as _) }
    }
    #[doc = "MMC Tx hold request counter register."]
    #[inline(always)]
    pub const fn mmc_tx_hrcr(self) -> crate::common::Reg<regs::MmcTxHrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08acusize) as _) }
    }
    #[doc = "MMC FPE Rx interrupt status register."]
    #[inline(always)]
    pub const fn mmc_fpe_rx_isr(self) -> crate::common::Reg<regs::MmcFpeRxIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[doc = "MMC FPE Rx interrupt mask register."]
    #[inline(always)]
    pub const fn mmc_fpe_rx_imr(self) -> crate::common::Reg<regs::MmcFpeRxImr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c4usize) as _) }
    }
    #[doc = "MMC Rx packet assembly error register."]
    #[inline(always)]
    pub const fn rx_packet_asm_err(self) -> crate::common::Reg<regs::RxPacketAsmErr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c8usize) as _) }
    }
    #[doc = "MMC Rx packet SMD error register."]
    #[inline(always)]
    pub const fn rx_packet_smd_err(self) -> crate::common::Reg<regs::RxPacketSmdErr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08ccusize) as _) }
    }
    #[doc = "MMC Rx packet assembly OK register."]
    #[inline(always)]
    pub const fn rx_packet_asm_okr(self) -> crate::common::Reg<regs::RxPacketAsmOkr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[doc = "MMC Rx FPE fragments counter register."]
    #[inline(always)]
    pub const fn rx_fpe_frag_cr(self) -> crate::common::Reg<regs::RxFpeFragCr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d4usize) as _) }
    }
    #[doc = "L3 and L4 control 0 register."]
    #[inline(always)]
    pub const fn macl3l4c0r(self) -> crate::common::Reg<regs::Macl3l4c0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Layer4 Address filter 0 register."]
    #[inline(always)]
    pub const fn macl4a0r(self) -> crate::common::Reg<regs::Macl4a0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "Layer3 Address 0 filter 0 register."]
    #[inline(always)]
    pub const fn macl3a00r(self) -> crate::common::Reg<regs::Macl3a00r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[doc = "Layer3 Address 1 filter 0 register."]
    #[inline(always)]
    pub const fn macl3a10r(self) -> crate::common::Reg<regs::Macl3a10r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    #[doc = "Layer3 Address 2 filter 0 register."]
    #[inline(always)]
    pub const fn macl3a20r(self) -> crate::common::Reg<regs::Macl3a20r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[doc = "Layer3 Address 3 filter 0 register."]
    #[inline(always)]
    pub const fn macl3a30r(self) -> crate::common::Reg<regs::Macl3a30r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x091cusize) as _) }
    }
    #[doc = "L3 and L4 control 1 register."]
    #[inline(always)]
    pub const fn macl3l4c1r(self) -> crate::common::Reg<regs::Macl3l4c1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[doc = "Layer 4 address filter 1 register."]
    #[inline(always)]
    pub const fn macl4a1r(self) -> crate::common::Reg<regs::Macl4a1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[doc = "Layer3 address 0 filter 1 Register."]
    #[inline(always)]
    pub const fn macl3a01r(self) -> crate::common::Reg<regs::Macl3a01r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "Layer3 address 1 filter 1 register."]
    #[inline(always)]
    pub const fn macl3a11r(self) -> crate::common::Reg<regs::Macl3a11r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[doc = "Layer3 address 2 filter 1 Register."]
    #[inline(always)]
    pub const fn macl3a21r(self) -> crate::common::Reg<regs::Macl3a21r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0948usize) as _) }
    }
    #[doc = "Layer3 address 3 filter 1 register."]
    #[inline(always)]
    pub const fn macl3a31r(self) -> crate::common::Reg<regs::Macl3a31r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x094cusize) as _) }
    }
    #[doc = "MAC Indirect Access Control register."]
    #[inline(always)]
    pub const fn mac_iacr(self) -> crate::common::Reg<regs::MacIacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a70usize) as _) }
    }
    #[doc = "MAC type-based Rx Queue mapping register."]
    #[inline(always)]
    pub const fn mac_tmrqr(self) -> crate::common::Reg<regs::MacTmrqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a74usize) as _) }
    }
    #[doc = "Timestamp control Register."]
    #[inline(always)]
    pub const fn mactscr(self) -> crate::common::Reg<regs::Mactscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Subsecond increment register."]
    #[inline(always)]
    pub const fn macssir(self) -> crate::common::Reg<regs::Macssir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "System time seconds register."]
    #[inline(always)]
    pub const fn macstsr(self) -> crate::common::Reg<regs::Macstsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "System time nanoseconds register."]
    #[inline(always)]
    pub const fn macstnr(self) -> crate::common::Reg<regs::Macstnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "System time seconds update register."]
    #[inline(always)]
    pub const fn macstsur(self) -> crate::common::Reg<regs::Macstsur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "System time nanoseconds update register."]
    #[inline(always)]
    pub const fn macstnur(self) -> crate::common::Reg<regs::Macstnur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Timestamp addend register."]
    #[inline(always)]
    pub const fn mactsar(self) -> crate::common::Reg<regs::Mactsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Timestamp status register."]
    #[inline(always)]
    pub const fn mactssr(self) -> crate::common::Reg<regs::Mactssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Tx timestamp status nanoseconds register."]
    #[inline(always)]
    pub const fn mac_tx_tssnr(self) -> crate::common::Reg<regs::MacTxTssnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Tx timestamp status seconds register."]
    #[inline(always)]
    pub const fn mac_tx_tsssr(self) -> crate::common::Reg<regs::MacTxTsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "Auxiliary control register."]
    #[inline(always)]
    pub const fn macacr(self) -> crate::common::Reg<regs::Macacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "Auxiliary timestamp nanoseconds register."]
    #[inline(always)]
    pub const fn macatsnr(self) -> crate::common::Reg<regs::Macatsnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b48usize) as _) }
    }
    #[doc = "Auxiliary timestamp seconds register."]
    #[inline(always)]
    pub const fn macatssr(self) -> crate::common::Reg<regs::Macatssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b4cusize) as _) }
    }
    #[doc = "Timestamp Ingress asymmetric correction register."]
    #[inline(always)]
    pub const fn mactsiacr(self) -> crate::common::Reg<regs::Mactsiacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[doc = "Timestamp Egress asymmetric correction register."]
    #[inline(always)]
    pub const fn mactseacr(self) -> crate::common::Reg<regs::Mactseacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[doc = "Timestamp Ingress correction nanosecond register."]
    #[inline(always)]
    pub const fn mactsicnr(self) -> crate::common::Reg<regs::Mactsicnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "Timestamp Egress correction nanosecond register."]
    #[inline(always)]
    pub const fn mactsecnr(self) -> crate::common::Reg<regs::Mactsecnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "Timestamp Ingress Latency register."]
    #[inline(always)]
    pub const fn mactsilr(self) -> crate::common::Reg<regs::Mactsilr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Timestamp Egress Latency register."]
    #[inline(always)]
    pub const fn mactselr(self) -> crate::common::Reg<regs::Mactselr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "PPS control register."]
    #[inline(always)]
    pub const fn macppscr(self) -> crate::common::Reg<regs::Macppscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[doc = "PPS target time seconds register."]
    #[inline(always)]
    pub const fn macppsttsr(self, n: usize) -> crate::common::Reg<regs::Macppsttsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize + n * 16usize) as _) }
    }
    #[doc = "PPS target time nanoseconds register."]
    #[inline(always)]
    pub const fn macppsttnr(self, n: usize) -> crate::common::Reg<regs::Macppsttnr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b84usize + n * 16usize) as _) }
    }
    #[doc = "PPS interval register."]
    #[inline(always)]
    pub const fn macppsir(self, n: usize) -> crate::common::Reg<regs::Macppsir, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b88usize + n * 16usize) as _) }
    }
    #[doc = "PPS width register."]
    #[inline(always)]
    pub const fn macppswr(self, n: usize) -> crate::common::Reg<regs::Macppswr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b8cusize + n * 16usize) as _) }
    }
    #[doc = "PTP Offload control register."]
    #[inline(always)]
    pub const fn macpocr(self) -> crate::common::Reg<regs::Macpocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc0usize) as _) }
    }
    #[doc = "PTP Source Port Identity 0 Register."]
    #[inline(always)]
    pub const fn macspi0r(self) -> crate::common::Reg<regs::Macspi0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc4usize) as _) }
    }
    #[doc = "PTP Source port identity 1 register."]
    #[inline(always)]
    pub const fn macspi1r(self) -> crate::common::Reg<regs::Macspi1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc8usize) as _) }
    }
    #[doc = "PTP Source port identity 2 register."]
    #[inline(always)]
    pub const fn macspi2r(self) -> crate::common::Reg<regs::Macspi2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bccusize) as _) }
    }
    #[doc = "Log message interval register."]
    #[inline(always)]
    pub const fn maclmir(self) -> crate::common::Reg<regs::Maclmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd0usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMtl {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMtl {}
unsafe impl Sync for EthernetMtl {}
impl EthernetMtl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operating mode Register."]
    #[inline(always)]
    pub const fn mtlomr(self) -> crate::common::Reg<regs::Mtlomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt status Register."]
    #[inline(always)]
    pub const fn mtlisr(self) -> crate::common::Reg<regs::Mtlisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Rx Queue and DMA Channel Mapping Register."]
    #[inline(always)]
    pub const fn mtlrxqdmamr(self) -> crate::common::Reg<regs::Mtlrxqdmamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TBS control register."]
    #[inline(always)]
    pub const fn mtltbscr(self) -> crate::common::Reg<regs::Mtltbscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "EST Control Register."]
    #[inline(always)]
    pub const fn mtlestcr(self) -> crate::common::Reg<regs::Mtlestcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "EST Extended Control Register."]
    #[inline(always)]
    pub const fn mtlestecr(self) -> crate::common::Reg<regs::Mtlestecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "EST Status Register."]
    #[inline(always)]
    pub const fn mtlestsr(self) -> crate::common::Reg<regs::Mtlestsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "EST Schedule Error Register."]
    #[inline(always)]
    pub const fn mtlestscher(self) -> crate::common::Reg<regs::Mtlestscher, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "EST Frame size Error Register."]
    #[inline(always)]
    pub const fn mtlestfser(self) -> crate::common::Reg<regs::Mtlestfser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "EST Frame size Capture Register."]
    #[inline(always)]
    pub const fn mtlestfscr(self) -> crate::common::Reg<regs::Mtlestfscr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "EST Interrupt Enable Register."]
    #[inline(always)]
    pub const fn mtlestier(self) -> crate::common::Reg<regs::Mtlestier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "EST Gate Control List Register."]
    #[inline(always)]
    pub const fn mtlestgclcr(self) -> crate::common::Reg<regs::Mtlestgclcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "EST Gate Control List Data Register."]
    #[inline(always)]
    pub const fn mtlestgcldr(self) -> crate::common::Reg<regs::Mtlestgcldr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FPE Frame Preemption Control Status Register."]
    #[inline(always)]
    pub const fn mtlfpecsr(self) -> crate::common::Reg<regs::Mtlfpecsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FPE Frame Preemption Advance Register."]
    #[inline(always)]
    pub const fn mtlfpear(self) -> crate::common::Reg<regs::Mtlfpear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Tx queue operating mode Register."]
    #[inline(always)]
    pub const fn mtl_tx_qomr(self, n: usize) -> crate::common::Reg<regs::MtlTxQomr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 64usize) as _) }
    }
    #[doc = "Tx queue underflow register."]
    #[inline(always)]
    pub const fn mtl_tx_qur(self, n: usize) -> crate::common::Reg<regs::MtlTxQur, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 64usize) as _) }
    }
    #[doc = "Tx queue debug register."]
    #[inline(always)]
    pub const fn mtl_tx_qdr(self, n: usize) -> crate::common::Reg<regs::MtlTxQdr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize + n * 64usize) as _) }
    }
    #[doc = "Tx queue ETS status Register."]
    #[inline(always)]
    pub const fn mtl_tx_qesr(self, n: usize) -> crate::common::Reg<regs::MtlTxQesr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize + n * 64usize) as _) }
    }
    #[doc = "Tx queue quantum weight register."]
    #[inline(always)]
    pub const fn mtl_tx_qqwr(self, n: usize) -> crate::common::Reg<regs::MtlTxQqwr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize + n * 64usize) as _) }
    }
    #[doc = "Queue interrupt control status Register."]
    #[inline(always)]
    pub const fn mtlqicsr(self, n: usize) -> crate::common::Reg<regs::Mtlqicsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize + n * 64usize) as _) }
    }
    #[doc = "Rx queue operating mode register."]
    #[inline(always)]
    pub const fn mtl_rx_qomr(self, n: usize) -> crate::common::Reg<regs::MtlRxQomr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize + n * 64usize) as _) }
    }
    #[doc = "Rx queue missed packet and overflow counter register."]
    #[inline(always)]
    pub const fn mtl_rx_qmpocr(self, n: usize) -> crate::common::Reg<regs::MtlRxQmpocr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize + n * 64usize) as _) }
    }
    #[doc = "Rx queue debug register."]
    #[inline(always)]
    pub const fn mtl_rx_qdr(self, n: usize) -> crate::common::Reg<regs::MtlRxQdr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize + n * 64usize) as _) }
    }
    #[doc = "Rx queue control register."]
    #[inline(always)]
    pub const fn mtl_rx_qcr(self, n: usize) -> crate::common::Reg<regs::MtlRxQcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize + n * 64usize) as _) }
    }
    #[doc = "Tx queue 1 ETS control Register."]
    #[inline(always)]
    pub const fn mtl_tx_q1ecr(self) -> crate::common::Reg<regs::MtlTxQ1ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Tx queue 1 send slope credit Register."]
    #[inline(always)]
    pub const fn mtl_tx_q1sscr(self) -> crate::common::Reg<regs::MtlTxQ1sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Tx Queue 1 hiCredit register."]
    #[inline(always)]
    pub const fn mtl_tx_q1hcr(self) -> crate::common::Reg<regs::MtlTxQ1hcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Tx queue 1 loCredit register."]
    #[inline(always)]
    pub const fn mtl_tx_q1lcr(self) -> crate::common::Reg<regs::MtlTxQ1lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
}
pub mod regs {
    #[doc = "AXI4 descriptor ACE control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaa4dacr(pub u32);
    impl Dmaa4dacr {
        #[doc = "Transmit DMA Write Descriptor Cache control."]
        #[must_use]
        #[inline(always)]
        pub const fn tdwc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit DMA Write Descriptor Cache control."]
        #[inline(always)]
        pub const fn set_tdwc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Transmit DMA Write Descriptor Domain control."]
        #[must_use]
        #[inline(always)]
        pub const fn tdwd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Transmit DMA Write Descriptor Domain control."]
        #[inline(always)]
        pub const fn set_tdwd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Receive DMA Read Descriptor Cache control."]
        #[must_use]
        #[inline(always)]
        pub const fn rdrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive DMA Read Descriptor Cache control."]
        #[inline(always)]
        pub const fn set_rdrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Dmaa4dacr {
        #[inline(always)]
        fn default() -> Dmaa4dacr {
            Dmaa4dacr(0)
        }
    }
    impl core::fmt::Debug for Dmaa4dacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaa4dacr")
                .field("tdwc", &self.tdwc())
                .field("tdwd", &self.tdwd())
                .field("rdrc", &self.rdrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaa4dacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaa4dacr {{ tdwc: {=u8:?}, tdwd: {=u8:?}, rdrc: {=u8:?} }}",
                self.tdwc(),
                self.tdwd(),
                self.rdrc()
            )
        }
    }
    #[doc = "AXI4 receive channel ACE control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaa4rxacr(pub u32);
    impl Dmaa4rxacr {
        #[doc = "Receive DMA Write Descriptor Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn rdwc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive DMA Write Descriptor Cache Control."]
        #[inline(always)]
        pub const fn set_rdwc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Receive DMA Payload Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn rpc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive DMA Payload Cache Control."]
        #[inline(always)]
        pub const fn set_rpc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Receive DMA Header Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn rhc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive DMA Header Cache Control."]
        #[inline(always)]
        pub const fn set_rhc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Receive DMA Buffer Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn rdc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive DMA Buffer Cache Control."]
        #[inline(always)]
        pub const fn set_rdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Dmaa4rxacr {
        #[inline(always)]
        fn default() -> Dmaa4rxacr {
            Dmaa4rxacr(0)
        }
    }
    impl core::fmt::Debug for Dmaa4rxacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaa4rxacr")
                .field("rdwc", &self.rdwc())
                .field("rpc", &self.rpc())
                .field("rhc", &self.rhc())
                .field("rdc", &self.rdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaa4rxacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaa4rxacr {{ rdwc: {=u8:?}, rpc: {=u8:?}, rhc: {=u8:?}, rdc: {=u8:?} }}",
                self.rdwc(),
                self.rpc(),
                self.rhc(),
                self.rdc()
            )
        }
    }
    #[doc = "AXI4 transmit channel ACE control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaa4txacr(pub u32);
    impl Dmaa4txacr {
        #[doc = "Transmit DMA Read Descriptor Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn tdrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit DMA Read Descriptor Cache Control."]
        #[inline(always)]
        pub const fn set_tdrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Transmit DMA Extended Packet Buffer or TSO Payload Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit DMA Extended Packet Buffer or TSO Payload Cache Control."]
        #[inline(always)]
        pub const fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Transmit DMA First Packet Buffer or TSO Header Cache Control."]
        #[must_use]
        #[inline(always)]
        pub const fn thc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit DMA First Packet Buffer or TSO Header Cache Control."]
        #[inline(always)]
        pub const fn set_thc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Dmaa4txacr {
        #[inline(always)]
        fn default() -> Dmaa4txacr {
            Dmaa4txacr(0)
        }
    }
    impl core::fmt::Debug for Dmaa4txacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaa4txacr")
                .field("tdrc", &self.tdrc())
                .field("tec", &self.tec())
                .field("thc", &self.thc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaa4txacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaa4txacr {{ tdrc: {=u8:?}, tec: {=u8:?}, thc: {=u8:?} }}",
                self.tdrc(),
                self.tec(),
                self.thc()
            )
        }
    }
    #[doc = "Channel receive control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacRxCr(pub u32);
    impl DmacRxCr {
        #[doc = "Start or Stop Receive."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Receive."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receive Buffer size."]
        #[must_use]
        #[inline(always)]
        pub const fn rbsz(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "Receive Buffer size."]
        #[inline(always)]
        pub const fn set_rbsz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u32) & 0x3fff) << 1usize);
        }
        #[doc = "Receive Programmable Burst Length."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Receive Programmable Burst Length."]
        #[inline(always)]
        pub const fn set_rxpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Rx AXI4 QOS."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Rx AXI4 QOS."]
        #[inline(always)]
        pub const fn set_rqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "DMA Rx Channel x Packet Flush."]
        #[must_use]
        #[inline(always)]
        pub const fn rpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Rx Channel x Packet Flush."]
        #[inline(always)]
        pub const fn set_rpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DmacRxCr {
        #[inline(always)]
        fn default() -> DmacRxCr {
            DmacRxCr(0)
        }
    }
    impl core::fmt::Debug for DmacRxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacRxCr")
                .field("sr", &self.sr())
                .field("rbsz", &self.rbsz())
                .field("rxpbl", &self.rxpbl())
                .field("rqos", &self.rqos())
                .field("rpf", &self.rpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacRxCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmacRxCr {{ sr: {=bool:?}, rbsz: {=u16:?}, rxpbl: {=u8:?}, rqos: {=u8:?}, rpf: {=bool:?} }}",
                self.sr(),
                self.rbsz(),
                self.rxpbl(),
                self.rqos(),
                self.rpf()
            )
        }
    }
    #[doc = "Channel Rx descriptor list address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacRxDlar(pub u32);
    impl DmacRxDlar {
        #[doc = "Start of Receive List."]
        #[must_use]
        #[inline(always)]
        pub const fn rdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Receive List."]
        #[inline(always)]
        pub const fn set_rdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacRxDlar {
        #[inline(always)]
        fn default() -> DmacRxDlar {
            DmacRxDlar(0)
        }
    }
    impl core::fmt::Debug for DmacRxDlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacRxDlar").field("rdesla", &self.rdesla()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacRxDlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacRxDlar {{ rdesla: {=u32:?} }}", self.rdesla())
        }
    }
    #[doc = "Channel Rx descriptor tail pointer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacRxDtpr(pub u32);
    impl DmacRxDtpr {
        #[doc = "Receive Descriptor Tail Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn rdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive Descriptor Tail Pointer."]
        #[inline(always)]
        pub const fn set_rdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacRxDtpr {
        #[inline(always)]
        fn default() -> DmacRxDtpr {
            DmacRxDtpr(0)
        }
    }
    impl core::fmt::Debug for DmacRxDtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacRxDtpr").field("rdt", &self.rdt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacRxDtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacRxDtpr {{ rdt: {=u32:?} }}", self.rdt())
        }
    }
    #[doc = "Channel Rx interrupt watchdog timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacRxIwtr(pub u32);
    impl DmacRxIwtr {
        #[doc = "Receive Interrupt Watchdog Timer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive Interrupt Watchdog Timer Count."]
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive Interrupt Watchdog Timer Count Units."]
        #[must_use]
        #[inline(always)]
        pub const fn rwtu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Interrupt Watchdog Timer Count Units."]
        #[inline(always)]
        pub const fn set_rwtu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for DmacRxIwtr {
        #[inline(always)]
        fn default() -> DmacRxIwtr {
            DmacRxIwtr(0)
        }
    }
    impl core::fmt::Debug for DmacRxIwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacRxIwtr")
                .field("rwt", &self.rwt())
                .field("rwtu", &self.rwtu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacRxIwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmacRxIwtr {{ rwt: {=u8:?}, rwtu: {=u8:?} }}",
                self.rwt(),
                self.rwtu()
            )
        }
    }
    #[doc = "Channel Rx descriptor ring length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacRxRlr(pub u32);
    impl DmacRxRlr {
        #[doc = "Receive Descriptor Ring Length."]
        #[must_use]
        #[inline(always)]
        pub const fn rdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Receive Descriptor Ring Length."]
        #[inline(always)]
        pub const fn set_rdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Alternate Receive Buffer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn arbs(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "Alternate Receive Buffer Size."]
        #[inline(always)]
        pub const fn set_arbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for DmacRxRlr {
        #[inline(always)]
        fn default() -> DmacRxRlr {
            DmacRxRlr(0)
        }
    }
    impl core::fmt::Debug for DmacRxRlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacRxRlr")
                .field("rdrl", &self.rdrl())
                .field("arbs", &self.arbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacRxRlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmacRxRlr {{ rdrl: {=u16:?}, arbs: {=u8:?} }}",
                self.rdrl(),
                self.arbs()
            )
        }
    }
    #[doc = "Channel transmit control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacTxCr(pub u32);
    impl DmacTxCr {
        #[doc = "Start or Stop Transmission Command."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Transmission Command."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Channel Weight."]
        #[must_use]
        #[inline(always)]
        pub const fn tcw(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Channel Weight."]
        #[inline(always)]
        pub const fn set_tcw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "Operate on Second Packet."]
        #[must_use]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Operate on Second Packet."]
        #[inline(always)]
        pub const fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TCP Segmentation Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TCP Segmentation Enabled."]
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Ignore PBL Requirement."]
        #[must_use]
        #[inline(always)]
        pub const fn ipbl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore PBL Requirement."]
        #[inline(always)]
        pub const fn set_ipbl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmit Programmable Burst Length."]
        #[must_use]
        #[inline(always)]
        pub const fn txpbl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit Programmable Burst Length."]
        #[inline(always)]
        pub const fn set_txpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Transmit QOS."]
        #[must_use]
        #[inline(always)]
        pub const fn tqos(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit QOS."]
        #[inline(always)]
        pub const fn set_tqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Enhanced Descriptor Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn edse(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Descriptor Enable."]
        #[inline(always)]
        pub const fn set_edse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DmacTxCr {
        #[inline(always)]
        fn default() -> DmacTxCr {
            DmacTxCr(0)
        }
    }
    impl core::fmt::Debug for DmacTxCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacTxCr")
                .field("st", &self.st())
                .field("tcw", &self.tcw())
                .field("osf", &self.osf())
                .field("tse", &self.tse())
                .field("ipbl", &self.ipbl())
                .field("txpbl", &self.txpbl())
                .field("tqos", &self.tqos())
                .field("edse", &self.edse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacTxCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmacTxCr {{ st: {=bool:?}, tcw: {=u8:?}, osf: {=bool:?}, tse: {=bool:?}, ipbl: {=bool:?}, txpbl: {=u8:?}, tqos: {=u8:?}, edse: {=bool:?} }}",
                self.st(),
                self.tcw(),
                self.osf(),
                self.tse(),
                self.ipbl(),
                self.txpbl(),
                self.tqos(),
                self.edse()
            )
        }
    }
    #[doc = "Channel Tx descriptor list address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacTxDlar(pub u32);
    impl DmacTxDlar {
        #[doc = "Start of Transmit List."]
        #[must_use]
        #[inline(always)]
        pub const fn tdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Transmit List."]
        #[inline(always)]
        pub const fn set_tdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacTxDlar {
        #[inline(always)]
        fn default() -> DmacTxDlar {
            DmacTxDlar(0)
        }
    }
    impl core::fmt::Debug for DmacTxDlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacTxDlar").field("tdesla", &self.tdesla()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacTxDlar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacTxDlar {{ tdesla: {=u32:?} }}", self.tdesla())
        }
    }
    #[doc = "Channel Tx descriptor tail pointer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacTxDtpr(pub u32);
    impl DmacTxDtpr {
        #[doc = "Transmit Descriptor Tail Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn tdt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit Descriptor Tail Pointer."]
        #[inline(always)]
        pub const fn set_tdt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmacTxDtpr {
        #[inline(always)]
        fn default() -> DmacTxDtpr {
            DmacTxDtpr(0)
        }
    }
    impl core::fmt::Debug for DmacTxDtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacTxDtpr").field("tdt", &self.tdt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacTxDtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacTxDtpr {{ tdt: {=u32:?} }}", self.tdt())
        }
    }
    #[doc = "Channel Tx descriptor ring length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacTxRlr(pub u32);
    impl DmacTxRlr {
        #[doc = "Transmit Descriptor Ring Length."]
        #[must_use]
        #[inline(always)]
        pub const fn tdrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Transmit Descriptor Ring Length."]
        #[inline(always)]
        pub const fn set_tdrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DmacTxRlr {
        #[inline(always)]
        fn default() -> DmacTxRlr {
            DmacTxRlr(0)
        }
    }
    impl core::fmt::Debug for DmacTxRlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmacTxRlr").field("tdrl", &self.tdrl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmacTxRlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmacTxRlr {{ tdrl: {=u16:?} }}", self.tdrl())
        }
    }
    #[doc = "Channel current application receive buffer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccaRxBr(pub u32);
    impl DmaccaRxBr {
        #[doc = "Application Receive Buffer Address Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn currbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Receive Buffer Address Pointer."]
        #[inline(always)]
        pub const fn set_currbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccaRxBr {
        #[inline(always)]
        fn default() -> DmaccaRxBr {
            DmaccaRxBr(0)
        }
    }
    impl core::fmt::Debug for DmaccaRxBr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccaRxBr")
                .field("currbufaptr", &self.currbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccaRxBr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccaRxBr {{ currbufaptr: {=u32:?} }}", self.currbufaptr())
        }
    }
    #[doc = "Channel current application receive descriptor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccaRxDr(pub u32);
    impl DmaccaRxDr {
        #[doc = "Application Receive Descriptor Address Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn currdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Receive Descriptor Address Pointer."]
        #[inline(always)]
        pub const fn set_currdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccaRxDr {
        #[inline(always)]
        fn default() -> DmaccaRxDr {
            DmaccaRxDr(0)
        }
    }
    impl core::fmt::Debug for DmaccaRxDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccaRxDr")
                .field("currdesaptr", &self.currdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccaRxDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccaRxDr {{ currdesaptr: {=u32:?} }}", self.currdesaptr())
        }
    }
    #[doc = "Channel current application transmit buffer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccaTxBr(pub u32);
    impl DmaccaTxBr {
        #[doc = "Application Transmit Buffer Address Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn curtbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Transmit Buffer Address Pointer."]
        #[inline(always)]
        pub const fn set_curtbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccaTxBr {
        #[inline(always)]
        fn default() -> DmaccaTxBr {
            DmaccaTxBr(0)
        }
    }
    impl core::fmt::Debug for DmaccaTxBr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccaTxBr")
                .field("curtbufaptr", &self.curtbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccaTxBr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccaTxBr {{ curtbufaptr: {=u32:?} }}", self.curtbufaptr())
        }
    }
    #[doc = "Channel current application transmit descriptor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaccaTxDr(pub u32);
    impl DmaccaTxDr {
        #[doc = "Application Transmit Descriptor Address Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn curtdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Application Transmit Descriptor Address Pointer."]
        #[inline(always)]
        pub const fn set_curtdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaccaTxDr {
        #[inline(always)]
        fn default() -> DmaccaTxDr {
            DmaccaTxDr(0)
        }
    }
    impl core::fmt::Debug for DmaccaTxDr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaccaTxDr")
                .field("curtdesaptr", &self.curtdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaccaTxDr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaccaTxDr {{ curtdesaptr: {=u32:?} }}", self.curtdesaptr())
        }
    }
    #[doc = "Channel control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaccr(pub u32);
    impl Dmaccr {
        #[doc = "Maximum Segment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn mss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Maximum Segment Size."]
        #[inline(always)]
        pub const fn set_mss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "8xPBL mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pblx8(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "8xPBL mode."]
        #[inline(always)]
        pub const fn set_pblx8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Descriptor Skip Length."]
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Descriptor Skip Length."]
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
    }
    impl Default for Dmaccr {
        #[inline(always)]
        fn default() -> Dmaccr {
            Dmaccr(0)
        }
    }
    impl core::fmt::Debug for Dmaccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaccr")
                .field("mss", &self.mss())
                .field("pblx8", &self.pblx8())
                .field("dsl", &self.dsl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaccr {{ mss: {=u16:?}, pblx8: {=bool:?}, dsl: {=u8:?} }}",
                self.mss(),
                self.pblx8(),
                self.dsl()
            )
        }
    }
    #[doc = "Channel interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacier(pub u32);
    impl Dmacier {
        #[doc = "Transmit Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Stopped Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Stopped Enable."]
        #[inline(always)]
        pub const fn set_txse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbue(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable Enable."]
        #[inline(always)]
        pub const fn set_tbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rbue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable Enable."]
        #[inline(always)]
        pub const fn set_rbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Stopped Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Stopped Enable."]
        #[inline(always)]
        pub const fn set_rse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rwte(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout Enable."]
        #[inline(always)]
        pub const fn set_rwte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt Enable."]
        #[inline(always)]
        pub const fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Early Receive Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt Enable."]
        #[inline(always)]
        pub const fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Fatal Bus Error Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fbee(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error Enable."]
        #[inline(always)]
        pub const fn set_fbee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Context Descriptor Error Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cdee(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Context Descriptor Error Enable."]
        #[inline(always)]
        pub const fn set_cdee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Abnormal Interrupt Summary Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary Enable."]
        #[inline(always)]
        pub const fn set_aie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Normal Interrupt Summary Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary Enable."]
        #[inline(always)]
        pub const fn set_nie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dmacier {
        #[inline(always)]
        fn default() -> Dmacier {
            Dmacier(0)
        }
    }
    impl core::fmt::Debug for Dmacier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacier")
                .field("tie", &self.tie())
                .field("txse", &self.txse())
                .field("tbue", &self.tbue())
                .field("rie", &self.rie())
                .field("rbue", &self.rbue())
                .field("rse", &self.rse())
                .field("rwte", &self.rwte())
                .field("etie", &self.etie())
                .field("erie", &self.erie())
                .field("fbee", &self.fbee())
                .field("cdee", &self.cdee())
                .field("aie", &self.aie())
                .field("nie", &self.nie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacier {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}",
                self.tie(),
                self.txse(),
                self.tbue(),
                self.rie(),
                self.rbue(),
                self.rse(),
                self.rwte(),
                self.etie(),
                self.erie(),
                self.fbee(),
                self.cdee(),
                self.aie(),
                self.nie()
            )
        }
    }
    #[doc = "Channel missed frame count register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacmfcr(pub u32);
    impl Dmacmfcr {
        #[doc = "Dropped Packet Counters."]
        #[must_use]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Dropped Packet Counters."]
        #[inline(always)]
        pub const fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow status of the MFC Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn mfco(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow status of the MFC Counter."]
        #[inline(always)]
        pub const fn set_mfco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Dmacmfcr {
        #[inline(always)]
        fn default() -> Dmacmfcr {
            Dmacmfcr(0)
        }
    }
    impl core::fmt::Debug for Dmacmfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacmfcr")
                .field("mfc", &self.mfc())
                .field("mfco", &self.mfco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacmfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacmfcr {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
                self.mfc(),
                self.mfco()
            )
        }
    }
    #[doc = "Channel slot function control status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacsfcsr(pub u32);
    impl Dmacsfcsr {
        #[doc = "Enable Slot Comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn esc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Slot Comparison."]
        #[inline(always)]
        pub const fn set_esc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Advance Slot Check."]
        #[must_use]
        #[inline(always)]
        pub const fn asc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Advance Slot Check."]
        #[inline(always)]
        pub const fn set_asc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Slot Interval Value."]
        #[must_use]
        #[inline(always)]
        pub const fn siv(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "Slot Interval Value."]
        #[inline(always)]
        pub const fn set_siv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "Reference Slot Number."]
        #[must_use]
        #[inline(always)]
        pub const fn rsn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Reference Slot Number."]
        #[inline(always)]
        pub const fn set_rsn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Dmacsfcsr {
        #[inline(always)]
        fn default() -> Dmacsfcsr {
            Dmacsfcsr(0)
        }
    }
    impl core::fmt::Debug for Dmacsfcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacsfcsr")
                .field("esc", &self.esc())
                .field("asc", &self.asc())
                .field("siv", &self.siv())
                .field("rsn", &self.rsn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacsfcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacsfcsr {{ esc: {=bool:?}, asc: {=bool:?}, siv: {=u16:?}, rsn: {=u8:?} }}",
                self.esc(),
                self.asc(),
                self.siv(),
                self.rsn()
            )
        }
    }
    #[doc = "Channel status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacsr(pub u32);
    impl Dmacsr {
        #[doc = "Transmit Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Process Stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Process Stopped."]
        #[inline(always)]
        pub const fn set_tps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable."]
        #[inline(always)]
        pub const fn set_tbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt."]
        #[inline(always)]
        pub const fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable."]
        #[must_use]
        #[inline(always)]
        pub const fn rbu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable."]
        #[inline(always)]
        pub const fn set_rbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Process Stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Process Stopped."]
        #[inline(always)]
        pub const fn set_rps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout."]
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn eti(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt."]
        #[inline(always)]
        pub const fn set_eti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Early Receive Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn eri(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt."]
        #[inline(always)]
        pub const fn set_eri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Fatal Bus Error."]
        #[must_use]
        #[inline(always)]
        pub const fn fbe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error."]
        #[inline(always)]
        pub const fn set_fbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Context Descriptor Error."]
        #[must_use]
        #[inline(always)]
        pub const fn cde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Context Descriptor Error."]
        #[inline(always)]
        pub const fn set_cde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Abnormal Interrupt Summary."]
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary."]
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Normal Interrupt Summary."]
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary."]
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Tx DMA Error Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn teb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Tx DMA Error Bits."]
        #[inline(always)]
        pub const fn set_teb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Rx DMA Error Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn reb(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[doc = "Rx DMA Error Bits."]
        #[inline(always)]
        pub const fn set_reb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
    }
    impl Default for Dmacsr {
        #[inline(always)]
        fn default() -> Dmacsr {
            Dmacsr(0)
        }
    }
    impl core::fmt::Debug for Dmacsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacsr")
                .field("ti", &self.ti())
                .field("tps", &self.tps())
                .field("tbu", &self.tbu())
                .field("ri", &self.ri())
                .field("rbu", &self.rbu())
                .field("rps", &self.rps())
                .field("rwt", &self.rwt())
                .field("eti", &self.eti())
                .field("eri", &self.eri())
                .field("fbe", &self.fbe())
                .field("cde", &self.cde())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("teb", &self.teb())
                .field("reb", &self.reb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacsr {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, eri: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}",
                self.ti(),
                self.tps(),
                self.tbu(),
                self.ri(),
                self.rbu(),
                self.rps(),
                self.rwt(),
                self.eti(),
                self.eri(),
                self.fbe(),
                self.cde(),
                self.ais(),
                self.nis(),
                self.teb(),
                self.reb()
            )
        }
    }
    #[doc = "Debug status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmadsr(pub u32);
    impl Dmadsr {
        #[doc = "AXI Master Write Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn axwhsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Master Write Channel."]
        #[inline(always)]
        pub const fn set_axwhsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI Master Read Channel Status."]
        #[must_use]
        #[inline(always)]
        pub const fn axrhsts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Master Read Channel Status."]
        #[inline(always)]
        pub const fn set_axrhsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA Channel Receive Process State."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 8usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "DMA Channel Receive Process State."]
        #[inline(always)]
        pub const fn set_rps(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 8usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "DMA Channel Transmit Process State."]
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 12usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "DMA Channel Transmit Process State."]
        #[inline(always)]
        pub const fn set_tps(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 12usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Dmadsr {
        #[inline(always)]
        fn default() -> Dmadsr {
            Dmadsr(0)
        }
    }
    impl core::fmt::Debug for Dmadsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmadsr")
                .field("axwhsts", &self.axwhsts())
                .field("axrhsts", &self.axrhsts())
                .field("rps[0]", &self.rps(0usize))
                .field("rps[1]", &self.rps(1usize))
                .field("tps[0]", &self.tps(0usize))
                .field("tps[1]", &self.tps(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmadsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmadsr {{ axwhsts: {=bool:?}, axrhsts: {=bool:?}, rps[0]: {=u8:?}, rps[1]: {=u8:?}, tps[0]: {=u8:?}, tps[1]: {=u8:?} }}",
                self.axwhsts(),
                self.axrhsts(),
                self.rps(0usize),
                self.rps(1usize),
                self.tps(0usize),
                self.tps(1usize)
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaisr(pub u32);
    impl Dmaisr {
        #[doc = "DMA Channel Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn dcis(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Interrupt Status."]
        #[inline(always)]
        pub const fn set_dcis(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "MTL Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mtlis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Interrupt Status."]
        #[inline(always)]
        pub const fn set_mtlis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn macis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Interrupt Status."]
        #[inline(always)]
        pub const fn set_macis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Dmaisr {
        #[inline(always)]
        fn default() -> Dmaisr {
            Dmaisr(0)
        }
    }
    impl core::fmt::Debug for Dmaisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaisr")
                .field("dcis[0]", &self.dcis(0usize))
                .field("dcis[1]", &self.dcis(1usize))
                .field("mtlis", &self.mtlis())
                .field("macis", &self.macis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaisr {{ dcis[0]: {=bool:?}, dcis[1]: {=bool:?}, mtlis: {=bool:?}, macis: {=bool:?} }}",
                self.dcis(0usize),
                self.dcis(1usize),
                self.mtlis(),
                self.macis()
            )
        }
    }
    #[doc = "AXI4 LPI Entry Interval register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmalpiei(pub u32);
    impl Dmalpiei {
        #[doc = "LPI Entry Interval."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiei(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "LPI Entry Interval."]
        #[inline(always)]
        pub const fn set_lpiei(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Dmalpiei {
        #[inline(always)]
        fn default() -> Dmalpiei {
            Dmalpiei(0)
        }
    }
    impl core::fmt::Debug for Dmalpiei {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmalpiei").field("lpiei", &self.lpiei()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmalpiei {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmalpiei {{ lpiei: {=u8:?} }}", self.lpiei())
        }
    }
    #[doc = "DMA mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamr(pub u32);
    impl Dmamr {
        #[doc = "Software Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn swr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset."]
        #[inline(always)]
        pub const fn set_swr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Arbitration Algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn taa(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Arbitration Algorithm."]
        #[inline(always)]
        pub const fn set_taa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "Descriptor Posted Write."]
        #[must_use]
        #[inline(always)]
        pub const fn dspw(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Descriptor Posted Write."]
        #[inline(always)]
        pub const fn set_dspw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmit priority."]
        #[must_use]
        #[inline(always)]
        pub const fn txpr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit priority."]
        #[inline(always)]
        pub const fn set_txpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Interrupt Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn intm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Interrupt Mode."]
        #[inline(always)]
        pub const fn set_intm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Dmamr {
        #[inline(always)]
        fn default() -> Dmamr {
            Dmamr(0)
        }
    }
    impl core::fmt::Debug for Dmamr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmamr")
                .field("swr", &self.swr())
                .field("taa", &self.taa())
                .field("dspw", &self.dspw())
                .field("txpr", &self.txpr())
                .field("intm", &self.intm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmamr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmamr {{ swr: {=bool:?}, taa: {=u8:?}, dspw: {=bool:?}, txpr: {=bool:?}, intm: {=u8:?} }}",
                self.swr(),
                self.taa(),
                self.dspw(),
                self.txpr(),
                self.intm()
            )
        }
    }
    #[doc = "System bus mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasbmr(pub u32);
    impl Dmasbmr {
        #[doc = "Fixed Burst Length."]
        #[must_use]
        #[inline(always)]
        pub const fn fb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed Burst Length."]
        #[inline(always)]
        pub const fn set_fb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI Burst Length 4."]
        #[must_use]
        #[inline(always)]
        pub const fn blen4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 4."]
        #[inline(always)]
        pub const fn set_blen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI Burst Length 8."]
        #[must_use]
        #[inline(always)]
        pub const fn blen8(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 8."]
        #[inline(always)]
        pub const fn set_blen8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI Burst Length 16."]
        #[must_use]
        #[inline(always)]
        pub const fn blen16(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 16."]
        #[inline(always)]
        pub const fn set_blen16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI Burst Length 32."]
        #[must_use]
        #[inline(always)]
        pub const fn blen32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 32."]
        #[inline(always)]
        pub const fn set_blen32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI Burst Length 64."]
        #[must_use]
        #[inline(always)]
        pub const fn blen64(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 64."]
        #[inline(always)]
        pub const fn set_blen64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AXI Burst Length 128."]
        #[must_use]
        #[inline(always)]
        pub const fn blen128(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 128."]
        #[inline(always)]
        pub const fn set_blen128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AXI Burst Length 256."]
        #[must_use]
        #[inline(always)]
        pub const fn blen256(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 256."]
        #[inline(always)]
        pub const fn set_blen256(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Automatic AXI LPI enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aale(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic AXI LPI enable."]
        #[inline(always)]
        pub const fn set_aale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Address-Aligned Beats."]
        #[must_use]
        #[inline(always)]
        pub const fn aal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Address-Aligned Beats."]
        #[inline(always)]
        pub const fn set_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1 Kbyte Boundary Crossing Enable for the AXI Master."]
        #[must_use]
        #[inline(always)]
        pub const fn onekbbe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1 Kbyte Boundary Crossing Enable for the AXI Master."]
        #[inline(always)]
        pub const fn set_onekbbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AXI Maximum Read Outstanding Request Limit."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_osr_lmt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "AXI Maximum Read Outstanding Request Limit."]
        #[inline(always)]
        pub const fn set_rd_osr_lmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "AXI Maximum Write Outstanding Request Limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_osr_lmt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "AXI Maximum Write Outstanding Request Limit."]
        #[inline(always)]
        pub const fn set_wr_osr_lmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Unlock on Magic Packet or Remote wake-up packet."]
        #[must_use]
        #[inline(always)]
        pub const fn lpi_xit_pkt(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock on Magic Packet or Remote wake-up packet."]
        #[inline(always)]
        pub const fn set_lpi_xit_pkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable Low Power Interface (LPI)."]
        #[must_use]
        #[inline(always)]
        pub const fn en_lpi(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Low Power Interface (LPI)."]
        #[inline(always)]
        pub const fn set_en_lpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmasbmr {
        #[inline(always)]
        fn default() -> Dmasbmr {
            Dmasbmr(0)
        }
    }
    impl core::fmt::Debug for Dmasbmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmasbmr")
                .field("fb", &self.fb())
                .field("blen4", &self.blen4())
                .field("blen8", &self.blen8())
                .field("blen16", &self.blen16())
                .field("blen32", &self.blen32())
                .field("blen64", &self.blen64())
                .field("blen128", &self.blen128())
                .field("blen256", &self.blen256())
                .field("aale", &self.aale())
                .field("aal", &self.aal())
                .field("onekbbe", &self.onekbbe())
                .field("rd_osr_lmt", &self.rd_osr_lmt())
                .field("wr_osr_lmt", &self.wr_osr_lmt())
                .field("lpi_xit_pkt", &self.lpi_xit_pkt())
                .field("en_lpi", &self.en_lpi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmasbmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmasbmr {{ fb: {=bool:?}, blen4: {=bool:?}, blen8: {=bool:?}, blen16: {=bool:?}, blen32: {=bool:?}, blen64: {=bool:?}, blen128: {=bool:?}, blen256: {=bool:?}, aale: {=bool:?}, aal: {=bool:?}, onekbbe: {=bool:?}, rd_osr_lmt: {=u8:?}, wr_osr_lmt: {=u8:?}, lpi_xit_pkt: {=bool:?}, en_lpi: {=bool:?} }}",
                self.fb(),
                self.blen4(),
                self.blen8(),
                self.blen16(),
                self.blen32(),
                self.blen64(),
                self.blen128(),
                self.blen256(),
                self.aale(),
                self.aal(),
                self.onekbbe(),
                self.rd_osr_lmt(),
                self.wr_osr_lmt(),
                self.lpi_xit_pkt(),
                self.en_lpi()
            )
        }
    }
    #[doc = "DMA TBS control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatbsctrl0r(pub u32);
    impl Dmatbsctrl0r {
        #[doc = "Fetch time offset valid."]
        #[must_use]
        #[inline(always)]
        pub const fn ftov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fetch time offset valid."]
        #[inline(always)]
        pub const fn set_ftov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fetch GSN offset."]
        #[must_use]
        #[inline(always)]
        pub const fn fgos(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Fetch GSN offset."]
        #[inline(always)]
        pub const fn set_fgos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Fetch time offset."]
        #[must_use]
        #[inline(always)]
        pub const fn ftos(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Fetch time offset."]
        #[inline(always)]
        pub const fn set_ftos(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Dmatbsctrl0r {
        #[inline(always)]
        fn default() -> Dmatbsctrl0r {
            Dmatbsctrl0r(0)
        }
    }
    impl core::fmt::Debug for Dmatbsctrl0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatbsctrl0r")
                .field("ftov", &self.ftov())
                .field("fgos", &self.fgos())
                .field("ftos", &self.ftos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmatbsctrl0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmatbsctrl0r {{ ftov: {=bool:?}, fgos: {=u8:?}, ftos: {=u32:?} }}",
                self.ftov(),
                self.fgos(),
                self.ftos()
            )
        }
    }
    #[doc = "One-microsecond-tick counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mac1ustcr(pub u32);
    impl Mac1ustcr {
        #[doc = "1 s tick Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn tic_1us_cntr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "1 s tick Counter."]
        #[inline(always)]
        pub const fn set_tic_1us_cntr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Mac1ustcr {
        #[inline(always)]
        fn default() -> Mac1ustcr {
            Mac1ustcr(0)
        }
    }
    impl core::fmt::Debug for Mac1ustcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mac1ustcr")
                .field("tic_1us_cntr", &self.tic_1us_cntr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mac1ustcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mac1ustcr {{ tic_1us_cntr: {=u16:?} }}", self.tic_1us_cntr())
        }
    }
    #[doc = "MAC Indirect Access Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacIacr(pub u32);
    impl MacIacr {
        #[doc = "Operation Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn ob(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Operation Busy."]
        #[inline(always)]
        pub const fn set_ob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command type."]
        #[must_use]
        #[inline(always)]
        pub const fn com(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command type."]
        #[inline(always)]
        pub const fn set_com(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Auto-increment."]
        #[must_use]
        #[inline(always)]
        pub const fn auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-increment."]
        #[inline(always)]
        pub const fn set_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Address Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn aoff(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Address Offset."]
        #[inline(always)]
        pub const fn set_aoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn msel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Mode Select."]
        #[inline(always)]
        pub const fn set_msel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for MacIacr {
        #[inline(always)]
        fn default() -> MacIacr {
            MacIacr(0)
        }
    }
    impl core::fmt::Debug for MacIacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacIacr")
                .field("ob", &self.ob())
                .field("com", &self.com())
                .field("auto", &self.auto())
                .field("aoff", &self.aoff())
                .field("msel", &self.msel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacIacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacIacr {{ ob: {=bool:?}, com: {=bool:?}, auto: {=bool:?}, aoff: {=u8:?}, msel: {=u8:?} }}",
                self.ob(),
                self.com(),
                self.auto(),
                self.aoff(),
                self.msel()
            )
        }
    }
    #[doc = "Rx flow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacRxFcr(pub u32);
    impl MacRxFcr {
        #[doc = "Receive Flow Control Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Flow Control Enable."]
        #[inline(always)]
        pub const fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Unicast Pause Packet Detect."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Pause Packet Detect."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MacRxFcr {
        #[inline(always)]
        fn default() -> MacRxFcr {
            MacRxFcr(0)
        }
    }
    impl core::fmt::Debug for MacRxFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacRxFcr")
                .field("rfe", &self.rfe())
                .field("up", &self.up())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacRxFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacRxFcr {{ rfe: {=bool:?}, up: {=bool:?} }}", self.rfe(), self.up())
        }
    }
    #[doc = "Rx Tx status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacRxTxSr(pub u32);
    impl MacRxTxSr {
        #[doc = "Transmit Jabber Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn tjt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Jabber Timeout."]
        #[inline(always)]
        pub const fn set_tjt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No Carrier."]
        #[must_use]
        #[inline(always)]
        pub const fn ncarr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No Carrier."]
        #[inline(always)]
        pub const fn set_ncarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loss of Carrier."]
        #[must_use]
        #[inline(always)]
        pub const fn lcarr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loss of Carrier."]
        #[inline(always)]
        pub const fn set_lcarr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Excessive Deferral."]
        #[must_use]
        #[inline(always)]
        pub const fn exdef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Excessive Deferral."]
        #[inline(always)]
        pub const fn set_exdef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Late Collision."]
        #[must_use]
        #[inline(always)]
        pub const fn lcol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Late Collision."]
        #[inline(always)]
        pub const fn set_lcol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Excessive Collisions."]
        #[must_use]
        #[inline(always)]
        pub const fn excol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Excessive Collisions."]
        #[inline(always)]
        pub const fn set_excol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive Watchdog Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout."]
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MacRxTxSr {
        #[inline(always)]
        fn default() -> MacRxTxSr {
            MacRxTxSr(0)
        }
    }
    impl core::fmt::Debug for MacRxTxSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacRxTxSr")
                .field("tjt", &self.tjt())
                .field("ncarr", &self.ncarr())
                .field("lcarr", &self.lcarr())
                .field("exdef", &self.exdef())
                .field("lcol", &self.lcol())
                .field("excol", &self.excol())
                .field("rwt", &self.rwt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacRxTxSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacRxTxSr {{ tjt: {=bool:?}, ncarr: {=bool:?}, lcarr: {=bool:?}, exdef: {=bool:?}, lcol: {=bool:?}, excol: {=bool:?}, rwt: {=bool:?} }}",
                self.tjt(),
                self.ncarr(),
                self.lcarr(),
                self.exdef(),
                self.lcol(),
                self.excol(),
                self.rwt()
            )
        }
    }
    #[doc = "MAC type-based Rx Queue mapping register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacTmrqr(pub u32);
    impl MacTmrqr {
        #[doc = "Type field Value."]
        #[must_use]
        #[inline(always)]
        pub const fn typ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Type field Value."]
        #[inline(always)]
        pub const fn set_typ(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Type Match Rx Queue Number."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Type Match Rx Queue Number."]
        #[inline(always)]
        pub const fn set_tmrq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Preemption or Express Packet."]
        #[must_use]
        #[inline(always)]
        pub const fn pfex(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Preemption or Express Packet."]
        #[inline(always)]
        pub const fn set_pfex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for MacTmrqr {
        #[inline(always)]
        fn default() -> MacTmrqr {
            MacTmrqr(0)
        }
    }
    impl core::fmt::Debug for MacTmrqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacTmrqr")
                .field("typ", &self.typ())
                .field("tmrq", &self.tmrq())
                .field("pfex", &self.pfex())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacTmrqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacTmrqr {{ typ: {=u16:?}, tmrq: {=u8:?}, pfex: {=bool:?} }}",
                self.typ(),
                self.tmrq(),
                self.pfex()
            )
        }
    }
    #[doc = "Tx timestamp status nanoseconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacTxTssnr(pub u32);
    impl MacTxTssnr {
        #[doc = "Transmit Timestamp Status Low."]
        #[must_use]
        #[inline(always)]
        pub const fn txtsslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Transmit Timestamp Status Low."]
        #[inline(always)]
        pub const fn set_txtsslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Transmit Timestamp Status Missed."]
        #[must_use]
        #[inline(always)]
        pub const fn txtssmis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Timestamp Status Missed."]
        #[inline(always)]
        pub const fn set_txtssmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacTxTssnr {
        #[inline(always)]
        fn default() -> MacTxTssnr {
            MacTxTssnr(0)
        }
    }
    impl core::fmt::Debug for MacTxTssnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacTxTssnr")
                .field("txtsslo", &self.txtsslo())
                .field("txtssmis", &self.txtssmis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacTxTssnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacTxTssnr {{ txtsslo: {=u32:?}, txtssmis: {=bool:?} }}",
                self.txtsslo(),
                self.txtssmis()
            )
        }
    }
    #[doc = "Tx timestamp status seconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacTxTsssr(pub u32);
    impl MacTxTsssr {
        #[doc = "Transmit Timestamp Status High."]
        #[must_use]
        #[inline(always)]
        pub const fn txtsshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit Timestamp Status High."]
        #[inline(always)]
        pub const fn set_txtsshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacTxTsssr {
        #[inline(always)]
        fn default() -> MacTxTsssr {
            MacTxTsssr(0)
        }
    }
    impl core::fmt::Debug for MacTxTsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacTxTsssr").field("txtsshi", &self.txtsshi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacTxTsssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacTxTsssr {{ txtsshi: {=u32:?} }}", self.txtsshi())
        }
    }
    #[doc = "MAC Address 0 high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "MAC Address0\\[47:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC Address0\\[47:32\\]."]
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "DMA Channel Select."]
        #[must_use]
        #[inline(always)]
        pub const fn dcs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Select."]
        #[inline(always)]
        pub const fn set_dcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Address Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca0hr {
        #[inline(always)]
        fn default() -> Maca0hr {
            Maca0hr(0)
        }
    }
    impl core::fmt::Debug for Maca0hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0hr")
                .field("addrhi", &self.addrhi())
                .field("dcs", &self.dcs())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0hr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maca0hr {{ addrhi: {=u16:?}, dcs: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.dcs(),
                self.ae()
            )
        }
    }
    #[doc = "MAC Address 0 low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "MAC Address x \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address x \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca0lr {
        #[inline(always)]
        fn default() -> Maca0lr {
            Maca0lr(0)
        }
    }
    impl core::fmt::Debug for Maca0lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0lr").field("addrlo", &self.addrlo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maca0lr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maca0lr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "Auxiliary control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macacr(pub u32);
    impl Macacr {
        #[doc = "Auxiliary Snapshot FIFO Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn atsfc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot FIFO Clear."]
        #[inline(always)]
        pub const fn set_atsfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Auxiliary Snapshot Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot Enable."]
        #[inline(always)]
        pub const fn set_atsen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 4usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Macacr {
        #[inline(always)]
        fn default() -> Macacr {
            Macacr(0)
        }
    }
    impl core::fmt::Debug for Macacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macacr")
                .field("atsfc", &self.atsfc())
                .field("atsen[0]", &self.atsen(0usize))
                .field("atsen[1]", &self.atsen(1usize))
                .field("atsen[2]", &self.atsen(2usize))
                .field("atsen[3]", &self.atsen(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macacr {{ atsfc: {=bool:?}, atsen[0]: {=bool:?}, atsen[1]: {=bool:?}, atsen[2]: {=bool:?}, atsen[3]: {=bool:?} }}",
                self.atsfc(),
                self.atsen(0usize),
                self.atsen(1usize),
                self.atsen(2usize),
                self.atsen(3usize)
            )
        }
    }
    #[doc = "MAC address high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macahr(pub u32);
    impl Macahr {
        #[doc = "MAC address \\[47:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address \\[47:32\\]."]
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "DMA Channel Select."]
        #[must_use]
        #[inline(always)]
        pub const fn dcs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Select."]
        #[inline(always)]
        pub const fn set_dcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Mask Byte Control."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask Byte Control."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macahr {
        #[inline(always)]
        fn default() -> Macahr {
            Macahr(0)
        }
    }
    impl core::fmt::Debug for Macahr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macahr")
                .field("addrhi", &self.addrhi())
                .field("dcs", &self.dcs())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macahr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macahr {{ addrhi: {=u16:?}, dcs: {=bool:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.dcs(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[doc = "MAC address low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macalr(pub u32);
    impl Macalr {
        #[doc = "MAC Address x \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address x \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macalr {
        #[inline(always)]
        fn default() -> Macalr {
            Macalr(0)
        }
    }
    impl core::fmt::Debug for Macalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macalr").field("addrlo", &self.addrlo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macalr {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "ARP address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macarpar(pub u32);
    impl Macarpar {
        #[doc = "ARP Protocol Address."]
        #[must_use]
        #[inline(always)]
        pub const fn arppa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ARP Protocol Address."]
        #[inline(always)]
        pub const fn set_arppa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macarpar {
        #[inline(always)]
        fn default() -> Macarpar {
            Macarpar(0)
        }
    }
    impl core::fmt::Debug for Macarpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macarpar").field("arppa", &self.arppa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macarpar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macarpar {{ arppa: {=u32:?} }}", self.arppa())
        }
    }
    #[doc = "Auxiliary timestamp nanoseconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macatsnr(pub u32);
    impl Macatsnr {
        #[doc = "Auxiliary Timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Auxiliary Timestamp."]
        #[inline(always)]
        pub const fn set_auxtslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Macatsnr {
        #[inline(always)]
        fn default() -> Macatsnr {
            Macatsnr(0)
        }
    }
    impl core::fmt::Debug for Macatsnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macatsnr").field("auxtslo", &self.auxtslo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macatsnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macatsnr {{ auxtslo: {=u32:?} }}", self.auxtslo())
        }
    }
    #[doc = "Auxiliary timestamp seconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macatssr(pub u32);
    impl Macatssr {
        #[doc = "Auxiliary Timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Auxiliary Timestamp."]
        #[inline(always)]
        pub const fn set_auxtshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macatssr {
        #[inline(always)]
        fn default() -> Macatssr {
            Macatssr(0)
        }
    }
    impl core::fmt::Debug for Macatssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macatssr").field("auxtshi", &self.auxtshi()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macatssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macatssr {{ auxtshi: {=u32:?} }}", self.auxtshi())
        }
    }
    #[doc = "Operating mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Receiver Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Enable."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmitter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Enable."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Preamble Length for Transmit packets."]
        #[must_use]
        #[inline(always)]
        pub const fn prelen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Preamble Length for Transmit packets."]
        #[inline(always)]
        pub const fn set_prelen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Deferral Check."]
        #[must_use]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Deferral Check."]
        #[inline(always)]
        pub const fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Back-Off Limit."]
        #[must_use]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Back-Off Limit."]
        #[inline(always)]
        pub const fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Disable Retry."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Retry."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Disable Carrier Sense During Transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Carrier Sense During Transmission."]
        #[inline(always)]
        pub const fn set_dcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Disable Receive Own."]
        #[must_use]
        #[inline(always)]
        pub const fn do_(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Receive Own."]
        #[inline(always)]
        pub const fn set_do_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Carrier Sense Before Transmission in Full-duplex mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ecrsfd(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Carrier Sense Before Transmission in Full-duplex mode."]
        #[inline(always)]
        pub const fn set_ecrsfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Mode."]
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Duplex Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex Mode."]
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MAC Speed."]
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Speed."]
        #[inline(always)]
        pub const fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port Select."]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Jumbo Packet Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn je(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Jumbo Packet Enable."]
        #[inline(always)]
        pub const fn set_je(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Jabber Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber Disable."]
        #[inline(always)]
        pub const fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Packet Burst Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn be(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Packet Burst Enable."]
        #[inline(always)]
        pub const fn set_be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Watchdog Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Disable."]
        #[inline(always)]
        pub const fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Automatic Pad or CRC Stripping."]
        #[must_use]
        #[inline(always)]
        pub const fn acs(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Pad or CRC Stripping."]
        #[inline(always)]
        pub const fn set_acs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CRC stripping for Type packets."]
        #[must_use]
        #[inline(always)]
        pub const fn cst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CRC stripping for Type packets."]
        #[inline(always)]
        pub const fn set_cst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "IEEE 802.3as Support for 2K Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn s2kp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 802.3as Support for 2K Packets."]
        #[inline(always)]
        pub const fn set_s2kp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Giant Packet Size Limit Control Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpslce(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Giant Packet Size Limit Control Enable."]
        #[inline(always)]
        pub const fn set_gpslce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Inter-Packet Gap."]
        #[must_use]
        #[inline(always)]
        pub const fn ipg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Inter-Packet Gap."]
        #[inline(always)]
        pub const fn set_ipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Checksum Offload."]
        #[must_use]
        #[inline(always)]
        pub const fn ipc(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Checksum Offload."]
        #[inline(always)]
        pub const fn set_ipc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Source Address Insertion or Replacement Control."]
        #[must_use]
        #[inline(always)]
        pub const fn sarc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Source Address Insertion or Replacement Control."]
        #[inline(always)]
        pub const fn set_sarc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "ARP Offload Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ARP Offload Enable."]
        #[inline(always)]
        pub const fn set_arpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maccr {
        #[inline(always)]
        fn default() -> Maccr {
            Maccr(0)
        }
    }
    impl core::fmt::Debug for Maccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccr")
                .field("re", &self.re())
                .field("te", &self.te())
                .field("prelen", &self.prelen())
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("dr", &self.dr())
                .field("dcrs", &self.dcrs())
                .field("do_", &self.do_())
                .field("ecrsfd", &self.ecrsfd())
                .field("lm", &self.lm())
                .field("dm", &self.dm())
                .field("fes", &self.fes())
                .field("ps", &self.ps())
                .field("je", &self.je())
                .field("jd", &self.jd())
                .field("be", &self.be())
                .field("wd", &self.wd())
                .field("acs", &self.acs())
                .field("cst", &self.cst())
                .field("s2kp", &self.s2kp())
                .field("gpslce", &self.gpslce())
                .field("ipg", &self.ipg())
                .field("ipc", &self.ipc())
                .field("sarc", &self.sarc())
                .field("arpen", &self.arpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maccr {{ re: {=bool:?}, te: {=bool:?}, prelen: {=u8:?}, dc: {=bool:?}, bl: {=u8:?}, dr: {=bool:?}, dcrs: {=bool:?}, do_: {=bool:?}, ecrsfd: {=bool:?}, lm: {=bool:?}, dm: {=bool:?}, fes: {=bool:?}, ps: {=bool:?}, je: {=bool:?}, jd: {=bool:?}, be: {=bool:?}, wd: {=bool:?}, acs: {=bool:?}, cst: {=bool:?}, s2kp: {=bool:?}, gpslce: {=bool:?}, ipg: {=u8:?}, ipc: {=bool:?}, sarc: {=u8:?}, arpen: {=bool:?} }}",
                self.re(),
                self.te(),
                self.prelen(),
                self.dc(),
                self.bl(),
                self.dr(),
                self.dcrs(),
                self.do_(),
                self.ecrsfd(),
                self.lm(),
                self.dm(),
                self.fes(),
                self.ps(),
                self.je(),
                self.jd(),
                self.be(),
                self.wd(),
                self.acs(),
                self.cst(),
                self.s2kp(),
                self.gpslce(),
                self.ipg(),
                self.ipc(),
                self.sarc(),
                self.arpen()
            )
        }
    }
    #[doc = "CSR software control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccsrswcr(pub u32);
    impl Maccsrswcr {
        #[doc = "Register Clear on Write 1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rcwe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Register Clear on Write 1 Enable."]
        #[inline(always)]
        pub const fn set_rcwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Slave Error Response Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Slave Error Response Enable."]
        #[inline(always)]
        pub const fn set_seen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Maccsrswcr {
        #[inline(always)]
        fn default() -> Maccsrswcr {
            Maccsrswcr(0)
        }
    }
    impl core::fmt::Debug for Maccsrswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccsrswcr")
                .field("rcwe", &self.rcwe())
                .field("seen", &self.seen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maccsrswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maccsrswcr {{ rcwe: {=bool:?}, seen: {=bool:?} }}",
                self.rcwe(),
                self.seen()
            )
        }
    }
    #[doc = "Debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macdr(pub u32);
    impl Macdr {
        #[doc = "MAC GMII or MII Receive Protocol Engine Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rpests(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MAC GMII or MII Receive Protocol Engine Status."]
        #[inline(always)]
        pub const fn set_rpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MAC Receive Packet Controller FIFO Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rfcfcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MAC Receive Packet Controller FIFO Status."]
        #[inline(always)]
        pub const fn set_rfcfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MAC GMII or MII Transmit Protocol Engine Status."]
        #[must_use]
        #[inline(always)]
        pub const fn tpests(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MAC GMII or MII Transmit Protocol Engine Status."]
        #[inline(always)]
        pub const fn set_tpests(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC Transmit Packet Controller Status."]
        #[must_use]
        #[inline(always)]
        pub const fn tfcsts(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "MAC Transmit Packet Controller Status."]
        #[inline(always)]
        pub const fn set_tfcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
    }
    impl Default for Macdr {
        #[inline(always)]
        fn default() -> Macdr {
            Macdr(0)
        }
    }
    impl core::fmt::Debug for Macdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macdr")
                .field("rpests", &self.rpests())
                .field("rfcfcsts", &self.rfcfcsts())
                .field("tpests", &self.tpests())
                .field("tfcsts", &self.tfcsts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macdr {{ rpests: {=bool:?}, rfcfcsts: {=u8:?}, tpests: {=bool:?}, tfcsts: {=u8:?} }}",
                self.rpests(),
                self.rfcfcsts(),
                self.tpests(),
                self.tfcsts()
            )
        }
    }
    #[doc = "Extended operating mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macecr(pub u32);
    impl Macecr {
        #[doc = "Giant Packet Size Limit."]
        #[must_use]
        #[inline(always)]
        pub const fn gpsl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Giant Packet Size Limit."]
        #[inline(always)]
        pub const fn set_gpsl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Disable CRC Checking for Received Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrcc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Disable CRC Checking for Received Packets."]
        #[inline(always)]
        pub const fn set_dcrcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Slow Protocol Detection Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Slow Protocol Detection Enable."]
        #[inline(always)]
        pub const fn set_spen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Unicast Slow Protocol Packet Detect."]
        #[must_use]
        #[inline(always)]
        pub const fn usp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Slow Protocol Packet Detect."]
        #[inline(always)]
        pub const fn set_usp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Extended Inter-Packet Gap Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eipgen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Extended Inter-Packet Gap Enable."]
        #[inline(always)]
        pub const fn set_eipgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Extended Inter-Packet Gap."]
        #[must_use]
        #[inline(always)]
        pub const fn eipg(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Extended Inter-Packet Gap."]
        #[inline(always)]
        pub const fn set_eipg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[doc = "ARP Packet Drop if IP Address Mismatch."]
        #[must_use]
        #[inline(always)]
        pub const fn apdim(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ARP Packet Drop if IP Address Mismatch."]
        #[inline(always)]
        pub const fn set_apdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Macecr {
        #[inline(always)]
        fn default() -> Macecr {
            Macecr(0)
        }
    }
    impl core::fmt::Debug for Macecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macecr")
                .field("gpsl", &self.gpsl())
                .field("dcrcc", &self.dcrcc())
                .field("spen", &self.spen())
                .field("usp", &self.usp())
                .field("eipgen", &self.eipgen())
                .field("eipg", &self.eipg())
                .field("apdim", &self.apdim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macecr {{ gpsl: {=u16:?}, dcrcc: {=bool:?}, spen: {=bool:?}, usp: {=bool:?}, eipgen: {=bool:?}, eipg: {=u8:?}, apdim: {=bool:?} }}",
                self.gpsl(),
                self.dcrcc(),
                self.spen(),
                self.usp(),
                self.eipgen(),
                self.eipg(),
                self.apdim()
            )
        }
    }
    #[doc = "FPE control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macfpecsr(pub u32);
    impl Macfpecsr {
        #[doc = "Enable Tx Frame Preemption."]
        #[must_use]
        #[inline(always)]
        pub const fn efpe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Tx Frame Preemption."]
        #[inline(always)]
        pub const fn set_efpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Send Verify mPacket."]
        #[must_use]
        #[inline(always)]
        pub const fn sver(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Send Verify mPacket."]
        #[inline(always)]
        pub const fn set_sver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Send Respond mPacket."]
        #[must_use]
        #[inline(always)]
        pub const fn srsp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send Respond mPacket."]
        #[inline(always)]
        pub const fn set_srsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Received Verify Frame."]
        #[must_use]
        #[inline(always)]
        pub const fn rver(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Received Verify Frame."]
        #[inline(always)]
        pub const fn set_rver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Received Respond Frame."]
        #[must_use]
        #[inline(always)]
        pub const fn rrsp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Received Respond Frame."]
        #[inline(always)]
        pub const fn set_rrsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Transmitted Verify Frame."]
        #[must_use]
        #[inline(always)]
        pub const fn tver(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted Verify Frame."]
        #[inline(always)]
        pub const fn set_tver(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Transmitted Respond Frame."]
        #[must_use]
        #[inline(always)]
        pub const fn trsp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted Respond Frame."]
        #[inline(always)]
        pub const fn set_trsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Macfpecsr {
        #[inline(always)]
        fn default() -> Macfpecsr {
            Macfpecsr(0)
        }
    }
    impl core::fmt::Debug for Macfpecsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macfpecsr")
                .field("efpe", &self.efpe())
                .field("sver", &self.sver())
                .field("srsp", &self.srsp())
                .field("rver", &self.rver())
                .field("rrsp", &self.rrsp())
                .field("tver", &self.tver())
                .field("trsp", &self.trsp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macfpecsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macfpecsr {{ efpe: {=bool:?}, sver: {=bool:?}, srsp: {=bool:?}, rver: {=bool:?}, rrsp: {=bool:?}, tver: {=bool:?}, trsp: {=bool:?} }}",
                self.efpe(),
                self.sver(),
                self.srsp(),
                self.rver(),
                self.rrsp(),
                self.tver(),
                self.trsp()
            )
        }
    }
    #[doc = "Hash table register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtr(pub u32);
    impl Machtr {
        #[doc = "MAC Hash Table First 32 Bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Hash Table First 32 Bits."]
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtr {
        #[inline(always)]
        fn default() -> Machtr {
            Machtr(0)
        }
    }
    impl core::fmt::Debug for Machtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machtr").field("ht", &self.ht()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Machtr {{ ht: {=u32:?} }}", self.ht())
        }
    }
    #[doc = "HW feature 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf0r(pub u32);
    impl Machwf0r {
        #[doc = "10 or 100 Mbps Support."]
        #[must_use]
        #[inline(always)]
        pub const fn miisel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "10 or 100 Mbps Support."]
        #[inline(always)]
        pub const fn set_miisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1000 Mbps Support."]
        #[must_use]
        #[inline(always)]
        pub const fn gmiisel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1000 Mbps Support."]
        #[inline(always)]
        pub const fn set_gmiisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Half-duplex Support."]
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex Support."]
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)."]
        #[must_use]
        #[inline(always)]
        pub const fn pcssel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)."]
        #[inline(always)]
        pub const fn set_pcssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "VLAN Hash Filter Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn vlhash(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Hash Filter Selected."]
        #[inline(always)]
        pub const fn set_vlhash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SMA (MDIO) Interface."]
        #[must_use]
        #[inline(always)]
        pub const fn smasel(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SMA (MDIO) Interface."]
        #[inline(always)]
        pub const fn set_smasel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PMT Remote wake-up Packet Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rwksel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Remote wake-up Packet Enable."]
        #[inline(always)]
        pub const fn set_rwksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PMT Magic Packet Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mgksel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Magic Packet Enable."]
        #[inline(always)]
        pub const fn set_mgksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RMON Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RMON Module Enable."]
        #[inline(always)]
        pub const fn set_mmcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ARP Offload Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn arpoffsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ARP Offload Enabled."]
        #[inline(always)]
        pub const fn set_arpoffsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IEEE 1588-2008 Timestamp Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tssel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 1588-2008 Timestamp Enabled."]
        #[inline(always)]
        pub const fn set_tssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Energy Efficient Ethernet Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn eeesel(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Energy Efficient Ethernet Enabled."]
        #[inline(always)]
        pub const fn set_eeesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit Checksum Offload Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn txcoesel(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Checksum Offload Enabled."]
        #[inline(always)]
        pub const fn set_txcoesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive Checksum Offload Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcoesel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Checksum Offload Enabled."]
        #[inline(always)]
        pub const fn set_rxcoesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MAC Addresses 1-31 Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn addmacadrsel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "MAC Addresses 1-31 Selected."]
        #[inline(always)]
        pub const fn set_addmacadrsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "MAC Addresses 32-63 Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn macadr32sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Addresses 32-63 Selected."]
        #[inline(always)]
        pub const fn set_macadr32sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MAC Addresses 64-127 Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn macadr64sel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MAC Addresses 64-127 Selected."]
        #[inline(always)]
        pub const fn set_macadr64sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timestamp System Time Source."]
        #[must_use]
        #[inline(always)]
        pub const fn tsstssel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "Timestamp System Time Source."]
        #[inline(always)]
        pub const fn set_tsstssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "Source Address or VLAN Insertion Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn savlanins(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address or VLAN Insertion Enable."]
        #[inline(always)]
        pub const fn set_savlanins(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Active PHY Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn actphysel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Active PHY Selected."]
        #[inline(always)]
        pub const fn set_actphysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Machwf0r {
        #[inline(always)]
        fn default() -> Machwf0r {
            Machwf0r(0)
        }
    }
    impl core::fmt::Debug for Machwf0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf0r")
                .field("miisel", &self.miisel())
                .field("gmiisel", &self.gmiisel())
                .field("hdsel", &self.hdsel())
                .field("pcssel", &self.pcssel())
                .field("vlhash", &self.vlhash())
                .field("smasel", &self.smasel())
                .field("rwksel", &self.rwksel())
                .field("mgksel", &self.mgksel())
                .field("mmcsel", &self.mmcsel())
                .field("arpoffsel", &self.arpoffsel())
                .field("tssel", &self.tssel())
                .field("eeesel", &self.eeesel())
                .field("txcoesel", &self.txcoesel())
                .field("rxcoesel", &self.rxcoesel())
                .field("addmacadrsel", &self.addmacadrsel())
                .field("macadr32sel", &self.macadr32sel())
                .field("macadr64sel", &self.macadr64sel())
                .field("tsstssel", &self.tsstssel())
                .field("savlanins", &self.savlanins())
                .field("actphysel", &self.actphysel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Machwf0r {{ miisel: {=bool:?}, gmiisel: {=bool:?}, hdsel: {=bool:?}, pcssel: {=bool:?}, vlhash: {=bool:?}, smasel: {=bool:?}, rwksel: {=bool:?}, mgksel: {=bool:?}, mmcsel: {=bool:?}, arpoffsel: {=bool:?}, tssel: {=bool:?}, eeesel: {=bool:?}, txcoesel: {=bool:?}, rxcoesel: {=bool:?}, addmacadrsel: {=u8:?}, macadr32sel: {=bool:?}, macadr64sel: {=bool:?}, tsstssel: {=u8:?}, savlanins: {=bool:?}, actphysel: {=u8:?} }}",
                self.miisel(),
                self.gmiisel(),
                self.hdsel(),
                self.pcssel(),
                self.vlhash(),
                self.smasel(),
                self.rwksel(),
                self.mgksel(),
                self.mmcsel(),
                self.arpoffsel(),
                self.tssel(),
                self.eeesel(),
                self.txcoesel(),
                self.rxcoesel(),
                self.addmacadrsel(),
                self.macadr32sel(),
                self.macadr64sel(),
                self.tsstssel(),
                self.savlanins(),
                self.actphysel()
            )
        }
    }
    #[doc = "HW feature 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf1r(pub u32);
    impl Machwf1r {
        #[doc = "MTL Receive FIFO Size."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MTL Receive FIFO Size."]
        #[inline(always)]
        pub const fn set_rxfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Single Port RAM Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spram(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Single Port RAM Enable."]
        #[inline(always)]
        pub const fn set_spram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MTL Transmit FIFO Size."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifosize(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "MTL Transmit FIFO Size."]
        #[inline(always)]
        pub const fn set_txfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "One-Step Timestamping Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn osten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "One-Step Timestamping Enable."]
        #[inline(always)]
        pub const fn set_osten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PTP Offload Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PTP Offload Enable."]
        #[inline(always)]
        pub const fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "IEEE 1588 High Word Register Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn advthword(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 1588 High Word Register Enable."]
        #[inline(always)]
        pub const fn set_advthword(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Address width."]
        #[must_use]
        #[inline(always)]
        pub const fn addr64(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Address width."]
        #[inline(always)]
        pub const fn set_addr64(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "DCB Feature Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCB Feature Enable."]
        #[inline(always)]
        pub const fn set_dcben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Split Header Feature Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sphen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Split Header Feature Enable."]
        #[inline(always)]
        pub const fn set_sphen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TCP Segmentation Offload Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tsoen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TCP Segmentation Offload Enable."]
        #[inline(always)]
        pub const fn set_tsoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DMA Debug Registers Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgmema(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Debug Registers Enable."]
        #[inline(always)]
        pub const fn set_dbgmema(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "AV Feature Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn avsel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "AV Feature Enable."]
        #[inline(always)]
        pub const fn set_avsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Rx Side Only AV Feature Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ravsel(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Rx Side Only AV Feature Enable."]
        #[inline(always)]
        pub const fn set_ravsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "One Step for PTP over UDP/IP Feature Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pouost(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "One Step for PTP over UDP/IP Feature Enable."]
        #[inline(always)]
        pub const fn set_pouost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Hash Table Size."]
        #[must_use]
        #[inline(always)]
        pub const fn hashtblsz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Hash Table Size."]
        #[inline(always)]
        pub const fn set_hashtblsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Total number of L3 or L4 Filters."]
        #[must_use]
        #[inline(always)]
        pub const fn l3l4fnum(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Total number of L3 or L4 Filters."]
        #[inline(always)]
        pub const fn set_l3l4fnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
    }
    impl Default for Machwf1r {
        #[inline(always)]
        fn default() -> Machwf1r {
            Machwf1r(0)
        }
    }
    impl core::fmt::Debug for Machwf1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf1r")
                .field("rxfifosize", &self.rxfifosize())
                .field("spram", &self.spram())
                .field("txfifosize", &self.txfifosize())
                .field("osten", &self.osten())
                .field("ptoen", &self.ptoen())
                .field("advthword", &self.advthword())
                .field("addr64", &self.addr64())
                .field("dcben", &self.dcben())
                .field("sphen", &self.sphen())
                .field("tsoen", &self.tsoen())
                .field("dbgmema", &self.dbgmema())
                .field("avsel", &self.avsel())
                .field("ravsel", &self.ravsel())
                .field("pouost", &self.pouost())
                .field("hashtblsz", &self.hashtblsz())
                .field("l3l4fnum", &self.l3l4fnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Machwf1r {{ rxfifosize: {=u8:?}, spram: {=bool:?}, txfifosize: {=u8:?}, osten: {=bool:?}, ptoen: {=bool:?}, advthword: {=bool:?}, addr64: {=u8:?}, dcben: {=bool:?}, sphen: {=bool:?}, tsoen: {=bool:?}, dbgmema: {=bool:?}, avsel: {=bool:?}, ravsel: {=bool:?}, pouost: {=bool:?}, hashtblsz: {=u8:?}, l3l4fnum: {=u8:?} }}",
                self.rxfifosize(),
                self.spram(),
                self.txfifosize(),
                self.osten(),
                self.ptoen(),
                self.advthword(),
                self.addr64(),
                self.dcben(),
                self.sphen(),
                self.tsoen(),
                self.dbgmema(),
                self.avsel(),
                self.ravsel(),
                self.pouost(),
                self.hashtblsz(),
                self.l3l4fnum()
            )
        }
    }
    #[doc = "HW feature 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf2r(pub u32);
    impl Machwf2r {
        #[doc = "Number of MTL Receive Queues."]
        #[must_use]
        #[inline(always)]
        pub const fn rxqcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of MTL Receive Queues."]
        #[inline(always)]
        pub const fn set_rxqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Number of MTL Transmit Queues."]
        #[must_use]
        #[inline(always)]
        pub const fn txqcnt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of MTL Transmit Queues."]
        #[inline(always)]
        pub const fn set_txqcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "Number of DMA Receive Channels."]
        #[must_use]
        #[inline(always)]
        pub const fn rxchcnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of DMA Receive Channels."]
        #[inline(always)]
        pub const fn set_rxchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Rx DMA Descriptor Cache Size in terms of 16-byte descriptors."]
        #[must_use]
        #[inline(always)]
        pub const fn rdcsz(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Rx DMA Descriptor Cache Size in terms of 16-byte descriptors."]
        #[inline(always)]
        pub const fn set_rdcsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Number of DMA Transmit Channels."]
        #[must_use]
        #[inline(always)]
        pub const fn txchcnt(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of DMA Transmit Channels."]
        #[inline(always)]
        pub const fn set_txchcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Tx DMA Descriptor Cache Size in terms of 16-byte descriptors."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcsz(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Tx DMA Descriptor Cache Size in terms of 16-byte descriptors."]
        #[inline(always)]
        pub const fn set_tdcsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Number of PPS Outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsoutnum(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Number of PPS Outputs."]
        #[inline(always)]
        pub const fn set_ppsoutnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Number of Auxiliary Snapshot Inputs."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsnapnum(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Auxiliary Snapshot Inputs."]
        #[inline(always)]
        pub const fn set_auxsnapnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Machwf2r {
        #[inline(always)]
        fn default() -> Machwf2r {
            Machwf2r(0)
        }
    }
    impl core::fmt::Debug for Machwf2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf2r")
                .field("rxqcnt", &self.rxqcnt())
                .field("txqcnt", &self.txqcnt())
                .field("rxchcnt", &self.rxchcnt())
                .field("rdcsz", &self.rdcsz())
                .field("txchcnt", &self.txchcnt())
                .field("tdcsz", &self.tdcsz())
                .field("ppsoutnum", &self.ppsoutnum())
                .field("auxsnapnum", &self.auxsnapnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Machwf2r {{ rxqcnt: {=u8:?}, txqcnt: {=u8:?}, rxchcnt: {=u8:?}, rdcsz: {=u8:?}, txchcnt: {=u8:?}, tdcsz: {=u8:?}, ppsoutnum: {=u8:?}, auxsnapnum: {=u8:?} }}",
                self.rxqcnt(),
                self.txqcnt(),
                self.rxchcnt(),
                self.rdcsz(),
                self.txchcnt(),
                self.tdcsz(),
                self.ppsoutnum(),
                self.auxsnapnum()
            )
        }
    }
    #[doc = "HW feature 3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machwf3r(pub u32);
    impl Machwf3r {
        #[doc = "Number of Extended VLAN Tag Filters Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn nrvf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Extended VLAN Tag Filters Enabled."]
        #[inline(always)]
        pub const fn set_nrvf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Queue/Channel based VLAN tag insertion on Tx enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cbtisel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Queue/Channel based VLAN tag insertion on Tx enable."]
        #[inline(always)]
        pub const fn set_cbtisel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Double VLAN processing enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dvlan(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Double VLAN processing enable."]
        #[inline(always)]
        pub const fn set_dvlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Broadcast/Multicast Packet Duplication."]
        #[must_use]
        #[inline(always)]
        pub const fn pdupsel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Broadcast/Multicast Packet Duplication."]
        #[inline(always)]
        pub const fn set_pdupsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Flexible Receive Parser Selected."]
        #[must_use]
        #[inline(always)]
        pub const fn frpsel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible Receive Parser Selected."]
        #[inline(always)]
        pub const fn set_frpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Flexible Receive Parser Buffer size."]
        #[must_use]
        #[inline(always)]
        pub const fn frpbs(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Flexible Receive Parser Buffer size."]
        #[inline(always)]
        pub const fn set_frpbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Flexible Receive Parser Table Entries size."]
        #[must_use]
        #[inline(always)]
        pub const fn frpes(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Flexible Receive Parser Table Entries size."]
        #[inline(always)]
        pub const fn set_frpes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Enhancements to Scheduled Traffic Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn estsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enhancements to Scheduled Traffic Enable."]
        #[inline(always)]
        pub const fn set_estsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Depth of the Gate Control List."]
        #[must_use]
        #[inline(always)]
        pub const fn estdep(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Depth of the Gate Control List."]
        #[inline(always)]
        pub const fn set_estdep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Width of the Time Interval field in the Gate Control List."]
        #[must_use]
        #[inline(always)]
        pub const fn estwid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Width of the Time Interval field in the Gate Control List."]
        #[inline(always)]
        pub const fn set_estwid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Frame Preemption Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fpesel(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Enable."]
        #[inline(always)]
        pub const fn set_fpesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Time-based scheduling Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbssel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Time-based scheduling Enable."]
        #[inline(always)]
        pub const fn set_tbssel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Automotive Safety Package."]
        #[must_use]
        #[inline(always)]
        pub const fn asp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Automotive Safety Package."]
        #[inline(always)]
        pub const fn set_asp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Machwf3r {
        #[inline(always)]
        fn default() -> Machwf3r {
            Machwf3r(0)
        }
    }
    impl core::fmt::Debug for Machwf3r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machwf3r")
                .field("nrvf", &self.nrvf())
                .field("cbtisel", &self.cbtisel())
                .field("dvlan", &self.dvlan())
                .field("pdupsel", &self.pdupsel())
                .field("frpsel", &self.frpsel())
                .field("frpbs", &self.frpbs())
                .field("frpes", &self.frpes())
                .field("estsel", &self.estsel())
                .field("estdep", &self.estdep())
                .field("estwid", &self.estwid())
                .field("fpesel", &self.fpesel())
                .field("tbssel", &self.tbssel())
                .field("asp", &self.asp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Machwf3r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Machwf3r {{ nrvf: {=u8:?}, cbtisel: {=bool:?}, dvlan: {=bool:?}, pdupsel: {=bool:?}, frpsel: {=bool:?}, frpbs: {=u8:?}, frpes: {=u8:?}, estsel: {=bool:?}, estdep: {=u8:?}, estwid: {=u8:?}, fpesel: {=bool:?}, tbssel: {=bool:?}, asp: {=u8:?} }}",
                self.nrvf(),
                self.cbtisel(),
                self.dvlan(),
                self.pdupsel(),
                self.frpsel(),
                self.frpbs(),
                self.frpes(),
                self.estsel(),
                self.estdep(),
                self.estwid(),
                self.fpesel(),
                self.tbssel(),
                self.asp()
            )
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macier(pub u32);
    impl Macier {
        #[doc = "RGMII Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rgsmiiie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RGMII Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rgsmiiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PHY Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn phyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Interrupt Enable."]
        #[inline(always)]
        pub const fn set_phyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PMT Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pmtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPI Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Enable."]
        #[inline(always)]
        pub const fn set_lpiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timestamp Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit Status Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txstsie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Status Interrupt Enable."]
        #[inline(always)]
        pub const fn set_txstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Receive Status Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxstsie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Status Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rxstsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Frame Preemption Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fpeie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Interrupt Enable."]
        #[inline(always)]
        pub const fn set_fpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MDIO Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mdioie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO Interrupt Enable."]
        #[inline(always)]
        pub const fn set_mdioie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Macier {
        #[inline(always)]
        fn default() -> Macier {
            Macier(0)
        }
    }
    impl core::fmt::Debug for Macier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macier")
                .field("rgsmiiie", &self.rgsmiiie())
                .field("phyie", &self.phyie())
                .field("pmtie", &self.pmtie())
                .field("lpiie", &self.lpiie())
                .field("tsie", &self.tsie())
                .field("txstsie", &self.txstsie())
                .field("rxstsie", &self.rxstsie())
                .field("fpeie", &self.fpeie())
                .field("mdioie", &self.mdioie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macier {{ rgsmiiie: {=bool:?}, phyie: {=bool:?}, pmtie: {=bool:?}, lpiie: {=bool:?}, tsie: {=bool:?}, txstsie: {=bool:?}, rxstsie: {=bool:?}, fpeie: {=bool:?}, mdioie: {=bool:?} }}",
                self.rgsmiiie(),
                self.phyie(),
                self.pmtie(),
                self.lpiie(),
                self.tsie(),
                self.txstsie(),
                self.rxstsie(),
                self.fpeie(),
                self.mdioie()
            )
        }
    }
    #[doc = "Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macisr(pub u32);
    impl Macisr {
        #[doc = "RGMII Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rgsmiiis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RGMII Interrupt Status."]
        #[inline(always)]
        pub const fn set_rgsmiiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PHY Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn phyis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Interrupt."]
        #[inline(always)]
        pub const fn set_phyis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PMT Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Status."]
        #[inline(always)]
        pub const fn set_pmtis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPI Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Status."]
        #[inline(always)]
        pub const fn set_lpiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Interrupt Status."]
        #[inline(always)]
        pub const fn set_mmcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrxis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Interrupt Status."]
        #[inline(always)]
        pub const fn set_mmcrxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Transmit Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmctxis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Interrupt Status."]
        #[inline(always)]
        pub const fn set_mmctxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timestamp Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn tsis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Status."]
        #[inline(always)]
        pub const fn set_tsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transmit Status Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn txstsis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Status Interrupt."]
        #[inline(always)]
        pub const fn set_txstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Receive Status Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rxstsis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Status Interrupt."]
        #[inline(always)]
        pub const fn set_rxstsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Frame Preemption Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn fpeis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Interrupt Status."]
        #[inline(always)]
        pub const fn set_fpeis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MDIO Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mdiois(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO Interrupt Status."]
        #[inline(always)]
        pub const fn set_mdiois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC FPE Transmit Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mftis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC FPE Transmit Interrupt Status."]
        #[inline(always)]
        pub const fn set_mftis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC FPE Receive Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn mfris(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC FPE Receive Interrupt Status."]
        #[inline(always)]
        pub const fn set_mfris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Macisr {
        #[inline(always)]
        fn default() -> Macisr {
            Macisr(0)
        }
    }
    impl core::fmt::Debug for Macisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macisr")
                .field("rgsmiiis", &self.rgsmiiis())
                .field("phyis", &self.phyis())
                .field("pmtis", &self.pmtis())
                .field("lpiis", &self.lpiis())
                .field("mmcis", &self.mmcis())
                .field("mmcrxis", &self.mmcrxis())
                .field("mmctxis", &self.mmctxis())
                .field("tsis", &self.tsis())
                .field("txstsis", &self.txstsis())
                .field("rxstsis", &self.rxstsis())
                .field("fpeis", &self.fpeis())
                .field("mdiois", &self.mdiois())
                .field("mftis", &self.mftis())
                .field("mfris", &self.mfris())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macisr {{ rgsmiiis: {=bool:?}, phyis: {=bool:?}, pmtis: {=bool:?}, lpiis: {=bool:?}, mmcis: {=bool:?}, mmcrxis: {=bool:?}, mmctxis: {=bool:?}, tsis: {=bool:?}, txstsis: {=bool:?}, rxstsis: {=bool:?}, fpeis: {=bool:?}, mdiois: {=bool:?}, mftis: {=bool:?}, mfris: {=bool:?} }}",
                self.rgsmiiis(),
                self.phyis(),
                self.pmtis(),
                self.lpiis(),
                self.mmcis(),
                self.mmcrxis(),
                self.mmctxis(),
                self.tsis(),
                self.txstsis(),
                self.rxstsis(),
                self.fpeis(),
                self.mdiois(),
                self.mftis(),
                self.mfris()
            )
        }
    }
    #[doc = "Inner VLAN inclusion register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macivir(pub u32);
    impl Macivir {
        #[doc = "VLAN Tag for Transmit Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag for Transmit Packets."]
        #[inline(always)]
        pub const fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Control in Transmit Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "VLAN Tag Control in Transmit Packets."]
        #[inline(always)]
        pub const fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VLAN Priority Control."]
        #[must_use]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Priority Control."]
        #[inline(always)]
        pub const fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "C-VLAN or S-VLAN."]
        #[must_use]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "C-VLAN or S-VLAN."]
        #[inline(always)]
        pub const fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "VLAN Tag Input."]
        #[must_use]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Input."]
        #[inline(always)]
        pub const fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Macivir {
        #[inline(always)]
        fn default() -> Macivir {
            Macivir(0)
        }
    }
    impl core::fmt::Debug for Macivir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macivir")
                .field("vlt", &self.vlt())
                .field("vlc", &self.vlc())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macivir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macivir {{ vlt: {=u16:?}, vlc: {=u8:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?} }}",
                self.vlt(),
                self.vlc(),
                self.vlp(),
                self.csvl(),
                self.vlti()
            )
        }
    }
    #[doc = "Layer3 Address 0 filter 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a00r(pub u32);
    impl Macl3a00r {
        #[doc = "Layer 3 Address 0 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a00(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 0 Field."]
        #[inline(always)]
        pub const fn set_l3a00(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a00r {
        #[inline(always)]
        fn default() -> Macl3a00r {
            Macl3a00r(0)
        }
    }
    impl core::fmt::Debug for Macl3a00r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a00r").field("l3a00", &self.l3a00()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a00r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a00r {{ l3a00: {=u32:?} }}", self.l3a00())
        }
    }
    #[doc = "Layer3 address 0 filter 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a01r(pub u32);
    impl Macl3a01r {
        #[doc = "Layer 3 Address 0 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a01(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 0 Field."]
        #[inline(always)]
        pub const fn set_l3a01(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a01r {
        #[inline(always)]
        fn default() -> Macl3a01r {
            Macl3a01r(0)
        }
    }
    impl core::fmt::Debug for Macl3a01r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a01r").field("l3a01", &self.l3a01()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a01r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a01r {{ l3a01: {=u32:?} }}", self.l3a01())
        }
    }
    #[doc = "Layer3 Address 1 filter 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a10r(pub u32);
    impl Macl3a10r {
        #[doc = "Layer 3 Address 1 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a10(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 1 Field."]
        #[inline(always)]
        pub const fn set_l3a10(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a10r {
        #[inline(always)]
        fn default() -> Macl3a10r {
            Macl3a10r(0)
        }
    }
    impl core::fmt::Debug for Macl3a10r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a10r").field("l3a10", &self.l3a10()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a10r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a10r {{ l3a10: {=u32:?} }}", self.l3a10())
        }
    }
    #[doc = "Layer3 address 1 filter 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a11r(pub u32);
    impl Macl3a11r {
        #[doc = "Layer 3 Address 1 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a11(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 1 Field."]
        #[inline(always)]
        pub const fn set_l3a11(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a11r {
        #[inline(always)]
        fn default() -> Macl3a11r {
            Macl3a11r(0)
        }
    }
    impl core::fmt::Debug for Macl3a11r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a11r").field("l3a11", &self.l3a11()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a11r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a11r {{ l3a11: {=u32:?} }}", self.l3a11())
        }
    }
    #[doc = "Layer3 Address 2 filter 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a20r(pub u32);
    impl Macl3a20r {
        #[doc = "Layer 3 Address 2 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a20(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 2 Field."]
        #[inline(always)]
        pub const fn set_l3a20(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a20r {
        #[inline(always)]
        fn default() -> Macl3a20r {
            Macl3a20r(0)
        }
    }
    impl core::fmt::Debug for Macl3a20r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a20r").field("l3a20", &self.l3a20()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a20r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a20r {{ l3a20: {=u32:?} }}", self.l3a20())
        }
    }
    #[doc = "Layer3 address 2 filter 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a21r(pub u32);
    impl Macl3a21r {
        #[doc = "Layer 3 Address 2 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a21(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 2 Field."]
        #[inline(always)]
        pub const fn set_l3a21(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a21r {
        #[inline(always)]
        fn default() -> Macl3a21r {
            Macl3a21r(0)
        }
    }
    impl core::fmt::Debug for Macl3a21r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a21r").field("l3a21", &self.l3a21()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a21r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a21r {{ l3a21: {=u32:?} }}", self.l3a21())
        }
    }
    #[doc = "Layer3 Address 3 filter 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a30r(pub u32);
    impl Macl3a30r {
        #[doc = "Layer 3 Address 3 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a30(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 3 Field."]
        #[inline(always)]
        pub const fn set_l3a30(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a30r {
        #[inline(always)]
        fn default() -> Macl3a30r {
            Macl3a30r(0)
        }
    }
    impl core::fmt::Debug for Macl3a30r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a30r").field("l3a30", &self.l3a30()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a30r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a30r {{ l3a30: {=u32:?} }}", self.l3a30())
        }
    }
    #[doc = "Layer3 address 3 filter 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3a31r(pub u32);
    impl Macl3a31r {
        #[doc = "Layer 3 Address 3 Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a31(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 3 Field."]
        #[inline(always)]
        pub const fn set_l3a31(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macl3a31r {
        #[inline(always)]
        fn default() -> Macl3a31r {
            Macl3a31r(0)
        }
    }
    impl core::fmt::Debug for Macl3a31r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3a31r").field("l3a31", &self.l3a31()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3a31r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macl3a31r {{ l3a31: {=u32:?} }}", self.l3a31())
        }
    }
    #[doc = "L3 and L4 control 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3l4c0r(pub u32);
    impl Macl3l4c0r {
        #[doc = "Layer 3 Protocol Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3pen0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 Protocol Enable."]
        #[inline(always)]
        pub const fn set_l3pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Layer 3 IP SA Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3sam0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Match Enable."]
        #[inline(always)]
        pub const fn set_l3sam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3saim0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l3saim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Layer 3 IP DA Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3dam0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Match Enable."]
        #[inline(always)]
        pub const fn set_l3dam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3daim0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l3daim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Layer 3 IP SA higher bits match."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hsbm0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP SA higher bits match."]
        #[inline(always)]
        pub const fn set_l3hsbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Layer 3 IP DA higher bits match."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hdbm0(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP DA higher bits match."]
        #[inline(always)]
        pub const fn set_l3hdbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Layer 4 Protocol Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4pen0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Protocol Enable."]
        #[inline(always)]
        pub const fn set_l4pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 4 Source Port Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spm0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Match Enable."]
        #[inline(always)]
        pub const fn set_l4spm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spim0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l4spim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Layer 4 Destination Port Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpm0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Match Enable."]
        #[inline(always)]
        pub const fn set_l4dpm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpim0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l4dpim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA Channel Number."]
        #[must_use]
        #[inline(always)]
        pub const fn dmchn0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Number."]
        #[inline(always)]
        pub const fn set_dmchn0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DMA Channel Select Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmchen0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Select Enable."]
        #[inline(always)]
        pub const fn set_dmchen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Macl3l4c0r {
        #[inline(always)]
        fn default() -> Macl3l4c0r {
            Macl3l4c0r(0)
        }
    }
    impl core::fmt::Debug for Macl3l4c0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3l4c0r")
                .field("l3pen0", &self.l3pen0())
                .field("l3sam0", &self.l3sam0())
                .field("l3saim0", &self.l3saim0())
                .field("l3dam0", &self.l3dam0())
                .field("l3daim0", &self.l3daim0())
                .field("l3hsbm0", &self.l3hsbm0())
                .field("l3hdbm0", &self.l3hdbm0())
                .field("l4pen0", &self.l4pen0())
                .field("l4spm0", &self.l4spm0())
                .field("l4spim0", &self.l4spim0())
                .field("l4dpm0", &self.l4dpm0())
                .field("l4dpim0", &self.l4dpim0())
                .field("dmchn0", &self.dmchn0())
                .field("dmchen0", &self.dmchen0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3l4c0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl3l4c0r {{ l3pen0: {=bool:?}, l3sam0: {=bool:?}, l3saim0: {=bool:?}, l3dam0: {=bool:?}, l3daim0: {=bool:?}, l3hsbm0: {=u8:?}, l3hdbm0: {=u8:?}, l4pen0: {=bool:?}, l4spm0: {=bool:?}, l4spim0: {=bool:?}, l4dpm0: {=bool:?}, l4dpim0: {=bool:?}, dmchn0: {=bool:?}, dmchen0: {=bool:?} }}",
                self.l3pen0(),
                self.l3sam0(),
                self.l3saim0(),
                self.l3dam0(),
                self.l3daim0(),
                self.l3hsbm0(),
                self.l3hdbm0(),
                self.l4pen0(),
                self.l4spm0(),
                self.l4spim0(),
                self.l4dpm0(),
                self.l4dpim0(),
                self.dmchn0(),
                self.dmchen0()
            )
        }
    }
    #[doc = "L3 and L4 control 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl3l4c1r(pub u32);
    impl Macl3l4c1r {
        #[doc = "Layer 3 Protocol Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3pen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 Protocol Enable."]
        #[inline(always)]
        pub const fn set_l3pen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Layer 3 IP SA Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3sam1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Match Enable."]
        #[inline(always)]
        pub const fn set_l3sam1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3saim1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l3saim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Layer 3 IP DA Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3dam1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Match Enable."]
        #[inline(always)]
        pub const fn set_l3dam1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l3daim1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l3daim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Layer 3 IP SA Higher Bits Match."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hsbm1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP SA Higher Bits Match."]
        #[inline(always)]
        pub const fn set_l3hsbm1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Layer 3 IP DA higher bits match."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hdbm1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP DA higher bits match."]
        #[inline(always)]
        pub const fn set_l3hdbm1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Layer 4 Protocol Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4pen1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Protocol Enable."]
        #[inline(always)]
        pub const fn set_l4pen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 4 Source Port Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spm1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Match Enable."]
        #[inline(always)]
        pub const fn set_l4spm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spim1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l4spim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Layer 4 Destination Port Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpm1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Match Enable."]
        #[inline(always)]
        pub const fn set_l4dpm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_l4dpim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA Channel Number."]
        #[must_use]
        #[inline(always)]
        pub const fn dmchn1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Number."]
        #[inline(always)]
        pub const fn set_dmchn1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DMA Channel Select Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmchen1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Select Enable."]
        #[inline(always)]
        pub const fn set_dmchen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Macl3l4c1r {
        #[inline(always)]
        fn default() -> Macl3l4c1r {
            Macl3l4c1r(0)
        }
    }
    impl core::fmt::Debug for Macl3l4c1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl3l4c1r")
                .field("l3pen1", &self.l3pen1())
                .field("l3sam1", &self.l3sam1())
                .field("l3saim1", &self.l3saim1())
                .field("l3dam1", &self.l3dam1())
                .field("l3daim1", &self.l3daim1())
                .field("l3hsbm1", &self.l3hsbm1())
                .field("l3hdbm1", &self.l3hdbm1())
                .field("l4pen1", &self.l4pen1())
                .field("l4spm1", &self.l4spm1())
                .field("l4spim1", &self.l4spim1())
                .field("l4dpm1", &self.l4dpm1())
                .field("l4dpim1", &self.l4dpim1())
                .field("dmchn1", &self.dmchn1())
                .field("dmchen1", &self.dmchen1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl3l4c1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl3l4c1r {{ l3pen1: {=bool:?}, l3sam1: {=bool:?}, l3saim1: {=bool:?}, l3dam1: {=bool:?}, l3daim1: {=bool:?}, l3hsbm1: {=u8:?}, l3hdbm1: {=u8:?}, l4pen1: {=bool:?}, l4spm1: {=bool:?}, l4spim1: {=bool:?}, l4dpm1: {=bool:?}, l4dpim1: {=bool:?}, dmchn1: {=bool:?}, dmchen1: {=bool:?} }}",
                self.l3pen1(),
                self.l3sam1(),
                self.l3saim1(),
                self.l3dam1(),
                self.l3daim1(),
                self.l3hsbm1(),
                self.l3hdbm1(),
                self.l4pen1(),
                self.l4spm1(),
                self.l4spim1(),
                self.l4dpm1(),
                self.l4dpim1(),
                self.dmchn1(),
                self.dmchen1()
            )
        }
    }
    #[doc = "Layer4 Address filter 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl4a0r(pub u32);
    impl Macl4a0r {
        #[doc = "Layer 4 Source Port Number Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l4sp0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Source Port Number Field."]
        #[inline(always)]
        pub const fn set_l4sp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Layer 4 Destination Port Number Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dp0(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Destination Port Number Field."]
        #[inline(always)]
        pub const fn set_l4dp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macl4a0r {
        #[inline(always)]
        fn default() -> Macl4a0r {
            Macl4a0r(0)
        }
    }
    impl core::fmt::Debug for Macl4a0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl4a0r")
                .field("l4sp0", &self.l4sp0())
                .field("l4dp0", &self.l4dp0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl4a0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl4a0r {{ l4sp0: {=u16:?}, l4dp0: {=u16:?} }}",
                self.l4sp0(),
                self.l4dp0()
            )
        }
    }
    #[doc = "Layer 4 address filter 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macl4a1r(pub u32);
    impl Macl4a1r {
        #[doc = "Layer 4 Source Port Number Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l4sp1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Source Port Number Field."]
        #[inline(always)]
        pub const fn set_l4sp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Layer 4 Destination Port Number Field."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dp1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Destination Port Number Field."]
        #[inline(always)]
        pub const fn set_l4dp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macl4a1r {
        #[inline(always)]
        fn default() -> Macl4a1r {
            Macl4a1r(0)
        }
    }
    impl core::fmt::Debug for Macl4a1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macl4a1r")
                .field("l4sp1", &self.l4sp1())
                .field("l4dp1", &self.l4dp1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macl4a1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macl4a1r {{ l4sp1: {=u16:?}, l4dp1: {=u16:?} }}",
                self.l4sp1(),
                self.l4dp1()
            )
        }
    }
    #[doc = "LPI control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maclcsr(pub u32);
    impl Maclcsr {
        #[doc = "Transmit LPI Entry."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Entry."]
        #[inline(always)]
        pub const fn set_tlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit LPI Exit."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpiex(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Exit."]
        #[inline(always)]
        pub const fn set_tlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive LPI Entry."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Entry."]
        #[inline(always)]
        pub const fn set_rlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive LPI Exit."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpiex(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Exit."]
        #[inline(always)]
        pub const fn set_rlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit LPI State."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpist(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI State."]
        #[inline(always)]
        pub const fn set_tlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive LPI State."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpist(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI State."]
        #[inline(always)]
        pub const fn set_rlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPI Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Enable."]
        #[inline(always)]
        pub const fn set_lpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY Link Status."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY Link Status Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn plsen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status Enable."]
        #[inline(always)]
        pub const fn set_plsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPI Tx Automate."]
        #[must_use]
        #[inline(always)]
        pub const fn lpitxa(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Tx Automate."]
        #[inline(always)]
        pub const fn set_lpitxa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "LPI Timer Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpite(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Timer Enable."]
        #[inline(always)]
        pub const fn set_lpite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "LPI Tx Clock Stop Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpitcse(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Tx Clock Stop Enable."]
        #[inline(always)]
        pub const fn set_lpitcse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Maclcsr {
        #[inline(always)]
        fn default() -> Maclcsr {
            Maclcsr(0)
        }
    }
    impl core::fmt::Debug for Maclcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maclcsr")
                .field("tlpien", &self.tlpien())
                .field("tlpiex", &self.tlpiex())
                .field("rlpien", &self.rlpien())
                .field("rlpiex", &self.rlpiex())
                .field("tlpist", &self.tlpist())
                .field("rlpist", &self.rlpist())
                .field("lpien", &self.lpien())
                .field("pls", &self.pls())
                .field("plsen", &self.plsen())
                .field("lpitxa", &self.lpitxa())
                .field("lpite", &self.lpite())
                .field("lpitcse", &self.lpitcse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maclcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maclcsr {{ tlpien: {=bool:?}, tlpiex: {=bool:?}, rlpien: {=bool:?}, rlpiex: {=bool:?}, tlpist: {=bool:?}, rlpist: {=bool:?}, lpien: {=bool:?}, pls: {=bool:?}, plsen: {=bool:?}, lpitxa: {=bool:?}, lpite: {=bool:?}, lpitcse: {=bool:?} }}",
                self.tlpien(),
                self.tlpiex(),
                self.rlpien(),
                self.rlpiex(),
                self.tlpist(),
                self.rlpist(),
                self.lpien(),
                self.pls(),
                self.plsen(),
                self.lpitxa(),
                self.lpite(),
                self.lpitcse()
            )
        }
    }
    #[doc = "LPI entry timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macletr(pub u32);
    impl Macletr {
        #[doc = "LPI Entry Timer."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiet(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "LPI Entry Timer."]
        #[inline(always)]
        pub const fn set_lpiet(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Macletr {
        #[inline(always)]
        fn default() -> Macletr {
            Macletr(0)
        }
    }
    impl core::fmt::Debug for Macletr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macletr").field("lpiet", &self.lpiet()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macletr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macletr {{ lpiet: {=u32:?} }}", self.lpiet())
        }
    }
    #[doc = "Log message interval register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maclmir(pub u32);
    impl Maclmir {
        #[doc = "Log Sync Interval."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Log Sync Interval."]
        #[inline(always)]
        pub const fn set_lsi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Delay_Req to SYNC Ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn drsyncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Delay_Req to SYNC Ratio."]
        #[inline(always)]
        pub const fn set_drsyncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Log Min Pdelay_Req Interval."]
        #[must_use]
        #[inline(always)]
        pub const fn lmpdri(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Log Min Pdelay_Req Interval."]
        #[inline(always)]
        pub const fn set_lmpdri(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Maclmir {
        #[inline(always)]
        fn default() -> Maclmir {
            Maclmir(0)
        }
    }
    impl core::fmt::Debug for Maclmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maclmir")
                .field("lsi", &self.lsi())
                .field("drsyncr", &self.drsyncr())
                .field("lmpdri", &self.lmpdri())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maclmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maclmir {{ lsi: {=u8:?}, drsyncr: {=u8:?}, lmpdri: {=u8:?} }}",
                self.lsi(),
                self.drsyncr(),
                self.lmpdri()
            )
        }
    }
    #[doc = "LPI timers control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macltcr(pub u32);
    impl Macltcr {
        #[doc = "LPI TW Timer."]
        #[must_use]
        #[inline(always)]
        pub const fn twt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "LPI TW Timer."]
        #[inline(always)]
        pub const fn set_twt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "LPI LS Timer."]
        #[must_use]
        #[inline(always)]
        pub const fn lst(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "LPI LS Timer."]
        #[inline(always)]
        pub const fn set_lst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Macltcr {
        #[inline(always)]
        fn default() -> Macltcr {
            Macltcr(0)
        }
    }
    impl core::fmt::Debug for Macltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macltcr")
                .field("twt", &self.twt())
                .field("lst", &self.lst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macltcr {{ twt: {=u16:?}, lst: {=u16:?} }}", self.twt(), self.lst())
        }
    }
    #[doc = "MDIO address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmdioar(pub u32);
    impl Macmdioar {
        #[doc = "GMII Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn gb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GMII Busy."]
        #[inline(always)]
        pub const fn set_gb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clause 45 PHY Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn c45e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clause 45 PHY Enable."]
        #[inline(always)]
        pub const fn set_c45e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GMII Operation Command."]
        #[must_use]
        #[inline(always)]
        pub const fn goc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "GMII Operation Command."]
        #[inline(always)]
        pub const fn set_goc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Skip Address Packet."]
        #[must_use]
        #[inline(always)]
        pub const fn skap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Skip Address Packet."]
        #[inline(always)]
        pub const fn set_skap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CSR Clock Range."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "CSR Clock Range."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Number of Training Clocks."]
        #[must_use]
        #[inline(always)]
        pub const fn ntc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Training Clocks."]
        #[inline(always)]
        pub const fn set_ntc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Register/Device Address."]
        #[must_use]
        #[inline(always)]
        pub const fn rda(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Register/Device Address."]
        #[inline(always)]
        pub const fn set_rda(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Physical Layer Address."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "Physical Layer Address."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Back to Back transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn btb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Back to Back transactions."]
        #[inline(always)]
        pub const fn set_btb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Preamble Suppression Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preamble Suppression Enable."]
        #[inline(always)]
        pub const fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Macmdioar {
        #[inline(always)]
        fn default() -> Macmdioar {
            Macmdioar(0)
        }
    }
    impl core::fmt::Debug for Macmdioar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmdioar")
                .field("gb", &self.gb())
                .field("c45e", &self.c45e())
                .field("goc", &self.goc())
                .field("skap", &self.skap())
                .field("cr", &self.cr())
                .field("ntc", &self.ntc())
                .field("rda", &self.rda())
                .field("pa", &self.pa())
                .field("btb", &self.btb())
                .field("pse", &self.pse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmdioar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macmdioar {{ gb: {=bool:?}, c45e: {=bool:?}, goc: {=u8:?}, skap: {=bool:?}, cr: {=u8:?}, ntc: {=u8:?}, rda: {=u8:?}, pa: {=u8:?}, btb: {=bool:?}, pse: {=bool:?} }}",
                self.gb(),
                self.c45e(),
                self.goc(),
                self.skap(),
                self.cr(),
                self.ntc(),
                self.rda(),
                self.pa(),
                self.btb(),
                self.pse()
            )
        }
    }
    #[doc = "MDIO data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmdiodr(pub u32);
    impl Macmdiodr {
        #[doc = "GMII Data."]
        #[must_use]
        #[inline(always)]
        pub const fn gd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GMII Data."]
        #[inline(always)]
        pub const fn set_gd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Register Address."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Register Address."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macmdiodr {
        #[inline(always)]
        fn default() -> Macmdiodr {
            Macmdiodr(0)
        }
    }
    impl core::fmt::Debug for Macmdiodr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmdiodr")
                .field("gd", &self.gd())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macmdiodr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macmdiodr {{ gd: {=u16:?}, ra: {=u16:?} }}", self.gd(), self.ra())
        }
    }
    #[doc = "PMT control status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpcsr(pub u32);
    impl Macpcsr {
        #[doc = "Power Down."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power Down."]
        #[inline(always)]
        pub const fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic Packet Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mgkpkten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Enable."]
        #[inline(always)]
        pub const fn set_mgkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remote wake-up Packet Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkpkten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up Packet Enable."]
        #[inline(always)]
        pub const fn set_rwkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Magic Packet Received."]
        #[must_use]
        #[inline(always)]
        pub const fn mgkprcvd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Received."]
        #[inline(always)]
        pub const fn set_mgkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Remote wake-up Packet Received."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkprcvd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up Packet Received."]
        #[inline(always)]
        pub const fn set_rwkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Unicast."]
        #[must_use]
        #[inline(always)]
        pub const fn glblucast(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Unicast."]
        #[inline(always)]
        pub const fn set_glblucast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Remote wake-up Packet Forwarding Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkpfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up Packet Forwarding Enable."]
        #[inline(always)]
        pub const fn set_rwkpfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Remote wake-up FIFO Pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkptr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Remote wake-up FIFO Pointer."]
        #[inline(always)]
        pub const fn set_rwkptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Remote wake-up Packet Filter Register Pointer Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkfiltrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up Packet Filter Register Pointer Reset."]
        #[inline(always)]
        pub const fn set_rwkfiltrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpcsr {
        #[inline(always)]
        fn default() -> Macpcsr {
            Macpcsr(0)
        }
    }
    impl core::fmt::Debug for Macpcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpcsr")
                .field("pwrdwn", &self.pwrdwn())
                .field("mgkpkten", &self.mgkpkten())
                .field("rwkpkten", &self.rwkpkten())
                .field("mgkprcvd", &self.mgkprcvd())
                .field("rwkprcvd", &self.rwkprcvd())
                .field("glblucast", &self.glblucast())
                .field("rwkpfe", &self.rwkpfe())
                .field("rwkptr", &self.rwkptr())
                .field("rwkfiltrst", &self.rwkfiltrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpcsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macpcsr {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, rwkpkten: {=bool:?}, mgkprcvd: {=bool:?}, rwkprcvd: {=bool:?}, glblucast: {=bool:?}, rwkpfe: {=bool:?}, rwkptr: {=u8:?}, rwkfiltrst: {=bool:?} }}",
                self.pwrdwn(),
                self.mgkpkten(),
                self.rwkpkten(),
                self.mgkprcvd(),
                self.rwkprcvd(),
                self.glblucast(),
                self.rwkpfe(),
                self.rwkptr(),
                self.rwkfiltrst()
            )
        }
    }
    #[doc = "Packet filtering control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpfr(pub u32);
    impl Macpfr {
        #[doc = "Promiscuous Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous Mode."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash Unicast."]
        #[must_use]
        #[inline(always)]
        pub const fn huc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Unicast."]
        #[inline(always)]
        pub const fn set_huc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash Multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn hmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Multicast."]
        #[inline(always)]
        pub const fn set_hmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DA Inverse Filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DA Inverse Filtering."]
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass All Multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass All Multicast."]
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable Broadcast Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Broadcast Packets."]
        #[inline(always)]
        pub const fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass Control Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass Control Packets."]
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "SA Inverse Filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SA Inverse Filtering."]
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source Address Filter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address Filter Enable."]
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or Perfect Filter."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or Perfect Filter."]
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "VLAN Tag Filter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vtfe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Filter Enable."]
        #[inline(always)]
        pub const fn set_vtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ipfe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable."]
        #[inline(always)]
        pub const fn set_ipfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Drop Non-TCP/UDP over IP Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn dntu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Non-TCP/UDP over IP Packets."]
        #[inline(always)]
        pub const fn set_dntu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Receive All."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive All."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpfr {
        #[inline(always)]
        fn default() -> Macpfr {
            Macpfr(0)
        }
    }
    impl core::fmt::Debug for Macpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpfr")
                .field("pr", &self.pr())
                .field("huc", &self.huc())
                .field("hmc", &self.hmc())
                .field("daif", &self.daif())
                .field("pm", &self.pm())
                .field("dbf", &self.dbf())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("vtfe", &self.vtfe())
                .field("ipfe", &self.ipfe())
                .field("dntu", &self.dntu())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macpfr {{ pr: {=bool:?}, huc: {=bool:?}, hmc: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {=bool:?}, pcf: {=u8:?}, saif: {=bool:?}, saf: {=bool:?}, hpf: {=bool:?}, vtfe: {=bool:?}, ipfe: {=bool:?}, dntu: {=bool:?}, ra: {=bool:?} }}",
                self.pr(),
                self.huc(),
                self.hmc(),
                self.daif(),
                self.pm(),
                self.dbf(),
                self.pcf(),
                self.saif(),
                self.saf(),
                self.hpf(),
                self.vtfe(),
                self.ipfe(),
                self.dntu(),
                self.ra()
            )
        }
    }
    #[doc = "PHYIF control status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macphycsr(pub u32);
    impl Macphycsr {
        #[doc = "Transmit Configuration in RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Configuration in RGMII."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Up or Down."]
        #[must_use]
        #[inline(always)]
        pub const fn lud(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Up or Down."]
        #[inline(always)]
        pub const fn set_lud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lnkmod(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Link Mode."]
        #[inline(always)]
        pub const fn set_lnkmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Link Speed."]
        #[must_use]
        #[inline(always)]
        pub const fn lnkspeed(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Link Speed."]
        #[inline(always)]
        pub const fn set_lnkspeed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Link Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lnksts(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Link Status."]
        #[inline(always)]
        pub const fn set_lnksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Macphycsr {
        #[inline(always)]
        fn default() -> Macphycsr {
            Macphycsr(0)
        }
    }
    impl core::fmt::Debug for Macphycsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macphycsr")
                .field("tc", &self.tc())
                .field("lud", &self.lud())
                .field("lnkmod", &self.lnkmod())
                .field("lnkspeed", &self.lnkspeed())
                .field("lnksts", &self.lnksts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macphycsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macphycsr {{ tc: {=bool:?}, lud: {=bool:?}, lnkmod: {=bool:?}, lnkspeed: {=u8:?}, lnksts: {=bool:?} }}",
                self.tc(),
                self.lud(),
                self.lnkmod(),
                self.lnkspeed(),
                self.lnksts()
            )
        }
    }
    #[doc = "PTP Offload control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpocr(pub u32);
    impl Macpocr {
        #[doc = "PTP Offload Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ptoen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PTP Offload Enable."]
        #[inline(always)]
        pub const fn set_ptoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic PTP SYNC message Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn asyncen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP SYNC message Enable."]
        #[inline(always)]
        pub const fn set_asyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Automatic PTP Pdelay_Req message Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn apdreqen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP Pdelay_Req message Enable."]
        #[inline(always)]
        pub const fn set_apdreqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Automatic PTP SYNC message Trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn asynctrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP SYNC message Trigger."]
        #[inline(always)]
        pub const fn set_asynctrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Automatic PTP Pdelay_Req message Trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn apdreqtrig(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic PTP Pdelay_Req message Trigger."]
        #[inline(always)]
        pub const fn set_apdreqtrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable PTO Delay Request/Response response generation."]
        #[must_use]
        #[inline(always)]
        pub const fn drrdis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable PTO Delay Request/Response response generation."]
        #[inline(always)]
        pub const fn set_drrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Disable Peer Delay Response response generation."]
        #[must_use]
        #[inline(always)]
        pub const fn pdrdis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Peer Delay Response response generation."]
        #[inline(always)]
        pub const fn set_pdrdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Domain Number."]
        #[must_use]
        #[inline(always)]
        pub const fn dn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Domain Number."]
        #[inline(always)]
        pub const fn set_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Macpocr {
        #[inline(always)]
        fn default() -> Macpocr {
            Macpocr(0)
        }
    }
    impl core::fmt::Debug for Macpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpocr")
                .field("ptoen", &self.ptoen())
                .field("asyncen", &self.asyncen())
                .field("apdreqen", &self.apdreqen())
                .field("asynctrig", &self.asynctrig())
                .field("apdreqtrig", &self.apdreqtrig())
                .field("drrdis", &self.drrdis())
                .field("pdrdis", &self.pdrdis())
                .field("dn", &self.dn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macpocr {{ ptoen: {=bool:?}, asyncen: {=bool:?}, apdreqen: {=bool:?}, asynctrig: {=bool:?}, apdreqtrig: {=bool:?}, drrdis: {=bool:?}, pdrdis: {=bool:?}, dn: {=u8:?} }}",
                self.ptoen(),
                self.asyncen(),
                self.apdreqen(),
                self.asynctrig(),
                self.apdreqtrig(),
                self.drrdis(),
                self.pdrdis(),
                self.dn()
            )
        }
    }
    #[doc = "PPS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppscr(pub u32);
    impl Macppscr {
        #[doc = "Flexible PPS Output (eth_ptp_pps_out) Control."]
        #[must_use]
        #[inline(always)]
        pub const fn ppscmd(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Flexible PPS Output (eth_ptp_pps_out) Control."]
        #[inline(always)]
        pub const fn set_ppscmd(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "Flexible PPS Output 0 Mode Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsen0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible PPS Output 0 Mode Enable."]
        #[inline(always)]
        pub const fn set_ppsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Target Time Register Mode for PPS Output."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 5usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS Output."]
        #[inline(always)]
        pub const fn set_trgtmodsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 5usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "MCGR Mode Enable for PPS Output."]
        #[must_use]
        #[inline(always)]
        pub const fn mcgren(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "MCGR Mode Enable for PPS Output."]
        #[inline(always)]
        pub const fn set_mcgren(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Time Select."]
        #[must_use]
        #[inline(always)]
        pub const fn timesel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Time Select."]
        #[inline(always)]
        pub const fn set_timesel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Macppscr {
        #[inline(always)]
        fn default() -> Macppscr {
            Macppscr(0)
        }
    }
    impl core::fmt::Debug for Macppscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppscr")
                .field("ppscmd[0]", &self.ppscmd(0usize))
                .field("ppscmd[1]", &self.ppscmd(1usize))
                .field("ppsen0", &self.ppsen0())
                .field("trgtmodsel[0]", &self.trgtmodsel(0usize))
                .field("trgtmodsel[1]", &self.trgtmodsel(1usize))
                .field("mcgren[0]", &self.mcgren(0usize))
                .field("mcgren[1]", &self.mcgren(1usize))
                .field("timesel", &self.timesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macppscr {{ ppscmd[0]: {=u8:?}, ppscmd[1]: {=u8:?}, ppsen0: {=bool:?}, trgtmodsel[0]: {=u8:?}, trgtmodsel[1]: {=u8:?}, mcgren[0]: {=bool:?}, mcgren[1]: {=bool:?}, timesel: {=bool:?} }}",
                self.ppscmd(0usize),
                self.ppscmd(1usize),
                self.ppsen0(),
                self.trgtmodsel(0usize),
                self.trgtmodsel(1usize),
                self.mcgren(0usize),
                self.mcgren(1usize),
                self.timesel()
            )
        }
    }
    #[doc = "PPS interval register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsir(pub u32);
    impl Macppsir {
        #[doc = "PPS Output Signal Interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS Output Signal Interval."]
        #[inline(always)]
        pub const fn set_ppsint0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macppsir {
        #[inline(always)]
        fn default() -> Macppsir {
            Macppsir(0)
        }
    }
    impl core::fmt::Debug for Macppsir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsir").field("ppsint0", &self.ppsint0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppsir {{ ppsint0: {=u32:?} }}", self.ppsint0())
        }
    }
    #[doc = "PPS target time nanoseconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsttnr(pub u32);
    impl Macppsttnr {
        #[doc = "Target Time Low for PPS Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ttsl0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Target Time Low for PPS Register."]
        #[inline(always)]
        pub const fn set_ttsl0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "PPS Target Time Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtbusy0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PPS Target Time Register Busy."]
        #[inline(always)]
        pub const fn set_trgtbusy0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macppsttnr {
        #[inline(always)]
        fn default() -> Macppsttnr {
            Macppsttnr(0)
        }
    }
    impl core::fmt::Debug for Macppsttnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsttnr")
                .field("ttsl0", &self.ttsl0())
                .field("trgtbusy0", &self.trgtbusy0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsttnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macppsttnr {{ ttsl0: {=u32:?}, trgtbusy0: {=bool:?} }}",
                self.ttsl0(),
                self.trgtbusy0()
            )
        }
    }
    #[doc = "PPS target time seconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppsttsr(pub u32);
    impl Macppsttsr {
        #[doc = "PPS Target Time Seconds Register."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrh0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS Target Time Seconds Register."]
        #[inline(always)]
        pub const fn set_tstrh0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macppsttsr {
        #[inline(always)]
        fn default() -> Macppsttsr {
            Macppsttsr(0)
        }
    }
    impl core::fmt::Debug for Macppsttsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppsttsr").field("tstrh0", &self.tstrh0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppsttsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppsttsr {{ tstrh0: {=u32:?} }}", self.tstrh0())
        }
    }
    #[doc = "PPS width register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macppswr(pub u32);
    impl Macppswr {
        #[doc = "PPS Output Signal Width."]
        #[must_use]
        #[inline(always)]
        pub const fn ppswidth0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS Output Signal Width."]
        #[inline(always)]
        pub const fn set_ppswidth0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macppswr {
        #[inline(always)]
        fn default() -> Macppswr {
            Macppswr(0)
        }
    }
    impl core::fmt::Debug for Macppswr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macppswr")
                .field("ppswidth0", &self.ppswidth0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macppswr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macppswr {{ ppswidth0: {=u32:?} }}", self.ppswidth0())
        }
    }
    #[doc = "MAC presentation time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macprstimr(pub u32);
    impl Macprstimr {
        #[doc = "MAC 1722 Presentation Time in ns."]
        #[must_use]
        #[inline(always)]
        pub const fn mptn(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC 1722 Presentation Time in ns."]
        #[inline(always)]
        pub const fn set_mptn(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macprstimr {
        #[inline(always)]
        fn default() -> Macprstimr {
            Macprstimr(0)
        }
    }
    impl core::fmt::Debug for Macprstimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macprstimr").field("mptn", &self.mptn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macprstimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macprstimr {{ mptn: {=u32:?} }}", self.mptn())
        }
    }
    #[doc = "MAC presentation time update register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macprstimur(pub u32);
    impl Macprstimur {
        #[doc = "MAC 1722 Presentation Time Update."]
        #[must_use]
        #[inline(always)]
        pub const fn mptu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC 1722 Presentation Time Update."]
        #[inline(always)]
        pub const fn set_mptu(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macprstimur {
        #[inline(always)]
        fn default() -> Macprstimur {
            Macprstimur(0)
        }
    }
    impl core::fmt::Debug for Macprstimur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macprstimur").field("mptu", &self.mptu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macprstimur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macprstimur {{ mptu: {=u32:?} }}", self.mptu())
        }
    }
    #[doc = "Tx Queue 0 flow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacqTxFcr(pub u32);
    impl MacqTxFcr {
        #[doc = "Flow Control Busy or Backpressure Activate."]
        #[must_use]
        #[inline(always)]
        pub const fn fcb_bpa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow Control Busy or Backpressure Activate."]
        #[inline(always)]
        pub const fn set_fcb_bpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Flow Control Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Flow Control Enable."]
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Pause Low Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Pause Low Threshold."]
        #[inline(always)]
        pub const fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Disable Zero-Quanta Pause."]
        #[must_use]
        #[inline(always)]
        pub const fn dzpq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Zero-Quanta Pause."]
        #[inline(always)]
        pub const fn set_dzpq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pause Time."]
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause Time."]
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MacqTxFcr {
        #[inline(always)]
        fn default() -> MacqTxFcr {
            MacqTxFcr(0)
        }
    }
    impl core::fmt::Debug for MacqTxFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacqTxFcr")
                .field("fcb_bpa", &self.fcb_bpa())
                .field("tfe", &self.tfe())
                .field("plt", &self.plt())
                .field("dzpq", &self.dzpq())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacqTxFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacqTxFcr {{ fcb_bpa: {=bool:?}, tfe: {=bool:?}, plt: {=u8:?}, dzpq: {=bool:?}, pt: {=u16:?} }}",
                self.fcb_bpa(),
                self.tfe(),
                self.plt(),
                self.dzpq(),
                self.pt()
            )
        }
    }
    #[doc = "Remote wake-up packet filter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrwkpfr(pub u32);
    impl Macrwkpfr {
        #[doc = "Remote wake-up packet filter."]
        #[must_use]
        #[inline(always)]
        pub const fn macrwkpfr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Remote wake-up packet filter."]
        #[inline(always)]
        pub const fn set_macrwkpfr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macrwkpfr {
        #[inline(always)]
        fn default() -> Macrwkpfr {
            Macrwkpfr(0)
        }
    }
    impl core::fmt::Debug for Macrwkpfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrwkpfr")
                .field("macrwkpfr", &self.macrwkpfr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrwkpfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macrwkpfr {{ macrwkpfr: {=u32:?} }}", self.macrwkpfr())
        }
    }
    #[doc = "Rx queue control 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrxqc0r(pub u32);
    impl Macrxqc0r {
        #[doc = "Receive Queue 0 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxq0en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Queue 0 Enable."]
        #[inline(always)]
        pub const fn set_rxq0en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Receive Queue 1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxq1en(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Queue 1 Enable."]
        #[inline(always)]
        pub const fn set_rxq1en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for Macrxqc0r {
        #[inline(always)]
        fn default() -> Macrxqc0r {
            Macrxqc0r(0)
        }
    }
    impl core::fmt::Debug for Macrxqc0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrxqc0r")
                .field("rxq0en", &self.rxq0en())
                .field("rxq1en", &self.rxq1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrxqc0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macrxqc0r {{ rxq0en: {=u8:?}, rxq1en: {=u8:?} }}",
                self.rxq0en(),
                self.rxq1en()
            )
        }
    }
    #[doc = "Rx queue control 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrxqc1r(pub u32);
    impl Macrxqc1r {
        #[doc = "AV Untagged Control Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn avcpq0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AV Untagged Control Packets Queue."]
        #[inline(always)]
        pub const fn set_avcpq0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AV Untagged Control Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn avcpq1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AV Untagged Control Packets Queue."]
        #[inline(always)]
        pub const fn set_avcpq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AV Untagged Control Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn avcpq2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AV Untagged Control Packets Queue."]
        #[inline(always)]
        pub const fn set_avcpq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PTP Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn ptpq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "PTP Packets Queue."]
        #[inline(always)]
        pub const fn set_ptpq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Untagged Packet Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn upq(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Untagged Packet Queue."]
        #[inline(always)]
        pub const fn set_upq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Multicast and Broadcast Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn mcbcq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Multicast and Broadcast Queue."]
        #[inline(always)]
        pub const fn set_mcbcq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Multicast and Broadcast Queue Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcbcqen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Multicast and Broadcast Queue Enable."]
        #[inline(always)]
        pub const fn set_mcbcqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Tagged AV Control Packets Queuing Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tacpqe(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Tagged AV Control Packets Queuing Enable."]
        #[inline(always)]
        pub const fn set_tacpqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
        #[must_use]
        #[inline(always)]
        pub const fn tpqc(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
        #[inline(always)]
        pub const fn set_tpqc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn fprq0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[inline(always)]
        pub const fn set_fprq0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn fprq1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[inline(always)]
        pub const fn set_fprq1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn fprq2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Preemption Residue Queue."]
        #[inline(always)]
        pub const fn set_fprq2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Overriding MC-BC queue priority select."]
        #[must_use]
        #[inline(always)]
        pub const fn omcbcq(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Overriding MC-BC queue priority select."]
        #[inline(always)]
        pub const fn set_omcbcq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Type Field Based Rx Queuing Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbrqe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Type Field Based Rx Queuing Enable."]
        #[inline(always)]
        pub const fn set_tbrqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Macrxqc1r {
        #[inline(always)]
        fn default() -> Macrxqc1r {
            Macrxqc1r(0)
        }
    }
    impl core::fmt::Debug for Macrxqc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrxqc1r")
                .field("avcpq0", &self.avcpq0())
                .field("avcpq1", &self.avcpq1())
                .field("avcpq2", &self.avcpq2())
                .field("ptpq", &self.ptpq())
                .field("upq", &self.upq())
                .field("mcbcq", &self.mcbcq())
                .field("mcbcqen", &self.mcbcqen())
                .field("tacpqe", &self.tacpqe())
                .field("tpqc", &self.tpqc())
                .field("fprq0", &self.fprq0())
                .field("fprq1", &self.fprq1())
                .field("fprq2", &self.fprq2())
                .field("omcbcq", &self.omcbcq())
                .field("tbrqe", &self.tbrqe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrxqc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macrxqc1r {{ avcpq0: {=bool:?}, avcpq1: {=bool:?}, avcpq2: {=bool:?}, ptpq: {=u8:?}, upq: {=u8:?}, mcbcq: {=u8:?}, mcbcqen: {=bool:?}, tacpqe: {=bool:?}, tpqc: {=u8:?}, fprq0: {=bool:?}, fprq1: {=bool:?}, fprq2: {=bool:?}, omcbcq: {=bool:?}, tbrqe: {=bool:?} }}",
                self.avcpq0(),
                self.avcpq1(),
                self.avcpq2(),
                self.ptpq(),
                self.upq(),
                self.mcbcq(),
                self.mcbcqen(),
                self.tacpqe(),
                self.tpqc(),
                self.fprq0(),
                self.fprq1(),
                self.fprq2(),
                self.omcbcq(),
                self.tbrqe()
            )
        }
    }
    #[doc = "Rx queue control 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrxqc2r(pub u32);
    impl Macrxqc2r {
        #[doc = "Priorities Selected in the Receive Queue 0."]
        #[must_use]
        #[inline(always)]
        pub const fn psrq0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Priorities Selected in the Receive Queue 0."]
        #[inline(always)]
        pub const fn set_psrq0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Priorities Selected in the Receive Queue 1."]
        #[must_use]
        #[inline(always)]
        pub const fn psrq1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Priorities Selected in the Receive Queue 1."]
        #[inline(always)]
        pub const fn set_psrq1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Macrxqc2r {
        #[inline(always)]
        fn default() -> Macrxqc2r {
            Macrxqc2r(0)
        }
    }
    impl core::fmt::Debug for Macrxqc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrxqc2r")
                .field("psrq0", &self.psrq0())
                .field("psrq1", &self.psrq1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrxqc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macrxqc2r {{ psrq0: {=u8:?}, psrq1: {=u8:?} }}",
                self.psrq0(),
                self.psrq1()
            )
        }
    }
    #[doc = "Rx Queue control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macrxqcr(pub u32);
    impl Macrxqcr {
        #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uffqe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
        #[inline(always)]
        pub const fn set_uffqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Unicast Address Filter Fail Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn uffq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Address Filter Fail Packets Queue."]
        #[inline(always)]
        pub const fn set_uffq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mffqe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
        #[inline(always)]
        pub const fn set_mffqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Multicast Address Filter Fail Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn mffq(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Multicast Address Filter Fail Packets Queue."]
        #[inline(always)]
        pub const fn set_mffq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "VLAN Tag Filter Fail Packets Queuing Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vffqe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Filter Fail Packets Queuing Enable."]
        #[inline(always)]
        pub const fn set_vffqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VLAN Tag Filter Fail Packets Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn vffq(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Filter Fail Packets Queue."]
        #[inline(always)]
        pub const fn set_vffq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Macrxqcr {
        #[inline(always)]
        fn default() -> Macrxqcr {
            Macrxqcr(0)
        }
    }
    impl core::fmt::Debug for Macrxqcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macrxqcr")
                .field("uffqe", &self.uffqe())
                .field("uffq", &self.uffq())
                .field("mffqe", &self.mffqe())
                .field("mffq", &self.mffq())
                .field("vffqe", &self.vffqe())
                .field("vffq", &self.vffq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macrxqcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macrxqcr {{ uffqe: {=bool:?}, uffq: {=bool:?}, mffqe: {=bool:?}, mffq: {=bool:?}, vffqe: {=bool:?}, vffq: {=bool:?} }}",
                self.uffqe(),
                self.uffq(),
                self.mffqe(),
                self.mffq(),
                self.vffqe(),
                self.vffq()
            )
        }
    }
    #[doc = "PTP Source Port Identity 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi0r(pub u32);
    impl Macspi0r {
        #[doc = "Source Port Identity 0."]
        #[must_use]
        #[inline(always)]
        pub const fn spi0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Port Identity 0."]
        #[inline(always)]
        pub const fn set_spi0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macspi0r {
        #[inline(always)]
        fn default() -> Macspi0r {
            Macspi0r(0)
        }
    }
    impl core::fmt::Debug for Macspi0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi0r").field("spi0", &self.spi0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi0r {{ spi0: {=u32:?} }}", self.spi0())
        }
    }
    #[doc = "PTP Source port identity 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi1r(pub u32);
    impl Macspi1r {
        #[doc = "Source Port Identity 1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Port Identity 1."]
        #[inline(always)]
        pub const fn set_spi1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macspi1r {
        #[inline(always)]
        fn default() -> Macspi1r {
            Macspi1r(0)
        }
    }
    impl core::fmt::Debug for Macspi1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi1r").field("spi1", &self.spi1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi1r {{ spi1: {=u32:?} }}", self.spi1())
        }
    }
    #[doc = "PTP Source port identity 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macspi2r(pub u32);
    impl Macspi2r {
        #[doc = "Source Port Identity 2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Source Port Identity 2."]
        #[inline(always)]
        pub const fn set_spi2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macspi2r {
        #[inline(always)]
        fn default() -> Macspi2r {
            Macspi2r(0)
        }
    }
    impl core::fmt::Debug for Macspi2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macspi2r").field("spi2", &self.spi2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macspi2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macspi2r {{ spi2: {=u16:?} }}", self.spi2())
        }
    }
    #[doc = "Subsecond increment register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macssir(pub u32);
    impl Macssir {
        #[doc = "Subsecond Increment Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ssinc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Subsecond Increment Value."]
        #[inline(always)]
        pub const fn set_ssinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Macssir {
        #[inline(always)]
        fn default() -> Macssir {
            Macssir(0)
        }
    }
    impl core::fmt::Debug for Macssir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macssir").field("ssinc", &self.ssinc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macssir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macssir {{ ssinc: {=u8:?} }}", self.ssinc())
        }
    }
    #[doc = "System time nanoseconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstnr(pub u32);
    impl Macstnr {
        #[doc = "Timestamp subseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp subseconds."]
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Macstnr {
        #[inline(always)]
        fn default() -> Macstnr {
            Macstnr(0)
        }
    }
    impl core::fmt::Debug for Macstnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstnr").field("tsss", &self.tsss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstnr {{ tsss: {=u32:?} }}", self.tsss())
        }
    }
    #[doc = "System time nanoseconds update register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstnur(pub u32);
    impl Macstnur {
        #[doc = "Timestamp subseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp subseconds."]
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Add or Subtract Time."]
        #[must_use]
        #[inline(always)]
        pub const fn addsub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add or Subtract Time."]
        #[inline(always)]
        pub const fn set_addsub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macstnur {
        #[inline(always)]
        fn default() -> Macstnur {
            Macstnur(0)
        }
    }
    impl core::fmt::Debug for Macstnur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstnur")
                .field("tsss", &self.tsss())
                .field("addsub", &self.addsub())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstnur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macstnur {{ tsss: {=u32:?}, addsub: {=bool:?} }}",
                self.tsss(),
                self.addsub()
            )
        }
    }
    #[doc = "System time seconds register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstsr(pub u32);
    impl Macstsr {
        #[doc = "Timestamp Second."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Second."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macstsr {
        #[inline(always)]
        fn default() -> Macstsr {
            Macstsr(0)
        }
    }
    impl core::fmt::Debug for Macstsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstsr").field("tss", &self.tss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstsr {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "System time seconds update register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macstsur(pub u32);
    impl Macstsur {
        #[doc = "Timestamp Seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Seconds."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Macstsur {
        #[inline(always)]
        fn default() -> Macstsur {
            Macstsur(0)
        }
    }
    impl core::fmt::Debug for Macstsur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macstsur").field("tss", &self.tss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macstsur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macstsur {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "Timestamp addend register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsar(pub u32);
    impl Mactsar {
        #[doc = "Timestamp Addend Register."]
        #[must_use]
        #[inline(always)]
        pub const fn tsar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Addend Register."]
        #[inline(always)]
        pub const fn set_tsar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsar {
        #[inline(always)]
        fn default() -> Mactsar {
            Mactsar(0)
        }
    }
    impl core::fmt::Debug for Mactsar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsar").field("tsar", &self.tsar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsar {{ tsar: {=u32:?} }}", self.tsar())
        }
    }
    #[doc = "Timestamp control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactscr(pub u32);
    impl Mactscr {
        #[doc = "Enable Timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn tsena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp."]
        #[inline(always)]
        pub const fn set_tsena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fine or Coarse Timestamp Update."]
        #[must_use]
        #[inline(always)]
        pub const fn tscfupdt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fine or Coarse Timestamp Update."]
        #[inline(always)]
        pub const fn set_tscfupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Initialize Timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn tsinit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Initialize Timestamp."]
        #[inline(always)]
        pub const fn set_tsinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Update Timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn tsupdt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Update Timestamp."]
        #[inline(always)]
        pub const fn set_tsupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Update Addend Register."]
        #[must_use]
        #[inline(always)]
        pub const fn tsaddreg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Update Addend Register."]
        #[inline(always)]
        pub const fn set_tsaddreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Presentation Time Generation Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ptge(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Presentation Time Generation Enable."]
        #[inline(always)]
        pub const fn set_ptge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Timestamp for All Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tsenall(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp for All Packets."]
        #[inline(always)]
        pub const fn set_tsenall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timestamp Digital or Binary Rollover Control."]
        #[must_use]
        #[inline(always)]
        pub const fn tsctrlssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Digital or Binary Rollover Control."]
        #[inline(always)]
        pub const fn set_tsctrlssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable PTP Packet Processing for Version 2 Format."]
        #[must_use]
        #[inline(always)]
        pub const fn tsver2ena(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PTP Packet Processing for Version 2 Format."]
        #[inline(always)]
        pub const fn set_tsver2ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Processing of PTP over Ethernet Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipena(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP over Ethernet Packets."]
        #[inline(always)]
        pub const fn set_tsipena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipv6ena(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP."]
        #[inline(always)]
        pub const fn set_tsipv6ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipv4ena(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP."]
        #[inline(always)]
        pub const fn set_tsipv4ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages."]
        #[must_use]
        #[inline(always)]
        pub const fn tsevntena(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages."]
        #[inline(always)]
        pub const fn set_tsevntena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master."]
        #[must_use]
        #[inline(always)]
        pub const fn tsmstrena(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master."]
        #[inline(always)]
        pub const fn set_tsmstrena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Select PTP packets for Taking Snapshots."]
        #[must_use]
        #[inline(always)]
        pub const fn snaptypsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Select PTP packets for Taking Snapshots."]
        #[inline(always)]
        pub const fn set_snaptypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Enable MAC Address for PTP Packet Filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn tsenmacaddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable MAC Address for PTP Packet Filtering."]
        #[inline(always)]
        pub const fn set_tsenmacaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "External System Time Input."]
        #[must_use]
        #[inline(always)]
        pub const fn esti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External System Time Input."]
        #[inline(always)]
        pub const fn set_esti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit Timestamp Status Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn txtsstsm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Timestamp Status Mode."]
        #[inline(always)]
        pub const fn set_txtsstsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "AV 802.1AS Mode Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn av8021asmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "AV 802.1AS Mode Enable."]
        #[inline(always)]
        pub const fn set_av8021asmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Mactscr {
        #[inline(always)]
        fn default() -> Mactscr {
            Mactscr(0)
        }
    }
    impl core::fmt::Debug for Mactscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactscr")
                .field("tsena", &self.tsena())
                .field("tscfupdt", &self.tscfupdt())
                .field("tsinit", &self.tsinit())
                .field("tsupdt", &self.tsupdt())
                .field("tsaddreg", &self.tsaddreg())
                .field("ptge", &self.ptge())
                .field("tsenall", &self.tsenall())
                .field("tsctrlssr", &self.tsctrlssr())
                .field("tsver2ena", &self.tsver2ena())
                .field("tsipena", &self.tsipena())
                .field("tsipv6ena", &self.tsipv6ena())
                .field("tsipv4ena", &self.tsipv4ena())
                .field("tsevntena", &self.tsevntena())
                .field("tsmstrena", &self.tsmstrena())
                .field("snaptypsel", &self.snaptypsel())
                .field("tsenmacaddr", &self.tsenmacaddr())
                .field("esti", &self.esti())
                .field("txtsstsm", &self.txtsstsm())
                .field("av8021asmen", &self.av8021asmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mactscr {{ tsena: {=bool:?}, tscfupdt: {=bool:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tsaddreg: {=bool:?}, ptge: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {=bool:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, esti: {=bool:?}, txtsstsm: {=bool:?}, av8021asmen: {=bool:?} }}",
                self.tsena(),
                self.tscfupdt(),
                self.tsinit(),
                self.tsupdt(),
                self.tsaddreg(),
                self.ptge(),
                self.tsenall(),
                self.tsctrlssr(),
                self.tsver2ena(),
                self.tsipena(),
                self.tsipv6ena(),
                self.tsipv4ena(),
                self.tsevntena(),
                self.tsmstrena(),
                self.snaptypsel(),
                self.tsenmacaddr(),
                self.esti(),
                self.txtsstsm(),
                self.av8021asmen()
            )
        }
    }
    #[doc = "Timestamp Egress asymmetric correction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactseacr(pub u32);
    impl Mactseacr {
        #[doc = "One-Step Timestamp Egress Asymmetry Correction."]
        #[must_use]
        #[inline(always)]
        pub const fn osteac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "One-Step Timestamp Egress Asymmetry Correction."]
        #[inline(always)]
        pub const fn set_osteac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactseacr {
        #[inline(always)]
        fn default() -> Mactseacr {
            Mactseacr(0)
        }
    }
    impl core::fmt::Debug for Mactseacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactseacr").field("osteac", &self.osteac()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactseacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactseacr {{ osteac: {=u32:?} }}", self.osteac())
        }
    }
    #[doc = "Timestamp Egress correction nanosecond register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsecnr(pub u32);
    impl Mactsecnr {
        #[doc = "Timestamp Egress Correction."]
        #[must_use]
        #[inline(always)]
        pub const fn tsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Egress Correction."]
        #[inline(always)]
        pub const fn set_tsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsecnr {
        #[inline(always)]
        fn default() -> Mactsecnr {
            Mactsecnr(0)
        }
    }
    impl core::fmt::Debug for Mactsecnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsecnr").field("tsec", &self.tsec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsecnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsecnr {{ tsec: {=u32:?} }}", self.tsec())
        }
    }
    #[doc = "Timestamp Egress Latency register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactselr(pub u32);
    impl Mactselr {
        #[doc = "Egress Timestamp Latency, in subnanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn etlsns(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Egress Timestamp Latency, in subnanoseconds."]
        #[inline(always)]
        pub const fn set_etlsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Egress Timestamp Latency, in nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn etlns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Egress Timestamp Latency, in nanoseconds."]
        #[inline(always)]
        pub const fn set_etlns(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mactselr {
        #[inline(always)]
        fn default() -> Mactselr {
            Mactselr(0)
        }
    }
    impl core::fmt::Debug for Mactselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactselr")
                .field("etlsns", &self.etlsns())
                .field("etlns", &self.etlns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mactselr {{ etlsns: {=u8:?}, etlns: {=u16:?} }}",
                self.etlsns(),
                self.etlns()
            )
        }
    }
    #[doc = "Timestamp Ingress asymmetric correction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsiacr(pub u32);
    impl Mactsiacr {
        #[doc = "One-Step Timestamp Ingress Asymmetry Correction."]
        #[must_use]
        #[inline(always)]
        pub const fn ostiac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "One-Step Timestamp Ingress Asymmetry Correction."]
        #[inline(always)]
        pub const fn set_ostiac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsiacr {
        #[inline(always)]
        fn default() -> Mactsiacr {
            Mactsiacr(0)
        }
    }
    impl core::fmt::Debug for Mactsiacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsiacr").field("ostiac", &self.ostiac()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsiacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsiacr {{ ostiac: {=u32:?} }}", self.ostiac())
        }
    }
    #[doc = "Timestamp Ingress correction nanosecond register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsicnr(pub u32);
    impl Mactsicnr {
        #[doc = "Timestamp Ingress Correction."]
        #[must_use]
        #[inline(always)]
        pub const fn tsic(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Ingress Correction."]
        #[inline(always)]
        pub const fn set_tsic(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mactsicnr {
        #[inline(always)]
        fn default() -> Mactsicnr {
            Mactsicnr(0)
        }
    }
    impl core::fmt::Debug for Mactsicnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsicnr").field("tsic", &self.tsic()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsicnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mactsicnr {{ tsic: {=u32:?} }}", self.tsic())
        }
    }
    #[doc = "Timestamp Ingress Latency register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactsilr(pub u32);
    impl Mactsilr {
        #[doc = "Ingress Timestamp Latency, in subnanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn itlsns(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Ingress Timestamp Latency, in subnanoseconds."]
        #[inline(always)]
        pub const fn set_itlsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Ingress Timestamp Latency, in nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn itlns(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Ingress Timestamp Latency, in nanoseconds."]
        #[inline(always)]
        pub const fn set_itlns(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Mactsilr {
        #[inline(always)]
        fn default() -> Mactsilr {
            Mactsilr(0)
        }
    }
    impl core::fmt::Debug for Mactsilr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactsilr")
                .field("itlsns", &self.itlsns())
                .field("itlns", &self.itlns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactsilr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mactsilr {{ itlsns: {=u8:?}, itlns: {=u16:?} }}",
                self.itlsns(),
                self.itlns()
            )
        }
    }
    #[doc = "Timestamp status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mactssr(pub u32);
    impl Mactssr {
        #[doc = "Timestamp Seconds Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn tssovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Seconds Overflow."]
        #[inline(always)]
        pub const fn set_tssovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timestamp Target Time Reached."]
        #[must_use]
        #[inline(always)]
        pub const fn tstargt(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Reached."]
        #[inline(always)]
        pub const fn set_tstargt(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Auxiliary Timestamp Trigger Snapshot."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtstrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Timestamp Trigger Snapshot."]
        #[inline(always)]
        pub const fn set_auxtstrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp Target Time Error."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Error."]
        #[inline(always)]
        pub const fn set_tstrgterr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tx Timestamp Status Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txtssis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Timestamp Status Interrupt Status."]
        #[inline(always)]
        pub const fn set_txtssis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn atsstn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier."]
        #[inline(always)]
        pub const fn set_atsstn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed."]
        #[must_use]
        #[inline(always)]
        pub const fn atsstm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed."]
        #[inline(always)]
        pub const fn set_atsstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots."]
        #[must_use]
        #[inline(always)]
        pub const fn atsns(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots."]
        #[inline(always)]
        pub const fn set_atsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Mactssr {
        #[inline(always)]
        fn default() -> Mactssr {
            Mactssr(0)
        }
    }
    impl core::fmt::Debug for Mactssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mactssr")
                .field("tssovf", &self.tssovf())
                .field("tstargt[0]", &self.tstargt(0usize))
                .field("tstargt[1]", &self.tstargt(1usize))
                .field("auxtstrig", &self.auxtstrig())
                .field("tstrgterr[0]", &self.tstrgterr(0usize))
                .field("tstrgterr[1]", &self.tstrgterr(1usize))
                .field("txtssis", &self.txtssis())
                .field("atsstn", &self.atsstn())
                .field("atsstm", &self.atsstm())
                .field("atsns", &self.atsns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mactssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mactssr {{ tssovf: {=bool:?}, tstargt[0]: {=bool:?}, tstargt[1]: {=bool:?}, auxtstrig: {=bool:?}, tstrgterr[0]: {=bool:?}, tstrgterr[1]: {=bool:?}, txtssis: {=bool:?}, atsstn: {=u8:?}, atsstm: {=bool:?}, atsns: {=u8:?} }}",
                self.tssovf(),
                self.tstargt(0usize),
                self.tstargt(1usize),
                self.auxtstrig(),
                self.tstrgterr(0usize),
                self.tstrgterr(1usize),
                self.txtssis(),
                self.atsstn(),
                self.atsstm(),
                self.atsns()
            )
        }
    }
    #[doc = "VLAN Hash table register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvhtr(pub u32);
    impl Macvhtr {
        #[doc = "VLAN Hash Table."]
        #[must_use]
        #[inline(always)]
        pub const fn vlht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Hash Table."]
        #[inline(always)]
        pub const fn set_vlht(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macvhtr {
        #[inline(always)]
        fn default() -> Macvhtr {
            Macvhtr(0)
        }
    }
    impl core::fmt::Debug for Macvhtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvhtr").field("vlht", &self.vlht()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvhtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macvhtr {{ vlht: {=u16:?} }}", self.vlht())
        }
    }
    #[doc = "VLAN inclusion register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvir(pub u32);
    impl Macvir {
        #[doc = "VLAN Tag for Transmit Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag for Transmit Packets."]
        #[inline(always)]
        pub const fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Control in Transmit Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "VLAN Tag Control in Transmit Packets."]
        #[inline(always)]
        pub const fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VLAN Priority Control."]
        #[must_use]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Priority Control."]
        #[inline(always)]
        pub const fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "C-VLAN or S-VLAN."]
        #[must_use]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "C-VLAN or S-VLAN."]
        #[inline(always)]
        pub const fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "VLAN Tag Input."]
        #[must_use]
        #[inline(always)]
        pub const fn vlti(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Input."]
        #[inline(always)]
        pub const fn set_vlti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Channel based tag insertion."]
        #[must_use]
        #[inline(always)]
        pub const fn cbti(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Channel based tag insertion."]
        #[inline(always)]
        pub const fn set_cbti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Read write control."]
        #[must_use]
        #[inline(always)]
        pub const fn rdwr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Read write control."]
        #[inline(always)]
        pub const fn set_rdwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macvir {
        #[inline(always)]
        fn default() -> Macvir {
            Macvir(0)
        }
    }
    impl core::fmt::Debug for Macvir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvir")
                .field("vlt", &self.vlt())
                .field("vlc", &self.vlc())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .field("vlti", &self.vlti())
                .field("cbti", &self.cbti())
                .field("addr", &self.addr())
                .field("rdwr", &self.rdwr())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvir {{ vlt: {=u16:?}, vlc: {=u8:?}, vlp: {=bool:?}, csvl: {=bool:?}, vlti: {=bool:?}, cbti: {=bool:?}, addr: {=bool:?}, rdwr: {=bool:?}, busy: {=bool:?} }}",
                self.vlt(),
                self.vlc(),
                self.vlp(),
                self.csvl(),
                self.vlti(),
                self.cbti(),
                self.addr(),
                self.rdwr(),
                self.busy()
            )
        }
    }
    #[doc = "Version register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvr(pub u32);
    impl Macvr {
        #[doc = "IP version."]
        #[must_use]
        #[inline(always)]
        pub const fn snpsver(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "IP version."]
        #[inline(always)]
        pub const fn set_snpsver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "ST-defined version."]
        #[must_use]
        #[inline(always)]
        pub const fn userver(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "ST-defined version."]
        #[inline(always)]
        pub const fn set_userver(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Macvr {
        #[inline(always)]
        fn default() -> Macvr {
            Macvr(0)
        }
    }
    impl core::fmt::Debug for Macvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvr")
                .field("snpsver", &self.snpsver())
                .field("userver", &self.userver())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvr {{ snpsver: {=u8:?}, userver: {=u8:?} }}",
                self.snpsver(),
                self.userver()
            )
        }
    }
    #[doc = "VLAN tag Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvtcr(pub u32);
    impl Macvtcr {
        #[doc = "Operation Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn ob(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Operation Busy."]
        #[inline(always)]
        pub const fn set_ob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command Type."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command Type."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn ofs(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Offset."]
        #[inline(always)]
        pub const fn set_ofs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn etv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison."]
        #[inline(always)]
        pub const fn set_etv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VLAN Tag Inverse Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vtim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Inverse Match Enable."]
        #[inline(always)]
        pub const fn set_vtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable S-VLAN."]
        #[must_use]
        #[inline(always)]
        pub const fn esvl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-VLAN."]
        #[inline(always)]
        pub const fn set_esvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Receive S-VLAN Match."]
        #[must_use]
        #[inline(always)]
        pub const fn ersvlm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Receive S-VLAN Match."]
        #[inline(always)]
        pub const fn set_ersvlm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Disable VLAN Type Check."]
        #[must_use]
        #[inline(always)]
        pub const fn dovltc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Disable VLAN Type Check."]
        #[inline(always)]
        pub const fn set_dovltc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable VLAN Tag Stripping on Receive."]
        #[must_use]
        #[inline(always)]
        pub const fn evls(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Enable VLAN Tag Stripping on Receive."]
        #[inline(always)]
        pub const fn set_evls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "Enable VLAN Tag in Rx status."]
        #[must_use]
        #[inline(always)]
        pub const fn evlrxs(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable VLAN Tag in Rx status."]
        #[inline(always)]
        pub const fn set_evlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VLAN Tag Hash Table Match Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vthm(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Hash Table Match Enable."]
        #[inline(always)]
        pub const fn set_vthm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Enable Double VLAN Processing."]
        #[must_use]
        #[inline(always)]
        pub const fn edvlp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Double VLAN Processing."]
        #[inline(always)]
        pub const fn set_edvlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Enable Inner VLAN Tag."]
        #[must_use]
        #[inline(always)]
        pub const fn erivlt(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Inner VLAN Tag."]
        #[inline(always)]
        pub const fn set_erivlt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Enable Inner VLAN Tag Stripping on Receive."]
        #[must_use]
        #[inline(always)]
        pub const fn eivls(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Enable Inner VLAN Tag Stripping on Receive."]
        #[inline(always)]
        pub const fn set_eivls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Enable Inner VLAN Tag in Rx Status."]
        #[must_use]
        #[inline(always)]
        pub const fn eivlrxs(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Inner VLAN Tag in Rx Status."]
        #[inline(always)]
        pub const fn set_eivlrxs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macvtcr {
        #[inline(always)]
        fn default() -> Macvtcr {
            Macvtcr(0)
        }
    }
    impl core::fmt::Debug for Macvtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvtcr")
                .field("ob", &self.ob())
                .field("ct", &self.ct())
                .field("ofs", &self.ofs())
                .field("etv", &self.etv())
                .field("vtim", &self.vtim())
                .field("esvl", &self.esvl())
                .field("ersvlm", &self.ersvlm())
                .field("dovltc", &self.dovltc())
                .field("evls", &self.evls())
                .field("evlrxs", &self.evlrxs())
                .field("vthm", &self.vthm())
                .field("edvlp", &self.edvlp())
                .field("erivlt", &self.erivlt())
                .field("eivls", &self.eivls())
                .field("eivlrxs", &self.eivlrxs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvtcr {{ ob: {=bool:?}, ct: {=bool:?}, ofs: {=u8:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, ersvlm: {=bool:?}, dovltc: {=bool:?}, evls: {=u8:?}, evlrxs: {=bool:?}, vthm: {=bool:?}, edvlp: {=bool:?}, erivlt: {=bool:?}, eivls: {=u8:?}, eivlrxs: {=bool:?} }}",
                self.ob(),
                self.ct(),
                self.ofs(),
                self.etv(),
                self.vtim(),
                self.esvl(),
                self.ersvlm(),
                self.dovltc(),
                self.evls(),
                self.evlrxs(),
                self.vthm(),
                self.edvlp(),
                self.erivlt(),
                self.eivls(),
                self.eivlrxs()
            )
        }
    }
    #[doc = "VLAN tag data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvtdr(pub u32);
    impl Macvtdr {
        #[doc = "VLAN Tag ID."]
        #[must_use]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag ID."]
        #[inline(always)]
        pub const fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ven(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Enable."]
        #[inline(always)]
        pub const fn set_ven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "12-bit or 16-bit VLAN comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn etv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "12-bit or 16-bit VLAN comparison."]
        #[inline(always)]
        pub const fn set_etv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Disable VLAN Type Comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn dovltc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Disable VLAN Type Comparison."]
        #[inline(always)]
        pub const fn set_dovltc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable S-VLAN Match for received Frames."]
        #[must_use]
        #[inline(always)]
        pub const fn ersvlm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-VLAN Match for received Frames."]
        #[inline(always)]
        pub const fn set_ersvlm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable Inner VLAN Tag Comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn erivlt(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Inner VLAN Tag Comparison."]
        #[inline(always)]
        pub const fn set_erivlt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Channel Number Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmachen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Number Enable."]
        #[inline(always)]
        pub const fn set_dmachen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DMA Channel Number."]
        #[must_use]
        #[inline(always)]
        pub const fn dmachn(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Channel Number."]
        #[inline(always)]
        pub const fn set_dmachn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Macvtdr {
        #[inline(always)]
        fn default() -> Macvtdr {
            Macvtdr(0)
        }
    }
    impl core::fmt::Debug for Macvtdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvtdr")
                .field("vid", &self.vid())
                .field("ven", &self.ven())
                .field("etv", &self.etv())
                .field("dovltc", &self.dovltc())
                .field("ersvlm", &self.ersvlm())
                .field("erivlt", &self.erivlt())
                .field("dmachen", &self.dmachen())
                .field("dmachn", &self.dmachn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macvtdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Macvtdr {{ vid: {=u16:?}, ven: {=bool:?}, etv: {=bool:?}, dovltc: {=bool:?}, ersvlm: {=bool:?}, erivlt: {=bool:?}, dmachen: {=bool:?}, dmachn: {=bool:?} }}",
                self.vid(),
                self.ven(),
                self.etv(),
                self.dovltc(),
                self.ersvlm(),
                self.erivlt(),
                self.dmachen(),
                self.dmachn()
            )
        }
    }
    #[doc = "Watchdog timeout register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macwtr(pub u32);
    impl Macwtr {
        #[doc = "Watchdog Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn wto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Watchdog Timeout."]
        #[inline(always)]
        pub const fn set_wto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Programmable Watchdog Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable Watchdog Enable."]
        #[inline(always)]
        pub const fn set_pwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Macwtr {
        #[inline(always)]
        fn default() -> Macwtr {
            Macwtr(0)
        }
    }
    impl core::fmt::Debug for Macwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macwtr")
                .field("wto", &self.wto())
                .field("pwe", &self.pwe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Macwtr {{ wto: {=u8:?}, pwe: {=bool:?} }}", self.wto(), self.pwe())
        }
    }
    #[doc = "MMC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcControl(pub u32);
    impl MmcControl {
        #[doc = "Counters Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Reset."]
        #[inline(always)]
        pub const fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter Stop Rollover."]
        #[must_use]
        #[inline(always)]
        pub const fn cntstopro(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter Stop Rollover."]
        #[inline(always)]
        pub const fn set_cntstopro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset on Read."]
        #[must_use]
        #[inline(always)]
        pub const fn rstonrd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on Read."]
        #[inline(always)]
        pub const fn set_rstonrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Counter Freeze."]
        #[must_use]
        #[inline(always)]
        pub const fn cntfreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Counter Freeze."]
        #[inline(always)]
        pub const fn set_cntfreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Counters Preset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Preset."]
        #[inline(always)]
        pub const fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Full-Half Preset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntprstlvl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Full-Half Preset."]
        #[inline(always)]
        pub const fn set_cntprstlvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn ucdbc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Packets."]
        #[inline(always)]
        pub const fn set_ucdbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MmcControl {
        #[inline(always)]
        fn default() -> MmcControl {
            MmcControl(0)
        }
    }
    impl core::fmt::Debug for MmcControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcControl")
                .field("cntrst", &self.cntrst())
                .field("cntstopro", &self.cntstopro())
                .field("rstonrd", &self.rstonrd())
                .field("cntfreez", &self.cntfreez())
                .field("cntprst", &self.cntprst())
                .field("cntprstlvl", &self.cntprstlvl())
                .field("ucdbc", &self.ucdbc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcControl {{ cntrst: {=bool:?}, cntstopro: {=bool:?}, rstonrd: {=bool:?}, cntfreez: {=bool:?}, cntprst: {=bool:?}, cntprstlvl: {=bool:?}, ucdbc: {=bool:?} }}",
                self.cntrst(),
                self.cntstopro(),
                self.rstonrd(),
                self.cntfreez(),
                self.cntprst(),
                self.cntprstlvl(),
                self.ucdbc()
            )
        }
    }
    #[doc = "MMC FPE Rx interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcFpeRxImr(pub u32);
    impl MmcFpeRxImr {
        #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn paecim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_paecim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Rx Packet SMD Error Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn psecim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet SMD Error Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_psecim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn paocim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_paocim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Rx FPE Fragment Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn fcim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx FPE Fragment Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_fcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for MmcFpeRxImr {
        #[inline(always)]
        fn default() -> MmcFpeRxImr {
            MmcFpeRxImr(0)
        }
    }
    impl core::fmt::Debug for MmcFpeRxImr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcFpeRxImr")
                .field("paecim", &self.paecim())
                .field("psecim", &self.psecim())
                .field("paocim", &self.paocim())
                .field("fcim", &self.fcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcFpeRxImr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcFpeRxImr {{ paecim: {=bool:?}, psecim: {=bool:?}, paocim: {=bool:?}, fcim: {=bool:?} }}",
                self.paecim(),
                self.psecim(),
                self.paocim(),
                self.fcim()
            )
        }
    }
    #[doc = "MMC FPE Rx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcFpeRxIsr(pub u32);
    impl MmcFpeRxIsr {
        #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn paecis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet Assembly Error Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_paecis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Rx Packet SMD Error Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn psecis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet SMD Error Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_psecis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn paocis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx Packet Assembly OK Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_paocis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Rx FPE Fragment Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn fcis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Rx FPE Fragment Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_fcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for MmcFpeRxIsr {
        #[inline(always)]
        fn default() -> MmcFpeRxIsr {
            MmcFpeRxIsr(0)
        }
    }
    impl core::fmt::Debug for MmcFpeRxIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcFpeRxIsr")
                .field("paecis", &self.paecis())
                .field("psecis", &self.psecis())
                .field("paocis", &self.paocis())
                .field("fcis", &self.fcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcFpeRxIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcFpeRxIsr {{ paecis: {=bool:?}, psecis: {=bool:?}, paocis: {=bool:?}, fcis: {=bool:?} }}",
                self.paecis(),
                self.psecis(),
                self.paocis(),
                self.fcis()
            )
        }
    }
    #[doc = "MMC FPE Tx fragment counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcFpeTxFcr(pub u32);
    impl MmcFpeTxFcr {
        #[doc = "Tx FPE Fragment counter."]
        #[must_use]
        #[inline(always)]
        pub const fn txffc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx FPE Fragment counter."]
        #[inline(always)]
        pub const fn set_txffc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MmcFpeTxFcr {
        #[inline(always)]
        fn default() -> MmcFpeTxFcr {
            MmcFpeTxFcr(0)
        }
    }
    impl core::fmt::Debug for MmcFpeTxFcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcFpeTxFcr").field("txffc", &self.txffc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcFpeTxFcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MmcFpeTxFcr {{ txffc: {=u32:?} }}", self.txffc())
        }
    }
    #[doc = "MMC FPE Tx interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcFpeTxImr(pub u32);
    impl MmcFpeTxImr {
        #[doc = "MMC Transmit Fragment Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn fcim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Fragment Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_fcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Transmit Hold Request Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Hold Request Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_hrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MmcFpeTxImr {
        #[inline(always)]
        fn default() -> MmcFpeTxImr {
            MmcFpeTxImr(0)
        }
    }
    impl core::fmt::Debug for MmcFpeTxImr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcFpeTxImr")
                .field("fcim", &self.fcim())
                .field("hrcim", &self.hrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcFpeTxImr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcFpeTxImr {{ fcim: {=bool:?}, hrcim: {=bool:?} }}",
                self.fcim(),
                self.hrcim()
            )
        }
    }
    #[doc = "MMC FPE Tx interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcFpeTxIsr(pub u32);
    impl MmcFpeTxIsr {
        #[doc = "MMC Tx FPE Fragment Counter Interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn fcis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Tx FPE Fragment Counter Interrupt status."]
        #[inline(always)]
        pub const fn set_fcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Tx Hold Request Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn hrcis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Tx Hold Request Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_hrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MmcFpeTxIsr {
        #[inline(always)]
        fn default() -> MmcFpeTxIsr {
            MmcFpeTxIsr(0)
        }
    }
    impl core::fmt::Debug for MmcFpeTxIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcFpeTxIsr")
                .field("fcis", &self.fcis())
                .field("hrcis", &self.hrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcFpeTxIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcFpeTxIsr {{ fcis: {=bool:?}, hrcis: {=bool:?} }}",
                self.fcis(),
                self.hrcis()
            )
        }
    }
    #[doc = "MMC Rx interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxInterrupt(pub u32);
    impl MmcRxInterrupt {
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerpis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_rxcrcerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerpis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_rxalgnerpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxucgpis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_rxucgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt status."]
        #[inline(always)]
        pub const fn set_rxlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive LPI transition counter interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI transition counter interrupt status."]
        #[inline(always)]
        pub const fn set_rxlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcRxInterrupt {
        #[inline(always)]
        fn default() -> MmcRxInterrupt {
            MmcRxInterrupt(0)
        }
    }
    impl core::fmt::Debug for MmcRxInterrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxInterrupt")
                .field("rxcrcerpis", &self.rxcrcerpis())
                .field("rxalgnerpis", &self.rxalgnerpis())
                .field("rxucgpis", &self.rxucgpis())
                .field("rxlpiuscis", &self.rxlpiuscis())
                .field("rxlpitrcis", &self.rxlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcRxInterrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcRxInterrupt {{ rxcrcerpis: {=bool:?}, rxalgnerpis: {=bool:?}, rxucgpis: {=bool:?}, rxlpiuscis: {=bool:?}, rxlpitrcis: {=bool:?} }}",
                self.rxcrcerpis(),
                self.rxalgnerpis(),
                self.rxucgpis(),
                self.rxlpiuscis(),
                self.rxlpitrcis()
            )
        }
    }
    #[doc = "MMC Rx interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcRxInterruptMask(pub u32);
    impl MmcRxInterruptMask {
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerpim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_rxcrcerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerpim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_rxalgnerpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn rxucgpim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_rxucgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI microsecond counter interrupt Mask."]
        #[inline(always)]
        pub const fn set_rxlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive LPI transition counter interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive LPI transition counter interrupt Mask."]
        #[inline(always)]
        pub const fn set_rxlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcRxInterruptMask {
        #[inline(always)]
        fn default() -> MmcRxInterruptMask {
            MmcRxInterruptMask(0)
        }
    }
    impl core::fmt::Debug for MmcRxInterruptMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcRxInterruptMask")
                .field("rxcrcerpim", &self.rxcrcerpim())
                .field("rxalgnerpim", &self.rxalgnerpim())
                .field("rxucgpim", &self.rxucgpim())
                .field("rxlpiuscim", &self.rxlpiuscim())
                .field("rxlpitrcim", &self.rxlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcRxInterruptMask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcRxInterruptMask {{ rxcrcerpim: {=bool:?}, rxalgnerpim: {=bool:?}, rxucgpim: {=bool:?}, rxlpiuscim: {=bool:?}, rxlpitrcim: {=bool:?} }}",
                self.rxcrcerpim(),
                self.rxalgnerpim(),
                self.rxucgpim(),
                self.rxlpiuscim(),
                self.rxlpitrcim()
            )
        }
    }
    #[doc = "MMC Tx hold request counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxHrcr(pub u32);
    impl MmcTxHrcr {
        #[doc = "Tx Hold Request Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn txhrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Hold Request Counter."]
        #[inline(always)]
        pub const fn set_txhrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MmcTxHrcr {
        #[inline(always)]
        fn default() -> MmcTxHrcr {
            MmcTxHrcr(0)
        }
    }
    impl core::fmt::Debug for MmcTxHrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxHrcr").field("txhrc", &self.txhrc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcTxHrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MmcTxHrcr {{ txhrc: {=u32:?} }}", self.txhrc())
        }
    }
    #[doc = "MMC Tx interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxInterrupt(pub u32);
    impl MmcTxInterrupt {
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txscolgpis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_txscolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgpis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_txmcolgpis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txgpktis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Status."]
        #[inline(always)]
        pub const fn set_txgpktis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpiuscis(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt status."]
        #[inline(always)]
        pub const fn set_txlpiuscis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Transmit LPI transition counter interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpitrcis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI transition counter interrupt status."]
        #[inline(always)]
        pub const fn set_txlpitrcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcTxInterrupt {
        #[inline(always)]
        fn default() -> MmcTxInterrupt {
            MmcTxInterrupt(0)
        }
    }
    impl core::fmt::Debug for MmcTxInterrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxInterrupt")
                .field("txscolgpis", &self.txscolgpis())
                .field("txmcolgpis", &self.txmcolgpis())
                .field("txgpktis", &self.txgpktis())
                .field("txlpiuscis", &self.txlpiuscis())
                .field("txlpitrcis", &self.txlpitrcis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcTxInterrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcTxInterrupt {{ txscolgpis: {=bool:?}, txmcolgpis: {=bool:?}, txgpktis: {=bool:?}, txlpiuscis: {=bool:?}, txlpitrcis: {=bool:?} }}",
                self.txscolgpis(),
                self.txmcolgpis(),
                self.txgpktis(),
                self.txlpiuscis(),
                self.txlpitrcis()
            )
        }
    }
    #[doc = "MMC Tx interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcTxInterruptMask(pub u32);
    impl MmcTxInterruptMask {
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn txscolgpim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_txscolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgpim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_txmcolgpim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn txgpktim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Packet Counter Interrupt Mask."]
        #[inline(always)]
        pub const fn set_txgpktim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpiuscim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI microsecond counter interrupt Mask."]
        #[inline(always)]
        pub const fn set_txlpiuscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Transmit LPI transition counter interrupt Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpitrcim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit LPI transition counter interrupt Mask."]
        #[inline(always)]
        pub const fn set_txlpitrcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MmcTxInterruptMask {
        #[inline(always)]
        fn default() -> MmcTxInterruptMask {
            MmcTxInterruptMask(0)
        }
    }
    impl core::fmt::Debug for MmcTxInterruptMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcTxInterruptMask")
                .field("txscolgpim", &self.txscolgpim())
                .field("txmcolgpim", &self.txmcolgpim())
                .field("txgpktim", &self.txgpktim())
                .field("txlpiuscim", &self.txlpiuscim())
                .field("txlpitrcim", &self.txlpitrcim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcTxInterruptMask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MmcTxInterruptMask {{ txscolgpim: {=bool:?}, txmcolgpim: {=bool:?}, txgpktim: {=bool:?}, txlpiuscim: {=bool:?}, txlpitrcim: {=bool:?} }}",
                self.txscolgpim(),
                self.txmcolgpim(),
                self.txgpktim(),
                self.txlpiuscim(),
                self.txlpitrcim()
            )
        }
    }
    #[doc = "Rx queue control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlRxQcr(pub u32);
    impl MtlRxQcr {
        #[doc = "Receive Queue Weight."]
        #[must_use]
        #[inline(always)]
        pub const fn rxq_wegt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Receive Queue Weight."]
        #[inline(always)]
        pub const fn set_rxq_wegt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Receive Queue Packet Arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn rxq_frm_arbit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Packet Arbitration."]
        #[inline(always)]
        pub const fn set_rxq_frm_arbit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for MtlRxQcr {
        #[inline(always)]
        fn default() -> MtlRxQcr {
            MtlRxQcr(0)
        }
    }
    impl core::fmt::Debug for MtlRxQcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlRxQcr")
                .field("rxq_wegt", &self.rxq_wegt())
                .field("rxq_frm_arbit", &self.rxq_frm_arbit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlRxQcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlRxQcr {{ rxq_wegt: {=u8:?}, rxq_frm_arbit: {=bool:?} }}",
                self.rxq_wegt(),
                self.rxq_frm_arbit()
            )
        }
    }
    #[doc = "Rx queue debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlRxQdr(pub u32);
    impl MtlRxQdr {
        #[doc = "MTL Rx Queue Write Controller Active Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rwcsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Rx Queue Write Controller Active Status."]
        #[inline(always)]
        pub const fn set_rwcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MTL Rx Queue Read Controller State."]
        #[must_use]
        #[inline(always)]
        pub const fn rrcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Rx Queue Read Controller State."]
        #[inline(always)]
        pub const fn set_rrcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MTL Rx Queue Fill-Level Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxqsts(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Rx Queue Fill-Level Status."]
        #[inline(always)]
        pub const fn set_rxqsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of Packets in Receive Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn prxq(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "Number of Packets in Receive Queue."]
        #[inline(always)]
        pub const fn set_prxq(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for MtlRxQdr {
        #[inline(always)]
        fn default() -> MtlRxQdr {
            MtlRxQdr(0)
        }
    }
    impl core::fmt::Debug for MtlRxQdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlRxQdr")
                .field("rwcsts", &self.rwcsts())
                .field("rrcsts", &self.rrcsts())
                .field("rxqsts", &self.rxqsts())
                .field("prxq", &self.prxq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlRxQdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlRxQdr {{ rwcsts: {=bool:?}, rrcsts: {=u8:?}, rxqsts: {=u8:?}, prxq: {=u16:?} }}",
                self.rwcsts(),
                self.rrcsts(),
                self.rxqsts(),
                self.prxq()
            )
        }
    }
    #[doc = "Rx queue missed packet and overflow counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlRxQmpocr(pub u32);
    impl MtlRxQmpocr {
        #[doc = "Overflow Packet Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfpktcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Overflow Packet Counter."]
        #[inline(always)]
        pub const fn set_ovfpktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow Counter Overflow Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ovfcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Counter Overflow Bit."]
        #[inline(always)]
        pub const fn set_ovfcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Missed Packet Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn mispktcnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Missed Packet Counter."]
        #[inline(always)]
        pub const fn set_mispktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
        #[doc = "Missed Packet Counter Overflow Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn miscntovf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Missed Packet Counter Overflow Bit."]
        #[inline(always)]
        pub const fn set_miscntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MtlRxQmpocr {
        #[inline(always)]
        fn default() -> MtlRxQmpocr {
            MtlRxQmpocr(0)
        }
    }
    impl core::fmt::Debug for MtlRxQmpocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlRxQmpocr")
                .field("ovfpktcnt", &self.ovfpktcnt())
                .field("ovfcntovf", &self.ovfcntovf())
                .field("mispktcnt", &self.mispktcnt())
                .field("miscntovf", &self.miscntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlRxQmpocr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlRxQmpocr {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}",
                self.ovfpktcnt(),
                self.ovfcntovf(),
                self.mispktcnt(),
                self.miscntovf()
            )
        }
    }
    #[doc = "Rx queue operating mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlRxQomr(pub u32);
    impl MtlRxQomr {
        #[doc = "Receive Queue Threshold Control."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Queue Threshold Control."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Forward Undersized Good Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn fup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Undersized Good Packets."]
        #[inline(always)]
        pub const fn set_fup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Forward Error Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn fep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Error Packets."]
        #[inline(always)]
        pub const fn set_fep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receive Queue Store and Forward."]
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Store and Forward."]
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_tcp_ef(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
        #[inline(always)]
        pub const fn set_dis_tcp_ef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Hardware Flow Control."]
        #[must_use]
        #[inline(always)]
        pub const fn ehfc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Hardware Flow Control."]
        #[inline(always)]
        pub const fn set_ehfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Threshold for Activating Flow Control (in Half-duplex and Full-duplex)."]
        #[must_use]
        #[inline(always)]
        pub const fn rfa(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Threshold for Activating Flow Control (in Half-duplex and Full-duplex)."]
        #[inline(always)]
        pub const fn set_rfa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Threshold for Deactivating Flow Control (in Half-duplex and Full-duplex modes)."]
        #[must_use]
        #[inline(always)]
        pub const fn rfd(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Threshold for Deactivating Flow Control (in Half-duplex and Full-duplex modes)."]
        #[inline(always)]
        pub const fn set_rfd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Receive Queue Size."]
        #[must_use]
        #[inline(always)]
        pub const fn rqs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Receive Queue Size."]
        #[inline(always)]
        pub const fn set_rqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for MtlRxQomr {
        #[inline(always)]
        fn default() -> MtlRxQomr {
            MtlRxQomr(0)
        }
    }
    impl core::fmt::Debug for MtlRxQomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlRxQomr")
                .field("rtc", &self.rtc())
                .field("fup", &self.fup())
                .field("fep", &self.fep())
                .field("rsf", &self.rsf())
                .field("dis_tcp_ef", &self.dis_tcp_ef())
                .field("ehfc", &self.ehfc())
                .field("rfa", &self.rfa())
                .field("rfd", &self.rfd())
                .field("rqs", &self.rqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlRxQomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlRxQomr {{ rtc: {=u8:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, dis_tcp_ef: {=bool:?}, ehfc: {=bool:?}, rfa: {=u8:?}, rfd: {=u8:?}, rqs: {=u8:?} }}",
                self.rtc(),
                self.fup(),
                self.fep(),
                self.rsf(),
                self.dis_tcp_ef(),
                self.ehfc(),
                self.rfa(),
                self.rfd(),
                self.rqs()
            )
        }
    }
    #[doc = "Tx queue 1 ETS control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQ1ecr(pub u32);
    impl MtlTxQ1ecr {
        #[doc = "AV Algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn avalg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AV Algorithm."]
        #[inline(always)]
        pub const fn set_avalg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Credit Control."]
        #[must_use]
        #[inline(always)]
        pub const fn cc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Credit Control."]
        #[inline(always)]
        pub const fn set_cc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Slot Count."]
        #[must_use]
        #[inline(always)]
        pub const fn slc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Slot Count."]
        #[inline(always)]
        pub const fn set_slc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for MtlTxQ1ecr {
        #[inline(always)]
        fn default() -> MtlTxQ1ecr {
            MtlTxQ1ecr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQ1ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQ1ecr")
                .field("avalg", &self.avalg())
                .field("cc", &self.cc())
                .field("slc", &self.slc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQ1ecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlTxQ1ecr {{ avalg: {=bool:?}, cc: {=bool:?}, slc: {=u8:?} }}",
                self.avalg(),
                self.cc(),
                self.slc()
            )
        }
    }
    #[doc = "Tx Queue 1 hiCredit register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQ1hcr(pub u32);
    impl MtlTxQ1hcr {
        #[doc = "hiCredit Value."]
        #[must_use]
        #[inline(always)]
        pub const fn hc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "hiCredit Value."]
        #[inline(always)]
        pub const fn set_hc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for MtlTxQ1hcr {
        #[inline(always)]
        fn default() -> MtlTxQ1hcr {
            MtlTxQ1hcr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQ1hcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQ1hcr").field("hc", &self.hc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQ1hcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MtlTxQ1hcr {{ hc: {=u32:?} }}", self.hc())
        }
    }
    #[doc = "Tx queue 1 loCredit register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQ1lcr(pub u32);
    impl MtlTxQ1lcr {
        #[doc = "loCredit Value."]
        #[must_use]
        #[inline(always)]
        pub const fn lc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "loCredit Value."]
        #[inline(always)]
        pub const fn set_lc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for MtlTxQ1lcr {
        #[inline(always)]
        fn default() -> MtlTxQ1lcr {
            MtlTxQ1lcr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQ1lcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQ1lcr").field("lc", &self.lc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQ1lcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MtlTxQ1lcr {{ lc: {=u32:?} }}", self.lc())
        }
    }
    #[doc = "Tx queue 1 send slope credit Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQ1sscr(pub u32);
    impl MtlTxQ1sscr {
        #[doc = "sendSlopeCredit Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ssc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "sendSlopeCredit Value."]
        #[inline(always)]
        pub const fn set_ssc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for MtlTxQ1sscr {
        #[inline(always)]
        fn default() -> MtlTxQ1sscr {
            MtlTxQ1sscr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQ1sscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQ1sscr").field("ssc", &self.ssc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQ1sscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MtlTxQ1sscr {{ ssc: {=u16:?} }}", self.ssc())
        }
    }
    #[doc = "Tx queue debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQdr(pub u32);
    impl MtlTxQdr {
        #[doc = "Transmit Queue in Pause."]
        #[must_use]
        #[inline(always)]
        pub const fn txqpaused(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue in Pause."]
        #[inline(always)]
        pub const fn set_txqpaused(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MTL Tx Queue Read Controller Status."]
        #[must_use]
        #[inline(always)]
        pub const fn trcsts(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "MTL Tx Queue Read Controller Status."]
        #[inline(always)]
        pub const fn set_trcsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "MTL Tx Queue Write Controller Status."]
        #[must_use]
        #[inline(always)]
        pub const fn twcsts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Queue Write Controller Status."]
        #[inline(always)]
        pub const fn set_twcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MTL Tx Queue Not Empty Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txqsts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Queue Not Empty Status."]
        #[inline(always)]
        pub const fn set_txqsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MTL Tx Status FIFO Full Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txstsfsts(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MTL Tx Status FIFO Full Status."]
        #[inline(always)]
        pub const fn set_txstsfsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Number of Packets in the Transmit Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn ptxq(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Packets in the Transmit Queue."]
        #[inline(always)]
        pub const fn set_ptxq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn stxstsf(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
        #[inline(always)]
        pub const fn set_stxstsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for MtlTxQdr {
        #[inline(always)]
        fn default() -> MtlTxQdr {
            MtlTxQdr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQdr")
                .field("txqpaused", &self.txqpaused())
                .field("trcsts", &self.trcsts())
                .field("twcsts", &self.twcsts())
                .field("txqsts", &self.txqsts())
                .field("txstsfsts", &self.txstsfsts())
                .field("ptxq", &self.ptxq())
                .field("stxstsf", &self.stxstsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlTxQdr {{ txqpaused: {=bool:?}, trcsts: {=u8:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}",
                self.txqpaused(),
                self.trcsts(),
                self.twcsts(),
                self.txqsts(),
                self.txstsfsts(),
                self.ptxq(),
                self.stxstsf()
            )
        }
    }
    #[doc = "Tx queue ETS status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQesr(pub u32);
    impl MtlTxQesr {
        #[doc = "Average Bits per Slot."]
        #[must_use]
        #[inline(always)]
        pub const fn abs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Average Bits per Slot."]
        #[inline(always)]
        pub const fn set_abs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MtlTxQesr {
        #[inline(always)]
        fn default() -> MtlTxQesr {
            MtlTxQesr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQesr").field("abs", &self.abs()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MtlTxQesr {{ abs: {=u32:?} }}", self.abs())
        }
    }
    #[doc = "Tx queue operating mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQomr(pub u32);
    impl MtlTxQomr {
        #[doc = "Flush Transmit Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn ftq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flush Transmit Queue."]
        #[inline(always)]
        pub const fn set_ftq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Store and Forward."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Store and Forward."]
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Queue Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txqen(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Transmit Queue Enable."]
        #[inline(always)]
        pub const fn set_txqen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Transmit Threshold Control."]
        #[must_use]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Threshold Control."]
        #[inline(always)]
        pub const fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Transmit queue size."]
        #[must_use]
        #[inline(always)]
        pub const fn tqs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit queue size."]
        #[inline(always)]
        pub const fn set_tqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for MtlTxQomr {
        #[inline(always)]
        fn default() -> MtlTxQomr {
            MtlTxQomr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQomr")
                .field("ftq", &self.ftq())
                .field("tsf", &self.tsf())
                .field("txqen", &self.txqen())
                .field("ttc", &self.ttc())
                .field("tqs", &self.tqs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlTxQomr {{ ftq: {=bool:?}, tsf: {=bool:?}, txqen: {=u8:?}, ttc: {=u8:?}, tqs: {=u8:?} }}",
                self.ftq(),
                self.tsf(),
                self.txqen(),
                self.ttc(),
                self.tqs()
            )
        }
    }
    #[doc = "Tx queue quantum weight register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQqwr(pub u32);
    impl MtlTxQqwr {
        #[doc = "IdleSlopeCredit or Weights."]
        #[must_use]
        #[inline(always)]
        pub const fn iscqw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "IdleSlopeCredit or Weights."]
        #[inline(always)]
        pub const fn set_iscqw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for MtlTxQqwr {
        #[inline(always)]
        fn default() -> MtlTxQqwr {
            MtlTxQqwr(0)
        }
    }
    impl core::fmt::Debug for MtlTxQqwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQqwr").field("iscqw", &self.iscqw()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQqwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MtlTxQqwr {{ iscqw: {=u16:?} }}", self.iscqw())
        }
    }
    #[doc = "Tx queue underflow register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MtlTxQur(pub u32);
    impl MtlTxQur {
        #[doc = "Underflow Packet Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn uffrmcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Underflow Packet Counter."]
        #[inline(always)]
        pub const fn set_uffrmcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Overflow Bit for Underflow Packet Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ufcntovf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Bit for Underflow Packet Counter."]
        #[inline(always)]
        pub const fn set_ufcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for MtlTxQur {
        #[inline(always)]
        fn default() -> MtlTxQur {
            MtlTxQur(0)
        }
    }
    impl core::fmt::Debug for MtlTxQur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MtlTxQur")
                .field("uffrmcnt", &self.uffrmcnt())
                .field("ufcntovf", &self.ufcntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MtlTxQur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MtlTxQur {{ uffrmcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
                self.uffrmcnt(),
                self.ufcntovf()
            )
        }
    }
    #[doc = "EST Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestcr(pub u32);
    impl Mtlestcr {
        #[doc = "Enable EST."]
        #[must_use]
        #[inline(always)]
        pub const fn eest(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable EST."]
        #[inline(always)]
        pub const fn set_eest(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Switch to S/W owned list."]
        #[must_use]
        #[inline(always)]
        pub const fn sswl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Switch to S/W owned list."]
        #[inline(always)]
        pub const fn set_sswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Do not Drop frames during Frame Size Error."]
        #[must_use]
        #[inline(always)]
        pub const fn ddbf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Do not Drop frames during Frame Size Error."]
        #[inline(always)]
        pub const fn set_ddbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Drop Frames causing Scheduling Error."]
        #[must_use]
        #[inline(always)]
        pub const fn dfbs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Frames causing Scheduling Error."]
        #[inline(always)]
        pub const fn set_dfbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Count to report Scheduling Error."]
        #[must_use]
        #[inline(always)]
        pub const fn lcse(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Count to report Scheduling Error."]
        #[inline(always)]
        pub const fn set_lcse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Time Interval Left Shift Amount."]
        #[must_use]
        #[inline(always)]
        pub const fn tils(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Time Interval Left Shift Amount."]
        #[inline(always)]
        pub const fn set_tils(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Current Time Offset Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ctov(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current Time Offset Value."]
        #[inline(always)]
        pub const fn set_ctov(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
        }
        #[doc = "PTP Time Offset Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ptov(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "PTP Time Offset Value."]
        #[inline(always)]
        pub const fn set_ptov(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Mtlestcr {
        #[inline(always)]
        fn default() -> Mtlestcr {
            Mtlestcr(0)
        }
    }
    impl core::fmt::Debug for Mtlestcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestcr")
                .field("eest", &self.eest())
                .field("sswl", &self.sswl())
                .field("ddbf", &self.ddbf())
                .field("dfbs", &self.dfbs())
                .field("lcse", &self.lcse())
                .field("tils", &self.tils())
                .field("ctov", &self.ctov())
                .field("ptov", &self.ptov())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlestcr {{ eest: {=bool:?}, sswl: {=bool:?}, ddbf: {=bool:?}, dfbs: {=bool:?}, lcse: {=u8:?}, tils: {=u8:?}, ctov: {=u16:?}, ptov: {=u8:?} }}",
                self.eest(),
                self.sswl(),
                self.ddbf(),
                self.dfbs(),
                self.lcse(),
                self.tils(),
                self.ctov(),
                self.ptov()
            )
        }
    }
    #[doc = "EST Extended Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestecr(pub u32);
    impl Mtlestecr {
        #[doc = "Overhead Bytes Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ovhd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Overhead Bytes Value."]
        #[inline(always)]
        pub const fn set_ovhd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Mtlestecr {
        #[inline(always)]
        fn default() -> Mtlestecr {
            Mtlestecr(0)
        }
    }
    impl core::fmt::Debug for Mtlestecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestecr").field("ovhd", &self.ovhd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mtlestecr {{ ovhd: {=u8:?} }}", self.ovhd())
        }
    }
    #[doc = "EST Frame size Capture Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestfscr(pub u32);
    impl Mtlestfscr {
        #[doc = "Frame Size of HLBF."]
        #[must_use]
        #[inline(always)]
        pub const fn hbfs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Frame Size of HLBF."]
        #[inline(always)]
        pub const fn set_hbfs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Queue Number of HLBF."]
        #[must_use]
        #[inline(always)]
        pub const fn hbfq(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Queue Number of HLBF."]
        #[inline(always)]
        pub const fn set_hbfq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mtlestfscr {
        #[inline(always)]
        fn default() -> Mtlestfscr {
            Mtlestfscr(0)
        }
    }
    impl core::fmt::Debug for Mtlestfscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestfscr")
                .field("hbfs", &self.hbfs())
                .field("hbfq", &self.hbfq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestfscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlestfscr {{ hbfs: {=u16:?}, hbfq: {=bool:?} }}",
                self.hbfs(),
                self.hbfq()
            )
        }
    }
    #[doc = "EST Frame size Error Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestfser(pub u32);
    impl Mtlestfser {
        #[doc = "Frame Size Error Queue Number."]
        #[must_use]
        #[inline(always)]
        pub const fn feqn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame Size Error Queue Number."]
        #[inline(always)]
        pub const fn set_feqn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Mtlestfser {
        #[inline(always)]
        fn default() -> Mtlestfser {
            Mtlestfser(0)
        }
    }
    impl core::fmt::Debug for Mtlestfser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestfser").field("feqn", &self.feqn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestfser {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mtlestfser {{ feqn: {=u8:?} }}", self.feqn())
        }
    }
    #[doc = "EST Gate Control List Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestgclcr(pub u32);
    impl Mtlestgclcr {
        #[doc = "Start Read/Write Operation."]
        #[must_use]
        #[inline(always)]
        pub const fn srwo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start Read/Write Operation."]
        #[inline(always)]
        pub const fn set_srwo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read 1, Write 0."]
        #[must_use]
        #[inline(always)]
        pub const fn r1w0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read 1, Write 0."]
        #[inline(always)]
        pub const fn set_r1w0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Gate Control Related Registers."]
        #[must_use]
        #[inline(always)]
        pub const fn gcrr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Gate Control Related Registers."]
        #[inline(always)]
        pub const fn set_gcrr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Debug Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Mode."]
        #[inline(always)]
        pub const fn set_dbgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Debug Mode Bank Select."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgb(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Mode Bank Select."]
        #[inline(always)]
        pub const fn set_dbgb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Gate Control List Address:."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Gate Control List Address:."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for Mtlestgclcr {
        #[inline(always)]
        fn default() -> Mtlestgclcr {
            Mtlestgclcr(0)
        }
    }
    impl core::fmt::Debug for Mtlestgclcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestgclcr")
                .field("srwo", &self.srwo())
                .field("r1w0", &self.r1w0())
                .field("gcrr", &self.gcrr())
                .field("dbgm", &self.dbgm())
                .field("dbgb", &self.dbgb())
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestgclcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlestgclcr {{ srwo: {=bool:?}, r1w0: {=bool:?}, gcrr: {=bool:?}, dbgm: {=bool:?}, dbgb: {=bool:?}, addr: {=u8:?} }}",
                self.srwo(),
                self.r1w0(),
                self.gcrr(),
                self.dbgm(),
                self.dbgb(),
                self.addr()
            )
        }
    }
    #[doc = "EST Gate Control List Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestgcldr(pub u32);
    impl Mtlestgcldr {
        #[doc = "Gate Control Data."]
        #[must_use]
        #[inline(always)]
        pub const fn gcd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Gate Control Data."]
        #[inline(always)]
        pub const fn set_gcd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mtlestgcldr {
        #[inline(always)]
        fn default() -> Mtlestgcldr {
            Mtlestgcldr(0)
        }
    }
    impl core::fmt::Debug for Mtlestgcldr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestgcldr").field("gcd", &self.gcd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestgcldr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mtlestgcldr {{ gcd: {=u32:?} }}", self.gcd())
        }
    }
    #[doc = "EST Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestier(pub u32);
    impl Mtlestier {
        #[doc = "Interrupt Enable for Switch List."]
        #[must_use]
        #[inline(always)]
        pub const fn iecc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for Switch List."]
        #[inline(always)]
        pub const fn set_iecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Enable for BTR Error."]
        #[must_use]
        #[inline(always)]
        pub const fn iebe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for BTR Error."]
        #[inline(always)]
        pub const fn set_iebe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt Enable for HLBF."]
        #[must_use]
        #[inline(always)]
        pub const fn iehf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for HLBF."]
        #[inline(always)]
        pub const fn set_iehf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt Enable for HLBS."]
        #[must_use]
        #[inline(always)]
        pub const fn iehs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for HLBS."]
        #[inline(always)]
        pub const fn set_iehs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt Enable for CGCE."]
        #[must_use]
        #[inline(always)]
        pub const fn cgce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for CGCE."]
        #[inline(always)]
        pub const fn set_cgce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Mtlestier {
        #[inline(always)]
        fn default() -> Mtlestier {
            Mtlestier(0)
        }
    }
    impl core::fmt::Debug for Mtlestier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestier")
                .field("iecc", &self.iecc())
                .field("iebe", &self.iebe())
                .field("iehf", &self.iehf())
                .field("iehs", &self.iehs())
                .field("cgce", &self.cgce())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlestier {{ iecc: {=bool:?}, iebe: {=bool:?}, iehf: {=bool:?}, iehs: {=bool:?}, cgce: {=bool:?} }}",
                self.iecc(),
                self.iebe(),
                self.iehf(),
                self.iehs(),
                self.cgce()
            )
        }
    }
    #[doc = "EST Schedule Error Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestscher(pub u32);
    impl Mtlestscher {
        #[doc = "Schedule Error Queue Number."]
        #[must_use]
        #[inline(always)]
        pub const fn seqn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Schedule Error Queue Number."]
        #[inline(always)]
        pub const fn set_seqn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Mtlestscher {
        #[inline(always)]
        fn default() -> Mtlestscher {
            Mtlestscher(0)
        }
    }
    impl core::fmt::Debug for Mtlestscher {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestscher").field("seqn", &self.seqn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestscher {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mtlestscher {{ seqn: {=u8:?} }}", self.seqn())
        }
    }
    #[doc = "EST Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlestsr(pub u32);
    impl Mtlestsr {
        #[doc = "Switch to S/W owned list Complete."]
        #[must_use]
        #[inline(always)]
        pub const fn swlc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Switch to S/W owned list Complete."]
        #[inline(always)]
        pub const fn set_swlc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BTR Error."]
        #[must_use]
        #[inline(always)]
        pub const fn btre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BTR Error."]
        #[inline(always)]
        pub const fn set_btre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Head-Of-Line Blocking due to Frame Size."]
        #[must_use]
        #[inline(always)]
        pub const fn hlbf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Head-Of-Line Blocking due to Frame Size."]
        #[inline(always)]
        pub const fn set_hlbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Head-Of-Line Blocking due to Scheduling."]
        #[must_use]
        #[inline(always)]
        pub const fn hlbs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Head-Of-Line Blocking due to Scheduling."]
        #[inline(always)]
        pub const fn set_hlbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Constant Gate Control Error."]
        #[must_use]
        #[inline(always)]
        pub const fn cgce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Constant Gate Control Error."]
        #[inline(always)]
        pub const fn set_cgce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "S/W owned list."]
        #[must_use]
        #[inline(always)]
        pub const fn swol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "S/W owned list."]
        #[inline(always)]
        pub const fn set_swol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BTR Error Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn btrl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "BTR Error Loop Count."]
        #[inline(always)]
        pub const fn set_btrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Current GCL slot number."]
        #[must_use]
        #[inline(always)]
        pub const fn cgsn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Current GCL slot number."]
        #[inline(always)]
        pub const fn set_cgsn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Mtlestsr {
        #[inline(always)]
        fn default() -> Mtlestsr {
            Mtlestsr(0)
        }
    }
    impl core::fmt::Debug for Mtlestsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlestsr")
                .field("swlc", &self.swlc())
                .field("btre", &self.btre())
                .field("hlbf", &self.hlbf())
                .field("hlbs", &self.hlbs())
                .field("cgce", &self.cgce())
                .field("swol", &self.swol())
                .field("btrl", &self.btrl())
                .field("cgsn", &self.cgsn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlestsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlestsr {{ swlc: {=bool:?}, btre: {=bool:?}, hlbf: {=bool:?}, hlbs: {=bool:?}, cgce: {=bool:?}, swol: {=bool:?}, btrl: {=u8:?}, cgsn: {=u8:?} }}",
                self.swlc(),
                self.btre(),
                self.hlbf(),
                self.hlbs(),
                self.cgce(),
                self.swol(),
                self.btrl(),
                self.cgsn()
            )
        }
    }
    #[doc = "FPE Frame Preemption Advance Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlfpear(pub u32);
    impl Mtlfpear {
        #[doc = "Hold Advance."]
        #[must_use]
        #[inline(always)]
        pub const fn hadv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Hold Advance."]
        #[inline(always)]
        pub const fn set_hadv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Release Advance."]
        #[must_use]
        #[inline(always)]
        pub const fn radv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Release Advance."]
        #[inline(always)]
        pub const fn set_radv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mtlfpear {
        #[inline(always)]
        fn default() -> Mtlfpear {
            Mtlfpear(0)
        }
    }
    impl core::fmt::Debug for Mtlfpear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlfpear")
                .field("hadv", &self.hadv())
                .field("radv", &self.radv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlfpear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlfpear {{ hadv: {=u16:?}, radv: {=u16:?} }}",
                self.hadv(),
                self.radv()
            )
        }
    }
    #[doc = "FPE Frame Preemption Control Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlfpecsr(pub u32);
    impl Mtlfpecsr {
        #[doc = "Additional Fragment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn afsz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Additional Fragment Size."]
        #[inline(always)]
        pub const fn set_afsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Preemption Classification."]
        #[must_use]
        #[inline(always)]
        pub const fn pec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Preemption Classification."]
        #[inline(always)]
        pub const fn set_pec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Hold/Release Status."]
        #[must_use]
        #[inline(always)]
        pub const fn hrs(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Hold/Release Status."]
        #[inline(always)]
        pub const fn set_hrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Mtlfpecsr {
        #[inline(always)]
        fn default() -> Mtlfpecsr {
            Mtlfpecsr(0)
        }
    }
    impl core::fmt::Debug for Mtlfpecsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlfpecsr")
                .field("afsz", &self.afsz())
                .field("pec", &self.pec())
                .field("hrs", &self.hrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlfpecsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlfpecsr {{ afsz: {=u8:?}, pec: {=u8:?}, hrs: {=bool:?} }}",
                self.afsz(),
                self.pec(),
                self.hrs()
            )
        }
    }
    #[doc = "Interrupt status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlisr(pub u32);
    impl Mtlisr {
        #[doc = "Queue interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn qis(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Queue interrupt status."]
        #[inline(always)]
        pub const fn set_qis(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "EST (TAS- 802.1Qbv) Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn estis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EST (TAS- 802.1Qbv) Interrupt Status."]
        #[inline(always)]
        pub const fn set_estis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Mtlisr {
        #[inline(always)]
        fn default() -> Mtlisr {
            Mtlisr(0)
        }
    }
    impl core::fmt::Debug for Mtlisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlisr")
                .field("qis[0]", &self.qis(0usize))
                .field("qis[1]", &self.qis(1usize))
                .field("estis", &self.estis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlisr {{ qis[0]: {=bool:?}, qis[1]: {=bool:?}, estis: {=bool:?} }}",
                self.qis(0usize),
                self.qis(1usize),
                self.estis()
            )
        }
    }
    #[doc = "Operating mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlomr(pub u32);
    impl Mtlomr {
        #[doc = "Drop Transmit Status."]
        #[must_use]
        #[inline(always)]
        pub const fn dtxsts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Transmit Status."]
        #[inline(always)]
        pub const fn set_dtxsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive Arbitration Algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn raa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Arbitration Algorithm."]
        #[inline(always)]
        pub const fn set_raa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tx Scheduling Algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn schalg(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Tx Scheduling Algorithm."]
        #[inline(always)]
        pub const fn set_schalg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Counters Preset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Preset."]
        #[inline(always)]
        pub const fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Counters Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Reset."]
        #[inline(always)]
        pub const fn set_cntclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Mtlomr {
        #[inline(always)]
        fn default() -> Mtlomr {
            Mtlomr(0)
        }
    }
    impl core::fmt::Debug for Mtlomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlomr")
                .field("dtxsts", &self.dtxsts())
                .field("raa", &self.raa())
                .field("schalg", &self.schalg())
                .field("cntprst", &self.cntprst())
                .field("cntclr", &self.cntclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlomr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlomr {{ dtxsts: {=bool:?}, raa: {=bool:?}, schalg: {=u8:?}, cntprst: {=bool:?}, cntclr: {=bool:?} }}",
                self.dtxsts(),
                self.raa(),
                self.schalg(),
                self.cntprst(),
                self.cntclr()
            )
        }
    }
    #[doc = "Queue interrupt control status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlqicsr(pub u32);
    impl Mtlqicsr {
        #[doc = "Transmit Queue Underflow Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txunfis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue Underflow Interrupt Status."]
        #[inline(always)]
        pub const fn set_txunfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Average Bits Per Slot Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn abpsis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Average Bits Per Slot Interrupt Status."]
        #[inline(always)]
        pub const fn set_abpsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Queue Underflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txuie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Queue Underflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_txuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Average Bits Per Slot Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn abpsie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Average Bits Per Slot Interrupt Enable."]
        #[inline(always)]
        pub const fn set_abpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receive Queue Overflow Interrupt Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxovfis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Overflow Interrupt Status."]
        #[inline(always)]
        pub const fn set_rxovfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive Queue Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Queue Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Mtlqicsr {
        #[inline(always)]
        fn default() -> Mtlqicsr {
            Mtlqicsr(0)
        }
    }
    impl core::fmt::Debug for Mtlqicsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlqicsr")
                .field("txunfis", &self.txunfis())
                .field("abpsis", &self.abpsis())
                .field("txuie", &self.txuie())
                .field("abpsie", &self.abpsie())
                .field("rxovfis", &self.rxovfis())
                .field("rxoie", &self.rxoie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlqicsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlqicsr {{ txunfis: {=bool:?}, abpsis: {=bool:?}, txuie: {=bool:?}, abpsie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
                self.txunfis(),
                self.abpsis(),
                self.txuie(),
                self.abpsie(),
                self.rxovfis(),
                self.rxoie()
            )
        }
    }
    #[doc = "Rx Queue and DMA Channel Mapping Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtlrxqdmamr(pub u32);
    impl Mtlrxqdmamr {
        #[doc = "Queue 0 Mapped to DMA Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn q0mdmach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Queue 0 Mapped to DMA Channel."]
        #[inline(always)]
        pub const fn set_q0mdmach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn q0ddmach(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection."]
        #[inline(always)]
        pub const fn set_q0ddmach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Queue 1 Mapped to DMA Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn q1mdmach(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Queue 1 Mapped to DMA Channel."]
        #[inline(always)]
        pub const fn set_q1mdmach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn q1ddmach(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection."]
        #[inline(always)]
        pub const fn set_q1ddmach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mtlrxqdmamr {
        #[inline(always)]
        fn default() -> Mtlrxqdmamr {
            Mtlrxqdmamr(0)
        }
    }
    impl core::fmt::Debug for Mtlrxqdmamr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtlrxqdmamr")
                .field("q0mdmach", &self.q0mdmach())
                .field("q0ddmach", &self.q0ddmach())
                .field("q1mdmach", &self.q1mdmach())
                .field("q1ddmach", &self.q1ddmach())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtlrxqdmamr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtlrxqdmamr {{ q0mdmach: {=bool:?}, q0ddmach: {=bool:?}, q1mdmach: {=bool:?}, q1ddmach: {=bool:?} }}",
                self.q0mdmach(),
                self.q0ddmach(),
                self.q1mdmach(),
                self.q1ddmach()
            )
        }
    }
    #[doc = "TBS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtltbscr(pub u32);
    impl Mtltbscr {
        #[doc = "EST offset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn estm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EST offset mode."]
        #[inline(always)]
        pub const fn set_estm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Launch expiry offset valid."]
        #[must_use]
        #[inline(always)]
        pub const fn leov(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Launch expiry offset valid."]
        #[inline(always)]
        pub const fn set_leov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Launch Expiry GSN Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn legos(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Launch Expiry GSN Offset."]
        #[inline(always)]
        pub const fn set_legos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Launch Expiry Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn leos(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Launch Expiry Offset."]
        #[inline(always)]
        pub const fn set_leos(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Mtltbscr {
        #[inline(always)]
        fn default() -> Mtltbscr {
            Mtltbscr(0)
        }
    }
    impl core::fmt::Debug for Mtltbscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mtltbscr")
                .field("estm", &self.estm())
                .field("leov", &self.leov())
                .field("legos", &self.legos())
                .field("leos", &self.leos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mtltbscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mtltbscr {{ estm: {=bool:?}, leov: {=bool:?}, legos: {=u8:?}, leos: {=u32:?} }}",
                self.estm(),
                self.leov(),
                self.legos(),
                self.leos()
            )
        }
    }
    #[doc = "Rx alignment error packets register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAlignmentErrorPackets(pub u32);
    impl RxAlignmentErrorPackets {
        #[doc = "Rx Alignment Error Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Alignment Error Packets."]
        #[inline(always)]
        pub const fn set_rxalgnerr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxAlignmentErrorPackets {
        #[inline(always)]
        fn default() -> RxAlignmentErrorPackets {
            RxAlignmentErrorPackets(0)
        }
    }
    impl core::fmt::Debug for RxAlignmentErrorPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxAlignmentErrorPackets")
                .field("rxalgnerr", &self.rxalgnerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxAlignmentErrorPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxAlignmentErrorPackets {{ rxalgnerr: {=u32:?} }}", self.rxalgnerr())
        }
    }
    #[doc = "Rx CRC error packets register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxCrcErrorPackets(pub u32);
    impl RxCrcErrorPackets {
        #[doc = "Rx CRC Error Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx CRC Error Packets."]
        #[inline(always)]
        pub const fn set_rxcrcerr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxCrcErrorPackets {
        #[inline(always)]
        fn default() -> RxCrcErrorPackets {
            RxCrcErrorPackets(0)
        }
    }
    impl core::fmt::Debug for RxCrcErrorPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxCrcErrorPackets")
                .field("rxcrcerr", &self.rxcrcerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxCrcErrorPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxCrcErrorPackets {{ rxcrcerr: {=u32:?} }}", self.rxcrcerr())
        }
    }
    #[doc = "MMC Rx FPE fragments counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxFpeFragCr(pub u32);
    impl RxFpeFragCr {
        #[doc = "Rx FPE Fragment Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ffc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx FPE Fragment Counter."]
        #[inline(always)]
        pub const fn set_ffc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxFpeFragCr {
        #[inline(always)]
        fn default() -> RxFpeFragCr {
            RxFpeFragCr(0)
        }
    }
    impl core::fmt::Debug for RxFpeFragCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxFpeFragCr").field("ffc", &self.ffc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxFpeFragCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxFpeFragCr {{ ffc: {=u32:?} }}", self.ffc())
        }
    }
    #[doc = "Rx LPI transition counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLpiTranCntr(pub u32);
    impl RxLpiTranCntr {
        #[doc = "Rx LPI Transition counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpitrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx LPI Transition counter."]
        #[inline(always)]
        pub const fn set_rxlpitrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxLpiTranCntr {
        #[inline(always)]
        fn default() -> RxLpiTranCntr {
            RxLpiTranCntr(0)
        }
    }
    impl core::fmt::Debug for RxLpiTranCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLpiTranCntr")
                .field("rxlpitrc", &self.rxlpitrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxLpiTranCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxLpiTranCntr {{ rxlpitrc: {=u32:?} }}", self.rxlpitrc())
        }
    }
    #[doc = "Rx LPI microsecond counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLpiUsecCntr(pub u32);
    impl RxLpiUsecCntr {
        #[doc = "Rx LPI Microseconds Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlpiusc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx LPI Microseconds Counter."]
        #[inline(always)]
        pub const fn set_rxlpiusc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxLpiUsecCntr {
        #[inline(always)]
        fn default() -> RxLpiUsecCntr {
            RxLpiUsecCntr(0)
        }
    }
    impl core::fmt::Debug for RxLpiUsecCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLpiUsecCntr")
                .field("rxlpiusc", &self.rxlpiusc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxLpiUsecCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxLpiUsecCntr {{ rxlpiusc: {=u32:?} }}", self.rxlpiusc())
        }
    }
    #[doc = "MMC Rx packet assembly error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxPacketAsmErr(pub u32);
    impl RxPacketAsmErr {
        #[doc = "Rx Packet Assembly Error Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn paec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Packet Assembly Error Counter."]
        #[inline(always)]
        pub const fn set_paec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxPacketAsmErr {
        #[inline(always)]
        fn default() -> RxPacketAsmErr {
            RxPacketAsmErr(0)
        }
    }
    impl core::fmt::Debug for RxPacketAsmErr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxPacketAsmErr").field("paec", &self.paec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxPacketAsmErr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxPacketAsmErr {{ paec: {=u32:?} }}", self.paec())
        }
    }
    #[doc = "MMC Rx packet assembly OK register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxPacketAsmOkr(pub u32);
    impl RxPacketAsmOkr {
        #[doc = "Rx Packet Assembly OK Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn paoc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Packet Assembly OK Counter."]
        #[inline(always)]
        pub const fn set_paoc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxPacketAsmOkr {
        #[inline(always)]
        fn default() -> RxPacketAsmOkr {
            RxPacketAsmOkr(0)
        }
    }
    impl core::fmt::Debug for RxPacketAsmOkr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxPacketAsmOkr").field("paoc", &self.paoc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxPacketAsmOkr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxPacketAsmOkr {{ paoc: {=u32:?} }}", self.paoc())
        }
    }
    #[doc = "MMC Rx packet SMD error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxPacketSmdErr(pub u32);
    impl RxPacketSmdErr {
        #[doc = "Rx Packet SMD Error Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn psec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Packet SMD Error Counter."]
        #[inline(always)]
        pub const fn set_psec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxPacketSmdErr {
        #[inline(always)]
        fn default() -> RxPacketSmdErr {
            RxPacketSmdErr(0)
        }
    }
    impl core::fmt::Debug for RxPacketSmdErr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxPacketSmdErr").field("psec", &self.psec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxPacketSmdErr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxPacketSmdErr {{ psec: {=u32:?} }}", self.psec())
        }
    }
    #[doc = "Rx unicast packets good register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxUnicastPacketsGood(pub u32);
    impl RxUnicastPacketsGood {
        #[doc = "Rx Unicast Packets Good."]
        #[must_use]
        #[inline(always)]
        pub const fn rxucastg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Rx Unicast Packets Good."]
        #[inline(always)]
        pub const fn set_rxucastg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxUnicastPacketsGood {
        #[inline(always)]
        fn default() -> RxUnicastPacketsGood {
            RxUnicastPacketsGood(0)
        }
    }
    impl core::fmt::Debug for RxUnicastPacketsGood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxUnicastPacketsGood")
                .field("rxucastg", &self.rxucastg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxUnicastPacketsGood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxUnicastPacketsGood {{ rxucastg: {=u32:?} }}", self.rxucastg())
        }
    }
    #[doc = "Tx LPI transition counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxLpiTranCntr(pub u32);
    impl TxLpiTranCntr {
        #[doc = "Tx LPI Transition counter."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpitrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx LPI Transition counter."]
        #[inline(always)]
        pub const fn set_txlpitrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxLpiTranCntr {
        #[inline(always)]
        fn default() -> TxLpiTranCntr {
            TxLpiTranCntr(0)
        }
    }
    impl core::fmt::Debug for TxLpiTranCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxLpiTranCntr")
                .field("txlpitrc", &self.txlpitrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxLpiTranCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxLpiTranCntr {{ txlpitrc: {=u32:?} }}", self.txlpitrc())
        }
    }
    #[doc = "Tx LPI microsecond timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxLpiUsecCntr(pub u32);
    impl TxLpiUsecCntr {
        #[doc = "Tx LPI Microseconds Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn txlpiusc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx LPI Microseconds Counter."]
        #[inline(always)]
        pub const fn set_txlpiusc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxLpiUsecCntr {
        #[inline(always)]
        fn default() -> TxLpiUsecCntr {
            TxLpiUsecCntr(0)
        }
    }
    impl core::fmt::Debug for TxLpiUsecCntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxLpiUsecCntr")
                .field("txlpiusc", &self.txlpiusc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxLpiUsecCntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxLpiUsecCntr {{ txlpiusc: {=u32:?} }}", self.txlpiusc())
        }
    }
    #[doc = "Tx multiple collision good packets register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxMultipleCollisionGoodPackets(pub u32);
    impl TxMultipleCollisionGoodPackets {
        #[doc = "Tx Multiple Collision Good Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn txmultcolg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Multiple Collision Good Packets."]
        #[inline(always)]
        pub const fn set_txmultcolg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxMultipleCollisionGoodPackets {
        #[inline(always)]
        fn default() -> TxMultipleCollisionGoodPackets {
            TxMultipleCollisionGoodPackets(0)
        }
    }
    impl core::fmt::Debug for TxMultipleCollisionGoodPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxMultipleCollisionGoodPackets")
                .field("txmultcolg", &self.txmultcolg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxMultipleCollisionGoodPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxMultipleCollisionGoodPackets {{ txmultcolg: {=u32:?} }}",
                self.txmultcolg()
            )
        }
    }
    #[doc = "Tx packet count good register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxPacketCountGood(pub u32);
    impl TxPacketCountGood {
        #[doc = "Tx Packet Count Good."]
        #[must_use]
        #[inline(always)]
        pub const fn txpktg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Packet Count Good."]
        #[inline(always)]
        pub const fn set_txpktg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxPacketCountGood {
        #[inline(always)]
        fn default() -> TxPacketCountGood {
            TxPacketCountGood(0)
        }
    }
    impl core::fmt::Debug for TxPacketCountGood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxPacketCountGood")
                .field("txpktg", &self.txpktg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxPacketCountGood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxPacketCountGood {{ txpktg: {=u32:?} }}", self.txpktg())
        }
    }
    #[doc = "Tx single collision good packets register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxSingleCollisionGoodPackets(pub u32);
    impl TxSingleCollisionGoodPackets {
        #[doc = "Tx Single Collision Good Packets."]
        #[must_use]
        #[inline(always)]
        pub const fn txsnglcolg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx Single Collision Good Packets."]
        #[inline(always)]
        pub const fn set_txsnglcolg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxSingleCollisionGoodPackets {
        #[inline(always)]
        fn default() -> TxSingleCollisionGoodPackets {
            TxSingleCollisionGoodPackets(0)
        }
    }
    impl core::fmt::Debug for TxSingleCollisionGoodPackets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxSingleCollisionGoodPackets")
                .field("txsnglcolg", &self.txsnglcolg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxSingleCollisionGoodPackets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxSingleCollisionGoodPackets {{ txsnglcolg: {=u32:?} }}",
                self.txsnglcolg()
            )
        }
    }
}
