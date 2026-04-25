#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital camera interface pixel pipeline."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcmipp {
    ptr: *mut u8,
}
unsafe impl Send for Dcmipp {}
unsafe impl Sync for Dcmipp {}
impl Dcmipp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DCMIPP IPPLUG global register 1."]
    #[inline(always)]
    pub const fn ipgr1(self) -> crate::common::Reg<regs::Ipgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG global register 2."]
    #[inline(always)]
    pub const fn ipgr2(self) -> crate::common::Reg<regs::Ipgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG global register 3."]
    #[inline(always)]
    pub const fn ipgr3(self) -> crate::common::Reg<regs::Ipgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG identification register."]
    #[inline(always)]
    pub const fn ipgr8(self) -> crate::common::Reg<regs::Ipgr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[inline(always)]
    pub const fn ipc1r1(self) -> crate::common::Reg<regs::Ipc1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[inline(always)]
    pub const fn ipc1r2(self) -> crate::common::Reg<regs::Ipc1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[inline(always)]
    pub const fn ipc1r3(self) -> crate::common::Reg<regs::Ipc1r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[inline(always)]
    pub const fn ipc2r1(self) -> crate::common::Reg<regs::Ipc2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[inline(always)]
    pub const fn ipc2r2(self) -> crate::common::Reg<regs::Ipc2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[inline(always)]
    pub const fn ipc2r3(self) -> crate::common::Reg<regs::Ipc2r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[inline(always)]
    pub const fn ipc3r1(self) -> crate::common::Reg<regs::Ipc3r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[inline(always)]
    pub const fn ipc3r2(self) -> crate::common::Reg<regs::Ipc3r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[inline(always)]
    pub const fn ipc3r3(self) -> crate::common::Reg<regs::Ipc3r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[inline(always)]
    pub const fn ipc4r1(self) -> crate::common::Reg<regs::Ipc4r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[inline(always)]
    pub const fn ipc4r2(self) -> crate::common::Reg<regs::Ipc4r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[inline(always)]
    pub const fn ipc4r3(self) -> crate::common::Reg<regs::Ipc4r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[inline(always)]
    pub const fn ipc5r1(self) -> crate::common::Reg<regs::Ipc5r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[inline(always)]
    pub const fn ipc5r2(self) -> crate::common::Reg<regs::Ipc5r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[inline(always)]
    pub const fn ipc5r3(self) -> crate::common::Reg<regs::Ipc5r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "DCMIPP parallel interface control register."]
    #[inline(always)]
    pub const fn prcr(self) -> crate::common::Reg<regs::Prcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "DCMIPP parallel interface embedded synchronization code register."]
    #[inline(always)]
    pub const fn prescr(self) -> crate::common::Reg<regs::Prescr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "DCMIPP parallel interface embedded synchronization unmask register."]
    #[inline(always)]
    pub const fn presur(self) -> crate::common::Reg<regs::Presur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "DCMIPP parallel interface interrupt enable register."]
    #[inline(always)]
    pub const fn prier(self) -> crate::common::Reg<regs::Prier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "DCMIPP parallel interface status register."]
    #[inline(always)]
    pub const fn prsr(self) -> crate::common::Reg<regs::Prsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "DCMIPP parallel interface interrupt clear register."]
    #[inline(always)]
    pub const fn prfcr(self) -> crate::common::Reg<regs::Prfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "DCMIPP common configuration register."]
    #[inline(always)]
    pub const fn cmcr(self) -> crate::common::Reg<regs::Cmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "DCMIPP common frame counter register."]
    #[inline(always)]
    pub const fn cmfrcr(self) -> crate::common::Reg<regs::Cmfrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "DCMIPP common interrupt enable register."]
    #[inline(always)]
    pub const fn cmier(self) -> crate::common::Reg<regs::Cmier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "DCMIPP common status register 1."]
    #[inline(always)]
    pub const fn cmsr1(self) -> crate::common::Reg<regs::Cmsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "DCMIPP common status register 2."]
    #[inline(always)]
    pub const fn cmsr2(self) -> crate::common::Reg<regs::Cmsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "DCMIPP common interrupt clear register."]
    #[inline(always)]
    pub const fn cmfcr(self) -> crate::common::Reg<regs::Cmfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "DCMIPP Pipe0 flow selection configuration register."]
    #[inline(always)]
    pub const fn p0fscr(self) -> crate::common::Reg<regs::P0fscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 flow control configuration register."]
    #[inline(always)]
    pub const fn p0fctcr(self) -> crate::common::Reg<regs::P0fctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 stat/crop start register."]
    #[inline(always)]
    pub const fn p0scstr(self) -> crate::common::Reg<regs::P0scstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 stat/crop size register."]
    #[inline(always)]
    pub const fn p0scszr(self) -> crate::common::Reg<regs::P0scszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 dump counter register."]
    #[inline(always)]
    pub const fn p0dccntr(self) -> crate::common::Reg<regs::P0dccntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 dump limit register."]
    #[inline(always)]
    pub const fn p0dclmtr(self) -> crate::common::Reg<regs::P0dclmtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 pixel packer configuration register."]
    #[inline(always)]
    pub const fn p0ppcr(self) -> crate::common::Reg<regs::P0ppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p0ppm0ar1(self) -> crate::common::Reg<regs::P0ppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p0ppm0ar2(self) -> crate::common::Reg<regs::P0ppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 status Memory0 address register."]
    #[inline(always)]
    pub const fn p0stm0ar(self) -> crate::common::Reg<regs::P0stm0ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 interrupt enable register."]
    #[inline(always)]
    pub const fn p0ier(self) -> crate::common::Reg<regs::P0ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 status register."]
    #[inline(always)]
    pub const fn p0sr(self) -> crate::common::Reg<regs::P0sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 interrupt clear register."]
    #[inline(always)]
    pub const fn p0fcr(self) -> crate::common::Reg<regs::P0fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current flow selection configuration register."]
    #[inline(always)]
    pub const fn p0cfscr(self) -> crate::common::Reg<regs::P0cfscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current flow control configuration register."]
    #[inline(always)]
    pub const fn p0cfctcr(self) -> crate::common::Reg<regs::P0cfctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current stat/crop start register."]
    #[inline(always)]
    pub const fn p0cscstr(self) -> crate::common::Reg<regs::P0cscstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current stat/crop size register."]
    #[inline(always)]
    pub const fn p0cscszr(self) -> crate::common::Reg<regs::P0cscszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer configuration register."]
    #[inline(always)]
    pub const fn p0cppcr(self) -> crate::common::Reg<regs::P0cppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p0cppm0ar1(self) -> crate::common::Reg<regs::P0cppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p0cppm0ar2(self) -> crate::common::Reg<regs::P0cppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c8usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 flow selection configuration register."]
    #[inline(always)]
    pub const fn p1fscr(self) -> crate::common::Reg<regs::P1fscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 stat removal configuration register."]
    #[inline(always)]
    pub const fn p1srcr(self) -> crate::common::Reg<regs::P1srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 bad pixel removal control register."]
    #[inline(always)]
    pub const fn p1bprcr(self) -> crate::common::Reg<regs::P1bprcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 bad pixel removal status register."]
    #[inline(always)]
    pub const fn p1bprsr(self) -> crate::common::Reg<regs::P1bprsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 decimation register."]
    #[inline(always)]
    pub const fn p1decr(self) -> crate::common::Reg<regs::P1decr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0830usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 black level calibration control register."]
    #[inline(always)]
    pub const fn p1blccr(self) -> crate::common::Reg<regs::P1blccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 exposure control register 1."]
    #[inline(always)]
    pub const fn p1excr1(self) -> crate::common::Reg<regs::P1excr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 exposure control register 2."]
    #[inline(always)]
    pub const fn p1excr2(self) -> crate::common::Reg<regs::P1excr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics1 control register."]
    #[inline(always)]
    pub const fn p1st1cr(self) -> crate::common::Reg<regs::P1st1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics 2 control register."]
    #[inline(always)]
    pub const fn p1st2cr(self) -> crate::common::Reg<regs::P1st2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics 3 control register."]
    #[inline(always)]
    pub const fn p1st3cr(self) -> crate::common::Reg<regs::P1st3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics window start register."]
    #[inline(always)]
    pub const fn p1ststr(self) -> crate::common::Reg<regs::P1ststr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x085cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics window size register."]
    #[inline(always)]
    pub const fn p1stszr(self) -> crate::common::Reg<regs::P1stszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics 1 status register."]
    #[inline(always)]
    pub const fn p1st1sr(self) -> crate::common::Reg<regs::P1st1sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0864usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics 2 status register."]
    #[inline(always)]
    pub const fn p1st2sr(self) -> crate::common::Reg<regs::P1st2sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0868usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 statistics 3 status register."]
    #[inline(always)]
    pub const fn p1st3sr(self) -> crate::common::Reg<regs::P1st3sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x086cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 demosaicing configuration register."]
    #[inline(always)]
    pub const fn p1dmcr(self) -> crate::common::Reg<regs::P1dmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0870usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv configuration register."]
    #[inline(always)]
    pub const fn p1cccr(self) -> crate::common::Reg<regs::P1cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv red coefficient register 1."]
    #[inline(always)]
    pub const fn p1ccrr1(self) -> crate::common::Reg<regs::P1ccrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv red coefficient register 2."]
    #[inline(always)]
    pub const fn p1ccrr2(self) -> crate::common::Reg<regs::P1ccrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0888usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv green coefficient register 1."]
    #[inline(always)]
    pub const fn p1ccgr1(self) -> crate::common::Reg<regs::P1ccgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x088cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv green coefficient register 2."]
    #[inline(always)]
    pub const fn p1ccgr2(self) -> crate::common::Reg<regs::P1ccgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "DCMIPP Pipex ColorConv blue coefficient register 1."]
    #[inline(always)]
    pub const fn p1ccbr1(self) -> crate::common::Reg<regs::P1ccbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ColorConv blue coefficient register 2."]
    #[inline(always)]
    pub const fn p1ccbr2(self) -> crate::common::Reg<regs::P1ccbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0898usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 1."]
    #[inline(always)]
    pub const fn p1ctcr1(self) -> crate::common::Reg<regs::P1ctcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 2."]
    #[inline(always)]
    pub const fn p1ctcr2(self) -> crate::common::Reg<regs::P1ctcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a4usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 3."]
    #[inline(always)]
    pub const fn p1ctcr3(self) -> crate::common::Reg<regs::P1ctcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a8usize) as _) }
    }
    #[doc = "DCMIPP Pipex flow control configuration register."]
    #[inline(always)]
    pub const fn p1fctcr(self) -> crate::common::Reg<regs::P1fctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "DCMIPP Pipex crop window start register."]
    #[inline(always)]
    pub const fn p1crstr(self) -> crate::common::Reg<regs::P1crstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "DCMIPP Pipex crop window size register."]
    #[inline(always)]
    pub const fn p1crszr(self) -> crate::common::Reg<regs::P1crszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0908usize) as _) }
    }
    #[doc = "DCMIPP Pipex decimation register."]
    #[inline(always)]
    pub const fn p1dccr(self) -> crate::common::Reg<regs::P1dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x090cusize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize configuration register."]
    #[inline(always)]
    pub const fn p1dscr(self) -> crate::common::Reg<regs::P1dscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize ratio register."]
    #[inline(always)]
    pub const fn p1dsrtior(self) -> crate::common::Reg<regs::P1dsrtior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0914usize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize destination size register."]
    #[inline(always)]
    pub const fn p1dsszr(self) -> crate::common::Reg<regs::P1dsszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[doc = "DCMIPP Pipex common ROI configuration register."]
    #[inline(always)]
    pub const fn p1cmricr(self) -> crate::common::Reg<regs::P1cmricr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI1 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri1cr1(self) -> crate::common::Reg<regs::P1ri1cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0924usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI1 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri1cr2(self) -> crate::common::Reg<regs::P1ri1cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0928usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI2 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri2cr1(self) -> crate::common::Reg<regs::P1ri2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x092cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI2 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri2cr2(self) -> crate::common::Reg<regs::P1ri2cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI3 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri3cr1(self) -> crate::common::Reg<regs::P1ri3cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI3 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri3cr2(self) -> crate::common::Reg<regs::P1ri3cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0938usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI4 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri4cr1(self) -> crate::common::Reg<regs::P1ri4cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x093cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI4 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri4cr2(self) -> crate::common::Reg<regs::P1ri4cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI5 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri5cr1(self) -> crate::common::Reg<regs::P1ri5cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI5 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri5cr2(self) -> crate::common::Reg<regs::P1ri5cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0948usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI6 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri6cr1(self) -> crate::common::Reg<regs::P1ri6cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x094cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI6 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri6cr2(self) -> crate::common::Reg<regs::P1ri6cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI7 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri7cr1(self) -> crate::common::Reg<regs::P1ri7cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0954usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI7 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri7cr2(self) -> crate::common::Reg<regs::P1ri7cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0958usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI8 configuration register 1."]
    #[inline(always)]
    pub const fn p1ri8cr1(self) -> crate::common::Reg<regs::P1ri8cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x095cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 ROI8 configuration register 2."]
    #[inline(always)]
    pub const fn p1ri8cr2(self) -> crate::common::Reg<regs::P1ri8cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "DCMIPP Pipex gamma configuration register."]
    #[inline(always)]
    pub const fn p1gmcr(self) -> crate::common::Reg<regs::P1gmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0970usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv configuration register."]
    #[inline(always)]
    pub const fn p1yuvcr(self) -> crate::common::Reg<regs::P1yuvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv red coefficient register 1."]
    #[inline(always)]
    pub const fn p1yuvrr1(self) -> crate::common::Reg<regs::P1yuvrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0984usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv red coefficient register 2."]
    #[inline(always)]
    pub const fn p1yuvrr2(self) -> crate::common::Reg<regs::P1yuvrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0988usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv green coefficient register 1."]
    #[inline(always)]
    pub const fn p1yuvgr1(self) -> crate::common::Reg<regs::P1yuvgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x098cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv green coefficient register 2."]
    #[inline(always)]
    pub const fn p1yuvgr2(self) -> crate::common::Reg<regs::P1yuvgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0990usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUVConv blue coefficient register 1."]
    #[inline(always)]
    pub const fn p1yuvbr1(self) -> crate::common::Reg<regs::P1yuvbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0994usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 YUV blue coefficient register 2."]
    #[inline(always)]
    pub const fn p1yuvbr2(self) -> crate::common::Reg<regs::P1yuvbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0998usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 pixel packer configuration register."]
    #[inline(always)]
    pub const fn p1ppcr(self) -> crate::common::Reg<regs::P1ppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p1ppm0ar1(self) -> crate::common::Reg<regs::P1ppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c4usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p1ppm0ar2(self) -> crate::common::Reg<regs::P1ppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c8usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory0 pitch register."]
    #[inline(always)]
    pub const fn p1ppm0pr(self) -> crate::common::Reg<regs::P1ppm0pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09ccusize) as _) }
    }
    #[doc = "DCMIPP Pipex status Memory0 address register."]
    #[inline(always)]
    pub const fn p1stm0ar(self) -> crate::common::Reg<regs::P1stm0ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 address register 1."]
    #[inline(always)]
    pub const fn p1ppm1ar1(self) -> crate::common::Reg<regs::P1ppm1ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d4usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 address register 2."]
    #[inline(always)]
    pub const fn p1ppm1ar2(self) -> crate::common::Reg<regs::P1ppm1ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d8usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 pitch register."]
    #[inline(always)]
    pub const fn p1ppm1pr(self) -> crate::common::Reg<regs::P1ppm1pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09dcusize) as _) }
    }
    #[doc = "DCMIPP Pipex status Memory1 address register."]
    #[inline(always)]
    pub const fn p1stm1ar(self) -> crate::common::Reg<regs::P1stm1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer memory2 address register 1."]
    #[inline(always)]
    pub const fn p1ppm2ar1(self) -> crate::common::Reg<regs::P1ppm2ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e4usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer memory2 address register 2."]
    #[inline(always)]
    pub const fn p1ppm2ar2(self) -> crate::common::Reg<regs::P1ppm2ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e8usize) as _) }
    }
    #[doc = "DCMIPP Pipex status Memory2 address register."]
    #[inline(always)]
    pub const fn p1stm2ar(self) -> crate::common::Reg<regs::P1stm2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 interrupt enable register."]
    #[inline(always)]
    pub const fn p1ier(self) -> crate::common::Reg<regs::P1ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f4usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 status register."]
    #[inline(always)]
    pub const fn p1sr(self) -> crate::common::Reg<regs::P1sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 interrupt clear register."]
    #[inline(always)]
    pub const fn p1fcr(self) -> crate::common::Reg<regs::P1fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09fcusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current flow selection configuration register."]
    #[inline(always)]
    pub const fn p1cfscr(self) -> crate::common::Reg<regs::P1cfscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a04usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current bad pixel removal register."]
    #[inline(always)]
    pub const fn p1cbprcr(self) -> crate::common::Reg<regs::P1cbprcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a24usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current black level calibration control register."]
    #[inline(always)]
    pub const fn p1cblccr(self) -> crate::common::Reg<regs::P1cblccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a40usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current exposure control register 1."]
    #[inline(always)]
    pub const fn p1cexcr1(self) -> crate::common::Reg<regs::P1cexcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a44usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current exposure control register 2."]
    #[inline(always)]
    pub const fn p1cexcr2(self) -> crate::common::Reg<regs::P1cexcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a48usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current statistics 1 control register."]
    #[inline(always)]
    pub const fn p1cst1cr(self) -> crate::common::Reg<regs::P1cst1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a50usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current statistics 2 control register."]
    #[inline(always)]
    pub const fn p1cst2cr(self) -> crate::common::Reg<regs::P1cst2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a54usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current statistics 3 control register."]
    #[inline(always)]
    pub const fn p1cst3cr(self) -> crate::common::Reg<regs::P1cst3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a58usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current statistics window start register."]
    #[inline(always)]
    pub const fn p1cststr(self) -> crate::common::Reg<regs::P1cststr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a5cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current statistics window size register."]
    #[inline(always)]
    pub const fn p1cstszr(self) -> crate::common::Reg<regs::P1cstszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a60usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv configuration register."]
    #[inline(always)]
    pub const fn p1ccccr(self) -> crate::common::Reg<regs::P1ccccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a80usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv red coefficient register 1."]
    #[inline(always)]
    pub const fn p1cccrr1(self) -> crate::common::Reg<regs::P1cccrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a84usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv red coefficient register 2."]
    #[inline(always)]
    pub const fn p1cccrr2(self) -> crate::common::Reg<regs::P1cccrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a88usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv green coefficient register 1."]
    #[inline(always)]
    pub const fn p1cccgr1(self) -> crate::common::Reg<regs::P1cccgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a8cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv green coefficient register 2."]
    #[inline(always)]
    pub const fn p1cccgr2(self) -> crate::common::Reg<regs::P1cccgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a90usize) as _) }
    }
    #[doc = "DCMIPP Pipex current ColorConv blue coefficient register 1."]
    #[inline(always)]
    pub const fn p1cccbr1(self) -> crate::common::Reg<regs::P1cccbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a94usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv blue coefficient register 2."]
    #[inline(always)]
    pub const fn p1cccbr2(self) -> crate::common::Reg<regs::P1cccbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a98usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 1."]
    #[inline(always)]
    pub const fn p1cctcr1(self) -> crate::common::Reg<regs::P1cctcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa0usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 2."]
    #[inline(always)]
    pub const fn p1cctcr2(self) -> crate::common::Reg<regs::P1cctcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa4usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 3."]
    #[inline(always)]
    pub const fn p1cctcr3(self) -> crate::common::Reg<regs::P1cctcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa8usize) as _) }
    }
    #[doc = "DCMIPP Pipex current flow control configuration register."]
    #[inline(always)]
    pub const fn p1cfctcr(self) -> crate::common::Reg<regs::P1cfctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "DCMIPP Pipex current crop window start register."]
    #[inline(always)]
    pub const fn p1ccrstr(self) -> crate::common::Reg<regs::P1ccrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "DCMIPP Pipex current crop window size register."]
    #[inline(always)]
    pub const fn p1ccrszr(self) -> crate::common::Reg<regs::P1ccrszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "DCMIPP Pipex current decimation register."]
    #[inline(always)]
    pub const fn p1cdccr(self) -> crate::common::Reg<regs::P1cdccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize configuration register."]
    #[inline(always)]
    pub const fn p1cdscr(self) -> crate::common::Reg<regs::P1cdscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize ratio register."]
    #[inline(always)]
    pub const fn p1cdsrtior(self) -> crate::common::Reg<regs::P1cdsrtior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize destination size register."]
    #[inline(always)]
    pub const fn p1cdsszr(self) -> crate::common::Reg<regs::P1cdsszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "DCMIPP Pipex current common ROI configuration register."]
    #[inline(always)]
    pub const fn p1ccmricr(self) -> crate::common::Reg<regs::P1ccmricr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI1 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri1cr1(self) -> crate::common::Reg<regs::P1cri1cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b24usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI1 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri1cr2(self) -> crate::common::Reg<regs::P1cri1cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b28usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI2 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri2cr1(self) -> crate::common::Reg<regs::P1cri2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b2cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI2 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri2cr2(self) -> crate::common::Reg<regs::P1cri2cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI3 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri3cr1(self) -> crate::common::Reg<regs::P1cri3cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI3 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri3cr2(self) -> crate::common::Reg<regs::P1cri3cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b38usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI4 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri4cr1(self) -> crate::common::Reg<regs::P1cri4cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b3cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI4 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri4cr2(self) -> crate::common::Reg<regs::P1cri4cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI5 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri5cr1(self) -> crate::common::Reg<regs::P1cri5cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b44usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI5 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri5cr2(self) -> crate::common::Reg<regs::P1cri5cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b48usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI6 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri6cr1(self) -> crate::common::Reg<regs::P1cri6cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b4cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI6 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri6cr2(self) -> crate::common::Reg<regs::P1cri6cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI7 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri7cr1(self) -> crate::common::Reg<regs::P1cri7cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI7 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri7cr2(self) -> crate::common::Reg<regs::P1cri7cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI8 configuration register 1."]
    #[inline(always)]
    pub const fn p1cri8cr1(self) -> crate::common::Reg<regs::P1cri8cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current ROI8 configuration register 2."]
    #[inline(always)]
    pub const fn p1cri8cr2(self) -> crate::common::Reg<regs::P1cri8cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer configuration register."]
    #[inline(always)]
    pub const fn p1cppcr(self) -> crate::common::Reg<regs::P1cppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc0usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p1cppm0ar1(self) -> crate::common::Reg<regs::P1cppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc4usize) as _) }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p1cppm0ar2(self) -> crate::common::Reg<regs::P1cppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bc8usize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory0 pitch register."]
    #[inline(always)]
    pub const fn p1cppm0pr(self) -> crate::common::Reg<regs::P1cppm0pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bccusize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 address register 1."]
    #[inline(always)]
    pub const fn p1cppm1ar1(self) -> crate::common::Reg<regs::P1cppm1ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd4usize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 address register 2."]
    #[inline(always)]
    pub const fn p1cppm1ar2(self) -> crate::common::Reg<regs::P1cppm1ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bd8usize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 pitch register."]
    #[inline(always)]
    pub const fn p1cppm1pr(self) -> crate::common::Reg<regs::P1cppm1pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bdcusize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory2 address register 1."]
    #[inline(always)]
    pub const fn p1cppm2ar1(self) -> crate::common::Reg<regs::P1cppm2ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0be4usize) as _) }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory2 address register 1."]
    #[inline(always)]
    pub const fn p1cppm2ar2(self) -> crate::common::Reg<regs::P1cppm2ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0be8usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 flow selection configuration register."]
    #[inline(always)]
    pub const fn p2fscr(self) -> crate::common::Reg<regs::P2fscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "DCMIPP Pipex flow control configuration register."]
    #[inline(always)]
    pub const fn p2fctcr(self) -> crate::common::Reg<regs::P2fctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[doc = "DCMIPP Pipex crop window start register."]
    #[inline(always)]
    pub const fn p2crstr(self) -> crate::common::Reg<regs::P2crstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[doc = "DCMIPP Pipex crop window size register."]
    #[inline(always)]
    pub const fn p2crszr(self) -> crate::common::Reg<regs::P2crszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[doc = "DCMIPP Pipex decimation register."]
    #[inline(always)]
    pub const fn p2dccr(self) -> crate::common::Reg<regs::P2dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d0cusize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize configuration register."]
    #[inline(always)]
    pub const fn p2dscr(self) -> crate::common::Reg<regs::P2dscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d10usize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize ratio register."]
    #[inline(always)]
    pub const fn p2dsrtior(self) -> crate::common::Reg<regs::P2dsrtior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d14usize) as _) }
    }
    #[doc = "DCMIPP Pipex downsize destination size register."]
    #[inline(always)]
    pub const fn p2dsszr(self) -> crate::common::Reg<regs::P2dsszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d18usize) as _) }
    }
    #[doc = "DCMIPP Pipex common ROI configuration register."]
    #[inline(always)]
    pub const fn p2cmricr(self) -> crate::common::Reg<regs::P2cmricr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d20usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI1 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri1cr1(self) -> crate::common::Reg<regs::P2ri1cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d24usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI1 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri1cr2(self) -> crate::common::Reg<regs::P2ri1cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d28usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI2 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri2cr1(self) -> crate::common::Reg<regs::P2ri2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d2cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI2 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri2cr2(self) -> crate::common::Reg<regs::P2ri2cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI3 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri3cr1(self) -> crate::common::Reg<regs::P2ri3cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d34usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI3 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri3cr2(self) -> crate::common::Reg<regs::P2ri3cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d38usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI4 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri4cr1(self) -> crate::common::Reg<regs::P2ri4cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d3cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI4 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri4cr2(self) -> crate::common::Reg<regs::P2ri4cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d40usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI5 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri5cr1(self) -> crate::common::Reg<regs::P2ri5cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d44usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI5 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri5cr2(self) -> crate::common::Reg<regs::P2ri5cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d48usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI6 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri6cr1(self) -> crate::common::Reg<regs::P2ri6cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d4cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI6 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri6cr2(self) -> crate::common::Reg<regs::P2ri6cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d50usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI7 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri7cr1(self) -> crate::common::Reg<regs::P2ri7cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d54usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI7 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri7cr2(self) -> crate::common::Reg<regs::P2ri7cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d58usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI8 configuration register 1."]
    #[inline(always)]
    pub const fn p2ri8cr1(self) -> crate::common::Reg<regs::P2ri8cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d5cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 ROI8 configuration register 2."]
    #[inline(always)]
    pub const fn p2ri8cr2(self) -> crate::common::Reg<regs::P2ri8cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d60usize) as _) }
    }
    #[doc = "DCMIPP Pipex gamma configuration register."]
    #[inline(always)]
    pub const fn p2gmcr(self) -> crate::common::Reg<regs::P2gmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d70usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 pixel packer configuration register."]
    #[inline(always)]
    pub const fn p2ppcr(self) -> crate::common::Reg<regs::P2ppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dc0usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p2ppm0ar1(self) -> crate::common::Reg<regs::P2ppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dc4usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p2ppm0ar2(self) -> crate::common::Reg<regs::P2ppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dc8usize) as _) }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory0 pitch register."]
    #[inline(always)]
    pub const fn p2ppm0pr(self) -> crate::common::Reg<regs::P2ppm0pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dccusize) as _) }
    }
    #[doc = "DCMIPP Pipex status Memory0 address register."]
    #[inline(always)]
    pub const fn p2stm0ar(self) -> crate::common::Reg<regs::P2stm0ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dd0usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 interrupt enable register."]
    #[inline(always)]
    pub const fn p2ier(self) -> crate::common::Reg<regs::P2ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0df4usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 status register."]
    #[inline(always)]
    pub const fn p2sr(self) -> crate::common::Reg<regs::P2sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0df8usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 interrupt clear register."]
    #[inline(always)]
    pub const fn p2fcr(self) -> crate::common::Reg<regs::P2fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dfcusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current flow selection configuration register."]
    #[inline(always)]
    pub const fn p2cfscr(self) -> crate::common::Reg<regs::P2cfscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "DCMIPP Pipex current flow control configuration register."]
    #[inline(always)]
    pub const fn p2cfctcr(self) -> crate::common::Reg<regs::P2cfctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "DCMIPP Pipex current crop window start register."]
    #[inline(always)]
    pub const fn p2ccrstr(self) -> crate::common::Reg<regs::P2ccrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f04usize) as _) }
    }
    #[doc = "DCMIPP Pipex current crop window size register."]
    #[inline(always)]
    pub const fn p2ccrszr(self) -> crate::common::Reg<regs::P2ccrszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f08usize) as _) }
    }
    #[doc = "DCMIPP Pipex current decimation register."]
    #[inline(always)]
    pub const fn p2cdccr(self) -> crate::common::Reg<regs::P2cdccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f0cusize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize configuration register."]
    #[inline(always)]
    pub const fn p2cdscr(self) -> crate::common::Reg<regs::P2cdscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f10usize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize ratio register."]
    #[inline(always)]
    pub const fn p2cdsrtior(self) -> crate::common::Reg<regs::P2cdsrtior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f14usize) as _) }
    }
    #[doc = "DCMIPP Pipex current downsize destination size register."]
    #[inline(always)]
    pub const fn p2cdsszr(self) -> crate::common::Reg<regs::P2cdsszr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f18usize) as _) }
    }
    #[doc = "DCMIPP Pipex current common ROI configuration register."]
    #[inline(always)]
    pub const fn p2ccmricr(self) -> crate::common::Reg<regs::P2ccmricr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f20usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI1 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri1cr1(self) -> crate::common::Reg<regs::P2cri1cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f24usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI1 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri1cr2(self) -> crate::common::Reg<regs::P2cri1cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f28usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI2 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri2cr1(self) -> crate::common::Reg<regs::P2cri2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f2cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI2 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri2cr2(self) -> crate::common::Reg<regs::P2cri2cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f30usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI3 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri3cr1(self) -> crate::common::Reg<regs::P2cri3cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f34usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI3 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri3cr2(self) -> crate::common::Reg<regs::P2cri3cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f38usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI4 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri4cr1(self) -> crate::common::Reg<regs::P2cri4cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f3cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI4 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri4cr2(self) -> crate::common::Reg<regs::P2cri4cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f40usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI5 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri5cr1(self) -> crate::common::Reg<regs::P2cri5cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f44usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI5 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri5cr2(self) -> crate::common::Reg<regs::P2cri5cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f48usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI6 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri6cr1(self) -> crate::common::Reg<regs::P2cri6cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f4cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI6 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri6cr2(self) -> crate::common::Reg<regs::P2cri6cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f50usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI7 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri7cr1(self) -> crate::common::Reg<regs::P2cri7cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f54usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI7 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri7cr2(self) -> crate::common::Reg<regs::P2cri7cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f58usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI8 configuration register 1."]
    #[inline(always)]
    pub const fn p2cri8cr1(self) -> crate::common::Reg<regs::P2cri8cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f5cusize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current ROI8 configuration register 2."]
    #[inline(always)]
    pub const fn p2cri8cr2(self) -> crate::common::Reg<regs::P2cri8cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f60usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer configuration register."]
    #[inline(always)]
    pub const fn p2cppcr(self) -> crate::common::Reg<regs::P2cppcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer Memory0 address register 1."]
    #[inline(always)]
    pub const fn p2cppm0ar1(self) -> crate::common::Reg<regs::P2cppm0ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer Memory0 address register 2."]
    #[inline(always)]
    pub const fn p2cppm0ar2(self) -> crate::common::Reg<regs::P2cppm0ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc8usize) as _) }
    }
}
pub mod regs {
    #[doc = "DCMIPP common configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmcr(pub u32);
    impl Cmcr {
        #[doc = "input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn insel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "input selection."]
        #[inline(always)]
        pub const fn set_insel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Pipe selection for the frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn psfc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Pipe selection for the frame counter."]
        #[inline(always)]
        pub const fn set_psfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Clear frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cfc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear frame counter."]
        #[inline(always)]
        pub const fn set_cfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Swap R/U and B/V."]
        #[must_use]
        #[inline(always)]
        pub const fn swaprb(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Swap R/U and B/V."]
        #[inline(always)]
        pub const fn set_swaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cmcr {
        #[inline(always)]
        fn default() -> Cmcr {
            Cmcr(0)
        }
    }
    impl core::fmt::Debug for Cmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmcr")
                .field("insel", &self.insel())
                .field("psfc", &self.psfc())
                .field("cfc", &self.cfc())
                .field("swaprb", &self.swaprb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmcr {{ insel: {=bool:?}, psfc: {=u8:?}, cfc: {=bool:?}, swaprb: {=bool:?} }}",
                self.insel(),
                self.psfc(),
                self.cfc(),
                self.swaprb()
            )
        }
    }
    #[doc = "DCMIPP common interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmfcr(pub u32);
    impl Cmfcr {
        #[doc = "AXI transfer error interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn catxerrf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI transfer error interrupt status clear."]
        #[inline(always)]
        pub const fn set_catxerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Synchronization error interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cprerrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error interrupt status clear."]
        #[inline(always)]
        pub const fn set_cprerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0linef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp0linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0framef(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp0framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0vsyncf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp0vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "limit interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0limitf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "limit interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp0limitf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0ovrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp0ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp1linef(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp1linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp1framef(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp1framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp1vsyncf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp1vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp1ovrf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp1ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp2linef(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp2linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp2framef(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp2framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp2vsyncf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp2vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cp2ovrf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_cp2ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cmfcr {
        #[inline(always)]
        fn default() -> Cmfcr {
            Cmfcr(0)
        }
    }
    impl core::fmt::Debug for Cmfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmfcr")
                .field("catxerrf", &self.catxerrf())
                .field("cprerrf", &self.cprerrf())
                .field("cp0linef", &self.cp0linef())
                .field("cp0framef", &self.cp0framef())
                .field("cp0vsyncf", &self.cp0vsyncf())
                .field("cp0limitf", &self.cp0limitf())
                .field("cp0ovrf", &self.cp0ovrf())
                .field("cp1linef", &self.cp1linef())
                .field("cp1framef", &self.cp1framef())
                .field("cp1vsyncf", &self.cp1vsyncf())
                .field("cp1ovrf", &self.cp1ovrf())
                .field("cp2linef", &self.cp2linef())
                .field("cp2framef", &self.cp2framef())
                .field("cp2vsyncf", &self.cp2vsyncf())
                .field("cp2ovrf", &self.cp2ovrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmfcr {{ catxerrf: {=bool:?}, cprerrf: {=bool:?}, cp0linef: {=bool:?}, cp0framef: {=bool:?}, cp0vsyncf: {=bool:?}, cp0limitf: {=bool:?}, cp0ovrf: {=bool:?}, cp1linef: {=bool:?}, cp1framef: {=bool:?}, cp1vsyncf: {=bool:?}, cp1ovrf: {=bool:?}, cp2linef: {=bool:?}, cp2framef: {=bool:?}, cp2vsyncf: {=bool:?}, cp2ovrf: {=bool:?} }}" , self . catxerrf () , self . cprerrf () , self . cp0linef () , self . cp0framef () , self . cp0vsyncf () , self . cp0limitf () , self . cp0ovrf () , self . cp1linef () , self . cp1framef () , self . cp1vsyncf () , self . cp1ovrf () , self . cp2linef () , self . cp2framef () , self . cp2vsyncf () , self . cp2ovrf ())
        }
    }
    #[doc = "DCMIPP common frame counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmfrcr(pub u32);
    impl Cmfrcr {
        #[doc = "Frame counter, read-only, loops around."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame counter, read-only, loops around."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmfrcr {
        #[inline(always)]
        fn default() -> Cmfrcr {
            Cmfrcr(0)
        }
    }
    impl core::fmt::Debug for Cmfrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmfrcr").field("frmcnt", &self.frmcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmfrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmfrcr {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "DCMIPP common interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmier(pub u32);
    impl Cmier {
        #[doc = "AXI transfer error interrupt enable for IPPLUG."]
        #[must_use]
        #[inline(always)]
        pub const fn atxerrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI transfer error interrupt enable for IPPLUG."]
        #[inline(always)]
        pub const fn set_atxerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Limit interrupt enable for the parallel Interface."]
        #[must_use]
        #[inline(always)]
        pub const fn prerrie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Limit interrupt enable for the parallel Interface."]
        #[inline(always)]
        pub const fn set_prerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Multi-line capture complete interrupt enable for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0lineie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt enable for Pipe0."]
        #[inline(always)]
        pub const fn set_p0lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Frame capture complete interrupt enable for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0frameie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt enable for Pipe0."]
        #[inline(always)]
        pub const fn set_p0frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Vertical sync interrupt enable for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0vsyncie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical sync interrupt enable for Pipe0."]
        #[inline(always)]
        pub const fn set_p0vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Limit interrupt enable for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0limitie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Limit interrupt enable for Pipe0."]
        #[inline(always)]
        pub const fn set_p0limitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Overrun interrupt enable for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0ovrie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable for Pipe0."]
        #[inline(always)]
        pub const fn set_p0ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Multi-line capture complete interrupt status clear for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1lineie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear for Pipe1."]
        #[inline(always)]
        pub const fn set_p1lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frame capture complete interrupt enable for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1frameie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt enable for Pipe1."]
        #[inline(always)]
        pub const fn set_p1frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Vertical sync interrupt enable for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1vsyncie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical sync interrupt enable for Pipe1."]
        #[inline(always)]
        pub const fn set_p1vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Overrun interrupt enable for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1ovrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable for Pipe1."]
        #[inline(always)]
        pub const fn set_p1ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Multi-line capture complete interrupt enable for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2lineie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt enable for Pipe2."]
        #[inline(always)]
        pub const fn set_p2lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Frame capture complete interrupt enable for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2frameie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt enable for Pipe2."]
        #[inline(always)]
        pub const fn set_p2frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Vertical sync interrupt enable for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2vsyncie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical sync interrupt enable for Pipe2."]
        #[inline(always)]
        pub const fn set_p2vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Overrun interrupt status enable for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2ovrie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status enable for Pipe2."]
        #[inline(always)]
        pub const fn set_p2ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cmier {
        #[inline(always)]
        fn default() -> Cmier {
            Cmier(0)
        }
    }
    impl core::fmt::Debug for Cmier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmier")
                .field("atxerrie", &self.atxerrie())
                .field("prerrie", &self.prerrie())
                .field("p0lineie", &self.p0lineie())
                .field("p0frameie", &self.p0frameie())
                .field("p0vsyncie", &self.p0vsyncie())
                .field("p0limitie", &self.p0limitie())
                .field("p0ovrie", &self.p0ovrie())
                .field("p1lineie", &self.p1lineie())
                .field("p1frameie", &self.p1frameie())
                .field("p1vsyncie", &self.p1vsyncie())
                .field("p1ovrie", &self.p1ovrie())
                .field("p2lineie", &self.p2lineie())
                .field("p2frameie", &self.p2frameie())
                .field("p2vsyncie", &self.p2vsyncie())
                .field("p2ovrie", &self.p2ovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmier {{ atxerrie: {=bool:?}, prerrie: {=bool:?}, p0lineie: {=bool:?}, p0frameie: {=bool:?}, p0vsyncie: {=bool:?}, p0limitie: {=bool:?}, p0ovrie: {=bool:?}, p1lineie: {=bool:?}, p1frameie: {=bool:?}, p1vsyncie: {=bool:?}, p1ovrie: {=bool:?}, p2lineie: {=bool:?}, p2frameie: {=bool:?}, p2vsyncie: {=bool:?}, p2ovrie: {=bool:?} }}" , self . atxerrie () , self . prerrie () , self . p0lineie () , self . p0frameie () , self . p0vsyncie () , self . p0limitie () , self . p0ovrie () , self . p1lineie () , self . p1frameie () , self . p1vsyncie () , self . p1ovrie () , self . p2lineie () , self . p2frameie () , self . p2vsyncie () , self . p2ovrie ())
        }
    }
    #[doc = "DCMIPP common status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmsr1(pub u32);
    impl Cmsr1 {
        #[doc = "This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise."]
        #[must_use]
        #[inline(always)]
        pub const fn prhsync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise."]
        #[inline(always)]
        pub const fn set_prhsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise."]
        #[must_use]
        #[inline(always)]
        pub const fn prvsync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise."]
        #[inline(always)]
        pub const fn set_prvsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Last line LSB bit, sampled at Frame capture complete event for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0lstline(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at Frame capture complete event for Pipe0."]
        #[inline(always)]
        pub const fn set_p0lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Last frame LSB bit, sampled at Frame capture complete event for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0lstfrm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at Frame capture complete event for Pipe0."]
        #[inline(always)]
        pub const fn set_p0lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0cptact(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe0."]
        #[inline(always)]
        pub const fn set_p0cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Last line LSB bit, sampled at Frame capture complete event for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1lstline(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at Frame capture complete event for Pipe1."]
        #[inline(always)]
        pub const fn set_p1lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1lstfrm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event for Pipe1."]
        #[inline(always)]
        pub const fn set_p1lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1cptact(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe1."]
        #[inline(always)]
        pub const fn set_p1cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2lstline(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event for Pipe2."]
        #[inline(always)]
        pub const fn set_p2lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2lstfrm(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event for Pipe2."]
        #[inline(always)]
        pub const fn set_p2lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2cptact(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Active frame capture (active from start-of-frame to frame complete) for Pipe2."]
        #[inline(always)]
        pub const fn set_p2cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cmsr1 {
        #[inline(always)]
        fn default() -> Cmsr1 {
            Cmsr1(0)
        }
    }
    impl core::fmt::Debug for Cmsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmsr1")
                .field("prhsync", &self.prhsync())
                .field("prvsync", &self.prvsync())
                .field("p0lstline", &self.p0lstline())
                .field("p0lstfrm", &self.p0lstfrm())
                .field("p0cptact", &self.p0cptact())
                .field("p1lstline", &self.p1lstline())
                .field("p1lstfrm", &self.p1lstfrm())
                .field("p1cptact", &self.p1cptact())
                .field("p2lstline", &self.p2lstline())
                .field("p2lstfrm", &self.p2lstfrm())
                .field("p2cptact", &self.p2cptact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmsr1 {{ prhsync: {=bool:?}, prvsync: {=bool:?}, p0lstline: {=bool:?}, p0lstfrm: {=bool:?}, p0cptact: {=bool:?}, p1lstline: {=bool:?}, p1lstfrm: {=bool:?}, p1cptact: {=bool:?}, p2lstline: {=bool:?}, p2lstfrm: {=bool:?}, p2cptact: {=bool:?} }}" , self . prhsync () , self . prvsync () , self . p0lstline () , self . p0lstfrm () , self . p0cptact () , self . p1lstline () , self . p1lstfrm () , self . p1cptact () , self . p2lstline () , self . p2lstfrm () , self . p2cptact ())
        }
    }
    #[doc = "DCMIPP common status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmsr2(pub u32);
    impl Cmsr2 {
        #[doc = "AXI transfer error interrupt status flag for the IPPLUG."]
        #[must_use]
        #[inline(always)]
        pub const fn atxerrf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI transfer error interrupt status flag for the IPPLUG."]
        #[inline(always)]
        pub const fn set_atxerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Synchronization error raw interrupt status for the parallel interface."]
        #[must_use]
        #[inline(always)]
        pub const fn prerrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error raw interrupt status for the parallel interface."]
        #[inline(always)]
        pub const fn set_prerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0linef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe0."]
        #[inline(always)]
        pub const fn set_p0linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0framef(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe0."]
        #[inline(always)]
        pub const fn set_p0framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "VSYNC raw interrupt status for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0vsyncf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status for Pipe0."]
        #[inline(always)]
        pub const fn set_p0vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Limit raw interrupt status for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0limitf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Limit raw interrupt status for Pipe0."]
        #[inline(always)]
        pub const fn set_p0limitf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Overrun raw interrupt status for Pipe0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0ovrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status for Pipe0."]
        #[inline(always)]
        pub const fn set_p0ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1linef(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe1."]
        #[inline(always)]
        pub const fn set_p1linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1framef(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe1."]
        #[inline(always)]
        pub const fn set_p1framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "VSYNC raw interrupt status for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1vsyncf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status for Pipe1."]
        #[inline(always)]
        pub const fn set_p1vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Overrun raw interrupt status for Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1ovrf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status for Pipe1."]
        #[inline(always)]
        pub const fn set_p1ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2linef(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status for Pipe2."]
        #[inline(always)]
        pub const fn set_p2linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2framef(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status for Pipe2."]
        #[inline(always)]
        pub const fn set_p2framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "VSYNC raw interrupt status for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2vsyncf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status for Pipe2."]
        #[inline(always)]
        pub const fn set_p2vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Overrun raw interrupt status for Pipe2."]
        #[must_use]
        #[inline(always)]
        pub const fn p2ovrf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status for Pipe2."]
        #[inline(always)]
        pub const fn set_p2ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cmsr2 {
        #[inline(always)]
        fn default() -> Cmsr2 {
            Cmsr2(0)
        }
    }
    impl core::fmt::Debug for Cmsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmsr2")
                .field("atxerrf", &self.atxerrf())
                .field("prerrf", &self.prerrf())
                .field("p0linef", &self.p0linef())
                .field("p0framef", &self.p0framef())
                .field("p0vsyncf", &self.p0vsyncf())
                .field("p0limitf", &self.p0limitf())
                .field("p0ovrf", &self.p0ovrf())
                .field("p1linef", &self.p1linef())
                .field("p1framef", &self.p1framef())
                .field("p1vsyncf", &self.p1vsyncf())
                .field("p1ovrf", &self.p1ovrf())
                .field("p2linef", &self.p2linef())
                .field("p2framef", &self.p2framef())
                .field("p2vsyncf", &self.p2vsyncf())
                .field("p2ovrf", &self.p2ovrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cmsr2 {{ atxerrf: {=bool:?}, prerrf: {=bool:?}, p0linef: {=bool:?}, p0framef: {=bool:?}, p0vsyncf: {=bool:?}, p0limitf: {=bool:?}, p0ovrf: {=bool:?}, p1linef: {=bool:?}, p1framef: {=bool:?}, p1vsyncf: {=bool:?}, p1ovrf: {=bool:?}, p2linef: {=bool:?}, p2framef: {=bool:?}, p2vsyncf: {=bool:?}, p2ovrf: {=bool:?} }}" , self . atxerrf () , self . prerrf () , self . p0linef () , self . p0framef () , self . p0vsyncf () , self . p0limitf () , self . p0ovrf () , self . p1linef () , self . p1framef () , self . p1vsyncf () , self . p1ovrf () , self . p2linef () , self . p2framef () , self . p2vsyncf () , self . p2ovrf ())
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc1r1(pub u32);
    impl Ipc1r1 {
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[must_use]
        #[inline(always)]
        pub const fn traffic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[inline(always)]
        pub const fn set_traffic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Maximum outstanding transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn otr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Maximum outstanding transactions."]
        #[inline(always)]
        pub const fn set_otr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Ipc1r1 {
        #[inline(always)]
        fn default() -> Ipc1r1 {
            Ipc1r1(0)
        }
    }
    impl core::fmt::Debug for Ipc1r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc1r1")
                .field("traffic", &self.traffic())
                .field("otr", &self.otr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc1r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc1r1 {{ traffic: {=u8:?}, otr: {=u8:?} }}",
                self.traffic(),
                self.otr()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc1r2(pub u32);
    impl Ipc1r2 {
        #[doc = "Non-user, must be kept at reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn svcmapping(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Non-user, must be kept at reset value."]
        #[inline(always)]
        pub const fn set_svcmapping(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn wlru(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[inline(always)]
        pub const fn set_wlru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ipc1r2 {
        #[inline(always)]
        fn default() -> Ipc1r2 {
            Ipc1r2(0)
        }
    }
    impl core::fmt::Debug for Ipc1r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc1r2")
                .field("svcmapping", &self.svcmapping())
                .field("wlru", &self.wlru())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc1r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc1r2 {{ svcmapping: {=u8:?}, wlru: {=u8:?} }}",
                self.svcmapping(),
                self.wlru()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc1r3(pub u32);
    impl Ipc1r3 {
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregend(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregend(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Ipc1r3 {
        #[inline(always)]
        fn default() -> Ipc1r3 {
            Ipc1r3(0)
        }
    }
    impl core::fmt::Debug for Ipc1r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc1r3")
                .field("dpregstart", &self.dpregstart())
                .field("dpregend", &self.dpregend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc1r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc1r3 {{ dpregstart: {=u16:?}, dpregend: {=u16:?} }}",
                self.dpregstart(),
                self.dpregend()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc2r1(pub u32);
    impl Ipc2r1 {
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[must_use]
        #[inline(always)]
        pub const fn traffic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[inline(always)]
        pub const fn set_traffic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Maximum outstanding transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn otr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Maximum outstanding transactions."]
        #[inline(always)]
        pub const fn set_otr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Ipc2r1 {
        #[inline(always)]
        fn default() -> Ipc2r1 {
            Ipc2r1(0)
        }
    }
    impl core::fmt::Debug for Ipc2r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc2r1")
                .field("traffic", &self.traffic())
                .field("otr", &self.otr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc2r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc2r1 {{ traffic: {=u8:?}, otr: {=u8:?} }}",
                self.traffic(),
                self.otr()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc2r2(pub u32);
    impl Ipc2r2 {
        #[doc = "Non-user, must be kept at reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn svcmapping(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Non-user, must be kept at reset value."]
        #[inline(always)]
        pub const fn set_svcmapping(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn wlru(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[inline(always)]
        pub const fn set_wlru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ipc2r2 {
        #[inline(always)]
        fn default() -> Ipc2r2 {
            Ipc2r2(0)
        }
    }
    impl core::fmt::Debug for Ipc2r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc2r2")
                .field("svcmapping", &self.svcmapping())
                .field("wlru", &self.wlru())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc2r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc2r2 {{ svcmapping: {=u8:?}, wlru: {=u8:?} }}",
                self.svcmapping(),
                self.wlru()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc2r3(pub u32);
    impl Ipc2r3 {
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregend(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregend(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Ipc2r3 {
        #[inline(always)]
        fn default() -> Ipc2r3 {
            Ipc2r3(0)
        }
    }
    impl core::fmt::Debug for Ipc2r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc2r3")
                .field("dpregstart", &self.dpregstart())
                .field("dpregend", &self.dpregend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc2r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc2r3 {{ dpregstart: {=u16:?}, dpregend: {=u16:?} }}",
                self.dpregstart(),
                self.dpregend()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc3r1(pub u32);
    impl Ipc3r1 {
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[must_use]
        #[inline(always)]
        pub const fn traffic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[inline(always)]
        pub const fn set_traffic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Maximum outstanding transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn otr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Maximum outstanding transactions."]
        #[inline(always)]
        pub const fn set_otr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Ipc3r1 {
        #[inline(always)]
        fn default() -> Ipc3r1 {
            Ipc3r1(0)
        }
    }
    impl core::fmt::Debug for Ipc3r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc3r1")
                .field("traffic", &self.traffic())
                .field("otr", &self.otr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc3r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc3r1 {{ traffic: {=u8:?}, otr: {=u8:?} }}",
                self.traffic(),
                self.otr()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc3r2(pub u32);
    impl Ipc3r2 {
        #[doc = "Non-user, must be kept at reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn svcmapping(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Non-user, must be kept at reset value."]
        #[inline(always)]
        pub const fn set_svcmapping(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn wlru(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[inline(always)]
        pub const fn set_wlru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ipc3r2 {
        #[inline(always)]
        fn default() -> Ipc3r2 {
            Ipc3r2(0)
        }
    }
    impl core::fmt::Debug for Ipc3r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc3r2")
                .field("svcmapping", &self.svcmapping())
                .field("wlru", &self.wlru())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc3r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc3r2 {{ svcmapping: {=u8:?}, wlru: {=u8:?} }}",
                self.svcmapping(),
                self.wlru()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc3r3(pub u32);
    impl Ipc3r3 {
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregend(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregend(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Ipc3r3 {
        #[inline(always)]
        fn default() -> Ipc3r3 {
            Ipc3r3(0)
        }
    }
    impl core::fmt::Debug for Ipc3r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc3r3")
                .field("dpregstart", &self.dpregstart())
                .field("dpregend", &self.dpregend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc3r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc3r3 {{ dpregstart: {=u16:?}, dpregend: {=u16:?} }}",
                self.dpregstart(),
                self.dpregend()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc4r1(pub u32);
    impl Ipc4r1 {
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[must_use]
        #[inline(always)]
        pub const fn traffic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[inline(always)]
        pub const fn set_traffic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Maximum outstanding transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn otr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Maximum outstanding transactions."]
        #[inline(always)]
        pub const fn set_otr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Ipc4r1 {
        #[inline(always)]
        fn default() -> Ipc4r1 {
            Ipc4r1(0)
        }
    }
    impl core::fmt::Debug for Ipc4r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc4r1")
                .field("traffic", &self.traffic())
                .field("otr", &self.otr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc4r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc4r1 {{ traffic: {=u8:?}, otr: {=u8:?} }}",
                self.traffic(),
                self.otr()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc4r2(pub u32);
    impl Ipc4r2 {
        #[doc = "Non-user, must be kept at reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn svcmapping(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Non-user, must be kept at reset value."]
        #[inline(always)]
        pub const fn set_svcmapping(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn wlru(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[inline(always)]
        pub const fn set_wlru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ipc4r2 {
        #[inline(always)]
        fn default() -> Ipc4r2 {
            Ipc4r2(0)
        }
    }
    impl core::fmt::Debug for Ipc4r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc4r2")
                .field("svcmapping", &self.svcmapping())
                .field("wlru", &self.wlru())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc4r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc4r2 {{ svcmapping: {=u8:?}, wlru: {=u8:?} }}",
                self.svcmapping(),
                self.wlru()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc4r3(pub u32);
    impl Ipc4r3 {
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregend(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregend(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Ipc4r3 {
        #[inline(always)]
        fn default() -> Ipc4r3 {
            Ipc4r3(0)
        }
    }
    impl core::fmt::Debug for Ipc4r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc4r3")
                .field("dpregstart", &self.dpregstart())
                .field("dpregend", &self.dpregend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc4r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc4r3 {{ dpregstart: {=u16:?}, dpregend: {=u16:?} }}",
                self.dpregstart(),
                self.dpregend()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc5r1(pub u32);
    impl Ipc5r1 {
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[must_use]
        #[inline(always)]
        pub const fn traffic(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst size as power of 2 of 8-byte units."]
        #[inline(always)]
        pub const fn set_traffic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Maximum outstanding transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn otr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Maximum outstanding transactions."]
        #[inline(always)]
        pub const fn set_otr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Ipc5r1 {
        #[inline(always)]
        fn default() -> Ipc5r1 {
            Ipc5r1(0)
        }
    }
    impl core::fmt::Debug for Ipc5r1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc5r1")
                .field("traffic", &self.traffic())
                .field("otr", &self.otr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc5r1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc5r1 {{ traffic: {=u8:?}, otr: {=u8:?} }}",
                self.traffic(),
                self.otr()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc5r2(pub u32);
    impl Ipc5r2 {
        #[doc = "Non-user, must be kept at reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn svcmapping(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Non-user, must be kept at reset value."]
        #[inline(always)]
        pub const fn set_svcmapping(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[must_use]
        #[inline(always)]
        pub const fn wlru(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Ratio for WLRU\\[3:0\\]
arbitration."]
        #[inline(always)]
        pub const fn set_wlru(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Ipc5r2 {
        #[inline(always)]
        fn default() -> Ipc5r2 {
            Ipc5r2(0)
        }
    }
    impl core::fmt::Debug for Ipc5r2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc5r2")
                .field("svcmapping", &self.svcmapping())
                .field("wlru", &self.wlru())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc5r2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc5r2 {{ svcmapping: {=u8:?}, wlru: {=u8:?} }}",
                self.svcmapping(),
                self.wlru()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG Clientx register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipc5r3(pub u32);
    impl Ipc5r3 {
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Start word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[must_use]
        #[inline(always)]
        pub const fn dpregend(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "End word (AXI width = 64 bits) of the FIFO of Clientx."]
        #[inline(always)]
        pub const fn set_dpregend(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Ipc5r3 {
        #[inline(always)]
        fn default() -> Ipc5r3 {
            Ipc5r3(0)
        }
    }
    impl core::fmt::Debug for Ipc5r3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipc5r3")
                .field("dpregstart", &self.dpregstart())
                .field("dpregend", &self.dpregend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipc5r3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipc5r3 {{ dpregstart: {=u16:?}, dpregend: {=u16:?} }}",
                self.dpregstart(),
                self.dpregend()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG global register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipgr1(pub u32);
    impl Ipgr1 {
        #[doc = "Memory page size, as power of 2 of 64-byte units:."]
        #[must_use]
        #[inline(always)]
        pub const fn memorypage(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Memory page size, as power of 2 of 64-byte units:."]
        #[inline(always)]
        pub const fn set_memorypage(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Quality of service."]
        #[must_use]
        #[inline(always)]
        pub const fn qos_mode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Quality of service."]
        #[inline(always)]
        pub const fn set_qos_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ipgr1 {
        #[inline(always)]
        fn default() -> Ipgr1 {
            Ipgr1(0)
        }
    }
    impl core::fmt::Debug for Ipgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipgr1")
                .field("memorypage", &self.memorypage())
                .field("qos_mode", &self.qos_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipgr1 {{ memorypage: {=u8:?}, qos_mode: {=bool:?} }}",
                self.memorypage(),
                self.qos_mode()
            )
        }
    }
    #[doc = "DCMIPP IPPLUG global register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipgr2(pub u32);
    impl Ipgr2 {
        #[doc = "Request to lock the IP-Plug, to allow reconfiguration."]
        #[must_use]
        #[inline(always)]
        pub const fn pstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Request to lock the IP-Plug, to allow reconfiguration."]
        #[inline(always)]
        pub const fn set_pstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ipgr2 {
        #[inline(always)]
        fn default() -> Ipgr2 {
            Ipgr2(0)
        }
    }
    impl core::fmt::Debug for Ipgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipgr2").field("pstart", &self.pstart()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipgr2 {{ pstart: {=bool:?} }}", self.pstart())
        }
    }
    #[doc = "DCMIPP IPPLUG global register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipgr3(pub u32);
    impl Ipgr3 {
        #[doc = "Status of IP-Plug."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Status of IP-Plug."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ipgr3 {
        #[inline(always)]
        fn default() -> Ipgr3 {
            Ipgr3(0)
        }
    }
    impl core::fmt::Debug for Ipgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipgr3").field("idle", &self.idle()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipgr3 {{ idle: {=bool:?} }}", self.idle())
        }
    }
    #[doc = "DCMIPP IPPLUG identification register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipgr8(pub u32);
    impl Ipgr8 {
        #[doc = "Division identifier (0x14)."]
        #[must_use]
        #[inline(always)]
        pub const fn did(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Division identifier (0x14)."]
        #[inline(always)]
        pub const fn set_did(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Revision identifier (0x03)."]
        #[must_use]
        #[inline(always)]
        pub const fn revid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Revision identifier (0x03)."]
        #[inline(always)]
        pub const fn set_revid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Architecture identifier (0x04)."]
        #[must_use]
        #[inline(always)]
        pub const fn archiid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Architecture identifier (0x04)."]
        #[inline(always)]
        pub const fn set_archiid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "IP identifier (0xAA)."]
        #[must_use]
        #[inline(always)]
        pub const fn ippid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IP identifier (0xAA)."]
        #[inline(always)]
        pub const fn set_ippid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ipgr8 {
        #[inline(always)]
        fn default() -> Ipgr8 {
            Ipgr8(0)
        }
    }
    impl core::fmt::Debug for Ipgr8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipgr8")
                .field("did", &self.did())
                .field("revid", &self.revid())
                .field("archiid", &self.archiid())
                .field("ippid", &self.ippid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipgr8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipgr8 {{ did: {=u8:?}, revid: {=u8:?}, archiid: {=u8:?}, ippid: {=u8:?} }}",
                self.did(),
                self.revid(),
                self.archiid(),
                self.ippid()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 current flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cfctcr(pub u32);
    impl P0cfctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P0cfctcr {
        #[inline(always)]
        fn default() -> P0cfctcr {
            P0cfctcr(0)
        }
    }
    impl core::fmt::Debug for P0cfctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cfctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cfctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0cfctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 current flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cfscr(pub u32);
    impl P0cfscr {
        #[doc = "Current data type selection ID A."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Current data type selection ID A."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Current data type selection ID B."]
        #[must_use]
        #[inline(always)]
        pub const fn dtidb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Current data type selection ID B."]
        #[inline(always)]
        pub const fn set_dtidb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Current flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Current flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Current activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P0cfscr {
        #[inline(always)]
        fn default() -> P0cfscr {
            P0cfscr(0)
        }
    }
    impl core::fmt::Debug for P0cfscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cfscr")
                .field("dtida", &self.dtida())
                .field("dtidb", &self.dtidb())
                .field("dtmode", &self.dtmode())
                .field("vc", &self.vc())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cfscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0cfscr {{ dtida: {=u8:?}, dtidb: {=u8:?}, dtmode: {=u8:?}, vc: {=u8:?}, pipen: {=bool:?} }}",
                self.dtida(),
                self.dtidb(),
                self.dtmode(),
                self.vc(),
                self.pipen()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cppcr(pub u32);
    impl P0cppcr {
        #[doc = "Swaps, within a 32-bit word, byte 0 vs. 1 and byte 2 vs. 3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV."]
        #[must_use]
        #[inline(always)]
        pub const fn swapyuv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps, within a 32-bit word, byte 0 vs. 1 and byte 2 vs. 3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV."]
        #[inline(always)]
        pub const fn set_swapyuv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment."]
        #[must_use]
        #[inline(always)]
        pub const fn pad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Current Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment."]
        #[inline(always)]
        pub const fn set_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Current CSI header dump enable."]
        #[must_use]
        #[inline(always)]
        pub const fn headeren(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Current CSI header dump enable."]
        #[inline(always)]
        pub const fn set_headeren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Current Byte select mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bsm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Current Byte select mode."]
        #[inline(always)]
        pub const fn set_bsm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Current odd/even byte select (byte select start)."]
        #[must_use]
        #[inline(always)]
        pub const fn oebs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Current odd/even byte select (byte select start)."]
        #[inline(always)]
        pub const fn set_oebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Current Line select mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lsm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Current Line select mode."]
        #[inline(always)]
        pub const fn set_lsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Current odd/even line select (ine select start)."]
        #[must_use]
        #[inline(always)]
        pub const fn oels(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Current odd/even line select (ine select start)."]
        #[inline(always)]
        pub const fn set_oels(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Current amount of capture completed lines for LINE event and interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Current amount of capture completed lines for LINE event and interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for P0cppcr {
        #[inline(always)]
        fn default() -> P0cppcr {
            P0cppcr(0)
        }
    }
    impl core::fmt::Debug for P0cppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cppcr")
                .field("swapyuv", &self.swapyuv())
                .field("pad", &self.pad())
                .field("headeren", &self.headeren())
                .field("bsm", &self.bsm())
                .field("oebs", &self.oebs())
                .field("lsm", &self.lsm())
                .field("oels", &self.oels())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P0cppcr {{ swapyuv: {=bool:?}, pad: {=bool:?}, headeren: {=bool:?}, bsm: {=u8:?}, oebs: {=bool:?}, lsm: {=bool:?}, oels: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?} }}" , self . swapyuv () , self . pad () , self . headeren () , self . bsm () , self . oebs () , self . lsm () , self . oels () , self . linemult () , self . dbm ())
        }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cppm0ar1(pub u32);
    impl P0cppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P0cppm0ar1 {
        #[inline(always)]
        fn default() -> P0cppm0ar1 {
            P0cppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P0cppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0cppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe0 current pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cppm0ar2(pub u32);
    impl P0cppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P0cppm0ar2 {
        #[inline(always)]
        fn default() -> P0cppm0ar2 {
            P0cppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P0cppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0cppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe0 current stat/crop start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cscstr(pub u32);
    impl P0cscstr {
        #[doc = "Current horizontal start, from 0 to 4094 words wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 words wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P0cscstr {
        #[inline(always)]
        fn default() -> P0cscstr {
            P0cscstr(0)
        }
    }
    impl core::fmt::Debug for P0cscstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cscstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cscstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0cscstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 current stat/crop size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0cscszr(pub u32);
    impl P0cscszr {
        #[doc = "Current horizontal size, from 0 to 4094 word wide (data 32-bit)."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 word wide (data 32-bit)."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current value of the POSNEG bit."]
        #[must_use]
        #[inline(always)]
        pub const fn posneg(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Current value of the POSNEG bit."]
        #[inline(always)]
        pub const fn set_posneg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Current value of the ENABLE bit."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current value of the ENABLE bit."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P0cscszr {
        #[inline(always)]
        fn default() -> P0cscszr {
            P0cscszr(0)
        }
    }
    impl core::fmt::Debug for P0cscszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0cscszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("posneg", &self.posneg())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0cscszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0cscszr {{ hsize: {=u16:?}, vsize: {=u16:?}, posneg: {=bool:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.posneg(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 dump counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0dccntr(pub u32);
    impl P0dccntr {
        #[doc = "Number of data dumped during the frame."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Number of data dumped during the frame."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for P0dccntr {
        #[inline(always)]
        fn default() -> P0dccntr {
            P0dccntr(0)
        }
    }
    impl core::fmt::Debug for P0dccntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0dccntr").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0dccntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0dccntr {{ cnt: {=u32:?} }}", self.cnt())
        }
    }
    #[doc = "DCMIPP Pipe0 dump limit register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0dclmtr(pub u32);
    impl P0dclmtr {
        #[doc = "Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation."]
        #[must_use]
        #[inline(always)]
        pub const fn limit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation."]
        #[inline(always)]
        pub const fn set_limit(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P0dclmtr {
        #[inline(always)]
        fn default() -> P0dclmtr {
            P0dclmtr(0)
        }
    }
    impl core::fmt::Debug for P0dclmtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0dclmtr")
                .field("limit", &self.limit())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0dclmtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0dclmtr {{ limit: {=u32:?}, enable: {=bool:?} }}",
                self.limit(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0fcr(pub u32);
    impl P0fcr {
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clinef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_clinef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cframef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cframef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cvsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "limit interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn climitf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "limit interrupt status clear."]
        #[inline(always)]
        pub const fn set_climitf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn covrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_covrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P0fcr {
        #[inline(always)]
        fn default() -> P0fcr {
            P0fcr(0)
        }
    }
    impl core::fmt::Debug for P0fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0fcr")
                .field("clinef", &self.clinef())
                .field("cframef", &self.cframef())
                .field("cvsyncf", &self.cvsyncf())
                .field("climitf", &self.climitf())
                .field("covrf", &self.covrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P0fcr {{ clinef: {=bool:?}, cframef: {=bool:?}, cvsyncf: {=bool:?}, climitf: {=bool:?}, covrf: {=bool:?} }}" , self . clinef () , self . cframef () , self . cvsyncf () , self . climitf () , self . covrf ())
        }
    }
    #[doc = "DCMIPP Pipe0 flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0fctcr(pub u32);
    impl P0fctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P0fctcr {
        #[inline(always)]
        fn default() -> P0fctcr {
            P0fctcr(0)
        }
    }
    impl core::fmt::Debug for P0fctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0fctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0fctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0fctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0fscr(pub u32);
    impl P0fscr {
        #[doc = "Data type selection ID A."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type selection ID A."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type selection ID B."]
        #[must_use]
        #[inline(always)]
        pub const fn dtidb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type selection ID B."]
        #[inline(always)]
        pub const fn set_dtidb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P0fscr {
        #[inline(always)]
        fn default() -> P0fscr {
            P0fscr(0)
        }
    }
    impl core::fmt::Debug for P0fscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0fscr")
                .field("dtida", &self.dtida())
                .field("dtidb", &self.dtidb())
                .field("dtmode", &self.dtmode())
                .field("vc", &self.vc())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0fscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0fscr {{ dtida: {=u8:?}, dtidb: {=u8:?}, dtmode: {=u8:?}, vc: {=u8:?}, pipen: {=bool:?} }}",
                self.dtida(),
                self.dtidb(),
                self.dtmode(),
                self.vc(),
                self.pipen()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0ier(pub u32);
    impl P0ier {
        #[doc = "Multi-line capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lineie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frameie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC interrupt enable."]
        #[inline(always)]
        pub const fn set_vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Limit interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn limitie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Limit interrupt enable."]
        #[inline(always)]
        pub const fn set_limitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Overrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable."]
        #[inline(always)]
        pub const fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P0ier {
        #[inline(always)]
        fn default() -> P0ier {
            P0ier(0)
        }
    }
    impl core::fmt::Debug for P0ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0ier")
                .field("lineie", &self.lineie())
                .field("frameie", &self.frameie())
                .field("vsyncie", &self.vsyncie())
                .field("limitie", &self.limitie())
                .field("ovrie", &self.ovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P0ier {{ lineie: {=bool:?}, frameie: {=bool:?}, vsyncie: {=bool:?}, limitie: {=bool:?}, ovrie: {=bool:?} }}" , self . lineie () , self . frameie () , self . vsyncie () , self . limitie () , self . ovrie ())
        }
    }
    #[doc = "DCMIPP Pipe0 pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0ppcr(pub u32);
    impl P0ppcr {
        #[doc = "Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV."]
        #[must_use]
        #[inline(always)]
        pub const fn swapyuv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV."]
        #[inline(always)]
        pub const fn set_swapyuv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment."]
        #[must_use]
        #[inline(always)]
        pub const fn pad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment."]
        #[inline(always)]
        pub const fn set_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI header dump enable."]
        #[must_use]
        #[inline(always)]
        pub const fn headeren(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CSI header dump enable."]
        #[inline(always)]
        pub const fn set_headeren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Byte select mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bsm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Byte select mode."]
        #[inline(always)]
        pub const fn set_bsm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Odd/even byte select (byte select start)."]
        #[must_use]
        #[inline(always)]
        pub const fn oebs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Odd/even byte select (byte select start)."]
        #[inline(always)]
        pub const fn set_oebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Line select mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lsm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Line select mode."]
        #[inline(always)]
        pub const fn set_lsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Odd/even line select (line select start)."]
        #[must_use]
        #[inline(always)]
        pub const fn oels(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Odd/even line select (line select start)."]
        #[inline(always)]
        pub const fn set_oels(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for P0ppcr {
        #[inline(always)]
        fn default() -> P0ppcr {
            P0ppcr(0)
        }
    }
    impl core::fmt::Debug for P0ppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0ppcr")
                .field("swapyuv", &self.swapyuv())
                .field("pad", &self.pad())
                .field("headeren", &self.headeren())
                .field("bsm", &self.bsm())
                .field("oebs", &self.oebs())
                .field("lsm", &self.lsm())
                .field("oels", &self.oels())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0ppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P0ppcr {{ swapyuv: {=bool:?}, pad: {=bool:?}, headeren: {=bool:?}, bsm: {=u8:?}, oebs: {=bool:?}, lsm: {=bool:?}, oels: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?} }}" , self . swapyuv () , self . pad () , self . headeren () , self . bsm () , self . oebs () , self . lsm () , self . oels () , self . linemult () , self . dbm ())
        }
    }
    #[doc = "DCMIPP Pipe0 pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0ppm0ar1(pub u32);
    impl P0ppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P0ppm0ar1 {
        #[inline(always)]
        fn default() -> P0ppm0ar1 {
            P0ppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P0ppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0ppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0ppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0ppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe0 pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0ppm0ar2(pub u32);
    impl P0ppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P0ppm0ar2 {
        #[inline(always)]
        fn default() -> P0ppm0ar2 {
            P0ppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P0ppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0ppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0ppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0ppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe0 stat/crop start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0scstr(pub u32);
    impl P0scstr {
        #[doc = "Horizontal start, from 0 to 4094 words wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 words wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P0scstr {
        #[inline(always)]
        fn default() -> P0scstr {
            P0scstr(0)
        }
    }
    impl core::fmt::Debug for P0scstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0scstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0scstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0scstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 stat/crop size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0scszr(pub u32);
    impl P0scszr {
        #[doc = "Horizontal size, from 0 to 4094 word wide (data 32-bit)."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 word wide (data 32-bit)."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "This bit is set and cleared by software. It has a meaning only if ENABLE bit is set."]
        #[must_use]
        #[inline(always)]
        pub const fn posneg(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set and cleared by software. It has a meaning only if ENABLE bit is set."]
        #[inline(always)]
        pub const fn set_posneg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "This bit is set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P0scszr {
        #[inline(always)]
        fn default() -> P0scszr {
            P0scszr(0)
        }
    }
    impl core::fmt::Debug for P0scszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0scszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("posneg", &self.posneg())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0scszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P0scszr {{ hsize: {=u16:?}, vsize: {=u16:?}, posneg: {=bool:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.posneg(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipe0 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0sr(pub u32);
    impl P0sr {
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn linef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn framef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status."]
        #[inline(always)]
        pub const fn set_vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Limit raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn limitf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Limit raw interrupt status."]
        #[inline(always)]
        pub const fn set_limitf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Overrun raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status."]
        #[inline(always)]
        pub const fn set_ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[must_use]
        #[inline(always)]
        pub const fn lstline(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[inline(always)]
        pub const fn set_lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number that can be delivered by the camera through the CSI2 interface."]
        #[must_use]
        #[inline(always)]
        pub const fn lstfrm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number that can be delivered by the camera through the CSI2 interface."]
        #[inline(always)]
        pub const fn set_lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Capture immediate status."]
        #[must_use]
        #[inline(always)]
        pub const fn cptact(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Capture immediate status."]
        #[inline(always)]
        pub const fn set_cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P0sr {
        #[inline(always)]
        fn default() -> P0sr {
            P0sr(0)
        }
    }
    impl core::fmt::Debug for P0sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0sr")
                .field("linef", &self.linef())
                .field("framef", &self.framef())
                .field("vsyncf", &self.vsyncf())
                .field("limitf", &self.limitf())
                .field("ovrf", &self.ovrf())
                .field("lstline", &self.lstline())
                .field("lstfrm", &self.lstfrm())
                .field("cptact", &self.cptact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P0sr {{ linef: {=bool:?}, framef: {=bool:?}, vsyncf: {=bool:?}, limitf: {=bool:?}, ovrf: {=bool:?}, lstline: {=bool:?}, lstfrm: {=bool:?}, cptact: {=bool:?} }}" , self . linef () , self . framef () , self . vsyncf () , self . limitf () , self . ovrf () , self . lstline () , self . lstfrm () , self . cptact ())
        }
    }
    #[doc = "DCMIPP Pipe0 status Memory0 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P0stm0ar(pub u32);
    impl P0stm0ar {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P0stm0ar {
        #[inline(always)]
        fn default() -> P0stm0ar {
            P0stm0ar(0)
        }
    }
    impl core::fmt::Debug for P0stm0ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P0stm0ar").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P0stm0ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P0stm0ar {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe1 black level calibration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1blccr(pub u32);
    impl P1blccr {
        #[doc = "Black level calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Black level calibration."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Black level calibration - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn blcb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Black level calibration - Blue."]
        #[inline(always)]
        pub const fn set_blcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Black level calibration - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn blcg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Black level calibration - Green."]
        #[inline(always)]
        pub const fn set_blcg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Black level calibration - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn blcr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Black level calibration - Red."]
        #[inline(always)]
        pub const fn set_blcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for P1blccr {
        #[inline(always)]
        fn default() -> P1blccr {
            P1blccr(0)
        }
    }
    impl core::fmt::Debug for P1blccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1blccr")
                .field("enable", &self.enable())
                .field("blcb", &self.blcb())
                .field("blcg", &self.blcg())
                .field("blcr", &self.blcr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1blccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1blccr {{ enable: {=bool:?}, blcb: {=u8:?}, blcg: {=u8:?}, blcr: {=u8:?} }}",
                self.enable(),
                self.blcb(),
                self.blcg(),
                self.blcr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 bad pixel removal control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1bprcr(pub u32);
    impl P1bprcr {
        #[doc = "Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Strength (aggressiveness) of the bad pixel detection."]
        #[must_use]
        #[inline(always)]
        pub const fn strength(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Strength (aggressiveness) of the bad pixel detection."]
        #[inline(always)]
        pub const fn set_strength(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for P1bprcr {
        #[inline(always)]
        fn default() -> P1bprcr {
            P1bprcr(0)
        }
    }
    impl core::fmt::Debug for P1bprcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1bprcr")
                .field("enable", &self.enable())
                .field("strength", &self.strength())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1bprcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1bprcr {{ enable: {=bool:?}, strength: {=u8:?} }}",
                self.enable(),
                self.strength()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 bad pixel removal status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1bprsr(pub u32);
    impl P1bprsr {
        #[doc = "Amount of detected bad pixels."]
        #[must_use]
        #[inline(always)]
        pub const fn badcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Amount of detected bad pixels."]
        #[inline(always)]
        pub const fn set_badcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for P1bprsr {
        #[inline(always)]
        fn default() -> P1bprsr {
            P1bprsr(0)
        }
    }
    impl core::fmt::Debug for P1bprsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1bprsr").field("badcnt", &self.badcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1bprsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1bprsr {{ badcnt: {=u16:?} }}", self.badcnt())
        }
    }
    #[doc = "DCMIPP Pipe1 current black level calibration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cblccr(pub u32);
    impl P1cblccr {
        #[doc = "For current black level calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "For current black level calibration."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current black level calibration - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn blcb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Current black level calibration - Blue."]
        #[inline(always)]
        pub const fn set_blcb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Current black level calibration - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn blcg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Current black level calibration - Green."]
        #[inline(always)]
        pub const fn set_blcg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Current black level calibration - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn blcr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Current black level calibration - Red."]
        #[inline(always)]
        pub const fn set_blcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for P1cblccr {
        #[inline(always)]
        fn default() -> P1cblccr {
            P1cblccr(0)
        }
    }
    impl core::fmt::Debug for P1cblccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cblccr")
                .field("enable", &self.enable())
                .field("blcb", &self.blcb())
                .field("blcg", &self.blcg())
                .field("blcr", &self.blcr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cblccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cblccr {{ enable: {=bool:?}, blcb: {=u8:?}, blcg: {=u8:?}, blcr: {=u8:?} }}",
                self.enable(),
                self.blcb(),
                self.blcg(),
                self.blcr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current bad pixel removal register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cbprcr(pub u32);
    impl P1cbprcr {
        #[doc = "Current status of enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current status of enable bit."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current strength (aggressiveness) of the bad pixel detection:."]
        #[must_use]
        #[inline(always)]
        pub const fn strength(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Current strength (aggressiveness) of the bad pixel detection:."]
        #[inline(always)]
        pub const fn set_strength(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for P1cbprcr {
        #[inline(always)]
        fn default() -> P1cbprcr {
            P1cbprcr(0)
        }
    }
    impl core::fmt::Debug for P1cbprcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cbprcr")
                .field("enable", &self.enable())
                .field("strength", &self.strength())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cbprcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cbprcr {{ enable: {=bool:?}, strength: {=u8:?} }}",
                self.enable(),
                self.strength()
            )
        }
    }
    #[doc = "DCMIPP Pipex ColorConv blue coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccbr1(pub u32);
    impl P1ccbr1 {
        #[doc = "Coefficient row 3 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_br(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 3 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1ccbr1 {
        #[inline(always)]
        fn default() -> P1ccbr1 {
            P1ccbr1(0)
        }
    }
    impl core::fmt::Debug for P1ccbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccbr1")
                .field("br", &self.br())
                .field("bg", &self.bg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccbr1 {{ br: {=u16:?}, bg: {=u16:?} }}", self.br(), self.bg())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv blue coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccbr2(pub u32);
    impl P1ccbr2 {
        #[doc = "Coefficient row 3 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_bb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 3 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ba(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 3 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1ccbr2 {
        #[inline(always)]
        fn default() -> P1ccbr2 {
            P1ccbr2(0)
        }
    }
    impl core::fmt::Debug for P1ccbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccbr2")
                .field("bb", &self.bb())
                .field("ba", &self.ba())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccbr2 {{ bb: {=u16:?}, ba: {=u16:?} }}", self.bb(), self.ba())
        }
    }
    #[doc = "DCMIPP Pipex current ColorConv blue coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccbr1(pub u32);
    impl P1cccbr1 {
        #[doc = "Current coefficient row 3 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 3 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_br(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 3 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 3 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1cccbr1 {
        #[inline(always)]
        fn default() -> P1cccbr1 {
            P1cccbr1(0)
        }
    }
    impl core::fmt::Debug for P1cccbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccbr1")
                .field("br", &self.br())
                .field("bg", &self.bg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccbr1 {{ br: {=u16:?}, bg: {=u16:?} }}", self.br(), self.bg())
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv blue coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccbr2(pub u32);
    impl P1cccbr2 {
        #[doc = "Current coefficient row 3 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 3 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_bb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 3 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ba(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current coefficient row 3 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1cccbr2 {
        #[inline(always)]
        fn default() -> P1cccbr2 {
            P1cccbr2(0)
        }
    }
    impl core::fmt::Debug for P1cccbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccbr2")
                .field("bb", &self.bb())
                .field("ba", &self.ba())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccbr2 {{ bb: {=u16:?}, ba: {=u16:?} }}", self.bb(), self.ba())
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccccr(pub u32);
    impl P1ccccr {
        #[doc = "Current value applied."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current value applied."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output samples type used while CLAMP is activated."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output samples type used while CLAMP is activated."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clamp the output samples."]
        #[must_use]
        #[inline(always)]
        pub const fn clamp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clamp the output samples."]
        #[inline(always)]
        pub const fn set_clamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for P1ccccr {
        #[inline(always)]
        fn default() -> P1ccccr {
            P1ccccr(0)
        }
    }
    impl core::fmt::Debug for P1ccccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccccr")
                .field("enable", &self.enable())
                .field("type_", &self.type_())
                .field("clamp", &self.clamp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ccccr {{ enable: {=bool:?}, type_: {=bool:?}, clamp: {=bool:?} }}",
                self.enable(),
                self.type_(),
                self.clamp()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv green coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccgr1(pub u32);
    impl P1cccgr1 {
        #[doc = "Current coefficient row 2 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 2 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_gr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 2 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 2 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_gg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1cccgr1 {
        #[inline(always)]
        fn default() -> P1cccgr1 {
            P1cccgr1(0)
        }
    }
    impl core::fmt::Debug for P1cccgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccgr1")
                .field("gr", &self.gr())
                .field("gg", &self.gg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccgr1 {{ gr: {=u16:?}, gg: {=u16:?} }}", self.gr(), self.gg())
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv green coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccgr2(pub u32);
    impl P1cccgr2 {
        #[doc = "Current coefficient row 2 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 2 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_gb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 2 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ga(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current coefficient row 2 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ga(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1cccgr2 {
        #[inline(always)]
        fn default() -> P1cccgr2 {
            P1cccgr2(0)
        }
    }
    impl core::fmt::Debug for P1cccgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccgr2")
                .field("gb", &self.gb())
                .field("ga", &self.ga())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccgr2 {{ gb: {=u16:?}, ga: {=u16:?} }}", self.gb(), self.ga())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccr(pub u32);
    impl P1cccr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "output samples type used while CLAMP is activated."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "output samples type used while CLAMP is activated."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clamp the output samples."]
        #[must_use]
        #[inline(always)]
        pub const fn clamp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clamp the output samples."]
        #[inline(always)]
        pub const fn set_clamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for P1cccr {
        #[inline(always)]
        fn default() -> P1cccr {
            P1cccr(0)
        }
    }
    impl core::fmt::Debug for P1cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccr")
                .field("enable", &self.enable())
                .field("type_", &self.type_())
                .field("clamp", &self.clamp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cccr {{ enable: {=bool:?}, type_: {=bool:?}, clamp: {=bool:?} }}",
                self.enable(),
                self.type_(),
                self.clamp()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv red coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccrr1(pub u32);
    impl P1cccrr1 {
        #[doc = "Current coefficient row 1 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 1 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_rr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 1 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 1 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_rg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1cccrr1 {
        #[inline(always)]
        fn default() -> P1cccrr1 {
            P1cccrr1(0)
        }
    }
    impl core::fmt::Debug for P1cccrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccrr1")
                .field("rr", &self.rr())
                .field("rg", &self.rg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccrr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccrr1 {{ rr: {=u16:?}, rg: {=u16:?} }}", self.rr(), self.rg())
        }
    }
    #[doc = "DCMIPP Pipe1 current ColorConv red coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cccrr2(pub u32);
    impl P1cccrr2 {
        #[doc = "Current coefficient row 1 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Current coefficient row 1 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_rb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Current coefficient row 1 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current coefficient row 1 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1cccrr2 {
        #[inline(always)]
        fn default() -> P1cccrr2 {
            P1cccrr2(0)
        }
    }
    impl core::fmt::Debug for P1cccrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cccrr2")
                .field("rb", &self.rb())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cccrr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cccrr2 {{ rb: {=u16:?}, ra: {=u16:?} }}", self.rb(), self.ra())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv green coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccgr1(pub u32);
    impl P1ccgr1 {
        #[doc = "Coefficient row 2 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_gr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 2 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_gg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1ccgr1 {
        #[inline(always)]
        fn default() -> P1ccgr1 {
            P1ccgr1(0)
        }
    }
    impl core::fmt::Debug for P1ccgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccgr1")
                .field("gr", &self.gr())
                .field("gg", &self.gg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccgr1 {{ gr: {=u16:?}, gg: {=u16:?} }}", self.gr(), self.gg())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv green coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccgr2(pub u32);
    impl P1ccgr2 {
        #[doc = "Coefficient row 2 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_gb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 2 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ga(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 2 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ga(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1ccgr2 {
        #[inline(always)]
        fn default() -> P1ccgr2 {
            P1ccgr2(0)
        }
    }
    impl core::fmt::Debug for P1ccgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccgr2")
                .field("gb", &self.gb())
                .field("ga", &self.ga())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccgr2 {{ gb: {=u16:?}, ga: {=u16:?} }}", self.gb(), self.ga())
        }
    }
    #[doc = "DCMIPP Pipex current common ROI configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccmricr(pub u32);
    impl P1ccmricr {
        #[doc = "Current region of interest line size width."]
        #[must_use]
        #[inline(always)]
        pub const fn roilsz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Current region of interest line size width."]
        #[inline(always)]
        pub const fn set_roilsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Current region of interest 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 1 enable."]
        #[inline(always)]
        pub const fn set_roi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Current region of interest 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 2 enable."]
        #[inline(always)]
        pub const fn set_roi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Current region of interest 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 3 enable."]
        #[inline(always)]
        pub const fn set_roi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Current region of interest 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 4 enable."]
        #[inline(always)]
        pub const fn set_roi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Current region of interest 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 5 enable."]
        #[inline(always)]
        pub const fn set_roi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Current region of interest 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi6en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 6 enable."]
        #[inline(always)]
        pub const fn set_roi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Current region of interest 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi7en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 7 enable."]
        #[inline(always)]
        pub const fn set_roi7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Current region of interest 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi8en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 8 enable."]
        #[inline(always)]
        pub const fn set_roi8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P1ccmricr {
        #[inline(always)]
        fn default() -> P1ccmricr {
            P1ccmricr(0)
        }
    }
    impl core::fmt::Debug for P1ccmricr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccmricr")
                .field("roilsz", &self.roilsz())
                .field("roi1en", &self.roi1en())
                .field("roi2en", &self.roi2en())
                .field("roi3en", &self.roi3en())
                .field("roi4en", &self.roi4en())
                .field("roi5en", &self.roi5en())
                .field("roi6en", &self.roi6en())
                .field("roi7en", &self.roi7en())
                .field("roi8en", &self.roi8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccmricr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1ccmricr {{ roilsz: {=u8:?}, roi1en: {=bool:?}, roi2en: {=bool:?}, roi3en: {=bool:?}, roi4en: {=bool:?}, roi5en: {=bool:?}, roi6en: {=bool:?}, roi7en: {=bool:?}, roi8en: {=bool:?} }}" , self . roilsz () , self . roi1en () , self . roi2en () , self . roi3en () , self . roi4en () , self . roi5en () , self . roi6en () , self . roi7en () , self . roi8en ())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv red coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccrr1(pub u32);
    impl P1ccrr1 {
        #[doc = "Coefficient row 1 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_rr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 1 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_rg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1ccrr1 {
        #[inline(always)]
        fn default() -> P1ccrr1 {
            P1ccrr1(0)
        }
    }
    impl core::fmt::Debug for P1ccrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccrr1")
                .field("rr", &self.rr())
                .field("rg", &self.rg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccrr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccrr1 {{ rr: {=u16:?}, rg: {=u16:?} }}", self.rr(), self.rg())
        }
    }
    #[doc = "DCMIPP Pipe1 ColorConv red coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccrr2(pub u32);
    impl P1ccrr2 {
        #[doc = "Coefficient row 1 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_rb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 1 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 1 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1ccrr2 {
        #[inline(always)]
        fn default() -> P1ccrr2 {
            P1ccrr2(0)
        }
    }
    impl core::fmt::Debug for P1ccrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccrr2")
                .field("rb", &self.rb())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccrr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ccrr2 {{ rb: {=u16:?}, ra: {=u16:?} }}", self.rb(), self.ra())
        }
    }
    #[doc = "DCMIPP Pipex current crop window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccrstr(pub u32);
    impl P1ccrstr {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ccrstr {
        #[inline(always)]
        fn default() -> P1ccrstr {
            P1ccrstr(0)
        }
    }
    impl core::fmt::Debug for P1ccrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccrstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ccrstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipex current crop window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ccrszr(pub u32);
    impl P1ccrszr {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current ENABLE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current ENABLE bit value."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1ccrszr {
        #[inline(always)]
        fn default() -> P1ccrszr {
            P1ccrszr(0)
        }
    }
    impl core::fmt::Debug for P1ccrszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ccrszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ccrszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ccrszr {{ hsize: {=u16:?}, vsize: {=u16:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cctcr1(pub u32);
    impl P1cctcr1 {
        #[doc = "Current ENABLE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current ENABLE bit value."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current luminance increase for input luminance of 0 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum0(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Current luminance increase for input luminance of 0 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
    }
    impl Default for P1cctcr1 {
        #[inline(always)]
        fn default() -> P1cctcr1 {
            P1cctcr1(0)
        }
    }
    impl core::fmt::Debug for P1cctcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cctcr1")
                .field("enable", &self.enable())
                .field("lum0", &self.lum0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cctcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cctcr1 {{ enable: {=bool:?}, lum0: {=u8:?} }}",
                self.enable(),
                self.lum0()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cctcr2(pub u32);
    impl P1cctcr2 {
        #[doc = "Current luminance increase for input luminance of 128 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum4(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[doc = "Current luminance increase for input luminance of 128 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[doc = "Current luminance increase for input luminance of 96 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum3(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Current luminance increase for input luminance of 96 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[doc = "Current luminance increase for input luminance of 64 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum2(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Current luminance increase for input luminance of 64 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Current luminance increase for input luminance of 32 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum1(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[doc = "Current luminance increase for input luminance of 32 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
    }
    impl Default for P1cctcr2 {
        #[inline(always)]
        fn default() -> P1cctcr2 {
            P1cctcr2(0)
        }
    }
    impl core::fmt::Debug for P1cctcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cctcr2")
                .field("lum4", &self.lum4())
                .field("lum3", &self.lum3())
                .field("lum2", &self.lum2())
                .field("lum1", &self.lum1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cctcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cctcr2 {{ lum4: {=u8:?}, lum3: {=u8:?}, lum2: {=u8:?}, lum1: {=u8:?} }}",
                self.lum4(),
                self.lum3(),
                self.lum2(),
                self.lum1()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current contrast control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cctcr3(pub u32);
    impl P1cctcr3 {
        #[doc = "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum8(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[doc = "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum7(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[doc = "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum6(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum5(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
    }
    impl Default for P1cctcr3 {
        #[inline(always)]
        fn default() -> P1cctcr3 {
            P1cctcr3(0)
        }
    }
    impl core::fmt::Debug for P1cctcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cctcr3")
                .field("lum8", &self.lum8())
                .field("lum7", &self.lum7())
                .field("lum6", &self.lum6())
                .field("lum5", &self.lum5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cctcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cctcr3 {{ lum8: {=u8:?}, lum7: {=u8:?}, lum6: {=u8:?}, lum5: {=u8:?} }}",
                self.lum8(),
                self.lum7(),
                self.lum6(),
                self.lum5()
            )
        }
    }
    #[doc = "DCMIPP Pipex current decimation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cdccr(pub u32);
    impl P1cdccr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn hdec(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal decimation ratio."]
        #[inline(always)]
        pub const fn set_hdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Vertical decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn vdec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Vertical decimation ratio."]
        #[inline(always)]
        pub const fn set_vdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for P1cdccr {
        #[inline(always)]
        fn default() -> P1cdccr {
            P1cdccr(0)
        }
    }
    impl core::fmt::Debug for P1cdccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cdccr")
                .field("enable", &self.enable())
                .field("hdec", &self.hdec())
                .field("vdec", &self.vdec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cdccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cdccr {{ enable: {=bool:?}, hdec: {=u8:?}, vdec: {=u8:?} }}",
                self.enable(),
                self.hdec(),
                self.vdec()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cdscr(pub u32);
    impl P1cdscr {
        #[doc = "Current horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_hdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vdiv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_vdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Current value of bit ENABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current value of bit ENABLE."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1cdscr {
        #[inline(always)]
        fn default() -> P1cdscr {
            P1cdscr(0)
        }
    }
    impl core::fmt::Debug for P1cdscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cdscr")
                .field("hdiv", &self.hdiv())
                .field("vdiv", &self.vdiv())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cdscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cdscr {{ hdiv: {=u16:?}, vdiv: {=u16:?}, enable: {=bool:?} }}",
                self.hdiv(),
                self.vdiv(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize ratio register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cdsrtior(pub u32);
    impl P1cdsrtior {
        #[doc = "Current horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hratio(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_hratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vratio(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_vratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for P1cdsrtior {
        #[inline(always)]
        fn default() -> P1cdsrtior {
            P1cdsrtior(0)
        }
    }
    impl core::fmt::Debug for P1cdsrtior {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cdsrtior")
                .field("hratio", &self.hratio())
                .field("vratio", &self.vratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cdsrtior {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cdsrtior {{ hratio: {=u16:?}, vratio: {=u16:?} }}",
                self.hratio(),
                self.vratio()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize destination size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cdsszr(pub u32);
    impl P1cdsszr {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cdsszr {
        #[inline(always)]
        fn default() -> P1cdsszr {
            P1cdsszr(0)
        }
    }
    impl core::fmt::Debug for P1cdsszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cdsszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cdsszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cdsszr {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current exposure control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cexcr1(pub u32);
    impl P1cexcr1 {
        #[doc = "for exposure control (multiplication and shift)."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "for exposure control (multiplication and shift)."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current exposure multiplier - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn multr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Current exposure multiplier - Red."]
        #[inline(always)]
        pub const fn set_multr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Current exposure shift - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn shfr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Current exposure shift - Red."]
        #[inline(always)]
        pub const fn set_shfr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for P1cexcr1 {
        #[inline(always)]
        fn default() -> P1cexcr1 {
            P1cexcr1(0)
        }
    }
    impl core::fmt::Debug for P1cexcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cexcr1")
                .field("enable", &self.enable())
                .field("multr", &self.multr())
                .field("shfr", &self.shfr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cexcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cexcr1 {{ enable: {=bool:?}, multr: {=u8:?}, shfr: {=u8:?} }}",
                self.enable(),
                self.multr(),
                self.shfr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current exposure control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cexcr2(pub u32);
    impl P1cexcr2 {
        #[doc = "Current exposure multiplier - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn multb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "Current exposure multiplier - Blue."]
        #[inline(always)]
        pub const fn set_multb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "Current exposure shift - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn shfb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Current exposure shift - Blue."]
        #[inline(always)]
        pub const fn set_shfb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Current exposure multiplier - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn multg(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Current exposure multiplier - Green."]
        #[inline(always)]
        pub const fn set_multg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Current exposure shift - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn shfg(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Current exposure shift - Green."]
        #[inline(always)]
        pub const fn set_shfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for P1cexcr2 {
        #[inline(always)]
        fn default() -> P1cexcr2 {
            P1cexcr2(0)
        }
    }
    impl core::fmt::Debug for P1cexcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cexcr2")
                .field("multb", &self.multb())
                .field("shfb", &self.shfb())
                .field("multg", &self.multg())
                .field("shfg", &self.shfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cexcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cexcr2 {{ multb: {=u8:?}, shfb: {=u8:?}, multg: {=u8:?}, shfg: {=u8:?} }}",
                self.multb(),
                self.shfb(),
                self.multg(),
                self.shfg()
            )
        }
    }
    #[doc = "DCMIPP Pipex current flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cfctcr(pub u32);
    impl P1cfctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P1cfctcr {
        #[inline(always)]
        fn default() -> P1cfctcr {
            P1cfctcr(0)
        }
    }
    impl core::fmt::Debug for P1cfctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cfctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cfctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cfctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cfscr(pub u32);
    impl P1cfscr {
        #[doc = "Current data type ID A."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Current data type ID A."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Current data type ID B."]
        #[must_use]
        #[inline(always)]
        pub const fn dtidb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Current data type ID B."]
        #[inline(always)]
        pub const fn set_dtidb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Current differentiates Pipe2 vs. Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn pipediff(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Current differentiates Pipe2 vs. Pipe1."]
        #[inline(always)]
        pub const fn set_pipediff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Current flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Current flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Current force data type format."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Current force data type format."]
        #[inline(always)]
        pub const fn set_fdtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Current force data type format enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtfen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Current force data type format enable."]
        #[inline(always)]
        pub const fn set_fdtfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Current activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1cfscr {
        #[inline(always)]
        fn default() -> P1cfscr {
            P1cfscr(0)
        }
    }
    impl core::fmt::Debug for P1cfscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cfscr")
                .field("dtida", &self.dtida())
                .field("dtidb", &self.dtidb())
                .field("dtmode", &self.dtmode())
                .field("pipediff", &self.pipediff())
                .field("vc", &self.vc())
                .field("fdtf", &self.fdtf())
                .field("fdtfen", &self.fdtfen())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cfscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1cfscr {{ dtida: {=u8:?}, dtidb: {=u8:?}, dtmode: {=u8:?}, pipediff: {=bool:?}, vc: {=u8:?}, fdtf: {=u8:?}, fdtfen: {=bool:?}, pipen: {=bool:?} }}" , self . dtida () , self . dtidb () , self . dtmode () , self . pipediff () , self . vc () , self . fdtf () , self . fdtfen () , self . pipen ())
        }
    }
    #[doc = "DCMIPP Pipex common ROI configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cmricr(pub u32);
    impl P1cmricr {
        #[doc = "Region of interest line size width."]
        #[must_use]
        #[inline(always)]
        pub const fn roilsz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Region of interest line size width."]
        #[inline(always)]
        pub const fn set_roilsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Region of interest 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 1 enable."]
        #[inline(always)]
        pub const fn set_roi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Region of interest 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 2 enable."]
        #[inline(always)]
        pub const fn set_roi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Region of interest 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 3 enable."]
        #[inline(always)]
        pub const fn set_roi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Region of interest 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 4 enable."]
        #[inline(always)]
        pub const fn set_roi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Region of interest 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 5 enable."]
        #[inline(always)]
        pub const fn set_roi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Region of interest 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi6en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 6 enable."]
        #[inline(always)]
        pub const fn set_roi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Region of interest 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi7en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 7 enable."]
        #[inline(always)]
        pub const fn set_roi7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Region of interest 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi8en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 8 enable."]
        #[inline(always)]
        pub const fn set_roi8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P1cmricr {
        #[inline(always)]
        fn default() -> P1cmricr {
            P1cmricr(0)
        }
    }
    impl core::fmt::Debug for P1cmricr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cmricr")
                .field("roilsz", &self.roilsz())
                .field("roi1en", &self.roi1en())
                .field("roi2en", &self.roi2en())
                .field("roi3en", &self.roi3en())
                .field("roi4en", &self.roi4en())
                .field("roi5en", &self.roi5en())
                .field("roi6en", &self.roi6en())
                .field("roi7en", &self.roi7en())
                .field("roi8en", &self.roi8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cmricr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1cmricr {{ roilsz: {=u8:?}, roi1en: {=bool:?}, roi2en: {=bool:?}, roi3en: {=bool:?}, roi4en: {=bool:?}, roi5en: {=bool:?}, roi6en: {=bool:?}, roi7en: {=bool:?}, roi8en: {=bool:?} }}" , self . roilsz () , self . roi1en () , self . roi2en () , self . roi3en () , self . roi4en () , self . roi5en () , self . roi6en () , self . roi7en () , self . roi8en ())
        }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppcr(pub u32);
    impl P1cppcr {
        #[doc = "Memory format."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Memory format."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Swaps R-vs-B components if RGB, and U-vs-V components if YUV."]
        #[must_use]
        #[inline(always)]
        pub const fn swaprb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps R-vs-B components if RGB, and U-vs-V components if YUV."]
        #[inline(always)]
        pub const fn set_swaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Amount of capture completed lines for LINE Event and Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of capture completed lines for LINE Event and Interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Line multi address wrapping modulo."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawm(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Line multi address wrapping modulo."]
        #[inline(always)]
        pub const fn set_lmawm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[inline(always)]
        pub const fn set_lmawe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for P1cppcr {
        #[inline(always)]
        fn default() -> P1cppcr {
            P1cppcr(0)
        }
    }
    impl core::fmt::Debug for P1cppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppcr")
                .field("format", &self.format())
                .field("swaprb", &self.swaprb())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .field("lmawm", &self.lmawm())
                .field("lmawe", &self.lmawe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1cppcr {{ format: {=u8:?}, swaprb: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?}, lmawm: {=u8:?}, lmawe: {=bool:?} }}" , self . format () , self . swaprb () , self . linemult () , self . dbm () , self . lmawm () , self . lmawe ())
        }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm0ar1(pub u32);
    impl P1cppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm0ar1 {
        #[inline(always)]
        fn default() -> P1cppm0ar1 {
            P1cppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P1cppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe1 current pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm0ar2(pub u32);
    impl P1cppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm0ar2 {
        #[inline(always)]
        fn default() -> P1cppm0ar2 {
            P1cppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P1cppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory0 pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm0pr(pub u32);
    impl P1cppm0pr {
        #[doc = "Current number of bytes between the address of two consecutive lines."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Current number of bytes between the address of two consecutive lines."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for P1cppm0pr {
        #[inline(always)]
        fn default() -> P1cppm0pr {
            P1cppm0pr(0)
        }
    }
    impl core::fmt::Debug for P1cppm0pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm0pr").field("pitch", &self.pitch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm0pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm0pr {{ pitch: {=u16:?} }}", self.pitch())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm1ar1(pub u32);
    impl P1cppm1ar1 {
        #[doc = "Memory1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory1 address."]
        #[inline(always)]
        pub const fn set_m1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm1ar1 {
        #[inline(always)]
        fn default() -> P1cppm1ar1 {
            P1cppm1ar1(0)
        }
    }
    impl core::fmt::Debug for P1cppm1ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm1ar1").field("m1a", &self.m1a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm1ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm1ar1 {{ m1a: {=u32:?} }}", self.m1a())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm1ar2(pub u32);
    impl P1cppm1ar2 {
        #[doc = "Memory1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory1 address."]
        #[inline(always)]
        pub const fn set_m1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm1ar2 {
        #[inline(always)]
        fn default() -> P1cppm1ar2 {
            P1cppm1ar2(0)
        }
    }
    impl core::fmt::Debug for P1cppm1ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm1ar2").field("m1a", &self.m1a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm1ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm1ar2 {{ m1a: {=u32:?} }}", self.m1a())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory1 pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm1pr(pub u32);
    impl P1cppm1pr {
        #[doc = "Current number of bytes between the address of two consecutive lines."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Current number of bytes between the address of two consecutive lines."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for P1cppm1pr {
        #[inline(always)]
        fn default() -> P1cppm1pr {
            P1cppm1pr(0)
        }
    }
    impl core::fmt::Debug for P1cppm1pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm1pr").field("pitch", &self.pitch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm1pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm1pr {{ pitch: {=u16:?} }}", self.pitch())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory2 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm2ar1(pub u32);
    impl P1cppm2ar1 {
        #[doc = "Memory 2 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory 2 address."]
        #[inline(always)]
        pub const fn set_m2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm2ar1 {
        #[inline(always)]
        fn default() -> P1cppm2ar1 {
            P1cppm2ar1(0)
        }
    }
    impl core::fmt::Debug for P1cppm2ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm2ar1").field("m2a", &self.m2a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm2ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm2ar1 {{ m2a: {=u32:?} }}", self.m2a())
        }
    }
    #[doc = "DCMIPP Pipex current pixel packer Memory2 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cppm2ar2(pub u32);
    impl P1cppm2ar2 {
        #[doc = "Memory 2 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory 2 address."]
        #[inline(always)]
        pub const fn set_m2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1cppm2ar2 {
        #[inline(always)]
        fn default() -> P1cppm2ar2 {
            P1cppm2ar2(0)
        }
    }
    impl core::fmt::Debug for P1cppm2ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cppm2ar2").field("m2a", &self.m2a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cppm2ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1cppm2ar2 {{ m2a: {=u32:?} }}", self.m2a())
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI1 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri1cr1(pub u32);
    impl P1cri1cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri1cr1 {
        #[inline(always)]
        fn default() -> P1cri1cr1 {
            P1cri1cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri1cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri1cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri1cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri1cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI1 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri1cr2(pub u32);
    impl P1cri1cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri1cr2 {
        #[inline(always)]
        fn default() -> P1cri1cr2 {
            P1cri1cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri1cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri1cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri1cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri1cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI2 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri2cr1(pub u32);
    impl P1cri2cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri2cr1 {
        #[inline(always)]
        fn default() -> P1cri2cr1 {
            P1cri2cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri2cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri2cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri2cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri2cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri2cr2(pub u32);
    impl P1cri2cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri2cr2 {
        #[inline(always)]
        fn default() -> P1cri2cr2 {
            P1cri2cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri2cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri2cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri2cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri2cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI3 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri3cr1(pub u32);
    impl P1cri3cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri3cr1 {
        #[inline(always)]
        fn default() -> P1cri3cr1 {
            P1cri3cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri3cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri3cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri3cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri3cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI3 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri3cr2(pub u32);
    impl P1cri3cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri3cr2 {
        #[inline(always)]
        fn default() -> P1cri3cr2 {
            P1cri3cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri3cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri3cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri3cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri3cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI4 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri4cr1(pub u32);
    impl P1cri4cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri4cr1 {
        #[inline(always)]
        fn default() -> P1cri4cr1 {
            P1cri4cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri4cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri4cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri4cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri4cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI4 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri4cr2(pub u32);
    impl P1cri4cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri4cr2 {
        #[inline(always)]
        fn default() -> P1cri4cr2 {
            P1cri4cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri4cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri4cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri4cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri4cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI5 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri5cr1(pub u32);
    impl P1cri5cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri5cr1 {
        #[inline(always)]
        fn default() -> P1cri5cr1 {
            P1cri5cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri5cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri5cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri5cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri5cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI5 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri5cr2(pub u32);
    impl P1cri5cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri5cr2 {
        #[inline(always)]
        fn default() -> P1cri5cr2 {
            P1cri5cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri5cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri5cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri5cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri5cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI6 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri6cr1(pub u32);
    impl P1cri6cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri6cr1 {
        #[inline(always)]
        fn default() -> P1cri6cr1 {
            P1cri6cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri6cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri6cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri6cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri6cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI6 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri6cr2(pub u32);
    impl P1cri6cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri6cr2 {
        #[inline(always)]
        fn default() -> P1cri6cr2 {
            P1cri6cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri6cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri6cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri6cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri6cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI7 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri7cr1(pub u32);
    impl P1cri7cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri7cr1 {
        #[inline(always)]
        fn default() -> P1cri7cr1 {
            P1cri7cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri7cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri7cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri7cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri7cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI7 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri7cr2(pub u32);
    impl P1cri7cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri7cr2 {
        #[inline(always)]
        fn default() -> P1cri7cr2 {
            P1cri7cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri7cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri7cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri7cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri7cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI8 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri8cr1(pub u32);
    impl P1cri8cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1cri8cr1 {
        #[inline(always)]
        fn default() -> P1cri8cr1 {
            P1cri8cr1(0)
        }
    }
    impl core::fmt::Debug for P1cri8cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri8cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri8cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri8cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current ROI8 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cri8cr2(pub u32);
    impl P1cri8cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cri8cr2 {
        #[inline(always)]
        fn default() -> P1cri8cr2 {
            P1cri8cr2(0)
        }
    }
    impl core::fmt::Debug for P1cri8cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cri8cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cri8cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cri8cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipex crop window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1crstr(pub u32);
    impl P1crstr {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1crstr {
        #[inline(always)]
        fn default() -> P1crstr {
            P1crstr(0)
        }
    }
    impl core::fmt::Debug for P1crstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1crstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1crstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1crstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipex crop window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1crszr(pub u32);
    impl P1crszr {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1crszr {
        #[inline(always)]
        fn default() -> P1crszr {
            P1crszr(0)
        }
    }
    impl core::fmt::Debug for P1crszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1crszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1crszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1crszr {{ hsize: {=u16:?}, vsize: {=u16:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current statistics 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cst1cr(pub u32);
    impl P1cst1cr {
        #[doc = "Current enable bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current enable bit value."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Current bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Current source of statistics."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Current source of statistics."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Current statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Current statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Current accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Current accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for P1cst1cr {
        #[inline(always)]
        fn default() -> P1cst1cr {
            P1cst1cr(0)
        }
    }
    impl core::fmt::Debug for P1cst1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cst1cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .field("accu", &self.accu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cst1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cst1cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?}, accu: {=u32:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode(),
                self.accu()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current statistics 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cst2cr(pub u32);
    impl P1cst2cr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Statistics source."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Statistics source."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for P1cst2cr {
        #[inline(always)]
        fn default() -> P1cst2cr {
            P1cst2cr(0)
        }
    }
    impl core::fmt::Debug for P1cst2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cst2cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .field("accu", &self.accu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cst2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cst2cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?}, accu: {=u32:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode(),
                self.accu()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current statistics 3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cst3cr(pub u32);
    impl P1cst3cr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Current bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Statistics source."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Statistics source."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for P1cst3cr {
        #[inline(always)]
        fn default() -> P1cst3cr {
            P1cst3cr(0)
        }
    }
    impl core::fmt::Debug for P1cst3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cst3cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .field("accu", &self.accu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cst3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cst3cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?}, accu: {=u32:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode(),
                self.accu()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current statistics window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cststr(pub u32);
    impl P1cststr {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1cststr {
        #[inline(always)]
        fn default() -> P1cststr {
            P1cststr(0)
        }
    }
    impl core::fmt::Debug for P1cststr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cststr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cststr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cststr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 current statistics window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cstszr(pub u32);
    impl P1cstszr {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current CROPEN bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn cropen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current CROPEN bit value."]
        #[inline(always)]
        pub const fn set_cropen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1cstszr {
        #[inline(always)]
        fn default() -> P1cstszr {
            P1cstszr(0)
        }
    }
    impl core::fmt::Debug for P1cstszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1cstszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("cropen", &self.cropen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1cstszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1cstszr {{ hsize: {=u16:?}, vsize: {=u16:?}, cropen: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.cropen()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ctcr1(pub u32);
    impl P1ctcr1 {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum0(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 0 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
    }
    impl Default for P1ctcr1 {
        #[inline(always)]
        fn default() -> P1ctcr1 {
            P1ctcr1(0)
        }
    }
    impl core::fmt::Debug for P1ctcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ctcr1")
                .field("enable", &self.enable())
                .field("lum0", &self.lum0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ctcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ctcr1 {{ enable: {=bool:?}, lum0: {=u8:?} }}",
                self.enable(),
                self.lum0()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ctcr2(pub u32);
    impl P1ctcr2 {
        #[doc = "Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum4(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 128 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[doc = "Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum3(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 96 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[doc = "Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum2(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 64 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum1(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 32 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
    }
    impl Default for P1ctcr2 {
        #[inline(always)]
        fn default() -> P1ctcr2 {
            P1ctcr2(0)
        }
    }
    impl core::fmt::Debug for P1ctcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ctcr2")
                .field("lum4", &self.lum4())
                .field("lum3", &self.lum3())
                .field("lum2", &self.lum2())
                .field("lum1", &self.lum1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ctcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ctcr2 {{ lum4: {=u8:?}, lum3: {=u8:?}, lum2: {=u8:?}, lum1: {=u8:?} }}",
                self.lum4(),
                self.lum3(),
                self.lum2(),
                self.lum1()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 contrast control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ctcr3(pub u32);
    impl P1ctcr3 {
        #[doc = "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum8(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[doc = "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum7(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[doc = "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum6(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn lum5(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[doc = "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16)."]
        #[inline(always)]
        pub const fn set_lum5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
    }
    impl Default for P1ctcr3 {
        #[inline(always)]
        fn default() -> P1ctcr3 {
            P1ctcr3(0)
        }
    }
    impl core::fmt::Debug for P1ctcr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ctcr3")
                .field("lum8", &self.lum8())
                .field("lum7", &self.lum7())
                .field("lum6", &self.lum6())
                .field("lum5", &self.lum5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ctcr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ctcr3 {{ lum8: {=u8:?}, lum7: {=u8:?}, lum6: {=u8:?}, lum5: {=u8:?} }}",
                self.lum8(),
                self.lum7(),
                self.lum6(),
                self.lum5()
            )
        }
    }
    #[doc = "DCMIPP Pipex decimation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1dccr(pub u32);
    impl P1dccr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn hdec(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal decimation ratio."]
        #[inline(always)]
        pub const fn set_hdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Vertical decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn vdec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Vertical decimation ratio."]
        #[inline(always)]
        pub const fn set_vdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for P1dccr {
        #[inline(always)]
        fn default() -> P1dccr {
            P1dccr(0)
        }
    }
    impl core::fmt::Debug for P1dccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1dccr")
                .field("enable", &self.enable())
                .field("hdec", &self.hdec())
                .field("vdec", &self.vdec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1dccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1dccr {{ enable: {=bool:?}, hdec: {=u8:?}, vdec: {=u8:?} }}",
                self.enable(),
                self.hdec(),
                self.vdec()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 decimation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1decr(pub u32);
    impl P1decr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn hdec(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal decimation ratio."]
        #[inline(always)]
        pub const fn set_hdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Vertical decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn vdec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Vertical decimation ratio."]
        #[inline(always)]
        pub const fn set_vdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for P1decr {
        #[inline(always)]
        fn default() -> P1decr {
            P1decr(0)
        }
    }
    impl core::fmt::Debug for P1decr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1decr")
                .field("enable", &self.enable())
                .field("hdec", &self.hdec())
                .field("vdec", &self.vdec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1decr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1decr {{ enable: {=bool:?}, hdec: {=u8:?}, vdec: {=u8:?} }}",
                self.enable(),
                self.hdec(),
                self.vdec()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 demosaicing configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1dmcr(pub u32);
    impl P1dmcr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Raw Bayer type."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Raw Bayer type."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Strength of the peak detection."]
        #[must_use]
        #[inline(always)]
        pub const fn peak(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Strength of the peak detection."]
        #[inline(always)]
        pub const fn set_peak(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Strength of the vertical line detection."]
        #[must_use]
        #[inline(always)]
        pub const fn linev(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Strength of the vertical line detection."]
        #[inline(always)]
        pub const fn set_linev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Strength of the horizontal line detection."]
        #[must_use]
        #[inline(always)]
        pub const fn lineh(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Strength of the horizontal line detection."]
        #[inline(always)]
        pub const fn set_lineh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Strength of the edge detection."]
        #[must_use]
        #[inline(always)]
        pub const fn edge(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Strength of the edge detection."]
        #[inline(always)]
        pub const fn set_edge(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for P1dmcr {
        #[inline(always)]
        fn default() -> P1dmcr {
            P1dmcr(0)
        }
    }
    impl core::fmt::Debug for P1dmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1dmcr")
                .field("enable", &self.enable())
                .field("type_", &self.type_())
                .field("peak", &self.peak())
                .field("linev", &self.linev())
                .field("lineh", &self.lineh())
                .field("edge", &self.edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1dmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1dmcr {{ enable: {=bool:?}, type_: {=u8:?}, peak: {=u8:?}, linev: {=u8:?}, lineh: {=u8:?}, edge: {=u8:?} }}" , self . enable () , self . type_ () , self . peak () , self . linev () , self . lineh () , self . edge ())
        }
    }
    #[doc = "DCMIPP Pipex downsize configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1dscr(pub u32);
    impl P1dscr {
        #[doc = "Horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_hdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vdiv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_vdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1dscr {
        #[inline(always)]
        fn default() -> P1dscr {
            P1dscr(0)
        }
    }
    impl core::fmt::Debug for P1dscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1dscr")
                .field("hdiv", &self.hdiv())
                .field("vdiv", &self.vdiv())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1dscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1dscr {{ hdiv: {=u16:?}, vdiv: {=u16:?}, enable: {=bool:?} }}",
                self.hdiv(),
                self.vdiv(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex downsize ratio register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1dsrtior(pub u32);
    impl P1dsrtior {
        #[doc = "Horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hratio(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_hratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vratio(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_vratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for P1dsrtior {
        #[inline(always)]
        fn default() -> P1dsrtior {
            P1dsrtior(0)
        }
    }
    impl core::fmt::Debug for P1dsrtior {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1dsrtior")
                .field("hratio", &self.hratio())
                .field("vratio", &self.vratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1dsrtior {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1dsrtior {{ hratio: {=u16:?}, vratio: {=u16:?} }}",
                self.hratio(),
                self.vratio()
            )
        }
    }
    #[doc = "DCMIPP Pipex downsize destination size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1dsszr(pub u32);
    impl P1dsszr {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1dsszr {
        #[inline(always)]
        fn default() -> P1dsszr {
            P1dsszr(0)
        }
    }
    impl core::fmt::Debug for P1dsszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1dsszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1dsszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1dsszr {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 exposure control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1excr1(pub u32);
    impl P1excr1 {
        #[doc = "Exposure control (multiplication and shift) of all red, green and blue."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Exposure control (multiplication and shift) of all red, green and blue."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Exposure multiplier - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn multr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Exposure multiplier - Red."]
        #[inline(always)]
        pub const fn set_multr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Exposure shift - Red."]
        #[must_use]
        #[inline(always)]
        pub const fn shfr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Exposure shift - Red."]
        #[inline(always)]
        pub const fn set_shfr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for P1excr1 {
        #[inline(always)]
        fn default() -> P1excr1 {
            P1excr1(0)
        }
    }
    impl core::fmt::Debug for P1excr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1excr1")
                .field("enable", &self.enable())
                .field("multr", &self.multr())
                .field("shfr", &self.shfr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1excr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1excr1 {{ enable: {=bool:?}, multr: {=u8:?}, shfr: {=u8:?} }}",
                self.enable(),
                self.multr(),
                self.shfr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 exposure control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1excr2(pub u32);
    impl P1excr2 {
        #[doc = "Exposure multiplier - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn multb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "Exposure multiplier - Blue."]
        #[inline(always)]
        pub const fn set_multb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "Exposure shift - Blue."]
        #[must_use]
        #[inline(always)]
        pub const fn shfb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Exposure shift - Blue."]
        #[inline(always)]
        pub const fn set_shfb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Exposure multiplier - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn multg(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Exposure multiplier - Green."]
        #[inline(always)]
        pub const fn set_multg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Exposure shift - Green."]
        #[must_use]
        #[inline(always)]
        pub const fn shfg(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Exposure shift - Green."]
        #[inline(always)]
        pub const fn set_shfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for P1excr2 {
        #[inline(always)]
        fn default() -> P1excr2 {
            P1excr2(0)
        }
    }
    impl core::fmt::Debug for P1excr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1excr2")
                .field("multb", &self.multb())
                .field("shfb", &self.shfb())
                .field("multg", &self.multg())
                .field("shfg", &self.shfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1excr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1excr2 {{ multb: {=u8:?}, shfb: {=u8:?}, multg: {=u8:?}, shfg: {=u8:?} }}",
                self.multb(),
                self.shfb(),
                self.multg(),
                self.shfg()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1fcr(pub u32);
    impl P1fcr {
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clinef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_clinef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cframef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cframef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cvsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn covrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_covrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P1fcr {
        #[inline(always)]
        fn default() -> P1fcr {
            P1fcr(0)
        }
    }
    impl core::fmt::Debug for P1fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1fcr")
                .field("clinef", &self.clinef())
                .field("cframef", &self.cframef())
                .field("cvsyncf", &self.cvsyncf())
                .field("covrf", &self.covrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1fcr {{ clinef: {=bool:?}, cframef: {=bool:?}, cvsyncf: {=bool:?}, covrf: {=bool:?} }}",
                self.clinef(),
                self.cframef(),
                self.cvsyncf(),
                self.covrf()
            )
        }
    }
    #[doc = "DCMIPP Pipex flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1fctcr(pub u32);
    impl P1fctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P1fctcr {
        #[inline(always)]
        fn default() -> P1fctcr {
            P1fctcr(0)
        }
    }
    impl core::fmt::Debug for P1fctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1fctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1fctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1fctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1fscr(pub u32);
    impl P1fscr {
        #[doc = "Data type selection ID A."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type selection ID A."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type selection ID B."]
        #[must_use]
        #[inline(always)]
        pub const fn dtidb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type selection ID B."]
        #[inline(always)]
        pub const fn set_dtidb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Differentiates Pipe2 from Pipe1."]
        #[must_use]
        #[inline(always)]
        pub const fn pipediff(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Differentiates Pipe2 from Pipe1."]
        #[inline(always)]
        pub const fn set_pipediff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Force Datatype format."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Force Datatype format."]
        #[inline(always)]
        pub const fn set_fdtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Force Datatype format enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtfen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Force Datatype format enable."]
        #[inline(always)]
        pub const fn set_fdtfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1fscr {
        #[inline(always)]
        fn default() -> P1fscr {
            P1fscr(0)
        }
    }
    impl core::fmt::Debug for P1fscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1fscr")
                .field("dtida", &self.dtida())
                .field("dtidb", &self.dtidb())
                .field("dtmode", &self.dtmode())
                .field("pipediff", &self.pipediff())
                .field("vc", &self.vc())
                .field("fdtf", &self.fdtf())
                .field("fdtfen", &self.fdtfen())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1fscr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1fscr {{ dtida: {=u8:?}, dtidb: {=u8:?}, dtmode: {=u8:?}, pipediff: {=bool:?}, vc: {=u8:?}, fdtf: {=u8:?}, fdtfen: {=bool:?}, pipen: {=bool:?} }}" , self . dtida () , self . dtidb () , self . dtmode () , self . pipediff () , self . vc () , self . fdtf () , self . fdtfen () , self . pipen ())
        }
    }
    #[doc = "DCMIPP Pipex gamma configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1gmcr(pub u32);
    impl P1gmcr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for P1gmcr {
        #[inline(always)]
        fn default() -> P1gmcr {
            P1gmcr(0)
        }
    }
    impl core::fmt::Debug for P1gmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1gmcr").field("enable", &self.enable()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1gmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1gmcr {{ enable: {=bool:?} }}", self.enable())
        }
    }
    #[doc = "DCMIPP Pipe1 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ier(pub u32);
    impl P1ier {
        #[doc = "Multi-line capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lineie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frameie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC interrupt enable."]
        #[inline(always)]
        pub const fn set_vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable."]
        #[inline(always)]
        pub const fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P1ier {
        #[inline(always)]
        fn default() -> P1ier {
            P1ier(0)
        }
    }
    impl core::fmt::Debug for P1ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ier")
                .field("lineie", &self.lineie())
                .field("frameie", &self.frameie())
                .field("vsyncie", &self.vsyncie())
                .field("ovrie", &self.ovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ier {{ lineie: {=bool:?}, frameie: {=bool:?}, vsyncie: {=bool:?}, ovrie: {=bool:?} }}",
                self.lineie(),
                self.frameie(),
                self.vsyncie(),
                self.ovrie()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppcr(pub u32);
    impl P1ppcr {
        #[doc = "Memory format."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Memory format."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Swaps R-vs-B components if RGB, and U-vs-V components if YUV."]
        #[must_use]
        #[inline(always)]
        pub const fn swaprb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps R-vs-B components if RGB, and U-vs-V components if YUV."]
        #[inline(always)]
        pub const fn set_swaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Amount of capture completed lines for LINE Event and Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of capture completed lines for LINE Event and Interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Line multi address wrapping modulo."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawm(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Line multi address wrapping modulo."]
        #[inline(always)]
        pub const fn set_lmawm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[inline(always)]
        pub const fn set_lmawe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for P1ppcr {
        #[inline(always)]
        fn default() -> P1ppcr {
            P1ppcr(0)
        }
    }
    impl core::fmt::Debug for P1ppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppcr")
                .field("format", &self.format())
                .field("swaprb", &self.swaprb())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .field("lmawm", &self.lmawm())
                .field("lmawe", &self.lmawe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1ppcr {{ format: {=u8:?}, swaprb: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?}, lmawm: {=u8:?}, lmawe: {=bool:?} }}" , self . format () , self . swaprb () , self . linemult () , self . dbm () , self . lmawm () , self . lmawe ())
        }
    }
    #[doc = "DCMIPP Pipe1 pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm0ar1(pub u32);
    impl P1ppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm0ar1 {
        #[inline(always)]
        fn default() -> P1ppm0ar1 {
            P1ppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P1ppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe1 pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm0ar2(pub u32);
    impl P1ppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm0ar2 {
        #[inline(always)]
        fn default() -> P1ppm0ar2 {
            P1ppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P1ppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory0 pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm0pr(pub u32);
    impl P1ppm0pr {
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for P1ppm0pr {
        #[inline(always)]
        fn default() -> P1ppm0pr {
            P1ppm0pr(0)
        }
    }
    impl core::fmt::Debug for P1ppm0pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm0pr").field("pitch", &self.pitch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm0pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm0pr {{ pitch: {=u16:?} }}", self.pitch())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm1ar1(pub u32);
    impl P1ppm1ar1 {
        #[doc = "Memory1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory1 address."]
        #[inline(always)]
        pub const fn set_m1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm1ar1 {
        #[inline(always)]
        fn default() -> P1ppm1ar1 {
            P1ppm1ar1(0)
        }
    }
    impl core::fmt::Debug for P1ppm1ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm1ar1").field("m1a", &self.m1a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm1ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm1ar1 {{ m1a: {=u32:?} }}", self.m1a())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm1ar2(pub u32);
    impl P1ppm1ar2 {
        #[doc = "Memory1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory1 address."]
        #[inline(always)]
        pub const fn set_m1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm1ar2 {
        #[inline(always)]
        fn default() -> P1ppm1ar2 {
            P1ppm1ar2(0)
        }
    }
    impl core::fmt::Debug for P1ppm1ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm1ar2").field("m1a", &self.m1a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm1ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm1ar2 {{ m1a: {=u32:?} }}", self.m1a())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory1 pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm1pr(pub u32);
    impl P1ppm1pr {
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for P1ppm1pr {
        #[inline(always)]
        fn default() -> P1ppm1pr {
            P1ppm1pr(0)
        }
    }
    impl core::fmt::Debug for P1ppm1pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm1pr").field("pitch", &self.pitch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm1pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm1pr {{ pitch: {=u16:?} }}", self.pitch())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer memory2 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm2ar1(pub u32);
    impl P1ppm2ar1 {
        #[doc = "Memory 2 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory 2 address."]
        #[inline(always)]
        pub const fn set_m2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm2ar1 {
        #[inline(always)]
        fn default() -> P1ppm2ar1 {
            P1ppm2ar1(0)
        }
    }
    impl core::fmt::Debug for P1ppm2ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm2ar1").field("m2a", &self.m2a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm2ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm2ar1 {{ m2a: {=u32:?} }}", self.m2a())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer memory2 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ppm2ar2(pub u32);
    impl P1ppm2ar2 {
        #[doc = "Memory 2 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory 2 address."]
        #[inline(always)]
        pub const fn set_m2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1ppm2ar2 {
        #[inline(always)]
        fn default() -> P1ppm2ar2 {
            P1ppm2ar2(0)
        }
    }
    impl core::fmt::Debug for P1ppm2ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ppm2ar2").field("m2a", &self.m2a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ppm2ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1ppm2ar2 {{ m2a: {=u32:?} }}", self.m2a())
        }
    }
    #[doc = "DCMIPP Pipe1 ROI1 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri1cr1(pub u32);
    impl P1ri1cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri1cr1 {
        #[inline(always)]
        fn default() -> P1ri1cr1 {
            P1ri1cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri1cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri1cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri1cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri1cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI1 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri1cr2(pub u32);
    impl P1ri1cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri1cr2 {
        #[inline(always)]
        fn default() -> P1ri1cr2 {
            P1ri1cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri1cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri1cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri1cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri1cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI2 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri2cr1(pub u32);
    impl P1ri2cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri2cr1 {
        #[inline(always)]
        fn default() -> P1ri2cr1 {
            P1ri2cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri2cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri2cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri2cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri2cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri2cr2(pub u32);
    impl P1ri2cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri2cr2 {
        #[inline(always)]
        fn default() -> P1ri2cr2 {
            P1ri2cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri2cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri2cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri2cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri2cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI3 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri3cr1(pub u32);
    impl P1ri3cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri3cr1 {
        #[inline(always)]
        fn default() -> P1ri3cr1 {
            P1ri3cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri3cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri3cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri3cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri3cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI3 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri3cr2(pub u32);
    impl P1ri3cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri3cr2 {
        #[inline(always)]
        fn default() -> P1ri3cr2 {
            P1ri3cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri3cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri3cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri3cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri3cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI4 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri4cr1(pub u32);
    impl P1ri4cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri4cr1 {
        #[inline(always)]
        fn default() -> P1ri4cr1 {
            P1ri4cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri4cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri4cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri4cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri4cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI4 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri4cr2(pub u32);
    impl P1ri4cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri4cr2 {
        #[inline(always)]
        fn default() -> P1ri4cr2 {
            P1ri4cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri4cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri4cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri4cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri4cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI5 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri5cr1(pub u32);
    impl P1ri5cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri5cr1 {
        #[inline(always)]
        fn default() -> P1ri5cr1 {
            P1ri5cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri5cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri5cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri5cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri5cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI5 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri5cr2(pub u32);
    impl P1ri5cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri5cr2 {
        #[inline(always)]
        fn default() -> P1ri5cr2 {
            P1ri5cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri5cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri5cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri5cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri5cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI6 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri6cr1(pub u32);
    impl P1ri6cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri6cr1 {
        #[inline(always)]
        fn default() -> P1ri6cr1 {
            P1ri6cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri6cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri6cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri6cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri6cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI6 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri6cr2(pub u32);
    impl P1ri6cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri6cr2 {
        #[inline(always)]
        fn default() -> P1ri6cr2 {
            P1ri6cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri6cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri6cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri6cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri6cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI7 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri7cr1(pub u32);
    impl P1ri7cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri7cr1 {
        #[inline(always)]
        fn default() -> P1ri7cr1 {
            P1ri7cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri7cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri7cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri7cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri7cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI7 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri7cr2(pub u32);
    impl P1ri7cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri7cr2 {
        #[inline(always)]
        fn default() -> P1ri7cr2 {
            P1ri7cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri7cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri7cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri7cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri7cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI8 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri8cr1(pub u32);
    impl P1ri8cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P1ri8cr1 {
        #[inline(always)]
        fn default() -> P1ri8cr1 {
            P1ri8cr1(0)
        }
    }
    impl core::fmt::Debug for P1ri8cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri8cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri8cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri8cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 ROI8 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ri8cr2(pub u32);
    impl P1ri8cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ri8cr2 {
        #[inline(always)]
        fn default() -> P1ri8cr2 {
            P1ri8cr2(0)
        }
    }
    impl core::fmt::Debug for P1ri8cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ri8cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ri8cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ri8cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1sr(pub u32);
    impl P1sr {
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn linef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn framef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status."]
        #[inline(always)]
        pub const fn set_vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status."]
        #[inline(always)]
        pub const fn set_ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[must_use]
        #[inline(always)]
        pub const fn lstline(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[inline(always)]
        pub const fn set_lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number, which can be delivered by the camera through the CSI2 interface."]
        #[must_use]
        #[inline(always)]
        pub const fn lstfrm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number, which can be delivered by the camera through the CSI2 interface."]
        #[inline(always)]
        pub const fn set_lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Capture immediate status."]
        #[must_use]
        #[inline(always)]
        pub const fn cptact(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Capture immediate status."]
        #[inline(always)]
        pub const fn set_cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P1sr {
        #[inline(always)]
        fn default() -> P1sr {
            P1sr(0)
        }
    }
    impl core::fmt::Debug for P1sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1sr")
                .field("linef", &self.linef())
                .field("framef", &self.framef())
                .field("vsyncf", &self.vsyncf())
                .field("ovrf", &self.ovrf())
                .field("lstline", &self.lstline())
                .field("lstfrm", &self.lstfrm())
                .field("cptact", &self.cptact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P1sr {{ linef: {=bool:?}, framef: {=bool:?}, vsyncf: {=bool:?}, ovrf: {=bool:?}, lstline: {=bool:?}, lstfrm: {=bool:?}, cptact: {=bool:?} }}" , self . linef () , self . framef () , self . vsyncf () , self . ovrf () , self . lstline () , self . lstfrm () , self . cptact ())
        }
    }
    #[doc = "DCMIPP Pipe1 stat removal configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1srcr(pub u32);
    impl P1srcr {
        #[doc = "Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through."]
        #[must_use]
        #[inline(always)]
        pub const fn lastline(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through."]
        #[inline(always)]
        pub const fn set_lastline(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Amount of first lines to delete when CROPEN = 1."]
        #[must_use]
        #[inline(always)]
        pub const fn firstlinedel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of first lines to delete when CROPEN = 1."]
        #[inline(always)]
        pub const fn set_firstlinedel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Crop line enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cropen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Crop line enable."]
        #[inline(always)]
        pub const fn set_cropen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for P1srcr {
        #[inline(always)]
        fn default() -> P1srcr {
            P1srcr(0)
        }
    }
    impl core::fmt::Debug for P1srcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1srcr")
                .field("lastline", &self.lastline())
                .field("firstlinedel", &self.firstlinedel())
                .field("cropen", &self.cropen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1srcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1srcr {{ lastline: {=u16:?}, firstlinedel: {=u8:?}, cropen: {=bool:?} }}",
                self.lastline(),
                self.firstlinedel(),
                self.cropen()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 statistics1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st1cr(pub u32);
    impl P1st1cr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Current bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Current bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Statistics source."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Statistics source."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P1st1cr {
        #[inline(always)]
        fn default() -> P1st1cr {
            P1st1cr(0)
        }
    }
    impl core::fmt::Debug for P1st1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st1cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1st1cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 statistics 1 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st1sr(pub u32);
    impl P1st1sr {
        #[doc = "Accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for P1st1sr {
        #[inline(always)]
        fn default() -> P1st1sr {
            P1st1sr(0)
        }
    }
    impl core::fmt::Debug for P1st1sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st1sr").field("accu", &self.accu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st1sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1st1sr {{ accu: {=u32:?} }}", self.accu())
        }
    }
    #[doc = "DCMIPP Pipe1 statistics 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st2cr(pub u32);
    impl P1st2cr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Statistics source."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Statistics source."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P1st2cr {
        #[inline(always)]
        fn default() -> P1st2cr {
            P1st2cr(0)
        }
    }
    impl core::fmt::Debug for P1st2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st2cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1st2cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 statistics 2 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st2sr(pub u32);
    impl P1st2sr {
        #[doc = "accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for P1st2sr {
        #[inline(always)]
        fn default() -> P1st2sr {
            P1st2sr(0)
        }
    }
    impl core::fmt::Debug for P1st2sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st2sr").field("accu", &self.accu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st2sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1st2sr {{ accu: {=u32:?} }}", self.accu())
        }
    }
    #[doc = "DCMIPP Pipe1 statistics 3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st3cr(pub u32);
    impl P1st3cr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bin definition."]
        #[must_use]
        #[inline(always)]
        pub const fn bins(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Bin definition."]
        #[inline(always)]
        pub const fn set_bins(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Statistics source."]
        #[must_use]
        #[inline(always)]
        pub const fn src(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Statistics source."]
        #[inline(always)]
        pub const fn set_src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Statistics mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Statistics mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P1st3cr {
        #[inline(always)]
        fn default() -> P1st3cr {
            P1st3cr(0)
        }
    }
    impl core::fmt::Debug for P1st3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st3cr")
                .field("enable", &self.enable())
                .field("bins", &self.bins())
                .field("src", &self.src())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1st3cr {{ enable: {=bool:?}, bins: {=u8:?}, src: {=u8:?}, mode: {=bool:?} }}",
                self.enable(),
                self.bins(),
                self.src(),
                self.mode()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 statistics 3 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1st3sr(pub u32);
    impl P1st3sr {
        #[doc = "accumulation result, divided by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn accu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "accumulation result, divided by 256."]
        #[inline(always)]
        pub const fn set_accu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for P1st3sr {
        #[inline(always)]
        fn default() -> P1st3sr {
            P1st3sr(0)
        }
    }
    impl core::fmt::Debug for P1st3sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1st3sr").field("accu", &self.accu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1st3sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1st3sr {{ accu: {=u32:?} }}", self.accu())
        }
    }
    #[doc = "DCMIPP Pipex status Memory0 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1stm0ar(pub u32);
    impl P1stm0ar {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1stm0ar {
        #[inline(always)]
        fn default() -> P1stm0ar {
            P1stm0ar(0)
        }
    }
    impl core::fmt::Debug for P1stm0ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1stm0ar").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1stm0ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1stm0ar {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipex status Memory1 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1stm1ar(pub u32);
    impl P1stm1ar {
        #[doc = "Memory1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory1 address."]
        #[inline(always)]
        pub const fn set_m1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1stm1ar {
        #[inline(always)]
        fn default() -> P1stm1ar {
            P1stm1ar(0)
        }
    }
    impl core::fmt::Debug for P1stm1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1stm1ar").field("m1a", &self.m1a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1stm1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1stm1ar {{ m1a: {=u32:?} }}", self.m1a())
        }
    }
    #[doc = "DCMIPP Pipex status Memory2 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1stm2ar(pub u32);
    impl P1stm2ar {
        #[doc = "Memory2 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory2 address."]
        #[inline(always)]
        pub const fn set_m2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P1stm2ar {
        #[inline(always)]
        fn default() -> P1stm2ar {
            P1stm2ar(0)
        }
    }
    impl core::fmt::Debug for P1stm2ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1stm2ar").field("m2a", &self.m2a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1stm2ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1stm2ar {{ m2a: {=u32:?} }}", self.m2a())
        }
    }
    #[doc = "DCMIPP Pipe1 statistics window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1ststr(pub u32);
    impl P1ststr {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P1ststr {
        #[inline(always)]
        fn default() -> P1ststr {
            P1ststr(0)
        }
    }
    impl core::fmt::Debug for P1ststr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1ststr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1ststr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1ststr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 statistics window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1stszr(pub u32);
    impl P1stszr {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn cropen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_cropen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P1stszr {
        #[inline(always)]
        fn default() -> P1stszr {
            P1stszr(0)
        }
    }
    impl core::fmt::Debug for P1stszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1stszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("cropen", &self.cropen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1stszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1stszr {{ hsize: {=u16:?}, vsize: {=u16:?}, cropen: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.cropen()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv blue coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvbr1(pub u32);
    impl P1yuvbr1 {
        #[doc = "Coefficient row 3 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_br(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 3 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_bg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1yuvbr1 {
        #[inline(always)]
        fn default() -> P1yuvbr1 {
            P1yuvbr1(0)
        }
    }
    impl core::fmt::Debug for P1yuvbr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvbr1")
                .field("br", &self.br())
                .field("bg", &self.bg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvbr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvbr1 {{ br: {=u16:?}, bg: {=u16:?} }}", self.br(), self.bg())
        }
    }
    #[doc = "DCMIPP Pipe1 YUV blue coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvbr2(pub u32);
    impl P1yuvbr2 {
        #[doc = "Coefficient row 3 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn bb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 3 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_bb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 3 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ba(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 3 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1yuvbr2 {
        #[inline(always)]
        fn default() -> P1yuvbr2 {
            P1yuvbr2(0)
        }
    }
    impl core::fmt::Debug for P1yuvbr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvbr2")
                .field("bb", &self.bb())
                .field("ba", &self.ba())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvbr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvbr2 {{ bb: {=u16:?}, ba: {=u16:?} }}", self.bb(), self.ba())
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvcr(pub u32);
    impl P1yuvcr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output samples type used while CLAMP is activated."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output samples type used while CLAMP is activated."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clamp the output samples."]
        #[must_use]
        #[inline(always)]
        pub const fn clamp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clamp the output samples."]
        #[inline(always)]
        pub const fn set_clamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for P1yuvcr {
        #[inline(always)]
        fn default() -> P1yuvcr {
            P1yuvcr(0)
        }
    }
    impl core::fmt::Debug for P1yuvcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvcr")
                .field("enable", &self.enable())
                .field("type_", &self.type_())
                .field("clamp", &self.clamp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P1yuvcr {{ enable: {=bool:?}, type_: {=bool:?}, clamp: {=bool:?} }}",
                self.enable(),
                self.type_(),
                self.clamp()
            )
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv green coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvgr1(pub u32);
    impl P1yuvgr1 {
        #[doc = "Coefficient row 2 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_gr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 2 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_gg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1yuvgr1 {
        #[inline(always)]
        fn default() -> P1yuvgr1 {
            P1yuvgr1(0)
        }
    }
    impl core::fmt::Debug for P1yuvgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvgr1")
                .field("gr", &self.gr())
                .field("gg", &self.gg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvgr1 {{ gr: {=u16:?}, gg: {=u16:?} }}", self.gr(), self.gg())
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv green coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvgr2(pub u32);
    impl P1yuvgr2 {
        #[doc = "Coefficient row 2 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn gb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 2 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_gb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 2 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ga(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 2 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ga(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1yuvgr2 {
        #[inline(always)]
        fn default() -> P1yuvgr2 {
            P1yuvgr2(0)
        }
    }
    impl core::fmt::Debug for P1yuvgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvgr2")
                .field("gb", &self.gb())
                .field("ga", &self.ga())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvgr2 {{ gb: {=u16:?}, ga: {=u16:?} }}", self.gb(), self.ga())
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv red coefficient register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvrr1(pub u32);
    impl P1yuvrr1 {
        #[doc = "Coefficient row 1 column 1 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 1 of the matrix."]
        #[inline(always)]
        pub const fn set_rr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 1 column 2 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 2 of the matrix."]
        #[inline(always)]
        pub const fn set_rg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for P1yuvrr1 {
        #[inline(always)]
        fn default() -> P1yuvrr1 {
            P1yuvrr1(0)
        }
    }
    impl core::fmt::Debug for P1yuvrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvrr1")
                .field("rr", &self.rr())
                .field("rg", &self.rg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvrr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvrr1 {{ rr: {=u16:?}, rg: {=u16:?} }}", self.rr(), self.rg())
        }
    }
    #[doc = "DCMIPP Pipe1 YUVConv red coefficient register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1yuvrr2(pub u32);
    impl P1yuvrr2 {
        #[doc = "Coefficient row 1 column 3 of the matrix."]
        #[must_use]
        #[inline(always)]
        pub const fn rb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Coefficient row 1 column 3 of the matrix."]
        #[inline(always)]
        pub const fn set_rb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Coefficient row 1 of the added column (signed integer value)."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Coefficient row 1 of the added column (signed integer value)."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for P1yuvrr2 {
        #[inline(always)]
        fn default() -> P1yuvrr2 {
            P1yuvrr2(0)
        }
    }
    impl core::fmt::Debug for P1yuvrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P1yuvrr2")
                .field("rb", &self.rb())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P1yuvrr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P1yuvrr2 {{ rb: {=u16:?}, ra: {=u16:?} }}", self.rb(), self.ra())
        }
    }
    #[doc = "DCMIPP Pipex current common ROI configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ccmricr(pub u32);
    impl P2ccmricr {
        #[doc = "Current region of interest line size width."]
        #[must_use]
        #[inline(always)]
        pub const fn roilsz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Current region of interest line size width."]
        #[inline(always)]
        pub const fn set_roilsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Current region of interest 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 1 enable."]
        #[inline(always)]
        pub const fn set_roi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Current region of interest 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 2 enable."]
        #[inline(always)]
        pub const fn set_roi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Current region of interest 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 3 enable."]
        #[inline(always)]
        pub const fn set_roi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Current region of interest 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 4 enable."]
        #[inline(always)]
        pub const fn set_roi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Current region of interest 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 5 enable."]
        #[inline(always)]
        pub const fn set_roi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Current region of interest 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi6en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 6 enable."]
        #[inline(always)]
        pub const fn set_roi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Current region of interest 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi7en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 7 enable."]
        #[inline(always)]
        pub const fn set_roi7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Current region of interest 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi8en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Current region of interest 8 enable."]
        #[inline(always)]
        pub const fn set_roi8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P2ccmricr {
        #[inline(always)]
        fn default() -> P2ccmricr {
            P2ccmricr(0)
        }
    }
    impl core::fmt::Debug for P2ccmricr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ccmricr")
                .field("roilsz", &self.roilsz())
                .field("roi1en", &self.roi1en())
                .field("roi2en", &self.roi2en())
                .field("roi3en", &self.roi3en())
                .field("roi4en", &self.roi4en())
                .field("roi5en", &self.roi5en())
                .field("roi6en", &self.roi6en())
                .field("roi7en", &self.roi7en())
                .field("roi8en", &self.roi8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ccmricr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P2ccmricr {{ roilsz: {=u8:?}, roi1en: {=bool:?}, roi2en: {=bool:?}, roi3en: {=bool:?}, roi4en: {=bool:?}, roi5en: {=bool:?}, roi6en: {=bool:?}, roi7en: {=bool:?}, roi8en: {=bool:?} }}" , self . roilsz () , self . roi1en () , self . roi2en () , self . roi3en () , self . roi4en () , self . roi5en () , self . roi6en () , self . roi7en () , self . roi8en ())
        }
    }
    #[doc = "DCMIPP Pipex current crop window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ccrstr(pub u32);
    impl P2ccrstr {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ccrstr {
        #[inline(always)]
        fn default() -> P2ccrstr {
            P2ccrstr(0)
        }
    }
    impl core::fmt::Debug for P2ccrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ccrstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ccrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ccrstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipex current crop window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ccrszr(pub u32);
    impl P2ccrszr {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current ENABLE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current ENABLE bit value."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2ccrszr {
        #[inline(always)]
        fn default() -> P2ccrszr {
            P2ccrszr(0)
        }
    }
    impl core::fmt::Debug for P2ccrszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ccrszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ccrszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ccrszr {{ hsize: {=u16:?}, vsize: {=u16:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex current decimation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cdccr(pub u32);
    impl P2cdccr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn hdec(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal decimation ratio."]
        #[inline(always)]
        pub const fn set_hdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Vertical decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn vdec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Vertical decimation ratio."]
        #[inline(always)]
        pub const fn set_vdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for P2cdccr {
        #[inline(always)]
        fn default() -> P2cdccr {
            P2cdccr(0)
        }
    }
    impl core::fmt::Debug for P2cdccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cdccr")
                .field("enable", &self.enable())
                .field("hdec", &self.hdec())
                .field("vdec", &self.vdec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cdccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cdccr {{ enable: {=bool:?}, hdec: {=u8:?}, vdec: {=u8:?} }}",
                self.enable(),
                self.hdec(),
                self.vdec()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cdscr(pub u32);
    impl P2cdscr {
        #[doc = "Current horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_hdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vdiv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_vdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Current value of bit ENABLE."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current value of bit ENABLE."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2cdscr {
        #[inline(always)]
        fn default() -> P2cdscr {
            P2cdscr(0)
        }
    }
    impl core::fmt::Debug for P2cdscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cdscr")
                .field("hdiv", &self.hdiv())
                .field("vdiv", &self.vdiv())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cdscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cdscr {{ hdiv: {=u16:?}, vdiv: {=u16:?}, enable: {=bool:?} }}",
                self.hdiv(),
                self.vdiv(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize ratio register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cdsrtior(pub u32);
    impl P2cdsrtior {
        #[doc = "Current horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hratio(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_hratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vratio(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_vratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for P2cdsrtior {
        #[inline(always)]
        fn default() -> P2cdsrtior {
            P2cdsrtior(0)
        }
    }
    impl core::fmt::Debug for P2cdsrtior {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cdsrtior")
                .field("hratio", &self.hratio())
                .field("vratio", &self.vratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cdsrtior {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cdsrtior {{ hratio: {=u16:?}, vratio: {=u16:?} }}",
                self.hratio(),
                self.vratio()
            )
        }
    }
    #[doc = "DCMIPP Pipex current downsize destination size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cdsszr(pub u32);
    impl P2cdsszr {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cdsszr {
        #[inline(always)]
        fn default() -> P2cdsszr {
            P2cdsszr(0)
        }
    }
    impl core::fmt::Debug for P2cdsszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cdsszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cdsszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cdsszr {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipex current flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cfctcr(pub u32);
    impl P2cfctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P2cfctcr {
        #[inline(always)]
        fn default() -> P2cfctcr {
            P2cfctcr(0)
        }
    }
    impl core::fmt::Debug for P2cfctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cfctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cfctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cfctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cfscr(pub u32);
    impl P2cfscr {
        #[doc = "Current data type ID."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Current data type ID."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Current flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Current flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Current force data type format."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Current force data type format."]
        #[inline(always)]
        pub const fn set_fdtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Current force data type format enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtfen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Current force data type format enable."]
        #[inline(always)]
        pub const fn set_fdtfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Current activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Current activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2cfscr {
        #[inline(always)]
        fn default() -> P2cfscr {
            P2cfscr(0)
        }
    }
    impl core::fmt::Debug for P2cfscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cfscr")
                .field("dtida", &self.dtida())
                .field("vc", &self.vc())
                .field("fdtf", &self.fdtf())
                .field("fdtfen", &self.fdtfen())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cfscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cfscr {{ dtida: {=u8:?}, vc: {=u8:?}, fdtf: {=u8:?}, fdtfen: {=bool:?}, pipen: {=bool:?} }}",
                self.dtida(),
                self.vc(),
                self.fdtf(),
                self.fdtfen(),
                self.pipen()
            )
        }
    }
    #[doc = "DCMIPP Pipex common ROI configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cmricr(pub u32);
    impl P2cmricr {
        #[doc = "Region of interest line size width."]
        #[must_use]
        #[inline(always)]
        pub const fn roilsz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Region of interest line size width."]
        #[inline(always)]
        pub const fn set_roilsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Region of interest 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 1 enable."]
        #[inline(always)]
        pub const fn set_roi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Region of interest 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 2 enable."]
        #[inline(always)]
        pub const fn set_roi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Region of interest 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 3 enable."]
        #[inline(always)]
        pub const fn set_roi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Region of interest 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 4 enable."]
        #[inline(always)]
        pub const fn set_roi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Region of interest 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 5 enable."]
        #[inline(always)]
        pub const fn set_roi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Region of interest 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi6en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 6 enable."]
        #[inline(always)]
        pub const fn set_roi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Region of interest 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi7en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 7 enable."]
        #[inline(always)]
        pub const fn set_roi7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Region of interest 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roi8en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Region of interest 8 enable."]
        #[inline(always)]
        pub const fn set_roi8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P2cmricr {
        #[inline(always)]
        fn default() -> P2cmricr {
            P2cmricr(0)
        }
    }
    impl core::fmt::Debug for P2cmricr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cmricr")
                .field("roilsz", &self.roilsz())
                .field("roi1en", &self.roi1en())
                .field("roi2en", &self.roi2en())
                .field("roi3en", &self.roi3en())
                .field("roi4en", &self.roi4en())
                .field("roi5en", &self.roi5en())
                .field("roi6en", &self.roi6en())
                .field("roi7en", &self.roi7en())
                .field("roi8en", &self.roi8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cmricr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P2cmricr {{ roilsz: {=u8:?}, roi1en: {=bool:?}, roi2en: {=bool:?}, roi3en: {=bool:?}, roi4en: {=bool:?}, roi5en: {=bool:?}, roi6en: {=bool:?}, roi7en: {=bool:?}, roi8en: {=bool:?} }}" , self . roilsz () , self . roi1en () , self . roi2en () , self . roi3en () , self . roi4en () , self . roi5en () , self . roi6en () , self . roi7en () , self . roi8en ())
        }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cppcr(pub u32);
    impl P2cppcr {
        #[doc = "Memory format (only coplanar formats are supported in Pipe2)."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Memory format (only coplanar formats are supported in Pipe2)."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components."]
        #[must_use]
        #[inline(always)]
        pub const fn swaprb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components."]
        #[inline(always)]
        pub const fn set_swaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Line multi address wrapping modulo."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawm(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Line multi address wrapping modulo."]
        #[inline(always)]
        pub const fn set_lmawm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[inline(always)]
        pub const fn set_lmawe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for P2cppcr {
        #[inline(always)]
        fn default() -> P2cppcr {
            P2cppcr(0)
        }
    }
    impl core::fmt::Debug for P2cppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cppcr")
                .field("format", &self.format())
                .field("swaprb", &self.swaprb())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .field("lmawm", &self.lmawm())
                .field("lmawe", &self.lmawe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P2cppcr {{ format: {=u8:?}, swaprb: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?}, lmawm: {=u8:?}, lmawe: {=bool:?} }}" , self . format () , self . swaprb () , self . linemult () , self . dbm () , self . lmawm () , self . lmawe ())
        }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cppm0ar1(pub u32);
    impl P2cppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2cppm0ar1 {
        #[inline(always)]
        fn default() -> P2cppm0ar1 {
            P2cppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P2cppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2cppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe2 current pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cppm0ar2(pub u32);
    impl P2cppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2cppm0ar2 {
        #[inline(always)]
        fn default() -> P2cppm0ar2 {
            P2cppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P2cppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2cppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI1 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri1cr1(pub u32);
    impl P2cri1cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri1cr1 {
        #[inline(always)]
        fn default() -> P2cri1cr1 {
            P2cri1cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri1cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri1cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri1cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri1cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI1 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri1cr2(pub u32);
    impl P2cri1cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri1cr2 {
        #[inline(always)]
        fn default() -> P2cri1cr2 {
            P2cri1cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri1cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri1cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri1cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri1cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI2 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri2cr1(pub u32);
    impl P2cri2cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri2cr1 {
        #[inline(always)]
        fn default() -> P2cri2cr1 {
            P2cri2cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri2cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri2cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri2cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri2cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri2cr2(pub u32);
    impl P2cri2cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri2cr2 {
        #[inline(always)]
        fn default() -> P2cri2cr2 {
            P2cri2cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri2cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri2cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri2cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri2cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI3 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri3cr1(pub u32);
    impl P2cri3cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri3cr1 {
        #[inline(always)]
        fn default() -> P2cri3cr1 {
            P2cri3cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri3cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri3cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri3cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri3cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI3 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri3cr2(pub u32);
    impl P2cri3cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri3cr2 {
        #[inline(always)]
        fn default() -> P2cri3cr2 {
            P2cri3cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri3cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri3cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri3cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri3cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI4 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri4cr1(pub u32);
    impl P2cri4cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri4cr1 {
        #[inline(always)]
        fn default() -> P2cri4cr1 {
            P2cri4cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri4cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri4cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri4cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri4cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI4 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri4cr2(pub u32);
    impl P2cri4cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri4cr2 {
        #[inline(always)]
        fn default() -> P2cri4cr2 {
            P2cri4cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri4cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri4cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri4cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri4cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI5 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri5cr1(pub u32);
    impl P2cri5cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri5cr1 {
        #[inline(always)]
        fn default() -> P2cri5cr1 {
            P2cri5cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri5cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri5cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri5cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri5cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI5 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri5cr2(pub u32);
    impl P2cri5cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri5cr2 {
        #[inline(always)]
        fn default() -> P2cri5cr2 {
            P2cri5cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri5cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri5cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri5cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri5cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI6 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri6cr1(pub u32);
    impl P2cri6cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri6cr1 {
        #[inline(always)]
        fn default() -> P2cri6cr1 {
            P2cri6cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri6cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri6cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri6cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri6cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI6 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri6cr2(pub u32);
    impl P2cri6cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri6cr2 {
        #[inline(always)]
        fn default() -> P2cri6cr2 {
            P2cri6cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri6cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri6cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri6cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri6cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI7 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri7cr1(pub u32);
    impl P2cri7cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri7cr1 {
        #[inline(always)]
        fn default() -> P2cri7cr1 {
            P2cri7cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri7cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri7cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri7cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri7cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI7 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri7cr2(pub u32);
    impl P2cri7cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri7cr2 {
        #[inline(always)]
        fn default() -> P2cri7cr2 {
            P2cri7cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri7cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri7cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri7cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri7cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI8 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri8cr1(pub u32);
    impl P2cri8cr1 {
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Current color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Current color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Current color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2cri8cr1 {
        #[inline(always)]
        fn default() -> P2cri8cr1 {
            P2cri8cr1(0)
        }
    }
    impl core::fmt::Debug for P2cri8cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri8cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri8cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri8cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 current ROI8 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cri8cr2(pub u32);
    impl P2cri8cr2 {
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Current vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2cri8cr2 {
        #[inline(always)]
        fn default() -> P2cri8cr2 {
            P2cri8cr2(0)
        }
    }
    impl core::fmt::Debug for P2cri8cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2cri8cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2cri8cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2cri8cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipex crop window start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2crstr(pub u32);
    impl P2crstr {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2crstr {
        #[inline(always)]
        fn default() -> P2crstr {
            P2crstr(0)
        }
    }
    impl core::fmt::Debug for P2crstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2crstr")
                .field("hstart", &self.hstart())
                .field("vstart", &self.vstart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2crstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2crstr {{ hstart: {=u16:?}, vstart: {=u16:?} }}",
                self.hstart(),
                self.vstart()
            )
        }
    }
    #[doc = "DCMIPP Pipex crop window size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2crszr(pub u32);
    impl P2crszr {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2crszr {
        #[inline(always)]
        fn default() -> P2crszr {
            P2crszr(0)
        }
    }
    impl core::fmt::Debug for P2crszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2crszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2crszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2crszr {{ hsize: {=u16:?}, vsize: {=u16:?}, enable: {=bool:?} }}",
                self.hsize(),
                self.vsize(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex decimation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2dccr(pub u32);
    impl P2dccr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn hdec(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal decimation ratio."]
        #[inline(always)]
        pub const fn set_hdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Vertical decimation ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn vdec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Vertical decimation ratio."]
        #[inline(always)]
        pub const fn set_vdec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
    }
    impl Default for P2dccr {
        #[inline(always)]
        fn default() -> P2dccr {
            P2dccr(0)
        }
    }
    impl core::fmt::Debug for P2dccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2dccr")
                .field("enable", &self.enable())
                .field("hdec", &self.hdec())
                .field("vdec", &self.vdec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2dccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2dccr {{ enable: {=bool:?}, hdec: {=u8:?}, vdec: {=u8:?} }}",
                self.enable(),
                self.hdec(),
                self.vdec()
            )
        }
    }
    #[doc = "DCMIPP Pipex downsize configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2dscr(pub u32);
    impl P2dscr {
        #[doc = "Horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Horizontal division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_hdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vdiv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Vertical division factor, from 128 (8x) to 1023 (1x)."]
        #[inline(always)]
        pub const fn set_vdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2dscr {
        #[inline(always)]
        fn default() -> P2dscr {
            P2dscr(0)
        }
    }
    impl core::fmt::Debug for P2dscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2dscr")
                .field("hdiv", &self.hdiv())
                .field("vdiv", &self.vdiv())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2dscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2dscr {{ hdiv: {=u16:?}, vdiv: {=u16:?}, enable: {=bool:?} }}",
                self.hdiv(),
                self.vdiv(),
                self.enable()
            )
        }
    }
    #[doc = "DCMIPP Pipex downsize ratio register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2dsrtior(pub u32);
    impl P2dsrtior {
        #[doc = "Horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn hratio(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Horizontal ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_hratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[must_use]
        #[inline(always)]
        pub const fn vratio(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Vertical ratio, from 8192 (1x) to 65535 (8x)."]
        #[inline(always)]
        pub const fn set_vratio(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for P2dsrtior {
        #[inline(always)]
        fn default() -> P2dsrtior {
            P2dsrtior(0)
        }
    }
    impl core::fmt::Debug for P2dsrtior {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2dsrtior")
                .field("hratio", &self.hratio())
                .field("vratio", &self.vratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2dsrtior {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2dsrtior {{ hratio: {=u16:?}, vratio: {=u16:?} }}",
                self.hratio(),
                self.vratio()
            )
        }
    }
    #[doc = "DCMIPP Pipex downsize destination size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2dsszr(pub u32);
    impl P2dsszr {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2dsszr {
        #[inline(always)]
        fn default() -> P2dsszr {
            P2dsszr(0)
        }
    }
    impl core::fmt::Debug for P2dsszr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2dsszr")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2dsszr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2dsszr {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2fcr(pub u32);
    impl P2fcr {
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clinef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_clinef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cframef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture complete interrupt status clear."]
        #[inline(always)]
        pub const fn set_cframef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cvsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization interrupt status clear."]
        #[inline(always)]
        pub const fn set_cvsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn covrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt status clear."]
        #[inline(always)]
        pub const fn set_covrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P2fcr {
        #[inline(always)]
        fn default() -> P2fcr {
            P2fcr(0)
        }
    }
    impl core::fmt::Debug for P2fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2fcr")
                .field("clinef", &self.clinef())
                .field("cframef", &self.cframef())
                .field("cvsyncf", &self.cvsyncf())
                .field("covrf", &self.covrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2fcr {{ clinef: {=bool:?}, cframef: {=bool:?}, cvsyncf: {=bool:?}, covrf: {=bool:?} }}",
                self.clinef(),
                self.cframef(),
                self.cvsyncf(),
                self.covrf()
            )
        }
    }
    #[doc = "DCMIPP Pipex flow control configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2fctcr(pub u32);
    impl P2fctcr {
        #[doc = "Frame capture rate control."]
        #[must_use]
        #[inline(always)]
        pub const fn frate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Frame capture rate control."]
        #[inline(always)]
        pub const fn set_frate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Capture mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cptmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture mode."]
        #[inline(always)]
        pub const fn set_cptmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture requested."]
        #[must_use]
        #[inline(always)]
        pub const fn cptreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture requested."]
        #[inline(always)]
        pub const fn set_cptreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for P2fctcr {
        #[inline(always)]
        fn default() -> P2fctcr {
            P2fctcr(0)
        }
    }
    impl core::fmt::Debug for P2fctcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2fctcr")
                .field("frate", &self.frate())
                .field("cptmode", &self.cptmode())
                .field("cptreq", &self.cptreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2fctcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2fctcr {{ frate: {=u8:?}, cptmode: {=bool:?}, cptreq: {=bool:?} }}",
                self.frate(),
                self.cptmode(),
                self.cptreq()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 flow selection configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2fscr(pub u32);
    impl P2fscr {
        #[doc = "Data type ID."]
        #[must_use]
        #[inline(always)]
        pub const fn dtida(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type ID."]
        #[inline(always)]
        pub const fn set_dtida(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Flow selection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "Flow selection mode."]
        #[inline(always)]
        pub const fn set_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "Force data type format."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Force data type format."]
        #[inline(always)]
        pub const fn set_fdtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Force data type format enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdtfen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Force data type format enable."]
        #[inline(always)]
        pub const fn set_fdtfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Activation of PipeN."]
        #[must_use]
        #[inline(always)]
        pub const fn pipen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Activation of PipeN."]
        #[inline(always)]
        pub const fn set_pipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for P2fscr {
        #[inline(always)]
        fn default() -> P2fscr {
            P2fscr(0)
        }
    }
    impl core::fmt::Debug for P2fscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2fscr")
                .field("dtida", &self.dtida())
                .field("vc", &self.vc())
                .field("fdtf", &self.fdtf())
                .field("fdtfen", &self.fdtfen())
                .field("pipen", &self.pipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2fscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2fscr {{ dtida: {=u8:?}, vc: {=u8:?}, fdtf: {=u8:?}, fdtfen: {=bool:?}, pipen: {=bool:?} }}",
                self.dtida(),
                self.vc(),
                self.fdtf(),
                self.fdtfen(),
                self.pipen()
            )
        }
    }
    #[doc = "DCMIPP Pipex gamma configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2gmcr(pub u32);
    impl P2gmcr {
        #[doc = "None."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "None."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for P2gmcr {
        #[inline(always)]
        fn default() -> P2gmcr {
            P2gmcr(0)
        }
    }
    impl core::fmt::Debug for P2gmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2gmcr").field("enable", &self.enable()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2gmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2gmcr {{ enable: {=bool:?} }}", self.enable())
        }
    }
    #[doc = "DCMIPP Pipe2 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ier(pub u32);
    impl P2ier {
        #[doc = "Multi-line capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lineie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_lineie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frameie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed interrupt enable."]
        #[inline(always)]
        pub const fn set_frameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC interrupt enable."]
        #[inline(always)]
        pub const fn set_vsyncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable."]
        #[inline(always)]
        pub const fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for P2ier {
        #[inline(always)]
        fn default() -> P2ier {
            P2ier(0)
        }
    }
    impl core::fmt::Debug for P2ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ier")
                .field("lineie", &self.lineie())
                .field("frameie", &self.frameie())
                .field("vsyncie", &self.vsyncie())
                .field("ovrie", &self.ovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ier {{ lineie: {=bool:?}, frameie: {=bool:?}, vsyncie: {=bool:?}, ovrie: {=bool:?} }}",
                self.lineie(),
                self.frameie(),
                self.vsyncie(),
                self.ovrie()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 pixel packer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ppcr(pub u32);
    impl P2ppcr {
        #[doc = "Memory format (only coplanar formats are supported in Pipe2)."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Memory format (only coplanar formats are supported in Pipe2)."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components."]
        #[must_use]
        #[inline(always)]
        pub const fn swaprb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components."]
        #[inline(always)]
        pub const fn set_swaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn linemult(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Amount of capture completed lines for LINE event and interrupt."]
        #[inline(always)]
        pub const fn set_linemult(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Double buffer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Double buffer mode."]
        #[inline(always)]
        pub const fn set_dbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Line multi address wrapping modulo."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawm(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Line multi address wrapping modulo."]
        #[inline(always)]
        pub const fn set_lmawm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lmawe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Line multi address wrapping enable bit."]
        #[inline(always)]
        pub const fn set_lmawe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for P2ppcr {
        #[inline(always)]
        fn default() -> P2ppcr {
            P2ppcr(0)
        }
    }
    impl core::fmt::Debug for P2ppcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ppcr")
                .field("format", &self.format())
                .field("swaprb", &self.swaprb())
                .field("linemult", &self.linemult())
                .field("dbm", &self.dbm())
                .field("lmawm", &self.lmawm())
                .field("lmawe", &self.lmawe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ppcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P2ppcr {{ format: {=u8:?}, swaprb: {=bool:?}, linemult: {=u8:?}, dbm: {=bool:?}, lmawm: {=u8:?}, lmawe: {=bool:?} }}" , self . format () , self . swaprb () , self . linemult () , self . dbm () , self . lmawm () , self . lmawe ())
        }
    }
    #[doc = "DCMIPP Pipe2 pixel packer Memory0 address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ppm0ar1(pub u32);
    impl P2ppm0ar1 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2ppm0ar1 {
        #[inline(always)]
        fn default() -> P2ppm0ar1 {
            P2ppm0ar1(0)
        }
    }
    impl core::fmt::Debug for P2ppm0ar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ppm0ar1").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ppm0ar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2ppm0ar1 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipe2 pixel packer Memory0 address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ppm0ar2(pub u32);
    impl P2ppm0ar2 {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2ppm0ar2 {
        #[inline(always)]
        fn default() -> P2ppm0ar2 {
            P2ppm0ar2(0)
        }
    }
    impl core::fmt::Debug for P2ppm0ar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ppm0ar2").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ppm0ar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2ppm0ar2 {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP Pipex pixel packer Memory0 pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ppm0pr(pub u32);
    impl P2ppm0pr {
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Number of bytes between the address of two consecutive lines."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for P2ppm0pr {
        #[inline(always)]
        fn default() -> P2ppm0pr {
            P2ppm0pr(0)
        }
    }
    impl core::fmt::Debug for P2ppm0pr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ppm0pr").field("pitch", &self.pitch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ppm0pr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2ppm0pr {{ pitch: {=u16:?} }}", self.pitch())
        }
    }
    #[doc = "DCMIPP Pipe2 ROI1 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri1cr1(pub u32);
    impl P2ri1cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri1cr1 {
        #[inline(always)]
        fn default() -> P2ri1cr1 {
            P2ri1cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri1cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri1cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri1cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri1cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI1 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri1cr2(pub u32);
    impl P2ri1cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri1cr2 {
        #[inline(always)]
        fn default() -> P2ri1cr2 {
            P2ri1cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri1cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri1cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri1cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri1cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI2 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri2cr1(pub u32);
    impl P2ri2cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri2cr1 {
        #[inline(always)]
        fn default() -> P2ri2cr1 {
            P2ri2cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri2cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri2cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri2cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri2cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri2cr2(pub u32);
    impl P2ri2cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri2cr2 {
        #[inline(always)]
        fn default() -> P2ri2cr2 {
            P2ri2cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri2cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri2cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri2cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri2cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI3 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri3cr1(pub u32);
    impl P2ri3cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri3cr1 {
        #[inline(always)]
        fn default() -> P2ri3cr1 {
            P2ri3cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri3cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri3cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri3cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri3cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI3 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri3cr2(pub u32);
    impl P2ri3cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri3cr2 {
        #[inline(always)]
        fn default() -> P2ri3cr2 {
            P2ri3cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri3cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri3cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri3cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri3cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI4 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri4cr1(pub u32);
    impl P2ri4cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri4cr1 {
        #[inline(always)]
        fn default() -> P2ri4cr1 {
            P2ri4cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri4cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri4cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri4cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri4cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI4 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri4cr2(pub u32);
    impl P2ri4cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri4cr2 {
        #[inline(always)]
        fn default() -> P2ri4cr2 {
            P2ri4cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri4cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri4cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri4cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri4cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI5 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri5cr1(pub u32);
    impl P2ri5cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri5cr1 {
        #[inline(always)]
        fn default() -> P2ri5cr1 {
            P2ri5cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri5cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri5cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri5cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri5cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI5 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri5cr2(pub u32);
    impl P2ri5cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri5cr2 {
        #[inline(always)]
        fn default() -> P2ri5cr2 {
            P2ri5cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri5cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri5cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri5cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri5cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI6 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri6cr1(pub u32);
    impl P2ri6cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri6cr1 {
        #[inline(always)]
        fn default() -> P2ri6cr1 {
            P2ri6cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri6cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri6cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri6cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri6cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI6 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri6cr2(pub u32);
    impl P2ri6cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri6cr2 {
        #[inline(always)]
        fn default() -> P2ri6cr2 {
            P2ri6cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri6cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri6cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri6cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri6cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI7 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri7cr1(pub u32);
    impl P2ri7cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri7cr1 {
        #[inline(always)]
        fn default() -> P2ri7cr1 {
            P2ri7cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri7cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri7cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri7cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri7cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI7 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri7cr2(pub u32);
    impl P2ri7cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri7cr2 {
        #[inline(always)]
        fn default() -> P2ri7cr2 {
            P2ri7cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri7cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri7cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri7cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri7cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI8 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri8cr1(pub u32);
    impl P2ri8cr1 {
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hstart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal start, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Color line blue."]
        #[must_use]
        #[inline(always)]
        pub const fn clb(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Color line blue."]
        #[inline(always)]
        pub const fn set_clb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Color line green."]
        #[must_use]
        #[inline(always)]
        pub const fn clg(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Color line green."]
        #[inline(always)]
        pub const fn set_clg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vstart(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical start, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vstart(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Color line red."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Color line red."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for P2ri8cr1 {
        #[inline(always)]
        fn default() -> P2ri8cr1 {
            P2ri8cr1(0)
        }
    }
    impl core::fmt::Debug for P2ri8cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri8cr1")
                .field("hstart", &self.hstart())
                .field("clb", &self.clb())
                .field("clg", &self.clg())
                .field("vstart", &self.vstart())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri8cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri8cr1 {{ hstart: {=u16:?}, clb: {=u8:?}, clg: {=u8:?}, vstart: {=u16:?}, clr: {=u8:?} }}",
                self.hstart(),
                self.clb(),
                self.clg(),
                self.vstart(),
                self.clr()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 ROI8 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2ri8cr2(pub u32);
    impl P2ri8cr2 {
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[must_use]
        #[inline(always)]
        pub const fn hsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal size, from 0 to 4094 pixels wide."]
        #[inline(always)]
        pub const fn set_hsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[must_use]
        #[inline(always)]
        pub const fn vsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Vertical size, from 0 to 4094 pixels high."]
        #[inline(always)]
        pub const fn set_vsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for P2ri8cr2 {
        #[inline(always)]
        fn default() -> P2ri8cr2 {
            P2ri8cr2(0)
        }
    }
    impl core::fmt::Debug for P2ri8cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2ri8cr2")
                .field("hsize", &self.hsize())
                .field("vsize", &self.vsize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2ri8cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "P2ri8cr2 {{ hsize: {=u16:?}, vsize: {=u16:?} }}",
                self.hsize(),
                self.vsize()
            )
        }
    }
    #[doc = "DCMIPP Pipe2 status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2sr(pub u32);
    impl P2sr {
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn linef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-line capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_linef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn framef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Frame capture completed raw interrupt status."]
        #[inline(always)]
        pub const fn set_framef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VSYNC raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn vsyncf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VSYNC raw interrupt status."]
        #[inline(always)]
        pub const fn set_vsyncf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun raw interrupt status."]
        #[inline(always)]
        pub const fn set_ovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[must_use]
        #[inline(always)]
        pub const fn lstline(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Last line LSB bit, sampled at frame capture complete event."]
        #[inline(always)]
        pub const fn set_lstline(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number which can be delivered by the camera through the CSI2 interface."]
        #[must_use]
        #[inline(always)]
        pub const fn lstfrm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number which can be delivered by the camera through the CSI2 interface."]
        #[inline(always)]
        pub const fn set_lstfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Capture immediate status."]
        #[must_use]
        #[inline(always)]
        pub const fn cptact(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Capture immediate status."]
        #[inline(always)]
        pub const fn set_cptact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for P2sr {
        #[inline(always)]
        fn default() -> P2sr {
            P2sr(0)
        }
    }
    impl core::fmt::Debug for P2sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2sr")
                .field("linef", &self.linef())
                .field("framef", &self.framef())
                .field("vsyncf", &self.vsyncf())
                .field("ovrf", &self.ovrf())
                .field("lstline", &self.lstline())
                .field("lstfrm", &self.lstfrm())
                .field("cptact", &self.cptact())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "P2sr {{ linef: {=bool:?}, framef: {=bool:?}, vsyncf: {=bool:?}, ovrf: {=bool:?}, lstline: {=bool:?}, lstfrm: {=bool:?}, cptact: {=bool:?} }}" , self . linef () , self . framef () , self . vsyncf () , self . ovrf () , self . lstline () , self . lstfrm () , self . cptact ())
        }
    }
    #[doc = "DCMIPP Pipex status Memory0 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2stm0ar(pub u32);
    impl P2stm0ar {
        #[doc = "Memory0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn m0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Memory0 address."]
        #[inline(always)]
        pub const fn set_m0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2stm0ar {
        #[inline(always)]
        fn default() -> P2stm0ar {
            P2stm0ar(0)
        }
    }
    impl core::fmt::Debug for P2stm0ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P2stm0ar").field("m0a", &self.m0a()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P2stm0ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P2stm0ar {{ m0a: {=u32:?} }}", self.m0a())
        }
    }
    #[doc = "DCMIPP parallel interface control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prcr(pub u32);
    impl Prcr {
        #[doc = "Embedded synchronization select."]
        #[must_use]
        #[inline(always)]
        pub const fn ess(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Embedded synchronization select."]
        #[inline(always)]
        pub const fn set_ess(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pixel clock polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pckpol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel clock polarity."]
        #[inline(always)]
        pub const fn set_pckpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Horizontal synchronization polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn hspol(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal synchronization polarity."]
        #[inline(always)]
        pub const fn set_hspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Vertical synchronization polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn vspol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical synchronization polarity."]
        #[inline(always)]
        pub const fn set_vspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Extended data mode."]
        #[must_use]
        #[inline(always)]
        pub const fn edm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Extended data mode."]
        #[inline(always)]
        pub const fn set_edm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Parallel interface enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Parallel interface enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Other values: data are captured and output as-is only through the data/dump pipeline (e.g. JPEG or byte input format)."]
        #[must_use]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Other values: data are captured and output as-is only through the data/dump pipeline (e.g. JPEG or byte input format)."]
        #[inline(always)]
        pub const fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn swapcycles(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles."]
        #[inline(always)]
        pub const fn set_swapcycles(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Swap LSB vs. MSB within each received component."]
        #[must_use]
        #[inline(always)]
        pub const fn swapbits(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Swap LSB vs. MSB within each received component."]
        #[inline(always)]
        pub const fn set_swapbits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Prcr {
        #[inline(always)]
        fn default() -> Prcr {
            Prcr(0)
        }
    }
    impl core::fmt::Debug for Prcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prcr")
                .field("ess", &self.ess())
                .field("pckpol", &self.pckpol())
                .field("hspol", &self.hspol())
                .field("vspol", &self.vspol())
                .field("edm", &self.edm())
                .field("enable", &self.enable())
                .field("format", &self.format())
                .field("swapcycles", &self.swapcycles())
                .field("swapbits", &self.swapbits())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Prcr {{ ess: {=bool:?}, pckpol: {=bool:?}, hspol: {=bool:?}, vspol: {=bool:?}, edm: {=u8:?}, enable: {=bool:?}, format: {=u8:?}, swapcycles: {=bool:?}, swapbits: {=bool:?} }}" , self . ess () , self . pckpol () , self . hspol () , self . vspol () , self . edm () , self . enable () , self . format () , self . swapcycles () , self . swapbits ())
        }
    }
    #[doc = "DCMIPP parallel interface embedded synchronization code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prescr(pub u32);
    impl Prescr {
        #[doc = "Frame start delimiter code."]
        #[must_use]
        #[inline(always)]
        pub const fn fsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame start delimiter code."]
        #[inline(always)]
        pub const fn set_fsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Line start delimiter code."]
        #[must_use]
        #[inline(always)]
        pub const fn lsc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Line start delimiter code."]
        #[inline(always)]
        pub const fn set_lsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Line end delimiter code."]
        #[must_use]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Line end delimiter code."]
        #[inline(always)]
        pub const fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Frame end delimiter code."]
        #[must_use]
        #[inline(always)]
        pub const fn fec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Frame end delimiter code."]
        #[inline(always)]
        pub const fn set_fec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Prescr {
        #[inline(always)]
        fn default() -> Prescr {
            Prescr(0)
        }
    }
    impl core::fmt::Debug for Prescr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prescr")
                .field("fsc", &self.fsc())
                .field("lsc", &self.lsc())
                .field("lec", &self.lec())
                .field("fec", &self.fec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prescr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prescr {{ fsc: {=u8:?}, lsc: {=u8:?}, lec: {=u8:?}, fec: {=u8:?} }}",
                self.fsc(),
                self.lsc(),
                self.lec(),
                self.fec()
            )
        }
    }
    #[doc = "DCMIPP parallel interface embedded synchronization unmask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Presur(pub u32);
    impl Presur {
        #[doc = "Frame start delimiter unmask."]
        #[must_use]
        #[inline(always)]
        pub const fn fsu(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame start delimiter unmask."]
        #[inline(always)]
        pub const fn set_fsu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Line start delimiter unmask."]
        #[must_use]
        #[inline(always)]
        pub const fn lsu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Line start delimiter unmask."]
        #[inline(always)]
        pub const fn set_lsu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Line end delimiter unmask."]
        #[must_use]
        #[inline(always)]
        pub const fn leu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Line end delimiter unmask."]
        #[inline(always)]
        pub const fn set_leu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Frame end delimiter unmask."]
        #[must_use]
        #[inline(always)]
        pub const fn feu(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Frame end delimiter unmask."]
        #[inline(always)]
        pub const fn set_feu(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Presur {
        #[inline(always)]
        fn default() -> Presur {
            Presur(0)
        }
    }
    impl core::fmt::Debug for Presur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Presur")
                .field("fsu", &self.fsu())
                .field("lsu", &self.lsu())
                .field("leu", &self.leu())
                .field("feu", &self.feu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Presur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Presur {{ fsu: {=u8:?}, lsu: {=u8:?}, leu: {=u8:?}, feu: {=u8:?} }}",
                self.fsu(),
                self.lsu(),
                self.leu(),
                self.feu()
            )
        }
    }
    #[doc = "DCMIPP parallel interface interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prfcr(pub u32);
    impl Prfcr {
        #[doc = "Synchronization error interrupt status clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error interrupt status clear."]
        #[inline(always)]
        pub const fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Prfcr {
        #[inline(always)]
        fn default() -> Prfcr {
            Prfcr(0)
        }
    }
    impl core::fmt::Debug for Prfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prfcr").field("cerrf", &self.cerrf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Prfcr {{ cerrf: {=bool:?} }}", self.cerrf())
        }
    }
    #[doc = "DCMIPP parallel interface interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prier(pub u32);
    impl Prier {
        #[doc = "Synchronization error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error interrupt enable."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Prier {
        #[inline(always)]
        fn default() -> Prier {
            Prier(0)
        }
    }
    impl core::fmt::Debug for Prier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prier").field("errie", &self.errie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Prier {{ errie: {=bool:?} }}", self.errie())
        }
    }
    #[doc = "DCMIPP parallel interface status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prsr(pub u32);
    impl Prsr {
        #[doc = "Synchronization error raw interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error raw interrupt status."]
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "This bit gives the state of the HSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit, and cleared otherwise."]
        #[must_use]
        #[inline(always)]
        pub const fn hsync(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "This bit gives the state of the HSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit, and cleared otherwise."]
        #[inline(always)]
        pub const fn set_hsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "This bit gives the state of the VSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit, and cleared otherwise."]
        #[must_use]
        #[inline(always)]
        pub const fn vsync(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "This bit gives the state of the VSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit, and cleared otherwise."]
        #[inline(always)]
        pub const fn set_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Prsr {
        #[inline(always)]
        fn default() -> Prsr {
            Prsr(0)
        }
    }
    impl core::fmt::Debug for Prsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prsr")
                .field("errf", &self.errf())
                .field("hsync", &self.hsync())
                .field("vsync", &self.vsync())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prsr {{ errf: {=bool:?}, hsync: {=bool:?}, vsync: {=bool:?} }}",
                self.errf(),
                self.hsync(),
                self.vsync()
            )
        }
    }
}
