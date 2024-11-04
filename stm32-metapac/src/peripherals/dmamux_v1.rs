#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DMAMUX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamux {
    ptr: *mut u8,
}
unsafe impl Send for Dmamux {}
unsafe impl Sync for Dmamux {}
impl Dmamux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn cfr(self) -> crate::common::Reg<regs::Csr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rgcr(self, n: usize) -> crate::common::Reg<regs::Rgcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "DMAMux - DMA request generator status register"]
    #[inline(always)]
    pub const fn rgsr(self) -> crate::common::Reg<regs::Rgsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "DMAMux - DMA request generator clear flag register"]
    #[inline(always)]
    pub const fn rgcfr(self) -> crate::common::Reg<regs::Rgsr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
}
pub mod regs {
    #[doc = "DMAMux - DMA request line multiplexer channel x control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Input DMA request line selected"]
        #[inline(always)]
        pub const fn dmareq_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Input DMA request line selected"]
        #[inline(always)]
        pub fn set_dmareq_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Interrupt enable at synchronization event overrun"]
        #[inline(always)]
        pub const fn soie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable at synchronization event overrun"]
        #[inline(always)]
        pub fn set_soie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Event generation enable/disable"]
        #[inline(always)]
        pub const fn ege(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Event generation enable/disable"]
        #[inline(always)]
        pub fn set_ege(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Synchronous operating mode enable/disable"]
        #[inline(always)]
        pub const fn se(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous operating mode enable/disable"]
        #[inline(always)]
        pub fn set_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
        #[inline(always)]
        pub const fn spol(&self) -> super::vals::Pol {
            let val = (self.0 >> 17usize) & 0x03;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
        #[inline(always)]
        pub fn set_spol(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
        }
        #[doc = "Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
        #[inline(always)]
        pub const fn nbreq(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
        #[inline(always)]
        pub fn set_nbreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "Synchronization input selected"]
        #[inline(always)]
        pub const fn sync_id(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Synchronization input selected"]
        #[inline(always)]
        pub fn set_sync_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "DMAMUX request line multiplexer interrupt channel status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Synchronization overrun event flag"]
        #[inline(always)]
        pub const fn sof(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Synchronization overrun event flag"]
        #[inline(always)]
        pub fn set_sof(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "DMAMux - DMA request generator channel x control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgcr(pub u32);
    impl Rgcr {
        #[doc = "DMA request trigger input selected"]
        #[inline(always)]
        pub const fn sig_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA request trigger input selected"]
        #[inline(always)]
        pub fn set_sig_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Interrupt enable at trigger event overrun"]
        #[inline(always)]
        pub const fn oie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable at trigger event overrun"]
        #[inline(always)]
        pub fn set_oie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA request generator channel enable/disable"]
        #[inline(always)]
        pub const fn ge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request generator channel enable/disable"]
        #[inline(always)]
        pub fn set_ge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
        #[inline(always)]
        pub const fn gpol(&self) -> super::vals::Pol {
            let val = (self.0 >> 17usize) & 0x03;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
        #[inline(always)]
        pub fn set_gpol(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
        }
        #[doc = "Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
        #[inline(always)]
        pub const fn gnbreq(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
        #[inline(always)]
        pub fn set_gnbreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
    }
    impl Default for Rgcr {
        #[inline(always)]
        fn default() -> Rgcr {
            Rgcr(0)
        }
    }
    #[doc = "DMAMux - DMA request generator status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgsr(pub u32);
    impl Rgsr {
        #[doc = "Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
        #[inline(always)]
        pub const fn of(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
        #[inline(always)]
        pub fn set_of(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rgsr {
        #[inline(always)]
        fn default() -> Rgsr {
            Rgsr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pol {
        #[doc = "No event, i.e. no synchronization nor detection"]
        NOEDGE = 0x0,
        #[doc = "Rising edge"]
        RISINGEDGE = 0x01,
        #[doc = "Falling edge"]
        FALLINGEDGE = 0x02,
        #[doc = "Rising and falling edges"]
        BOTHEDGES = 0x03,
    }
    impl Pol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pol {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pol {
        #[inline(always)]
        fn from(val: u8) -> Pol {
            Pol::from_bits(val)
        }
    }
    impl From<Pol> for u8 {
        #[inline(always)]
        fn from(val: Pol) -> u8 {
            Pol::to_bits(val)
        }
    }
}
