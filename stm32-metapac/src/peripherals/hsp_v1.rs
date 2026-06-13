#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hardware signal processor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsp {
    ptr: *mut u8,
}
unsafe impl Send for Hsp {}
unsafe impl Sync for Hsp {}
impl Hsp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSP control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "HSP CPU to HSP semaphore register"]
    #[inline(always)]
    pub const fn c2hsemr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "HSP CPU to HSP message data register"]
    #[inline(always)]
    pub const fn c2hmsgdr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "HSP HSP to CPU semaphore register"]
    #[inline(always)]
    pub const fn h2csemr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "HSP HSP to CPU message data register"]
    #[inline(always)]
    pub const fn h2cmsgdr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "HSP DCMD command status register"]
    #[inline(always)]
    pub const fn dcmdsr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "HSP DCMD pointer status register"]
    #[inline(always)]
    pub const fn dcmdptsr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "HSP DCMD command ID register"]
    #[inline(always)]
    pub const fn dcmdidr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "HSP DCMD address pointer register"]
    #[inline(always)]
    pub const fn dcmdptr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "HSP event enable register"]
    #[inline(always)]
    pub const fn evtenr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "HSP event synchronization enable register"]
    #[inline(always)]
    pub const fn esyncenr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "HSP event synchronization source register"]
    #[inline(always)]
    pub const fn esyncsrcr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 4usize) as _) }
    }
    #[doc = "HSP pending event level register"]
    #[inline(always)]
    pub const fn pevtlr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "HSP interfaces enable register"]
    #[inline(always)]
    pub const fn itfenr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "HSP event source register"]
    #[inline(always)]
    pub const fn evtsrcr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize + n * 4usize) as _) }
    }
    #[doc = "HSP BUFF configuration register"]
    #[inline(always)]
    pub const fn buffcfgr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "HSP BUFF data register"]
    #[inline(always)]
    pub const fn buffdr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize + n * 4usize) as _) }
    }
    #[doc = "HSP TRGIN configuration register"]
    #[inline(always)]
    pub const fn trgincfgr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "HSP TRGO configuration register"]
    #[inline(always)]
    pub const fn trgocfgr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "HSP CPU shared event generator register"]
    #[inline(always)]
    pub const fn csegr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "HSP CPU dedicated event generation register"]
    #[inline(always)]
    pub const fn cdegr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "HSP TRGIN input selection register"]
    #[inline(always)]
    pub const fn trginselr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize + n * 4usize) as _) }
    }
    #[doc = "HSP break output configuration register"]
    #[inline(always)]
    pub const fn bkocfgr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize + n * 4usize) as _) }
    }
    #[doc = "HSP break input configuration register"]
    #[inline(always)]
    pub const fn bkicfgr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "HSP firmware error register"]
    #[inline(always)]
    pub const fn fwerr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "HSP parameter register"]
    #[inline(always)]
    pub const fn paramr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize + n * 4usize) as _) }
    }
    #[doc = "HSP SPE interrupt enable register"]
    #[inline(always)]
    pub const fn spe_ier(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "HSP SPE interrupt status register"]
    #[inline(always)]
    pub const fn spe_isr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "HSP task comparator unit register"]
    #[inline(always)]
    pub const fn tcucfgr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "HSP task overlap control register"]
    #[inline(always)]
    pub const fn tovlpcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "HSP snoop register"]
    #[inline(always)]
    pub const fn snpr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "HSP conflict counter register"]
    #[inline(always)]
    pub const fn ccntr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "HSP data capture register"]
    #[inline(always)]
    pub const fn capdr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "HSP capture control register"]
    #[inline(always)]
    pub const fn capcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "HSP error interrupt enable register"]
    #[inline(always)]
    pub const fn err_ier(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "HSP events interrupt enable register"]
    #[inline(always)]
    pub const fn evt_ier(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "HSP processing event interrupt enable register"]
    #[inline(always)]
    pub const fn pfctevt_ier(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "HSP error interrupt status register"]
    #[inline(always)]
    pub const fn err_isr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "HSP events interrupt status register"]
    #[inline(always)]
    pub const fn evt_isr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "HSP processing event interrupt status register"]
    #[inline(always)]
    pub const fn pfctevt_isr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "HSP error information register"]
    #[inline(always)]
    pub const fn errinfr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "HSP error interrupt clear register"]
    #[inline(always)]
    pub const fn err_icr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "HSP events interrupt clear register"]
    #[inline(always)]
    pub const fn evt_icr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "HSP processing event interrupt clear register"]
    #[inline(always)]
    pub const fn pfctevt_icr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "HSP firmware versions register"]
    #[inline(always)]
    pub const fn fwverr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
}
