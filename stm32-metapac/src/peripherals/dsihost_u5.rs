#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DSI Host."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsihost {
    ptr: *mut u8,
}
unsafe impl Send for Dsihost {}
unsafe impl Sync for Dsihost {}
impl Dsihost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DSI Host version register."]
    #[inline(always)]
    pub const fn vr(self) -> crate::common::Reg<regs::Vr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DSI Host control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DSI Host clock control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DSI Host LTDC VCID register."]
    #[inline(always)]
    pub const fn lvcidr(self) -> crate::common::Reg<regs::Lvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DSI Host LTDC color coding register."]
    #[inline(always)]
    pub const fn lcolcr(self) -> crate::common::Reg<regs::Lcolcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DSI Host LTDC polarity configuration register."]
    #[inline(always)]
    pub const fn lpcr(self) -> crate::common::Reg<regs::Lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DSI Host low-power mode configuration register."]
    #[inline(always)]
    pub const fn lpmcr(self) -> crate::common::Reg<regs::Lpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DSI Host protocol configuration register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DSI Host generic VCID register."]
    #[inline(always)]
    pub const fn gvcidr(self) -> crate::common::Reg<regs::Gvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "DSI Host mode configuration register."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "DSI Host video mode configuration register."]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "DSI Host video packet configuration register."]
    #[inline(always)]
    pub const fn vpcr(self) -> crate::common::Reg<regs::Vpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "DSI Host video chunks configuration register."]
    #[inline(always)]
    pub const fn vccr(self) -> crate::common::Reg<regs::Vccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "DSI Host video null packet configuration register."]
    #[inline(always)]
    pub const fn vnpcr(self) -> crate::common::Reg<regs::Vnpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DSI Host video HSA configuration register."]
    #[inline(always)]
    pub const fn vhsacr(self) -> crate::common::Reg<regs::Vhsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DSI Host video HBP configuration register."]
    #[inline(always)]
    pub const fn vhbpcr(self) -> crate::common::Reg<regs::Vhbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "DSI Host video line configuration register."]
    #[inline(always)]
    pub const fn vlcr(self) -> crate::common::Reg<regs::Vlcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "DSI Host video VSA configuration register."]
    #[inline(always)]
    pub const fn vvsacr(self) -> crate::common::Reg<regs::Vvsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "DSI Host video VBP configuration register."]
    #[inline(always)]
    pub const fn vvbpcr(self) -> crate::common::Reg<regs::Vvbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "DSI Host video VFP configuration register."]
    #[inline(always)]
    pub const fn vvfpcr(self) -> crate::common::Reg<regs::Vvfpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "DSI Host video VA configuration register."]
    #[inline(always)]
    pub const fn vvacr(self) -> crate::common::Reg<regs::Vvacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "DSI Host LTDC command configuration register."]
    #[inline(always)]
    pub const fn lccr(self) -> crate::common::Reg<regs::Lccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "DSI Host command mode configuration register."]
    #[inline(always)]
    pub const fn cmcr(self) -> crate::common::Reg<regs::Cmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "DSI Host generic header configuration register."]
    #[inline(always)]
    pub const fn ghcr(self) -> crate::common::Reg<regs::Ghcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "DSI Host generic payload data register."]
    #[inline(always)]
    pub const fn gpdr(self) -> crate::common::Reg<regs::Gpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "DSI Host generic packet status register."]
    #[inline(always)]
    pub const fn gpsr(self) -> crate::common::Reg<regs::Gpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 0."]
    #[inline(always)]
    pub const fn tccr0(self) -> crate::common::Reg<regs::Tccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 1."]
    #[inline(always)]
    pub const fn tccr1(self) -> crate::common::Reg<regs::Tccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 2."]
    #[inline(always)]
    pub const fn tccr2(self) -> crate::common::Reg<regs::Tccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 3."]
    #[inline(always)]
    pub const fn tccr3(self) -> crate::common::Reg<regs::Tccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 4."]
    #[inline(always)]
    pub const fn tccr4(self) -> crate::common::Reg<regs::Tccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "DSI Host timeout counter configuration register 5."]
    #[inline(always)]
    pub const fn tccr5(self) -> crate::common::Reg<regs::Tccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "DSI Host clock lane configuration register."]
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Clcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "DSI Host clock lane timer configuration register."]
    #[inline(always)]
    pub const fn cltcr(self) -> crate::common::Reg<regs::Cltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "DSI Host data lane timer configuration register."]
    #[inline(always)]
    pub const fn dltcr(self) -> crate::common::Reg<regs::Dltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "DSI Host PHY control register."]
    #[inline(always)]
    pub const fn pctlr(self) -> crate::common::Reg<regs::Pctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "DSI Host PHY configuration register."]
    #[inline(always)]
    pub const fn pconfr(self) -> crate::common::Reg<regs::Pconfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "DSI Host PHY ULPS control register."]
    #[inline(always)]
    pub const fn pucr(self) -> crate::common::Reg<regs::Pucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "DSI Host PHY TX triggers configuration register."]
    #[inline(always)]
    pub const fn pttcr(self) -> crate::common::Reg<regs::Pttcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "DSI Host PHY status register."]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "DSI Host interrupt and status register 0."]
    #[inline(always)]
    pub const fn isr0(self) -> crate::common::Reg<regs::Isr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "DSI Host interrupt and status register 1."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "DSI Host interrupt enable register 0."]
    #[inline(always)]
    pub const fn ier0(self) -> crate::common::Reg<regs::Ier0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "DSI Host interrupt enable register 1."]
    #[inline(always)]
    pub const fn ier1(self) -> crate::common::Reg<regs::Ier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "DSI Host force interrupt register 0."]
    #[inline(always)]
    pub const fn fir0(self) -> crate::common::Reg<regs::Fir0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "DSI Host force interrupt register 1."]
    #[inline(always)]
    pub const fn fir1(self) -> crate::common::Reg<regs::Fir1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "DSI Host data lane timer read configuration register."]
    #[inline(always)]
    pub const fn dltrcr(self) -> crate::common::Reg<regs::Dltrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "DSI Host video shadow control register."]
    #[inline(always)]
    pub const fn vscr(self) -> crate::common::Reg<regs::Vscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "DSI Host LTDC current VCID register."]
    #[inline(always)]
    pub const fn lcvcidr(self) -> crate::common::Reg<regs::Lcvcidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "DSI Host LTDC current color coding register."]
    #[inline(always)]
    pub const fn lcccr(self) -> crate::common::Reg<regs::Lcccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "DSI Host low-power mode current configuration register."]
    #[inline(always)]
    pub const fn lpmccr(self) -> crate::common::Reg<regs::Lpmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "DSI Host video mode current configuration register."]
    #[inline(always)]
    pub const fn vmccr(self) -> crate::common::Reg<regs::Vmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "DSI Host video packet current configuration register."]
    #[inline(always)]
    pub const fn vpccr(self) -> crate::common::Reg<regs::Vpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "DSI Host video chunks current configuration register."]
    #[inline(always)]
    pub const fn vcccr(self) -> crate::common::Reg<regs::Vcccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "DSI Host video null packet current configuration register."]
    #[inline(always)]
    pub const fn vnpccr(self) -> crate::common::Reg<regs::Vnpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "DSI Host video HSA current configuration register."]
    #[inline(always)]
    pub const fn vhsaccr(self) -> crate::common::Reg<regs::Vhsaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "DSI Host video HBP current configuration register."]
    #[inline(always)]
    pub const fn vhbpccr(self) -> crate::common::Reg<regs::Vhbpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "DSI Host video line current configuration register."]
    #[inline(always)]
    pub const fn vlccr(self) -> crate::common::Reg<regs::Vlccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "DSI Host video VSA current configuration register."]
    #[inline(always)]
    pub const fn vvsaccr(self) -> crate::common::Reg<regs::Vvsaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "DSI Host video VBP current configuration register."]
    #[inline(always)]
    pub const fn vvbpccr(self) -> crate::common::Reg<regs::Vvbpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "DSI Host video VFP current configuration register."]
    #[inline(always)]
    pub const fn vvfpccr(self) -> crate::common::Reg<regs::Vvfpccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "DSI Host video VA current configuration register."]
    #[inline(always)]
    pub const fn vvaccr(self) -> crate::common::Reg<regs::Vvaccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "DSI Host FIFO and buffer status register."]
    #[inline(always)]
    pub const fn fbsr(self) -> crate::common::Reg<regs::Fbsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "DSI Wrapper configuration register."]
    #[inline(always)]
    pub const fn wcfgr(self) -> crate::common::Reg<regs::Wcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "DSI Wrapper control register."]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "DSI Wrapper interrupt enable register."]
    #[inline(always)]
    pub const fn wier(self) -> crate::common::Reg<regs::Wier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "DSI Wrapper interrupt and status register."]
    #[inline(always)]
    pub const fn wisr(self) -> crate::common::Reg<regs::Wisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "DSI Wrapper interrupt flag clear register."]
    #[inline(always)]
    pub const fn wifcr(self) -> crate::common::Reg<regs::Wifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "DSI Wrapper PHY configuration register 0."]
    #[inline(always)]
    pub const fn wpcr0(self) -> crate::common::Reg<regs::Wpcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "DSI Wrapper regulator and PLL control register."]
    #[inline(always)]
    pub const fn wrpcr(self) -> crate::common::Reg<regs::Wrpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "DSI bias configuration register."]
    #[inline(always)]
    pub const fn bcfgr(self) -> crate::common::Reg<regs::Bcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "DSI D-PHY clock band control register."]
    #[inline(always)]
    pub const fn dpcbcr(self) -> crate::common::Reg<regs::Dpcbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[doc = "DSI D-PHY clock skew rate control register."]
    #[inline(always)]
    pub const fn dpcsrcr(self) -> crate::common::Reg<regs::Dpcsrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c34usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 0 band control register."]
    #[inline(always)]
    pub const fn dpdl0bcr(self) -> crate::common::Reg<regs::Dpdl0bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c70usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 0 skew rate control register."]
    #[inline(always)]
    pub const fn dpdl0srcr(self) -> crate::common::Reg<regs::Dpdl0srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ca0usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 1 band control register."]
    #[inline(always)]
    pub const fn dpdl1bcr(self) -> crate::common::Reg<regs::Dpdl1bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d08usize) as _) }
    }
    #[doc = "DSI D-PHY data lane 1 skew rate control register."]
    #[inline(always)]
    pub const fn dpdl1srcr(self) -> crate::common::Reg<regs::Dpdl1srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d38usize) as _) }
    }
}
pub mod regs {
    #[doc = "DSI bias configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcfgr(pub u32);
    impl Bcfgr {
        #[doc = "Power-up This bit powers-up the reference bias for the MIPI D-PHY."]
        #[inline(always)]
        pub const fn pwrup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Power-up This bit powers-up the reference bias for the MIPI D-PHY."]
        #[inline(always)]
        pub fn set_pwrup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Bcfgr {
        #[inline(always)]
        fn default() -> Bcfgr {
            Bcfgr(0)
        }
    }
    #[doc = "DSI Host clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
        #[inline(always)]
        pub const fn txeckdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TX escape clock division This field indicates the division factor for the TX escape clock source (lanebyteclk). The values 0 and 1 stop the TX_ESC clock generation."]
        #[inline(always)]
        pub fn set_txeckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
        #[inline(always)]
        pub const fn tockdiv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timeout clock division This field indicates the division factor for the timeout clock used as the timing unit in the configuration of HS to LP and LP to HS transition error."]
        #[inline(always)]
        pub fn set_tockdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "DSI Host clock lane configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clcr(pub u32);
    impl Clcr {
        #[doc = "D-PHY clock control This bit controls the D-PHY clock state:."]
        #[inline(always)]
        pub const fn dpcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY clock control This bit controls the D-PHY clock state:."]
        #[inline(always)]
        pub fn set_dpcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
        #[inline(always)]
        pub const fn acr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic clock lane control This bit enables the automatic mechanism to stop providing clock in the clock lane when time allows."]
        #[inline(always)]
        pub fn set_acr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Clcr {
        #[inline(always)]
        fn default() -> Clcr {
            Clcr(0)
        }
    }
    #[doc = "DSI Host clock lane timer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cltcr(pub u32);
    impl Cltcr {
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY clock lane takes to go from lowâ\u{80}\u{91}power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY clock lane takes to go from lowâ\u{80}\u{91}power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY clock lane takes to go from highâ\u{80}\u{91}speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY clock lane takes to go from highâ\u{80}\u{91}speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Cltcr {
        #[inline(always)]
        fn default() -> Cltcr {
            Cltcr(0)
        }
    }
    #[doc = "DSI Host command mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmcr(pub u32);
    impl Cmcr {
        #[doc = "Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:."]
        #[inline(always)]
        pub const fn teare(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:."]
        #[inline(always)]
        pub fn set_teare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge request enable This bit enables the acknowledge request after each packet transmission:."]
        #[inline(always)]
        pub const fn are(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge request enable This bit enables the acknowledge request after each packet transmission:."]
        #[inline(always)]
        pub fn set_are(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw0tx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw1tx(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsw2tx(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsw2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr0tx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr1tx(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:."]
        #[inline(always)]
        pub const fn gsr2tx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:."]
        #[inline(always)]
        pub fn set_gsr2tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Generic long write transmission This bit configures the generic long write packet command transmission type :."]
        #[inline(always)]
        pub const fn glwtx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Generic long write transmission This bit configures the generic long write packet command transmission type :."]
        #[inline(always)]
        pub fn set_glwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsw0tx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsw0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsw1tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsw1tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub const fn dsr0tx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:."]
        #[inline(always)]
        pub fn set_dsr0tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DCS long write transmission This bit configures the DCS long write packet command transmission type:."]
        #[inline(always)]
        pub const fn dlwtx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DCS long write transmission This bit configures the DCS long write packet command transmission type:."]
        #[inline(always)]
        pub fn set_dlwtx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Maximum read packet size This bit configures the maximum read packet size command transmission type:."]
        #[inline(always)]
        pub const fn mrdps(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Maximum read packet size This bit configures the maximum read packet size command transmission type:."]
        #[inline(always)]
        pub fn set_mrdps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Cmcr {
        #[inline(always)]
        fn default() -> Cmcr {
            Cmcr(0)
        }
    }
    #[doc = "DSI Host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Enable This bit configures the DSI Host in either power-up mode or to reset."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable This bit configures the DSI Host in either power-up mode or to reset."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "DSI Host data lane timer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dltcr(pub u32);
    impl Dltcr {
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Low-power to high-speed time This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "High-speed to low-power time This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Dltcr {
        #[inline(always)]
        fn default() -> Dltcr {
            Dltcr(0)
        }
    }
    #[doc = "DSI Host data lane timer read configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dltrcr(pub u32);
    impl Dltrcr {
        #[doc = "Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
        #[inline(always)]
        pub const fn mrd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
        #[inline(always)]
        pub fn set_mrd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Dltrcr {
        #[inline(always)]
        fn default() -> Dltrcr {
            Dltrcr(0)
        }
    }
    #[doc = "DSI D-PHY clock band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpcbcr(pub u32);
    impl Dpcbcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
    }
    impl Default for Dpcbcr {
        #[inline(always)]
        fn default() -> Dpcbcr {
            Dpcbcr(0)
        }
    }
    #[doc = "DSI D-PHY clock skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpcsrcr(pub u32);
    impl Dpcsrcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpcsrcr {
        #[inline(always)]
        fn default() -> Dpcsrcr {
            Dpcsrcr(0)
        }
    }
    #[doc = "DSI D-PHY data lane 0 band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl0bcr(pub u32);
    impl Dpdl0bcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dpdl0bcr {
        #[inline(always)]
        fn default() -> Dpdl0bcr {
            Dpdl0bcr(0)
        }
    }
    #[doc = "DSI D-PHY data lane 0 skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl0srcr(pub u32);
    impl Dpdl0srcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpdl0srcr {
        #[inline(always)]
        fn default() -> Dpdl0srcr {
            Dpdl0srcr(0)
        }
    }
    #[doc = "DSI D-PHY data lane 1 band control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl1bcr(pub u32);
    impl Dpdl1bcr {
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub const fn bc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Band control This field selects the frequency band used by the D-PHY. Others: Reserved."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dpdl1bcr {
        #[inline(always)]
        fn default() -> Dpdl1bcr {
            Dpdl1bcr(0)
        }
    }
    #[doc = "DSI D-PHY data lane 1 skew rate control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpdl1srcr(pub u32);
    impl Dpdl1srcr {
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Slew rate control This field selects the slew rate for HS-TX speed. Others: Reserved."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dpdl1srcr {
        #[inline(always)]
        fn default() -> Dpdl1srcr {
            Dpdl1srcr(0)
        }
    }
    #[doc = "DSI Host FIFO and buffer status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fbsr(pub u32);
    impl Fbsr {
        #[doc = "Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:."]
        #[inline(always)]
        pub const fn vcwfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:."]
        #[inline(always)]
        pub fn set_vcwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:."]
        #[inline(always)]
        pub const fn vcwff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:."]
        #[inline(always)]
        pub fn set_vcwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub const fn vpwfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_vpwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub const fn vpwff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_vpwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub const fn acwfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub fn set_acwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub const fn acwff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:."]
        #[inline(always)]
        pub fn set_acwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub const fn apwfe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_apwfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub const fn apwff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:."]
        #[inline(always)]
        pub fn set_apwff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub const fn vpbe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_vpbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub const fn vpbf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_vpbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub const fn acbe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub fn set_acbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub const fn acbf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:."]
        #[inline(always)]
        pub fn set_acbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub const fn apbe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_apbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub const fn apbf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:."]
        #[inline(always)]
        pub fn set_apbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Fbsr {
        #[inline(always)]
        fn default() -> Fbsr {
            Fbsr(0)
        }
    }
    #[doc = "DSI Host force interrupt register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir0(pub u32);
    impl Fir0 {
        #[doc = "Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0."]
        #[inline(always)]
        pub const fn fae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 0 Writing one to this bit forces an acknowledge error 0."]
        #[inline(always)]
        pub fn set_fae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1."]
        #[inline(always)]
        pub const fn fae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 1 Writing one to this bit forces an acknowledge error 1."]
        #[inline(always)]
        pub fn set_fae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2."]
        #[inline(always)]
        pub const fn fae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 2 Writing one to this bit forces an acknowledge error 2."]
        #[inline(always)]
        pub fn set_fae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3."]
        #[inline(always)]
        pub const fn fae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 3 Writing one to this bit forces an acknowledge error 3."]
        #[inline(always)]
        pub fn set_fae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4."]
        #[inline(always)]
        pub const fn fae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 4 Writing one to this bit forces an acknowledge error 4."]
        #[inline(always)]
        pub fn set_fae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5."]
        #[inline(always)]
        pub const fn fae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 5 Writing one to this bit forces an acknowledge error 5."]
        #[inline(always)]
        pub fn set_fae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6."]
        #[inline(always)]
        pub const fn fae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 6 Writing one to this bit forces an acknowledge error 6."]
        #[inline(always)]
        pub fn set_fae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7."]
        #[inline(always)]
        pub const fn fae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 7 Writing one to this bit forces an acknowledge error 7."]
        #[inline(always)]
        pub fn set_fae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8."]
        #[inline(always)]
        pub const fn fae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 8 Writing one to this bit forces an acknowledge error 8."]
        #[inline(always)]
        pub fn set_fae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9."]
        #[inline(always)]
        pub const fn fae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 9 Writing one to this bit forces an acknowledge error 9."]
        #[inline(always)]
        pub fn set_fae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10."]
        #[inline(always)]
        pub const fn fae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 10 Writing one to this bit forces an acknowledge error 10."]
        #[inline(always)]
        pub fn set_fae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11."]
        #[inline(always)]
        pub const fn fae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 11 Writing one to this bit forces an acknowledge error 11."]
        #[inline(always)]
        pub fn set_fae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12."]
        #[inline(always)]
        pub const fn fae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 12 Writing one to this bit forces an acknowledge error 12."]
        #[inline(always)]
        pub fn set_fae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13."]
        #[inline(always)]
        pub const fn fae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 13 Writing one to this bit forces an acknowledge error 13."]
        #[inline(always)]
        pub fn set_fae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14."]
        #[inline(always)]
        pub const fn fae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 14 Writing one to this bit forces an acknowledge error 14."]
        #[inline(always)]
        pub fn set_fae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15."]
        #[inline(always)]
        pub const fn fae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Force acknowledge error 15 Writing one to this bit forces an acknowledge error 15."]
        #[inline(always)]
        pub fn set_fae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Force PHY error 0 Writing one to this bit forces a PHY error 0."]
        #[inline(always)]
        pub const fn fpe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 0 Writing one to this bit forces a PHY error 0."]
        #[inline(always)]
        pub fn set_fpe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Force PHY error 1 Writing one to this bit forces a PHY error 1."]
        #[inline(always)]
        pub const fn fpe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 1 Writing one to this bit forces a PHY error 1."]
        #[inline(always)]
        pub fn set_fpe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Force PHY error 2 Writing one to this bit forces a PHY error 2."]
        #[inline(always)]
        pub const fn fpe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 2 Writing one to this bit forces a PHY error 2."]
        #[inline(always)]
        pub fn set_fpe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Force PHY error 3 Writing one to this bit forces a PHY error 3."]
        #[inline(always)]
        pub const fn fpe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 3 Writing one to this bit forces a PHY error 3."]
        #[inline(always)]
        pub fn set_fpe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Force PHY error 4 Writing one to this bit forces a PHY error 4."]
        #[inline(always)]
        pub const fn fpe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Force PHY error 4 Writing one to this bit forces a PHY error 4."]
        #[inline(always)]
        pub fn set_fpe4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Fir0 {
        #[inline(always)]
        fn default() -> Fir0 {
            Fir0(0)
        }
    }
    #[doc = "DSI Host force interrupt register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fir1(pub u32);
    impl Fir1 {
        #[doc = "Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
        #[inline(always)]
        pub const fn ftohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force timeout high-speed transmission Writing one to this bit forces a timeout high-speed transmission."]
        #[inline(always)]
        pub fn set_ftohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
        #[inline(always)]
        pub const fn ftolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force timeout low-power reception Writing one to this bit forces a timeout low-power reception."]
        #[inline(always)]
        pub fn set_ftolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
        #[inline(always)]
        pub const fn feccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC single-bit error Writing one to this bit forces a ECC single-bit error."]
        #[inline(always)]
        pub fn set_feccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
        #[inline(always)]
        pub const fn feccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECC multi-bit error Writing one to this bit forces a ECC multi-bit error."]
        #[inline(always)]
        pub fn set_feccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Force CRC error Writing one to this bit forces a CRC error."]
        #[inline(always)]
        pub const fn fcrce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force CRC error Writing one to this bit forces a CRC error."]
        #[inline(always)]
        pub fn set_fcrce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force packet size error Writing one to this bit forces a packet size error."]
        #[inline(always)]
        pub const fn fpse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force packet size error Writing one to this bit forces a packet size error."]
        #[inline(always)]
        pub fn set_fpse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Force EoTp error Writing one to this bit forces a EoTp error."]
        #[inline(always)]
        pub const fn feotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force EoTp error Writing one to this bit forces a EoTp error."]
        #[inline(always)]
        pub fn set_feotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
        #[inline(always)]
        pub const fn flpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Force LTDC payload write error Writing one to this bit forces a LTDC payload write error."]
        #[inline(always)]
        pub fn set_flpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Force generic command write error Writing one to this bit forces a generic command write error."]
        #[inline(always)]
        pub const fn fgcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic command write error Writing one to this bit forces a generic command write error."]
        #[inline(always)]
        pub fn set_fgcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force generic payload write error Writing one to this bit forces a generic payload write error."]
        #[inline(always)]
        pub const fn fgpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload write error Writing one to this bit forces a generic payload write error."]
        #[inline(always)]
        pub fn set_fgpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
        #[inline(always)]
        pub const fn fgptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload transmit error Writing one to this bit forces a generic payload transmit error."]
        #[inline(always)]
        pub fn set_fgptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Force generic payload read error Writing one to this bit forces a generic payload read error."]
        #[inline(always)]
        pub const fn fgprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload read error Writing one to this bit forces a generic payload read error."]
        #[inline(always)]
        pub fn set_fgprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
        #[inline(always)]
        pub const fn fgprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force generic payload receive error Writing one to this bit forces a generic payload receive error."]
        #[inline(always)]
        pub fn set_fgprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
        #[inline(always)]
        pub const fn fpbue(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Force payload buffer underflow error Writing one to this bit forces a payload undrflow error."]
        #[inline(always)]
        pub fn set_fpbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Fir1 {
        #[inline(always)]
        fn default() -> Fir1 {
            Fir1(0)
        }
    }
    #[doc = "DSI Host generic header configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ghcr(pub u32);
    impl Ghcr {
        #[doc = "Type This field configures the packet data type of the header packet."]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Type This field configures the packet data type of the header packet."]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Channel This field configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Channel This field configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
        #[inline(always)]
        pub const fn wclsb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount LSB This field configures the less significant byte of the header packet word count for long packets, or data 0 for short packets."]
        #[inline(always)]
        pub fn set_wclsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
        #[inline(always)]
        pub const fn wcmsb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "WordCount MSB This field configures the most significant byte of the header packet's word count for long packets, or data 1 for short packets."]
        #[inline(always)]
        pub fn set_wcmsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ghcr {
        #[inline(always)]
        fn default() -> Ghcr {
            Ghcr(0)
        }
    }
    #[doc = "DSI Host generic payload data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpdr(pub u32);
    impl Gpdr {
        #[doc = "Payload byte 1 This field indicates the byte 1 of the packet payload."]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 1 This field indicates the byte 1 of the packet payload."]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Payload byte 2 This field indicates the byte 2 of the packet payload."]
        #[inline(always)]
        pub const fn data2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 2 This field indicates the byte 2 of the packet payload."]
        #[inline(always)]
        pub fn set_data2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Payload byte 3 This field indicates the byte 3 of the packet payload."]
        #[inline(always)]
        pub const fn data3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 3 This field indicates the byte 3 of the packet payload."]
        #[inline(always)]
        pub fn set_data3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Payload byte 4 This field indicates the byte 4 of the packet payload."]
        #[inline(always)]
        pub const fn data4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Payload byte 4 This field indicates the byte 4 of the packet payload."]
        #[inline(always)]
        pub fn set_data4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Gpdr {
        #[inline(always)]
        fn default() -> Gpdr {
            Gpdr(0)
        }
    }
    #[doc = "DSI Host generic packet status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpsr(pub u32);
    impl Gpsr {
        #[doc = "Command FIFO empty This bit indicates the empty status of the generic command FIFO:."]
        #[inline(always)]
        pub const fn cmdfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO empty This bit indicates the empty status of the generic command FIFO:."]
        #[inline(always)]
        pub fn set_cmdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Command FIFO full This bit indicates the full status of the generic command FIFO:."]
        #[inline(always)]
        pub const fn cmdff(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command FIFO full This bit indicates the full status of the generic command FIFO:."]
        #[inline(always)]
        pub fn set_cmdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:."]
        #[inline(always)]
        pub const fn pwrfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:."]
        #[inline(always)]
        pub fn set_pwrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:."]
        #[inline(always)]
        pub const fn pwrff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:."]
        #[inline(always)]
        pub fn set_pwrff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:."]
        #[inline(always)]
        pub const fn prdfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:."]
        #[inline(always)]
        pub fn set_prdfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:."]
        #[inline(always)]
        pub const fn prdff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:."]
        #[inline(always)]
        pub fn set_prdff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:."]
        #[inline(always)]
        pub const fn rcb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:."]
        #[inline(always)]
        pub fn set_rcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn cmdbe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Command buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_cmdbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Command buffer full This bit indicates the full status of the generic command internal buffer:."]
        #[inline(always)]
        pub const fn cmdbf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Command buffer full This bit indicates the full status of the generic command internal buffer:."]
        #[inline(always)]
        pub fn set_cmdbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn pbe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer empty This bit indicates the empty status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_pbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Payload buffer full This bit indicates the full status of the generic payload internal buffer:."]
        #[inline(always)]
        pub const fn pbf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer full This bit indicates the full status of the generic payload internal buffer:."]
        #[inline(always)]
        pub fn set_pbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Gpsr {
        #[inline(always)]
        fn default() -> Gpsr {
            Gpsr(0)
        }
    }
    #[doc = "DSI Host generic VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gvcidr(pub u32);
    impl Gvcidr {
        #[doc = "Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
        #[inline(always)]
        pub const fn vcidrx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID for reception This field indicates the generic interface read-back virtual channel identification."]
        #[inline(always)]
        pub fn set_vcidrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub const fn vcidtx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID for transmission This field indicates the generic interface virtual channel identification where the generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub fn set_vcidtx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for Gvcidr {
        #[inline(always)]
        fn default() -> Gvcidr {
            Gvcidr(0)
        }
    }
    #[doc = "DSI Host interrupt enable register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier0(pub u32);
    impl Ier0 {
        #[doc = "Acknowledge error 0 interrupt enable This bit enables the interrupt generation on acknowledge error 0."]
        #[inline(always)]
        pub const fn ae0ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 0 interrupt enable This bit enables the interrupt generation on acknowledge error 0."]
        #[inline(always)]
        pub fn set_ae0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge error 1 interrupt enable This bit enables the interrupt generation on acknowledge error 1."]
        #[inline(always)]
        pub const fn ae1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 1 interrupt enable This bit enables the interrupt generation on acknowledge error 1."]
        #[inline(always)]
        pub fn set_ae1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge error 2 interrupt enable This bit enables the interrupt generation on acknowledge error 2."]
        #[inline(always)]
        pub const fn ae2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 2 interrupt enable This bit enables the interrupt generation on acknowledge error 2."]
        #[inline(always)]
        pub fn set_ae2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge error 3 interrupt enable This bit enables the interrupt generation on acknowledge error 3."]
        #[inline(always)]
        pub const fn ae3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 3 interrupt enable This bit enables the interrupt generation on acknowledge error 3."]
        #[inline(always)]
        pub fn set_ae3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge error 4 interrupt enable This bit enables the interrupt generation on acknowledge error 4."]
        #[inline(always)]
        pub const fn ae4ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 4 interrupt enable This bit enables the interrupt generation on acknowledge error 4."]
        #[inline(always)]
        pub fn set_ae4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge error 5 interrupt enable This bit enables the interrupt generation on acknowledge error 5."]
        #[inline(always)]
        pub const fn ae5ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 5 interrupt enable This bit enables the interrupt generation on acknowledge error 5."]
        #[inline(always)]
        pub fn set_ae5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge error 6 interrupt enable This bit enables the interrupt generation on acknowledge error 6."]
        #[inline(always)]
        pub const fn ae6ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 6 interrupt enable This bit enables the interrupt generation on acknowledge error 6."]
        #[inline(always)]
        pub fn set_ae6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge error 7 interrupt enable This bit enables the interrupt generation on acknowledge error 7."]
        #[inline(always)]
        pub const fn ae7ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 7 interrupt enable This bit enables the interrupt generation on acknowledge error 7."]
        #[inline(always)]
        pub fn set_ae7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge error 8 interrupt enable This bit enables the interrupt generation on acknowledge error 8."]
        #[inline(always)]
        pub const fn ae8ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 8 interrupt enable This bit enables the interrupt generation on acknowledge error 8."]
        #[inline(always)]
        pub fn set_ae8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge error 9 interrupt enable This bit enables the interrupt generation on acknowledge error 9."]
        #[inline(always)]
        pub const fn ae9ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 9 interrupt enable This bit enables the interrupt generation on acknowledge error 9."]
        #[inline(always)]
        pub fn set_ae9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge error 10 interrupt enable This bit enables the interrupt generation on acknowledge error 10."]
        #[inline(always)]
        pub const fn ae10ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 10 interrupt enable This bit enables the interrupt generation on acknowledge error 10."]
        #[inline(always)]
        pub fn set_ae10ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge error 11 interrupt enable This bit enables the interrupt generation on acknowledge error 11."]
        #[inline(always)]
        pub const fn ae11ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 11 interrupt enable This bit enables the interrupt generation on acknowledge error 11."]
        #[inline(always)]
        pub fn set_ae11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge error 12 interrupt enable This bit enables the interrupt generation on acknowledge error 12."]
        #[inline(always)]
        pub const fn ae12ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 12 interrupt enable This bit enables the interrupt generation on acknowledge error 12."]
        #[inline(always)]
        pub fn set_ae12ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge error 13 interrupt enable This bit enables the interrupt generation on acknowledge error 13."]
        #[inline(always)]
        pub const fn ae13ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 13 interrupt enable This bit enables the interrupt generation on acknowledge error 13."]
        #[inline(always)]
        pub fn set_ae13ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge error 14 interrupt enable This bit enables the interrupt generation on acknowledge error 14."]
        #[inline(always)]
        pub const fn ae14ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 14 interrupt enable This bit enables the interrupt generation on acknowledge error 14."]
        #[inline(always)]
        pub fn set_ae14ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge error 15 interrupt enable This bit enables the interrupt generation on acknowledge error 15."]
        #[inline(always)]
        pub const fn ae15ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 15 interrupt enable This bit enables the interrupt generation on acknowledge error 15."]
        #[inline(always)]
        pub fn set_ae15ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY error 0 interrupt enable This bit enables the interrupt generation on PHY error 0."]
        #[inline(always)]
        pub const fn pe0ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 0 interrupt enable This bit enables the interrupt generation on PHY error 0."]
        #[inline(always)]
        pub fn set_pe0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY error 1 interrupt enable This bit enables the interrupt generation on PHY error 1."]
        #[inline(always)]
        pub const fn pe1ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 1 interrupt enable This bit enables the interrupt generation on PHY error 1."]
        #[inline(always)]
        pub fn set_pe1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY error 2 interrupt enable This bit enables the interrupt generation on PHY error 2."]
        #[inline(always)]
        pub const fn pe2ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 2 interrupt enable This bit enables the interrupt generation on PHY error 2."]
        #[inline(always)]
        pub fn set_pe2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY error 3 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub const fn pe3ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 3 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub fn set_pe3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY error 4 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub const fn pe4ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 4 interrupt enable This bit enables the interrupt generation on PHY error 4."]
        #[inline(always)]
        pub fn set_pe4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ier0 {
        #[inline(always)]
        fn default() -> Ier0 {
            Ier0(0)
        }
    }
    #[doc = "DSI Host interrupt enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier1(pub u32);
    impl Ier1 {
        #[doc = "Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission."]
        #[inline(always)]
        pub const fn tohstxie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission."]
        #[inline(always)]
        pub fn set_tohstxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception."]
        #[inline(always)]
        pub const fn tolprxie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception."]
        #[inline(always)]
        pub fn set_tolprxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error."]
        #[inline(always)]
        pub const fn eccseie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error."]
        #[inline(always)]
        pub fn set_eccseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error."]
        #[inline(always)]
        pub const fn eccmeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error."]
        #[inline(always)]
        pub fn set_eccmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error interrupt enable This bit enables the interrupt generation on CRC error."]
        #[inline(always)]
        pub const fn crceie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error interrupt enable This bit enables the interrupt generation on CRC error."]
        #[inline(always)]
        pub fn set_crceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet size error interrupt enable This bit enables the interrupt generation on packet size error."]
        #[inline(always)]
        pub const fn pseie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet size error interrupt enable This bit enables the interrupt generation on packet size error."]
        #[inline(always)]
        pub fn set_pseie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp error interrupt enable This bit enables the interrupt generation on EoTp error."]
        #[inline(always)]
        pub const fn eotpeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp error interrupt enable This bit enables the interrupt generation on EoTp error."]
        #[inline(always)]
        pub fn set_eotpeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error."]
        #[inline(always)]
        pub const fn lpwreie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error."]
        #[inline(always)]
        pub fn set_lpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error."]
        #[inline(always)]
        pub const fn gcwreie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error."]
        #[inline(always)]
        pub fn set_gcwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error."]
        #[inline(always)]
        pub const fn gpwreie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error."]
        #[inline(always)]
        pub fn set_gpwreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error."]
        #[inline(always)]
        pub const fn gptxeie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error."]
        #[inline(always)]
        pub fn set_gptxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error."]
        #[inline(always)]
        pub const fn gprdeie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error."]
        #[inline(always)]
        pub fn set_gprdeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error."]
        #[inline(always)]
        pub const fn gprxeie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error."]
        #[inline(always)]
        pub fn set_gprxeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Payload buffer underflow error interrupt enable This bit enables the interrupt generation on payload buffer underflow error."]
        #[inline(always)]
        pub const fn pbueie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer underflow error interrupt enable This bit enables the interrupt generation on payload buffer underflow error."]
        #[inline(always)]
        pub fn set_pbueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Ier1 {
        #[inline(always)]
        fn default() -> Ier1 {
            Ier1(0)
        }
    }
    #[doc = "DSI Host interrupt and status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr0(pub u32);
    impl Isr0 {
        #[doc = "Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 0 This bit retrieves the SoT error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 1 This bit retrieves the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 2 This bit retrieves the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 3 This bit retrieves the escape mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 4 This bit retrieves the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 5 This bit retrieves the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 6 This bit retrieves the false control error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 7 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 8 This bit retrieves the ECC error, single-bit (detected and corrected) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 9 This bit retrieves the ECC error, multi-bit (detected, not corrected) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 10 This bit retrieves the checksum error (long packet only) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 11 This bit retrieves the not recognized DSI data type from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 12 This bit retrieves the DSI VC ID Invalid from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 13 This bit retrieves the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 14 This bit retrieves the reserved (specific to the device) from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report."]
        #[inline(always)]
        pub const fn ae15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge error 15 This bit retrieves the DSI protocol violation from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ae15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0."]
        #[inline(always)]
        pub const fn pe0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 0 This bit indicates the ErrEsc escape entry error from lane 0."]
        #[inline(always)]
        pub fn set_pe0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0."]
        #[inline(always)]
        pub const fn pe1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 1 This bit indicates the ErrSyncEsc low-power transmission synchronization error from lane 0."]
        #[inline(always)]
        pub fn set_pe1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY error 2 This bit indicates the ErrControl error from lane 0."]
        #[inline(always)]
        pub const fn pe2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 2 This bit indicates the ErrControl error from lane 0."]
        #[inline(always)]
        pub fn set_pe2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0."]
        #[inline(always)]
        pub const fn pe3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 3 This bit indicates the LP0 contention error ErrContentionLP0 from lane 0."]
        #[inline(always)]
        pub fn set_pe3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0."]
        #[inline(always)]
        pub const fn pe4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PHY error 4 This bit indicates the LP1 contention error ErrContentionLP1 from lane 0."]
        #[inline(always)]
        pub fn set_pe4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Isr0 {
        #[inline(always)]
        fn default() -> Isr0 {
            Isr0(0)
        }
    }
    #[doc = "DSI Host interrupt and status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub const fn tohstx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub fn set_tohstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub const fn tolprx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
        #[inline(always)]
        pub fn set_tolprx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
        #[inline(always)]
        pub const fn eccse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
        #[inline(always)]
        pub fn set_eccse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
        #[inline(always)]
        pub const fn eccme(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
        #[inline(always)]
        pub fn set_eccme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC error This bit indicates that the CRC error is detected in the received packet payload."]
        #[inline(always)]
        pub const fn crce(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error This bit indicates that the CRC error is detected in the received packet payload."]
        #[inline(always)]
        pub fn set_crce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Packet size error This bit indicates that the packet size error is detected during the packet reception."]
        #[inline(always)]
        pub const fn pse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Packet size error This bit indicates that the packet size error is detected during the packet reception."]
        #[inline(always)]
        pub fn set_pse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub const fn eotpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub fn set_eotpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
        #[inline(always)]
        pub const fn lpwre(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
        #[inline(always)]
        pub fn set_lpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
        #[inline(always)]
        pub const fn gcwre(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
        #[inline(always)]
        pub fn set_gcwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
        #[inline(always)]
        pub const fn gpwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
        #[inline(always)]
        pub fn set_gpwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
        #[inline(always)]
        pub const fn gptxe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
        #[inline(always)]
        pub fn set_gptxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
        #[inline(always)]
        pub const fn gprde(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
        #[inline(always)]
        pub fn set_gprde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
        #[inline(always)]
        pub const fn gprxe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
        #[inline(always)]
        pub fn set_gprxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
        #[inline(always)]
        pub const fn pbue(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
        #[inline(always)]
        pub fn set_pbue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Isr1 {
        #[inline(always)]
        fn default() -> Isr1 {
            Isr1(0)
        }
    }
    #[doc = "DSI Host LTDC current color coding register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcccr(pub u32);
    impl Lcccr {
        #[doc = "Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color coding This field returns the current LTDC interface color coding. 0110-1111: reserved If LTDC interface in command mode is chosen and currently works in the command mode (CMDM=1), then 0110-1111: 24-bit."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely packed enable This bit returns the current state of the loosely packed variant to 18-bit configurations."]
        #[inline(always)]
        pub fn set_lpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Lcccr {
        #[inline(always)]
        fn default() -> Lcccr {
            Lcccr(0)
        }
    }
    #[doc = "DSI Host LTDC command configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lccr(pub u32);
    impl Lccr {
        #[doc = "Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
        #[inline(always)]
        pub const fn cmdsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Command size This field configures the maximum allowed size for an LTDC write memory command, measured in pixels. Automatic partitioning of data obtained from LTDC is permanently enabled."]
        #[inline(always)]
        pub fn set_cmdsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lccr {
        #[inline(always)]
        fn default() -> Lccr {
            Lccr(0)
        }
    }
    #[doc = "DSI Host LTDC color coding register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcolcr(pub u32);
    impl Lcolcr {
        #[doc = "Color coding This field configures the DPI color coding. Others: Reserved."]
        #[inline(always)]
        pub const fn colc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Color coding This field configures the DPI color coding. Others: Reserved."]
        #[inline(always)]
        pub fn set_colc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration."]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loosely packet enable This bit enables the loosely packed variant to 18-bit configuration."]
        #[inline(always)]
        pub fn set_lpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Lcolcr {
        #[inline(always)]
        fn default() -> Lcolcr {
            Lcolcr(0)
        }
    }
    #[doc = "DSI Host LTDC current VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcvcidr(pub u32);
    impl Lcvcidr {
        #[doc = "Virtual channel ID This field returns the virtual channel ID for the LTDC interface."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID This field returns the virtual channel ID for the LTDC interface."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lcvcidr {
        #[inline(always)]
        fn default() -> Lcvcidr {
            Lcvcidr(0)
        }
    }
    #[doc = "DSI Host LTDC polarity configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpcr(pub u32);
    impl Lpcr {
        #[doc = "Data enable polarity This bit configures the polarity of data enable pin."]
        #[inline(always)]
        pub const fn dep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data enable polarity This bit configures the polarity of data enable pin."]
        #[inline(always)]
        pub fn set_dep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VSYNC polarity This bit configures the polarity of VSYNC pin."]
        #[inline(always)]
        pub const fn vsp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC polarity This bit configures the polarity of VSYNC pin."]
        #[inline(always)]
        pub fn set_vsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSYNC polarity This bit configures the polarity of HSYNC pin."]
        #[inline(always)]
        pub const fn hsp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSYNC polarity This bit configures the polarity of HSYNC pin."]
        #[inline(always)]
        pub fn set_hsp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Lpcr {
        #[inline(always)]
        fn default() -> Lpcr {
            Lpcr(0)
        }
    }
    #[doc = "DSI Host low-power mode current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmccr(pub u32);
    impl Lpmccr {
        #[doc = "VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT largest packet size This field returns the current size, in bytes, of the largest packet that can fit in a line during VACT regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest packet size This field is returns the current size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions, for the transmission of commands in low-power mode."]
        #[inline(always)]
        pub fn set_lpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lpmccr {
        #[inline(always)]
        fn default() -> Lpmccr {
            Lpmccr(0)
        }
    }
    #[doc = "DSI Host low-power mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmcr(pub u32);
    impl Lpmcr {
        #[doc = "VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
        #[inline(always)]
        pub const fn vlpsize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "VACT largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VACT regions."]
        #[inline(always)]
        pub fn set_vlpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
        #[inline(always)]
        pub const fn lpsize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Largest packet size This field is used for the transmission of commands in low-power mode. It defines the size, in bytes, of the largest packet that can fit in a line during VSA, VBP and VFP regions."]
        #[inline(always)]
        pub fn set_lpsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lpmcr {
        #[inline(always)]
        fn default() -> Lpmcr {
            Lpmcr(0)
        }
    }
    #[doc = "DSI Host LTDC VCID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lvcidr(pub u32);
    impl Lvcidr {
        #[doc = "Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic."]
        #[inline(always)]
        pub const fn vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel ID These bits configure the virtual channel ID for the LTDC interface traffic."]
        #[inline(always)]
        pub fn set_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lvcidr {
        #[inline(always)]
        fn default() -> Lvcidr {
            Lvcidr(0)
        }
    }
    #[doc = "DSI Host mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Command mode This bit configures the DSI Host in either video or command mode."]
        #[inline(always)]
        pub const fn cmdm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command mode This bit configures the DSI Host in either video or command mode."]
        #[inline(always)]
        pub fn set_cmdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "DSI Host PHY configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pconfr(pub u32);
    impl Pconfr {
        #[doc = "Number of lanes This field configures the number of active data lanes: Others: Reserved."]
        #[inline(always)]
        pub const fn nl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Number of lanes This field configures the number of active data lanes: Others: Reserved."]
        #[inline(always)]
        pub fn set_nl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
        #[inline(always)]
        pub const fn sw_time(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state."]
        #[inline(always)]
        pub fn set_sw_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Pconfr {
        #[inline(always)]
        fn default() -> Pconfr {
            Pconfr(0)
        }
    }
    #[doc = "DSI Host protocol configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "EoTp transmission enable This bit enables the EoTP transmission."]
        #[inline(always)]
        pub const fn ettxe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp transmission enable This bit enables the EoTP transmission."]
        #[inline(always)]
        pub fn set_ettxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EoTp reception enable This bit enables the EoTp reception."]
        #[inline(always)]
        pub const fn etrxe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp reception enable This bit enables the EoTp reception."]
        #[inline(always)]
        pub fn set_etrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
        #[inline(always)]
        pub const fn btae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Bus-turn-around enable This bit enables the bus-turn-around (BTA) request."]
        #[inline(always)]
        pub fn set_btae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC reception enable This bit enables the ECC reception, error correction and reporting."]
        #[inline(always)]
        pub const fn eccrxe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC reception enable This bit enables the ECC reception, error correction and reporting."]
        #[inline(always)]
        pub fn set_eccrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reception enable This bit enables the CRC reception and error reporting."]
        #[inline(always)]
        pub const fn crcrxe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reception enable This bit enables the CRC reception and error reporting."]
        #[inline(always)]
        pub fn set_crcrxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
        #[inline(always)]
        pub const fn ettxlpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EoTp transmission in low-power enable This bit enables the EoTP transmission in low-power."]
        #[inline(always)]
        pub fn set_ettxlpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    #[doc = "DSI Host PHY control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pctlr(pub u32);
    impl Pctlr {
        #[doc = "Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital enable When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock enable This bit enables the D-PHY clock lane module:."]
        #[inline(always)]
        pub const fn cke(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable This bit enables the D-PHY clock lane module:."]
        #[inline(always)]
        pub fn set_cke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Pctlr {
        #[inline(always)]
        fn default() -> Pctlr {
            Pctlr(0)
        }
    }
    #[doc = "DSI Host PHY status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "PHY direction This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PHY direction This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub const fn pssc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state clock lane This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub fn set_pssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal."]
        #[inline(always)]
        pub const fn uanc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not clock lane This bit indicates the status of ulpsactivenotclklane D-PHY signal."]
        #[inline(always)]
        pub fn set_uanc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub const fn pss0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state lane 0 This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_pss0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub const fn uan0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_uan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal."]
        #[inline(always)]
        pub const fn rue0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX ULPS escape lane 0 This bit indicates the status of rxulpsesc0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_rue0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub const fn pss1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PHY stop state lane 1 This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub fn set_pss1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
        #[inline(always)]
        pub const fn uan1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS active not lane 1 This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
        #[inline(always)]
        pub fn set_uan1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Psr {
        #[inline(always)]
        fn default() -> Psr {
            Psr(0)
        }
    }
    #[doc = "DSI Host PHY TX triggers configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pttcr(pub u32);
    impl Pttcr {
        #[doc = "Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time."]
        #[inline(always)]
        pub const fn tx_trig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmission trigger Escape mode transmit trigger 0-3. Only one bit of TX_TRIG is asserted at any given time."]
        #[inline(always)]
        pub fn set_tx_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Pttcr {
        #[inline(always)]
        fn default() -> Pttcr {
            Pttcr(0)
        }
    }
    #[doc = "DSI Host PHY ULPS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucr(pub u32);
    impl Pucr {
        #[doc = "ULPS request on clock lane ULPS mode request on clock lane."]
        #[inline(always)]
        pub const fn urcl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS request on clock lane ULPS mode request on clock lane."]
        #[inline(always)]
        pub fn set_urcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ULPS exit on clock lane ULPS mode exit on clock lane."]
        #[inline(always)]
        pub const fn uecl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS exit on clock lane ULPS mode exit on clock lane."]
        #[inline(always)]
        pub fn set_uecl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ULPS request on data lane ULPS mode request on all active data lanes."]
        #[inline(always)]
        pub const fn urdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS request on data lane ULPS mode request on all active data lanes."]
        #[inline(always)]
        pub fn set_urdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ULPS exit on data lane ULPS mode exit on all active data lanes."]
        #[inline(always)]
        pub const fn uedl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ULPS exit on data lane ULPS mode exit on all active data lanes."]
        #[inline(always)]
        pub fn set_uedl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pucr {
        #[inline(always)]
        fn default() -> Pucr {
            Pucr(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr0(pub u32);
    impl Tccr0 {
        #[doc = "Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles)."]
        #[inline(always)]
        pub const fn lprx_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power reception timeout counter This field configures the timeout counter that triggers a low-power reception timeout contention detection (measured in TOCKDIV cycles)."]
        #[inline(always)]
        pub fn set_lprx_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one LINE data transmission *Â (1Â +Â 10%)."]
        #[inline(always)]
        pub const fn hstx_tocnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed transmission timeout counter This field configures the timeout counter that triggers a high-speed transmission timeout contention detection (measured in TOCKDIV cycles). If using the non-burst mode and there is no enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link returns the low-power state once per frame, then configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one FRAME data transmission *Â (1 + 10%) In burst mode, RGB pixel packets are time-compressed, leaving more time during a scan line. Therefore, if in burst mode and there is enough time to switch from high-speed to low-power and back in the period from one line data finishing to the next line sync start, the DSI link can return low-power mode and back in this time interval to save power. For this, configure the TOCKDIV and HSTX_TOCNT to be in accordance with: HSTX_TOCNT * lanebyteclkperiod * TOCKDIV â\u{89}¥ the time of one LINE data transmission *Â (1Â +Â 10%)."]
        #[inline(always)]
        pub fn set_hstx_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tccr0 {
        #[inline(always)]
        fn default() -> Tccr0 {
            Tccr0(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr1(pub u32);
    impl Tccr1 {
        #[doc = "High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn hsrd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a high-speed read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_hsrd_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr1 {
        #[inline(always)]
        fn default() -> Tccr1 {
            Tccr1(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr2(pub u32);
    impl Tccr2 {
        #[doc = "Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn lprd_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power read timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power read operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_lprd_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr2 {
        #[inline(always)]
        fn default() -> Tccr2 {
            Tccr2(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr3(pub u32);
    impl Tccr3 {
        #[doc = "High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn hswr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "High-speed write timeout counter This field sets a period for which the DSI Host keeps the link inactive after sending a high-speed write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_hswr_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Presp mode When set to 1, this bit ensures that the peripheral response timeout caused by HSWR_TOCNT is used only once per LTDC frame in command mode, when both the following conditions are met: dpivsync_edpiwms has risen and fallen. Packets originated from LTDC in command mode have been transmitted and its FIFO is empty again. In this scenario no non-LTDC command requests are sent to the D-PHY, even if there is traffic from generic interface ready to be sent, making it return to stop state. When it does so, PRESP_TO counter is activated and only when it finishes does the controller send any other traffic that is ready."]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Tccr3 {
        #[inline(always)]
        fn default() -> Tccr3 {
            Tccr3(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr4(pub u32);
    impl Tccr4 {
        #[doc = "Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn lpwr_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Low-power write timeout counter This field sets a period for which the DSI Host keeps the link still, after sending a low-power write operation. This period is measured in cycles of lanebyteclk. The counting starts when the D-PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_lpwr_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr4 {
        #[inline(always)]
        fn default() -> Tccr4 {
            Tccr4(0)
        }
    }
    #[doc = "DSI Host timeout counter configuration register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tccr5(pub u32);
    impl Tccr5 {
        #[doc = "Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the Dâ\u{80}\u{91}PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub const fn bta_tocnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bus-turn-around timeout counter This field sets a period for which the DSI Host keeps the link still, after completing a bus-turn-around. This period is measured in cycles of lanebyteclk. The counting starts when the Dâ\u{80}\u{91}PHY enters the Stop state and causes no interrupts."]
        #[inline(always)]
        pub fn set_bta_tocnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tccr5 {
        #[inline(always)]
        fn default() -> Tccr5 {
            Tccr5(0)
        }
    }
    #[doc = "DSI Host video chunks current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vcccr(pub u32);
    impl Vcccr {
        #[doc = "Number of chunks This field returns the number of chunks being transmitted during a line period."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of chunks This field returns the number of chunks being transmitted during a line period."]
        #[inline(always)]
        pub fn set_numc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vcccr {
        #[inline(always)]
        fn default() -> Vcccr {
            Vcccr(0)
        }
    }
    #[doc = "DSI Host video chunks configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vccr(pub u32);
    impl Vccr {
        #[doc = "Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line."]
        #[inline(always)]
        pub const fn numc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of chunks This register configures the number of chunks to be transmitted during a line period (a chunk consists of a video packet and a null packet). If set to 0 or 1, the video line is transmitted in a single packet. If set to 1, the packet is part of a chunk, so a null packet follows it if NPSIZE > 0. Otherwise, multiple chunks are used to transmit each video line."]
        #[inline(always)]
        pub fn set_numc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vccr {
        #[inline(always)]
        fn default() -> Vccr {
            Vccr(0)
        }
    }
    #[doc = "DSI Host video HBP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpccr(pub u32);
    impl Vhbpccr {
        #[doc = "Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhbpccr {
        #[inline(always)]
        fn default() -> Vhbpccr {
            Vhbpccr(0)
        }
    }
    #[doc = "DSI Host video HBP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhbpcr(pub u32);
    impl Vhbpcr {
        #[doc = "Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal back-porch duration This fields configures the horizontal back-porch period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhbpcr {
        #[inline(always)]
        fn default() -> Vhbpcr {
            Vhbpcr(0)
        }
    }
    #[doc = "DSI Host video HSA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsaccr(pub u32);
    impl Vhsaccr {
        #[doc = "Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhsaccr {
        #[inline(always)]
        fn default() -> Vhsaccr {
            Vhsaccr(0)
        }
    }
    #[doc = "DSI Host video HSA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vhsacr(pub u32);
    impl Vhsacr {
        #[doc = "Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal synchronism active duration This fields configures the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Vhsacr {
        #[inline(always)]
        fn default() -> Vhsacr {
            Vhsacr(0)
        }
    }
    #[doc = "DSI Host video line current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlccr(pub u32);
    impl Vlccr {
        #[doc = "Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Vlccr {
        #[inline(always)]
        fn default() -> Vlccr {
            Vlccr(0)
        }
    }
    #[doc = "DSI Host video line configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vlcr(pub u32);
    impl Vlcr {
        #[doc = "Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub const fn hline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Horizontal line duration This fields configures the total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_hline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Vlcr {
        #[inline(always)]
        fn default() -> Vlcr {
            Vlcr(0)
        }
    }
    #[doc = "DSI Host video mode current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmccr(pub u32);
    impl Vmccr {
        #[doc = "Video mode type This field returns the current video mode transmission type: 1x: Burst mode."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode type This field returns the current video mode transmission type: 1x: Burst mode."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical sync time enable This bit returns the current state of return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical back-porch enable This bit returns the current state of return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical front-porch enable This bit returns the current state of return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical active enable This bit returns the current state of return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal back-porch enable This bit returns the current state of return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub const fn lphfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal front-porch enable This bit returns the current state of return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Frame BTA acknowledge enable This bit returns the current state of request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power command enable This bit returns the current command transmission state in low-power mode."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power command enable This bit returns the current command transmission state in low-power mode."]
        #[inline(always)]
        pub fn set_lpce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vmccr {
        #[inline(always)]
        fn default() -> Vmccr {
            Vmccr(0)
        }
    }
    #[doc = "DSI Host video mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[doc = "Video mode type This field configures the video mode transmission type : 1x: Burst mode."]
        #[inline(always)]
        pub const fn vmt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Video mode type This field configures the video mode transmission type : 1x: Burst mode."]
        #[inline(always)]
        pub fn set_vmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub const fn lpvsae(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical sync active enable This bit enables to return to low-power inside the vertical sync time (VSA) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvsae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvbpe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical back-porch enable This bit enables to return to low-power inside the vertical back-porch (VBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub const fn lpvfpe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical front-porch enable This bit enables to return to low-power inside the vertical front-porch (VFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub const fn lpvae(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power vertical active enable This bit enables to return to low-power inside the vertical active (VACT) period when timing allows."]
        #[inline(always)]
        pub fn set_lpvae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub const fn lphbpe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal back-porch enable This bit enables the return to low-power inside the horizontal back-porch (HBP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphbpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub const fn lphfpe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power horizontal front-porch enable This bit enables the return to low-power inside the horizontal front-porch (HFP) period when timing allows."]
        #[inline(always)]
        pub fn set_lphfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn fbtaae(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Frame bus-turn-around acknowledge enable This bit enables the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_fbtaae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Low-power command enable This bit enables the command transmission only in low-power mode."]
        #[inline(always)]
        pub const fn lpce(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power command enable This bit enables the command transmission only in low-power mode."]
        #[inline(always)]
        pub fn set_lpce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Pattern generator enable This bit enables the video mode pattern generator."]
        #[inline(always)]
        pub const fn pge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator enable This bit enables the video mode pattern generator."]
        #[inline(always)]
        pub fn set_pge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pattern generator mode This bit configures the pattern generator mode."]
        #[inline(always)]
        pub const fn pgm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator mode This bit configures the pattern generator mode."]
        #[inline(always)]
        pub fn set_pgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Pattern generator orientation This bit configures the color bar orientation."]
        #[inline(always)]
        pub const fn pgo(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pattern generator orientation This bit configures the color bar orientation."]
        #[inline(always)]
        pub fn set_pgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    #[doc = "DSI Host video null packet current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpccr(pub u32);
    impl Vnpccr {
        #[doc = "Null packet size This field returns the number of bytes inside a null packet."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null packet size This field returns the number of bytes inside a null packet."]
        #[inline(always)]
        pub fn set_npsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vnpccr {
        #[inline(always)]
        fn default() -> Vnpccr {
            Vnpccr(0)
        }
    }
    #[doc = "DSI Host video null packet configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vnpcr(pub u32);
    impl Vnpcr {
        #[doc = "Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets."]
        #[inline(always)]
        pub const fn npsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Null packet size This field configures the number of bytes inside a null packet. Setting to 0 disables the null packets."]
        #[inline(always)]
        pub fn set_npsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Vnpcr {
        #[inline(always)]
        fn default() -> Vnpcr {
            Vnpcr(0)
        }
    }
    #[doc = "DSI Host video packet current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpccr(pub u32);
    impl Vpccr {
        #[doc = "Video packet size This field returns the number of pixels in a single video packet."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video packet size This field returns the number of pixels in a single video packet."]
        #[inline(always)]
        pub fn set_vpsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vpccr {
        #[inline(always)]
        fn default() -> Vpccr {
            Vpccr(0)
        }
    }
    #[doc = "DSI Host video packet configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vpcr(pub u32);
    impl Vpcr {
        #[doc = "Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
        #[inline(always)]
        pub const fn vpsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Video packet size This field configures the number of pixels in a single video packet. For 18-bit not loosely packed data types, this number must be a multiple of 4. For YCbCr data types, it must be a multiple of 2 as described in the DSI specification."]
        #[inline(always)]
        pub fn set_vpsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vpcr {
        #[inline(always)]
        fn default() -> Vpcr {
            Vpcr(0)
        }
    }
    #[doc = "DSI Host version register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vr(pub u32);
    impl Vr {
        #[doc = "Version of the DSI Host This read-only register contains the version of the DSI Host."]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Version of the DSI Host This read-only register contains the version of the DSI Host."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Vr {
        #[inline(always)]
        fn default() -> Vr {
            Vr(0)
        }
    }
    #[doc = "DSI Host video shadow control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vscr(pub u32);
    impl Vscr {
        #[doc = "Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable When set to 1, DSI Host LTDC interface receives the active configuration from the auxiliary registers. When this bit is set along with the UR bit, the auxiliary registers are automatically updated."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
        #[inline(always)]
        pub const fn ur(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update register When set to 1, the LTDC registers are copied to the auxiliary registers. After copying, this bit is auto cleared."]
        #[inline(always)]
        pub fn set_ur(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Vscr {
        #[inline(always)]
        fn default() -> Vscr {
            Vscr(0)
        }
    }
    #[doc = "DSI Host video VA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvaccr(pub u32);
    impl Vvaccr {
        #[doc = "Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_va(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vvaccr {
        #[inline(always)]
        fn default() -> Vvaccr {
            Vvaccr(0)
        }
    }
    #[doc = "DSI Host video VA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvacr(pub u32);
    impl Vvacr {
        #[doc = "Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn va(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Vertical active duration This fields configures the vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_va(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Vvacr {
        #[inline(always)]
        fn default() -> Vvacr {
            Vvacr(0)
        }
    }
    #[doc = "DSI Host video VBP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpccr(pub u32);
    impl Vvbpccr {
        #[doc = "Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvbpccr {
        #[inline(always)]
        fn default() -> Vvbpccr {
            Vvbpccr(0)
        }
    }
    #[doc = "DSI Host video VBP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvbpcr(pub u32);
    impl Vvbpcr {
        #[doc = "Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical back-porch duration This fields configures the vertical back-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvbpcr {
        #[inline(always)]
        fn default() -> Vvbpcr {
            Vvbpcr(0)
        }
    }
    #[doc = "DSI Host video VFP current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpccr(pub u32);
    impl Vvfpccr {
        #[doc = "Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vfp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvfpccr {
        #[inline(always)]
        fn default() -> Vvfpccr {
            Vvfpccr(0)
        }
    }
    #[doc = "DSI Host video VFP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvfpcr(pub u32);
    impl Vvfpcr {
        #[doc = "Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vfp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical front-porch duration This fields configures the vertical front-porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vfp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvfpcr {
        #[inline(always)]
        fn default() -> Vvfpcr {
            Vvfpcr(0)
        }
    }
    #[doc = "DSI Host video VSA current configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsaccr(pub u32);
    impl Vvsaccr {
        #[doc = "Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical synchronism active duration This field returns the current vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvsaccr {
        #[inline(always)]
        fn default() -> Vvsaccr {
            Vvsaccr(0)
        }
    }
    #[doc = "DSI Host video VSA configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vvsacr(pub u32);
    impl Vvsacr {
        #[doc = "Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vsa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical synchronism active duration This fields configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Vvsacr {
        #[inline(always)]
        fn default() -> Vvsacr {
            Vvsacr(0)
        }
    }
    #[doc = "DSI Wrapper configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcfgr(pub u32);
    impl Wcfgr {
        #[doc = "DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn dsim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DSI mode This bit selects the mode for the video transmission. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_dsim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub const fn colmux(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Color multiplexing This bit selects the color multiplexing used by DSI Host. This field must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub fn set_colmux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn tesrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TE source This bit selects the tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_tesrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn tepol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TE polarity This bit selects the polarity of the external pin tearing effect (TE) source. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_tepol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub const fn ar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic refresh This bit selects the refresh mode in DBI mode. This bit must only be changed when DSI Host is stopped (CR.EN = 0)."]
        #[inline(always)]
        pub fn set_ar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VSync polarity This bit selects the VSync edge on which the LTDC is halted. This bit must only be changed when DSI is stopped (WCR.DSIEN = 0 and CR.ENÂ =Â 0)."]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wcfgr {
        #[inline(always)]
        fn default() -> Wcfgr {
            Wcfgr(0)
        }
    }
    #[doc = "DSI Wrapper control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wcr(pub u32);
    impl Wcr {
        #[doc = "Color mode This bit controls the display color mode in video mode."]
        #[inline(always)]
        pub const fn colm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Color mode This bit controls the display color mode in video mode."]
        #[inline(always)]
        pub fn set_colm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Shutdown This bit controls the display shutdown in video mode."]
        #[inline(always)]
        pub const fn shtdn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Shutdown This bit controls the display shutdown in video mode."]
        #[inline(always)]
        pub fn set_shtdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC enable This bit enables the LTDC for a frame transfer in adapted command mode."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DSI enable This bit enables the DSI Wrapper."]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DSI enable This bit enables the DSI Wrapper."]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Wcr {
        #[inline(always)]
        fn default() -> Wcr {
            Wcr(0)
        }
    }
    #[doc = "DSI Wrapper interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wier(pub u32);
    impl Wier {
        #[doc = "Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect interrupt enable This bit enables the tearing effect interrupt."]
        #[inline(always)]
        pub fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of refresh interrupt enable This bit enables the end of refresh interrupt."]
        #[inline(always)]
        pub const fn erie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of refresh interrupt enable This bit enables the end of refresh interrupt."]
        #[inline(always)]
        pub fn set_erie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PLL lock interrupt enable This bit enables the PLL lock interrupt."]
        #[inline(always)]
        pub const fn plllie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock interrupt enable This bit enables the PLL lock interrupt."]
        #[inline(always)]
        pub fn set_plllie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
        #[inline(always)]
        pub const fn plluie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL unlock interrupt enable This bit enables the PLL unlock interrupt."]
        #[inline(always)]
        pub fn set_plluie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Wier {
        #[inline(always)]
        fn default() -> Wier {
            Wier(0)
        }
    }
    #[doc = "DSI Wrapper interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wifcr(pub u32);
    impl Wifcr {
        #[doc = "Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cteif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear tearing effect interrupt flag Write 1 clears the TEIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cteif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cerif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear end of refresh interrupt flag Write 1 clears the ERIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cerif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cplllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL lock interrupt flag Write 1 clears the PLLLIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cplllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the WSR register."]
        #[inline(always)]
        pub const fn cplluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PLL unlock interrupt flag Write 1 clears the PLLUIF flag in the WSR register."]
        #[inline(always)]
        pub fn set_cplluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Wifcr {
        #[inline(always)]
        fn default() -> Wifcr {
            Wifcr(0)
        }
    }
    #[doc = "DSI Wrapper interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wisr(pub u32);
    impl Wisr {
        #[doc = "Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
        #[inline(always)]
        pub const fn erif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
        #[inline(always)]
        pub fn set_erif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
        #[inline(always)]
        pub const fn pllls(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
        #[inline(always)]
        pub fn set_pllls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLL lock interrupt flag This bit is set when the PLL becomes locked."]
        #[inline(always)]
        pub const fn plllif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLL lock interrupt flag This bit is set when the PLL becomes locked."]
        #[inline(always)]
        pub fn set_plllif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
        #[inline(always)]
        pub const fn plluif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
        #[inline(always)]
        pub fn set_plluif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Wisr {
        #[inline(always)]
        fn default() -> Wisr {
            Wisr(0)
        }
    }
    #[doc = "DSI Wrapper PHY configuration register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpcr0(pub u32);
    impl Wpcr0 {
        #[doc = "Swap clock lane pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub const fn swcl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Swap clock lane pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub fn set_swcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Swap data lane 0 pins This bit swaps the pins on data lane 0."]
        #[inline(always)]
        pub const fn swdl0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Swap data lane 0 pins This bit swaps the pins on data lane 0."]
        #[inline(always)]
        pub fn set_swdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Swap data lane 1 pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub const fn swdl1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Swap data lane 1 pins This bit swaps the pins on clock lane."]
        #[inline(always)]
        pub fn set_swdl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub const fn ftxsmcl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub fn set_ftxsmcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub const fn ftxsmdl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence."]
        #[inline(always)]
        pub fn set_ftxsmdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Wpcr0 {
        #[inline(always)]
        fn default() -> Wpcr0 {
            Wpcr0(0)
        }
    }
    #[doc = "DSI Wrapper regulator and PLL control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpcr(pub u32);
    impl Wrpcr {
        #[doc = "PLL enable This bit enables the D-PHY PLL."]
        #[inline(always)]
        pub const fn pllen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable This bit enables the D-PHY PLL."]
        #[inline(always)]
        pub fn set_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2."]
        #[inline(always)]
        pub const fn ndiv(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL loop division factor This field configures the PLL loop division factor. 2: PLL loop divided by 2x2 ... 511: PLL loop divided by 511x2."]
        #[inline(always)]
        pub fn set_ndiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
        }
        #[doc = "PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511."]
        #[inline(always)]
        pub const fn idf(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL input division factor This field configures the PLL input division factor. 2: PLL input divided by 2 ... 511: PLL input divided by 511."]
        #[inline(always)]
        pub fn set_idf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
        }
        #[doc = "PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511."]
        #[inline(always)]
        pub const fn odf(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[doc = "PLL output division factor This field configures the PLL output division factor. 2: PLL output divided by 2 ... 511: PLL output divided by 511."]
        #[inline(always)]
        pub fn set_odf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
        }
    }
    impl Default for Wrpcr {
        #[inline(always)]
        fn default() -> Wrpcr {
            Wrpcr(0)
        }
    }
}
