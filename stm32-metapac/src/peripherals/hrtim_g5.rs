#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "High Resolution Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrtim {
    ptr: *mut u8,
}
unsafe impl Send for Hrtim {}
unsafe impl Sync for Hrtim {}
impl Hrtim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Timer Control Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[inline(always)]
    pub const fn micr(self) -> crate::common::Reg<regs::Micr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mdier(self) -> crate::common::Reg<regs::Mdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Master Timer Counter Register"]
    #[inline(always)]
    pub const fn mcntr(self) -> crate::common::Reg<regs::Mcntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Master Timer Period Register"]
    #[inline(always)]
    pub const fn mper(self) -> crate::common::Reg<regs::Mper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Master Timer Repetition Register"]
    #[inline(always)]
    pub const fn mrep(self) -> crate::common::Reg<regs::Mrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Master Timer Compare X Register"]
    #[inline(always)]
    pub const fn mcmp(self, n: usize) -> crate::common::Reg<regs::Mcmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr
                    .wrapping_add(0x1cusize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _,
            )
        }
    }
    #[doc = "High Resolution Timer: Timing Unit"]
    #[inline(always)]
    pub const fn tim(self, n: usize) -> HrtimTim {
        assert!(n < 6usize);
        unsafe { HrtimTim::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "High Resolution Timer: Control Register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::HrtimCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "High Resolution Timer: Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::HrtimCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::HrtimIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::HrtimIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::HrtimIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Enable Register"]
    #[inline(always)]
    pub const fn oenr(self) -> crate::common::Reg<regs::HrtimOenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Disable Register"]
    #[inline(always)]
    pub const fn odisr(self) -> crate::common::Reg<regs::HrtimOdisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Disable Status Register"]
    #[inline(always)]
    pub const fn odsr(self) -> crate::common::Reg<regs::HrtimOdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Control Register"]
    #[inline(always)]
    pub const fn bmcr(self) -> crate::common::Reg<regs::HrtimBmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Trigger Register"]
    #[inline(always)]
    pub const fn bmtrgr(self) -> crate::common::Reg<regs::HrtimBmtrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Compare Register"]
    #[inline(always)]
    pub const fn bmcmpr(self) -> crate::common::Reg<regs::HrtimBmcmpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Period Register"]
    #[inline(always)]
    pub const fn bmper(self) -> crate::common::Reg<regs::HrtimBmper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 1"]
    #[inline(always)]
    pub const fn eecr1(self) -> crate::common::Reg<regs::HrtimEecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 2"]
    #[inline(always)]
    pub const fn eecr2(self) -> crate::common::Reg<regs::HrtimEecr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 3"]
    #[inline(always)]
    pub const fn eecr3(self) -> crate::common::Reg<regs::HrtimEecr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0738usize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Trigger \\[1, 3\\]
Register"]
    #[inline(always)]
    pub const fn adc1r(self, n: usize) -> crate::common::Reg<regs::HrtimAdc1r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x073cusize + ([0usize, 8usize][n] as usize)) as _)
        }
    }
    #[doc = "High Resolution Timer: ADC Trigger \\[2, 4\\]
Register"]
    #[inline(always)]
    pub const fn adc2r(self, n: usize) -> crate::common::Reg<regs::HrtimAdc2r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize + ([0usize, 8usize][n] as usize)) as _)
        }
    }
    #[doc = "High Resolution Timer: DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(self) -> crate::common::Reg<regs::HrtimDllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 1"]
    #[inline(always)]
    pub const fn fltinr1(self) -> crate::common::Reg<regs::HrtimFltinr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 2"]
    #[inline(always)]
    pub const fn fltinr2(self) -> crate::common::Reg<regs::HrtimFltinr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0754usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[inline(always)]
    pub const fn bdmupr(self) -> crate::common::Reg<regs::HrtimBdmupr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0758usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Timer X update Register"]
    #[inline(always)]
    pub const fn bdtupr(self, n: usize) -> crate::common::Reg<regs::HrtimBdtupr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x075cusize + n * 4usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Data Register"]
    #[inline(always)]
    pub const fn bdmadr(self) -> crate::common::Reg<regs::HrtimBdmadr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Timer F update Register"]
    #[inline(always)]
    pub const fn bdtfupr(self) -> crate::common::Reg<regs::HrtimBdtfupr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0774usize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Extended Trigger Register"]
    #[inline(always)]
    pub const fn adcer(self) -> crate::common::Reg<regs::HrtimAdcer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0778usize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Trigger Update Register"]
    #[inline(always)]
    pub const fn adcur(self) -> crate::common::Reg<regs::HrtimAdcur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x077cusize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Post Scaler Register 1"]
    #[inline(always)]
    pub const fn adcps1(self) -> crate::common::Reg<regs::HrtimAdcps1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Post Scaler Register 2"]
    #[inline(always)]
    pub const fn adcps2(self) -> crate::common::Reg<regs::HrtimAdcps2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 3"]
    #[inline(always)]
    pub const fn fltinr3(self) -> crate::common::Reg<regs::HrtimFltinr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 4"]
    #[inline(always)]
    pub const fn fltinr4(self) -> crate::common::Reg<regs::HrtimFltinr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 5"]
    #[inline(always)]
    pub const fn fltinr5(self) -> crate::common::Reg<regs::HrtimFltinr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 4"]
    #[inline(always)]
    pub const fn eecr4(self) -> crate::common::Reg<regs::HrtimEecr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 5"]
    #[inline(always)]
    pub const fn eecr5(self) -> crate::common::Reg<regs::HrtimEecr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 6"]
    #[inline(always)]
    pub const fn eecr6(self) -> crate::common::Reg<regs::HrtimEecr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "High Resolution Timer: Timer External Event Control Register 7"]
    #[inline(always)]
    pub const fn eecr7(self) -> crate::common::Reg<regs::HrtimEecr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07acusize) as _) }
    }
    #[doc = "High Resolution Timer: Control Register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::HrtimCr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "High Resolution Timer: Option Register"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::HrtimOr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
}
#[doc = "High Resolution Timer: Timing Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrtimTim {
    ptr: *mut u8,
}
unsafe impl Send for HrtimTim {}
unsafe impl Sync for HrtimTim {}
impl HrtimTim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer X Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Timxcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer X Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Timxisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer X Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Timxicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer X DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Timxdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Timer X Counter Register"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Timxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Timer X Period Register"]
    #[inline(always)]
    pub const fn per(self) -> crate::common::Reg<regs::Timxper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Timer X Repetition Register"]
    #[inline(always)]
    pub const fn rep(self) -> crate::common::Reg<regs::Timxrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Timer X Compare X Register"]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Timxcmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr
                    .wrapping_add(0x1cusize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _,
            )
        }
    }
    #[doc = "Timer X Compare X Compound Register"]
    #[inline(always)]
    pub const fn cmpc(self) -> crate::common::Reg<regs::Timxcmpc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Timer X Capture X Register"]
    #[inline(always)]
    pub const fn cpt(self, n: usize) -> crate::common::Reg<regs::Timxcpt, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Timer X Deadtime Register"]
    #[inline(always)]
    pub const fn dt(self) -> crate::common::Reg<regs::Timxdt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Timer X Output X Set Register"]
    #[inline(always)]
    pub const fn setr(self, n: usize) -> crate::common::Reg<regs::Timxsetr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output X Reset Register"]
    #[inline(always)]
    pub const fn rstr(self, n: usize) -> crate::common::Reg<regs::Timxrstr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X External Event Filtering Register"]
    #[inline(always)]
    pub const fn eef(self, n: usize) -> crate::common::Reg<regs::Timxeef, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Reset Register"]
    #[inline(always)]
    pub const fn rst(self) -> crate::common::Reg<regs::Timxrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Timer X Chopper Register"]
    #[inline(always)]
    pub const fn chp(self) -> crate::common::Reg<regs::Timxchp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Timer X Capture X Control Register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Timxccr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output Register"]
    #[inline(always)]
    pub const fn outr(self) -> crate::common::Reg<regs::Timxoutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Timer X Fault Register"]
    #[inline(always)]
    pub const fn flt(self) -> crate::common::Reg<regs::Timxflt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Timer X Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Timxcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Timer X External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn eefr3(self) -> crate::common::Reg<regs::Timxeefr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Timer X External Event Selection Register"]
    #[inline(always)]
    pub const fn eesel(self) -> crate::common::Reg<regs::Timxeesel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Timer X Crossbar Override Trigger Register"]
    #[inline(always)]
    pub const fn cotrg(self) -> crate::common::Reg<regs::Timxcotrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Timer X Crossbar Override Forcing Event Register"]
    #[inline(always)]
    pub const fn cofe(self) -> crate::common::Reg<regs::Timxcofe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Timer X Crossbar Override Delayed Fault Event Register"]
    #[inline(always)]
    pub const fn code(self) -> crate::common::Reg<regs::Timxcode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs {
    #[doc = "High Resolution Timer: ADC Trigger 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdc1r(pub u32);
    impl HrtimAdc1r {
        #[doc = "ADC trigger 1 on master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1mc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on master compare 1."]
        #[inline(always)]
        pub const fn set_adc1mc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC trigger 1 on master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1mc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on master compare 2."]
        #[inline(always)]
        pub const fn set_adc1mc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC trigger 1 on master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1mc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on master compare 3."]
        #[inline(always)]
        pub const fn set_adc1mc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC trigger 1 on master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1mc4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on master compare 4."]
        #[inline(always)]
        pub const fn set_adc1mc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC trigger 1 on master period."]
        #[must_use]
        #[inline(always)]
        pub const fn adcmper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on master period."]
        #[inline(always)]
        pub const fn set_adcmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC trigger 1 on external event 1less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1eev1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on external event 1less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1eev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC trigger 1 on external event 2less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1eev2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on external event 2less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1eev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ADC trigger 1 on external event 3less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1eev3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on external event 3less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1eev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC trigger 1 on external event 4less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1eev4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on external event 4less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1eev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC trigger 1 on external event 5less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1eev5(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on external event 5less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1eev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC trigger 1 on timer F compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adctc2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer F compare 2."]
        #[inline(always)]
        pub const fn set_adctc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ADC trigger 1 on timer A compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tac3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer A compare 3."]
        #[inline(always)]
        pub const fn set_adc1tac3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ADC trigger 1 on timer A compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tac4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer A compare 4."]
        #[inline(always)]
        pub const fn set_adc1tac4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC trigger 1 on timer A period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1taper(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer A period."]
        #[inline(always)]
        pub const fn set_adc1taper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ADC trigger 1 on timer A reset and counter roll-overless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tarst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer A reset and counter roll-overless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1tarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC trigger 1 on timer F compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tfc3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer F compare 3."]
        #[inline(always)]
        pub const fn set_adc1tfc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC trigger 1 on timer B compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tbc3(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer B compare 3."]
        #[inline(always)]
        pub const fn set_adc1tbc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC trigger 1 on timer B compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tbc4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer B compare 4."]
        #[inline(always)]
        pub const fn set_adc1tbc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC trigger 1 on timer B period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tbper(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer B period."]
        #[inline(always)]
        pub const fn set_adc1tbper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC trigger 1 on timer B reset and counter roll-overless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tbrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer B reset and counter roll-overless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc1tbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADC trigger 1 on timer F compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tfc4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer F compare 4."]
        #[inline(always)]
        pub const fn set_adc1tfc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ADC trigger 1 on timer C compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tcc3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer C compare 3."]
        #[inline(always)]
        pub const fn set_adc1tcc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC trigger 1 on timer C compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tcc4(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer C compare 4."]
        #[inline(always)]
        pub const fn set_adc1tcc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC trigger 1 on timer C period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tcper(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer C period."]
        #[inline(always)]
        pub const fn set_adc1tcper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ADC trigger 1 on timer F period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tfper(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer F period."]
        #[inline(always)]
        pub const fn set_adc1tfper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC trigger 1 on timer D compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tdc3(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer D compare 3."]
        #[inline(always)]
        pub const fn set_adc1tdc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ADC trigger 1 on timer D compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tdc4(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer D compare 4."]
        #[inline(always)]
        pub const fn set_adc1tdc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ADC trigger 1 on timer D period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tdper(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer D period."]
        #[inline(always)]
        pub const fn set_adc1tdper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADC trigger 1 on timer F reset and counter roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tfrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer F reset and counter roll-over."]
        #[inline(always)]
        pub const fn set_adc1tfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC trigger 1 on timer E compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tec3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer E compare 3."]
        #[inline(always)]
        pub const fn set_adc1tec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADC trigger 1 on timer E compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1tec4(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer E compare 4."]
        #[inline(always)]
        pub const fn set_adc1tec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADC trigger 1 on timer E period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1teper(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 1 on timer E period."]
        #[inline(always)]
        pub const fn set_adc1teper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimAdc1r {
        #[inline(always)]
        fn default() -> HrtimAdc1r {
            HrtimAdc1r(0)
        }
    }
    impl core::fmt::Debug for HrtimAdc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdc1r")
                .field("adc1mc1", &self.adc1mc1())
                .field("adc1mc2", &self.adc1mc2())
                .field("adc1mc3", &self.adc1mc3())
                .field("adc1mc4", &self.adc1mc4())
                .field("adcmper", &self.adcmper())
                .field("adc1eev1", &self.adc1eev1())
                .field("adc1eev2", &self.adc1eev2())
                .field("adc1eev3", &self.adc1eev3())
                .field("adc1eev4", &self.adc1eev4())
                .field("adc1eev5", &self.adc1eev5())
                .field("adctc2", &self.adctc2())
                .field("adc1tac3", &self.adc1tac3())
                .field("adc1tac4", &self.adc1tac4())
                .field("adc1taper", &self.adc1taper())
                .field("adc1tarst", &self.adc1tarst())
                .field("adc1tfc3", &self.adc1tfc3())
                .field("adc1tbc3", &self.adc1tbc3())
                .field("adc1tbc4", &self.adc1tbc4())
                .field("adc1tbper", &self.adc1tbper())
                .field("adc1tbrst", &self.adc1tbrst())
                .field("adc1tfc4", &self.adc1tfc4())
                .field("adc1tcc3", &self.adc1tcc3())
                .field("adc1tcc4", &self.adc1tcc4())
                .field("adc1tcper", &self.adc1tcper())
                .field("adc1tfper", &self.adc1tfper())
                .field("adc1tdc3", &self.adc1tdc3())
                .field("adc1tdc4", &self.adc1tdc4())
                .field("adc1tdper", &self.adc1tdper())
                .field("adc1tfrst", &self.adc1tfrst())
                .field("adc1tec3", &self.adc1tec3())
                .field("adc1tec4", &self.adc1tec4())
                .field("adc1teper", &self.adc1teper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdc1r {{ adc1mc1: {=bool:?}, adc1mc2: {=bool:?}, adc1mc3: {=bool:?}, adc1mc4: {=bool:?}, adcmper: {=bool:?}, adc1eev1: {=bool:?}, adc1eev2: {=bool:?}, adc1eev3: {=bool:?}, adc1eev4: {=bool:?}, adc1eev5: {=bool:?}, adctc2: {=bool:?}, adc1tac3: {=bool:?}, adc1tac4: {=bool:?}, adc1taper: {=bool:?}, adc1tarst: {=bool:?}, adc1tfc3: {=bool:?}, adc1tbc3: {=bool:?}, adc1tbc4: {=bool:?}, adc1tbper: {=bool:?}, adc1tbrst: {=bool:?}, adc1tfc4: {=bool:?}, adc1tcc3: {=bool:?}, adc1tcc4: {=bool:?}, adc1tcper: {=bool:?}, adc1tfper: {=bool:?}, adc1tdc3: {=bool:?}, adc1tdc4: {=bool:?}, adc1tdper: {=bool:?}, adc1tfrst: {=bool:?}, adc1tec3: {=bool:?}, adc1tec4: {=bool:?}, adc1teper: {=bool:?} }}",
                self.adc1mc1(),
                self.adc1mc2(),
                self.adc1mc3(),
                self.adc1mc4(),
                self.adcmper(),
                self.adc1eev1(),
                self.adc1eev2(),
                self.adc1eev3(),
                self.adc1eev4(),
                self.adc1eev5(),
                self.adctc2(),
                self.adc1tac3(),
                self.adc1tac4(),
                self.adc1taper(),
                self.adc1tarst(),
                self.adc1tfc3(),
                self.adc1tbc3(),
                self.adc1tbc4(),
                self.adc1tbper(),
                self.adc1tbrst(),
                self.adc1tfc4(),
                self.adc1tcc3(),
                self.adc1tcc4(),
                self.adc1tcper(),
                self.adc1tfper(),
                self.adc1tdc3(),
                self.adc1tdc4(),
                self.adc1tdper(),
                self.adc1tfrst(),
                self.adc1tec3(),
                self.adc1tec4(),
                self.adc1teper()
            )
        }
    }
    #[doc = "High Resolution Timer: ADC Trigger 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdc2r(pub u32);
    impl HrtimAdc2r {
        #[doc = "ADC trigger 2 on master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2mc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on master compare 1."]
        #[inline(always)]
        pub const fn set_adc2mc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC trigger 2 on master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2mc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on master compare 2."]
        #[inline(always)]
        pub const fn set_adc2mc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC trigger 2 on master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2mc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on master compare 3."]
        #[inline(always)]
        pub const fn set_adc2mc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC trigger 2 on master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2mc4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on master compare 4."]
        #[inline(always)]
        pub const fn set_adc2mc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADC trigger 2 on master period."]
        #[must_use]
        #[inline(always)]
        pub const fn adcmper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on master period."]
        #[inline(always)]
        pub const fn set_adcmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC trigger 2 on external event 6less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2eev6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on external event 6less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2eev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC trigger 2 on external event 7less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2eev7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on external event 7less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2eev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ADC trigger 2 on external event 8less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2eev8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on external event 8less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2eev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC trigger 2 on external event 9less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2eev9(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on external event 9less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2eev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC trigger 2 on external event 10less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2eev10(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on external event 10less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2eev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC trigger 2 on timer A compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tac2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer A compare 2."]
        #[inline(always)]
        pub const fn set_adc2tac2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ADC trigger 2 on timer F compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tfc2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer F compare 2."]
        #[inline(always)]
        pub const fn set_adc2tfc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ADC trigger 2 on timer A compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tac4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer A compare 4."]
        #[inline(always)]
        pub const fn set_adc2tac4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC trigger 2 on timer A period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2taper(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer A period."]
        #[inline(always)]
        pub const fn set_adc2taper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ADC trigger 2 on timer B compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tbc2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer B compare 2."]
        #[inline(always)]
        pub const fn set_adc2tbc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC trigger 2 on timer F compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tfc3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer F compare 3."]
        #[inline(always)]
        pub const fn set_adc2tfc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC trigger 2 on timer B compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tbc4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer B compare 4."]
        #[inline(always)]
        pub const fn set_adc2tbc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC trigger 2 on timer B period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tbper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer B period."]
        #[inline(always)]
        pub const fn set_adc2tbper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC trigger 2 on timer C compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tcc2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer C compare 2."]
        #[inline(always)]
        pub const fn set_adc2tcc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC trigger 2 on timer F compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tfc4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer F compare 4."]
        #[inline(always)]
        pub const fn set_adc2tfc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADC trigger 2 on timer C compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tcc4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer C compare 4."]
        #[inline(always)]
        pub const fn set_adc2tcc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ADC trigger 2 on timer C period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tcper(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer C period."]
        #[inline(always)]
        pub const fn set_adc2tcper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC trigger 2 on timer C reset and counter roll-overless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tcrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer C reset and counter roll-overless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2tcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC trigger 2 on timer D compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tdc2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer D compare 2."]
        #[inline(always)]
        pub const fn set_adc2tdc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ADC trigger 2 on timer F period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tfper(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer F period."]
        #[inline(always)]
        pub const fn set_adc2tfper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC trigger 2 on timer D compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tdc4(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer D compare 4."]
        #[inline(always)]
        pub const fn set_adc2tdc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ADC trigger 2 on timer D period."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tdper(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer D period."]
        #[inline(always)]
        pub const fn set_adc2tdper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ADC trigger 2 on timer D reset and counter roll-overless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tdrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer D reset and counter roll-overless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_adc2tdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADC trigger 2 on timer E compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tec2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer E compare 2."]
        #[inline(always)]
        pub const fn set_adc2tec2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC trigger 2 on timer E compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tec3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer E compare 3."]
        #[inline(always)]
        pub const fn set_adc2tec3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADC trigger 2 on timer E compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2tec4(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer E compare 4."]
        #[inline(always)]
        pub const fn set_adc2tec4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADC trigger 2 on timer E reset and counter roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2terst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger 2 on timer E reset and counter roll-over."]
        #[inline(always)]
        pub const fn set_adc2terst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimAdc2r {
        #[inline(always)]
        fn default() -> HrtimAdc2r {
            HrtimAdc2r(0)
        }
    }
    impl core::fmt::Debug for HrtimAdc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdc2r")
                .field("adc2mc1", &self.adc2mc1())
                .field("adc2mc2", &self.adc2mc2())
                .field("adc2mc3", &self.adc2mc3())
                .field("adc2mc4", &self.adc2mc4())
                .field("adcmper", &self.adcmper())
                .field("adc2eev6", &self.adc2eev6())
                .field("adc2eev7", &self.adc2eev7())
                .field("adc2eev8", &self.adc2eev8())
                .field("adc2eev9", &self.adc2eev9())
                .field("adc2eev10", &self.adc2eev10())
                .field("adc2tac2", &self.adc2tac2())
                .field("adc2tfc2", &self.adc2tfc2())
                .field("adc2tac4", &self.adc2tac4())
                .field("adc2taper", &self.adc2taper())
                .field("adc2tbc2", &self.adc2tbc2())
                .field("adc2tfc3", &self.adc2tfc3())
                .field("adc2tbc4", &self.adc2tbc4())
                .field("adc2tbper", &self.adc2tbper())
                .field("adc2tcc2", &self.adc2tcc2())
                .field("adc2tfc4", &self.adc2tfc4())
                .field("adc2tcc4", &self.adc2tcc4())
                .field("adc2tcper", &self.adc2tcper())
                .field("adc2tcrst", &self.adc2tcrst())
                .field("adc2tdc2", &self.adc2tdc2())
                .field("adc2tfper", &self.adc2tfper())
                .field("adc2tdc4", &self.adc2tdc4())
                .field("adc2tdper", &self.adc2tdper())
                .field("adc2tdrst", &self.adc2tdrst())
                .field("adc2tec2", &self.adc2tec2())
                .field("adc2tec3", &self.adc2tec3())
                .field("adc2tec4", &self.adc2tec4())
                .field("adc2terst", &self.adc2terst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdc2r {{ adc2mc1: {=bool:?}, adc2mc2: {=bool:?}, adc2mc3: {=bool:?}, adc2mc4: {=bool:?}, adcmper: {=bool:?}, adc2eev6: {=bool:?}, adc2eev7: {=bool:?}, adc2eev8: {=bool:?}, adc2eev9: {=bool:?}, adc2eev10: {=bool:?}, adc2tac2: {=bool:?}, adc2tfc2: {=bool:?}, adc2tac4: {=bool:?}, adc2taper: {=bool:?}, adc2tbc2: {=bool:?}, adc2tfc3: {=bool:?}, adc2tbc4: {=bool:?}, adc2tbper: {=bool:?}, adc2tcc2: {=bool:?}, adc2tfc4: {=bool:?}, adc2tcc4: {=bool:?}, adc2tcper: {=bool:?}, adc2tcrst: {=bool:?}, adc2tdc2: {=bool:?}, adc2tfper: {=bool:?}, adc2tdc4: {=bool:?}, adc2tdper: {=bool:?}, adc2tdrst: {=bool:?}, adc2tec2: {=bool:?}, adc2tec3: {=bool:?}, adc2tec4: {=bool:?}, adc2terst: {=bool:?} }}",
                self.adc2mc1(),
                self.adc2mc2(),
                self.adc2mc3(),
                self.adc2mc4(),
                self.adcmper(),
                self.adc2eev6(),
                self.adc2eev7(),
                self.adc2eev8(),
                self.adc2eev9(),
                self.adc2eev10(),
                self.adc2tac2(),
                self.adc2tfc2(),
                self.adc2tac4(),
                self.adc2taper(),
                self.adc2tbc2(),
                self.adc2tfc3(),
                self.adc2tbc4(),
                self.adc2tbper(),
                self.adc2tcc2(),
                self.adc2tfc4(),
                self.adc2tcc4(),
                self.adc2tcper(),
                self.adc2tcrst(),
                self.adc2tdc2(),
                self.adc2tfper(),
                self.adc2tdc4(),
                self.adc2tdper(),
                self.adc2tdrst(),
                self.adc2tec2(),
                self.adc2tec3(),
                self.adc2tec4(),
                self.adc2terst()
            )
        }
    }
    #[doc = "HRTIM ADC Extended Trigger Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdcer(pub u32);
    impl HrtimAdcer {
        #[doc = "ADC trigger 5 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc5trg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 5 selection."]
        #[inline(always)]
        pub const fn set_adc5trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "ADC trigger 6 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc6trg(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 6 selection."]
        #[inline(always)]
        pub const fn set_adc6trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "ADC trigger 7 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc7trg(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 7 selection."]
        #[inline(always)]
        pub const fn set_adc7trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "ADC trigger 8 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc8trg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 8 selection."]
        #[inline(always)]
        pub const fn set_adc8trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "ADC trigger 9 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc9trg(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 9 selection."]
        #[inline(always)]
        pub const fn set_adc9trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "ADC trigger 10 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc10trg(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC trigger 10 selection."]
        #[inline(always)]
        pub const fn set_adc10trg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for HrtimAdcer {
        #[inline(always)]
        fn default() -> HrtimAdcer {
            HrtimAdcer(0)
        }
    }
    impl core::fmt::Debug for HrtimAdcer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdcer")
                .field("adc5trg", &self.adc5trg())
                .field("adc6trg", &self.adc6trg())
                .field("adc7trg", &self.adc7trg())
                .field("adc8trg", &self.adc8trg())
                .field("adc9trg", &self.adc9trg())
                .field("adc10trg", &self.adc10trg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdcer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdcer {{ adc5trg: {=u8:?}, adc6trg: {=u8:?}, adc7trg: {=u8:?}, adc8trg: {=u8:?}, adc9trg: {=u8:?}, adc10trg: {=u8:?} }}",
                self.adc5trg(),
                self.adc6trg(),
                self.adc7trg(),
                self.adc8trg(),
                self.adc9trg(),
                self.adc10trg()
            )
        }
    }
    #[doc = "HRTIM ADC Post Scaler Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdcps1(pub u32);
    impl HrtimAdcps1 {
        #[doc = "ADC 1 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc1psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 1 post scaler."]
        #[inline(always)]
        pub const fn set_adc1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "ADC 2 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc2psc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 2 post scaler."]
        #[inline(always)]
        pub const fn set_adc2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "ADC 3 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3psc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 3 post scaler."]
        #[inline(always)]
        pub const fn set_adc3psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "ADC 4 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4psc(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 4 post scaler."]
        #[inline(always)]
        pub const fn set_adc4psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "ADC 5 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc5psc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 5 post scaler."]
        #[inline(always)]
        pub const fn set_adc5psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for HrtimAdcps1 {
        #[inline(always)]
        fn default() -> HrtimAdcps1 {
            HrtimAdcps1(0)
        }
    }
    impl core::fmt::Debug for HrtimAdcps1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdcps1")
                .field("adc1psc", &self.adc1psc())
                .field("adc2psc", &self.adc2psc())
                .field("adc3psc", &self.adc3psc())
                .field("adc4psc", &self.adc4psc())
                .field("adc5psc", &self.adc5psc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdcps1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdcps1 {{ adc1psc: {=u8:?}, adc2psc: {=u8:?}, adc3psc: {=u8:?}, adc4psc: {=u8:?}, adc5psc: {=u8:?} }}",
                self.adc1psc(),
                self.adc2psc(),
                self.adc3psc(),
                self.adc4psc(),
                self.adc5psc()
            )
        }
    }
    #[doc = "HRTIM ADC Post Scaler Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdcps2(pub u32);
    impl HrtimAdcps2 {
        #[doc = "ADC 6 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc6psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 6 post scaler."]
        #[inline(always)]
        pub const fn set_adc6psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "ADC 7 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc7psc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 7 post scaler."]
        #[inline(always)]
        pub const fn set_adc7psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "ADC 8 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc8psc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 8 post scaler."]
        #[inline(always)]
        pub const fn set_adc8psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "ADC 9 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc9psc(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 9 post scaler."]
        #[inline(always)]
        pub const fn set_adc9psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "ADC 10 post scaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adc10psc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "ADC 10 post scaler."]
        #[inline(always)]
        pub const fn set_adc10psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for HrtimAdcps2 {
        #[inline(always)]
        fn default() -> HrtimAdcps2 {
            HrtimAdcps2(0)
        }
    }
    impl core::fmt::Debug for HrtimAdcps2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdcps2")
                .field("adc6psc", &self.adc6psc())
                .field("adc7psc", &self.adc7psc())
                .field("adc8psc", &self.adc8psc())
                .field("adc9psc", &self.adc9psc())
                .field("adc10psc", &self.adc10psc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdcps2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdcps2 {{ adc6psc: {=u8:?}, adc7psc: {=u8:?}, adc8psc: {=u8:?}, adc9psc: {=u8:?}, adc10psc: {=u8:?} }}",
                self.adc6psc(),
                self.adc7psc(),
                self.adc8psc(),
                self.adc9psc(),
                self.adc10psc()
            )
        }
    }
    #[doc = "HRTIM ADC Trigger Update Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdcur(pub u32);
    impl HrtimAdcur {
        #[doc = "ADC trigger 5 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad5usrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 5 update source."]
        #[inline(always)]
        pub const fn set_ad5usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "ADC trigger 6 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad6usrc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 6 update source."]
        #[inline(always)]
        pub const fn set_ad6usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "ADC trigger 7 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad7usrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 7 update source."]
        #[inline(always)]
        pub const fn set_ad7usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "ADC trigger 8 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad8usrc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 8 update source."]
        #[inline(always)]
        pub const fn set_ad8usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "ADC trigger 9 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad9usrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 9 update source."]
        #[inline(always)]
        pub const fn set_ad9usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "ADC trigger 10 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad10usrc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 10 update source."]
        #[inline(always)]
        pub const fn set_ad10usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for HrtimAdcur {
        #[inline(always)]
        fn default() -> HrtimAdcur {
            HrtimAdcur(0)
        }
    }
    impl core::fmt::Debug for HrtimAdcur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimAdcur")
                .field("ad5usrc", &self.ad5usrc())
                .field("ad6usrc", &self.ad6usrc())
                .field("ad7usrc", &self.ad7usrc())
                .field("ad8usrc", &self.ad8usrc())
                .field("ad9usrc", &self.ad9usrc())
                .field("ad10usrc", &self.ad10usrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdcur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimAdcur {{ ad5usrc: {=u8:?}, ad6usrc: {=u8:?}, ad7usrc: {=u8:?}, ad8usrc: {=u8:?}, ad9usrc: {=u8:?}, ad10usrc: {=u8:?} }}",
                self.ad5usrc(),
                self.ad6usrc(),
                self.ad7usrc(),
                self.ad8usrc(),
                self.ad9usrc(),
                self.ad10usrc()
            )
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Data Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdmadr(pub u32);
    impl HrtimBdmadr {
        #[doc = "Burst DMA data register."]
        #[must_use]
        #[inline(always)]
        pub const fn bdmadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Burst DMA data register."]
        #[inline(always)]
        pub const fn set_bdmadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HrtimBdmadr {
        #[inline(always)]
        fn default() -> HrtimBdmadr {
            HrtimBdmadr(0)
        }
    }
    impl core::fmt::Debug for HrtimBdmadr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBdmadr").field("bdmadr", &self.bdmadr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdmadr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HrtimBdmadr {{ bdmadr: {=u32:?} }}", self.bdmadr())
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdmupr(pub u32);
    impl HrtimBdmupr {
        #[doc = "MCR register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MCR register update enable."]
        #[inline(always)]
        pub const fn set_mcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MICR register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn micr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MICR register update enable."]
        #[inline(always)]
        pub const fn set_micr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDIER register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mdier(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MDIER register update enable."]
        #[inline(always)]
        pub const fn set_mdier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MCNTR register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MCNTR register update enable."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MPER register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MPER register update enable."]
        #[inline(always)]
        pub const fn set_mper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MREP register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mrep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MREP register update enable."]
        #[inline(always)]
        pub const fn set_mrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MCMP1R register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MCMP1R register update enable."]
        #[inline(always)]
        pub const fn set_mcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MCMP2R register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MCMP2R register update enable."]
        #[inline(always)]
        pub const fn set_mcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MCMP3R register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MCMP3R register update enable."]
        #[inline(always)]
        pub const fn set_mcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MCMP4R register update enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MCMP4R register update enable."]
        #[inline(always)]
        pub const fn set_mcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for HrtimBdmupr {
        #[inline(always)]
        fn default() -> HrtimBdmupr {
            HrtimBdmupr(0)
        }
    }
    impl core::fmt::Debug for HrtimBdmupr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBdmupr")
                .field("mcr", &self.mcr())
                .field("micr", &self.micr())
                .field("mdier", &self.mdier())
                .field("mcnt", &self.mcnt())
                .field("mper", &self.mper())
                .field("mrep", &self.mrep())
                .field("mcmp1", &self.mcmp1())
                .field("mcmp2", &self.mcmp2())
                .field("mcmp3", &self.mcmp3())
                .field("mcmp4", &self.mcmp4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdmupr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimBdmupr {{ mcr: {=bool:?}, micr: {=bool:?}, mdier: {=bool:?}, mcnt: {=bool:?}, mper: {=bool:?}, mrep: {=bool:?}, mcmp1: {=bool:?}, mcmp2: {=bool:?}, mcmp3: {=bool:?}, mcmp4: {=bool:?} }}",
                self.mcr(),
                self.micr(),
                self.mdier(),
                self.mcnt(),
                self.mper(),
                self.mrep(),
                self.mcmp1(),
                self.mcmp2(),
                self.mcmp3(),
                self.mcmp4()
            )
        }
    }
    #[doc = "HRTIM burst DMA timer F update register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdtfupr(pub u32);
    impl HrtimBdtfupr {
        #[doc = "HRTIM_TIMxCR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxCR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HRTIM_TIMxICR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timficr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxICR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timficr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HRTIM_TIMxDIER register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfdier(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxDIER register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfdier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HRTIM_CNTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CNTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HRTIM_PERxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_PERxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HRTIM_REPxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfrep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_REPxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HRTIM_CMP1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HRTIM_CMP2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HRTIM_CMP3xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP3xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HRTIM_CMP4xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP4xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HRTIM_DTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfdtr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_DTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HRTIM_SET1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfset1r(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_SET1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfset1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HRTIM_RST1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfrst1r(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RST1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfrst1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HRTIM_SET2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfset2r(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_SET2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfset2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HRTIM_RST2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfrst2r(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RST2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfrst2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "HRTIM_EEFxR1 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfeefr1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR1 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfeefr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HRTIM_EEFxR2 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfeefr2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR2 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfeefr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HRTIM_RSTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfrstr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RSTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfrstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HRTIM_CHPxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfchpr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CHPxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfchpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HRTIM_OUTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfoutr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_OUTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfoutr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HRTIM_FLTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timffltr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_FLTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timffltr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HRTIM_TIMxCR2 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcr2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxCR2 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "HRTIM_EEFxR3 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfeefr3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR3 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfeefr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "HRTIM_EESELxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfeeselr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EESELxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfeeselr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HRTIM_COTRGxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcotrgr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_COTRGxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcotrgr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HRTIM_COFExR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcofer(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_COFExR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcofer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HRTIM_CODExR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcoder(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CODExR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfcoder(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for HrtimBdtfupr {
        #[inline(always)]
        fn default() -> HrtimBdtfupr {
            HrtimBdtfupr(0)
        }
    }
    impl core::fmt::Debug for HrtimBdtfupr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBdtfupr")
                .field("timfcr", &self.timfcr())
                .field("timficr", &self.timficr())
                .field("timfdier", &self.timfdier())
                .field("timfcnt", &self.timfcnt())
                .field("timfper", &self.timfper())
                .field("timfrep", &self.timfrep())
                .field("timfcmp1", &self.timfcmp1())
                .field("timfcmp2", &self.timfcmp2())
                .field("timfcmp3", &self.timfcmp3())
                .field("timfcmp4", &self.timfcmp4())
                .field("timfdtr", &self.timfdtr())
                .field("timfset1r", &self.timfset1r())
                .field("timfrst1r", &self.timfrst1r())
                .field("timfset2r", &self.timfset2r())
                .field("timfrst2r", &self.timfrst2r())
                .field("timfeefr1", &self.timfeefr1())
                .field("timfeefr2", &self.timfeefr2())
                .field("timfrstr", &self.timfrstr())
                .field("timfchpr", &self.timfchpr())
                .field("timfoutr", &self.timfoutr())
                .field("timffltr", &self.timffltr())
                .field("timfcr2", &self.timfcr2())
                .field("timfeefr3", &self.timfeefr3())
                .field("timfeeselr", &self.timfeeselr())
                .field("timfcotrgr", &self.timfcotrgr())
                .field("timfcofer", &self.timfcofer())
                .field("timfcoder", &self.timfcoder())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdtfupr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimBdtfupr {{ timfcr: {=bool:?}, timficr: {=bool:?}, timfdier: {=bool:?}, timfcnt: {=bool:?}, timfper: {=bool:?}, timfrep: {=bool:?}, timfcmp1: {=bool:?}, timfcmp2: {=bool:?}, timfcmp3: {=bool:?}, timfcmp4: {=bool:?}, timfdtr: {=bool:?}, timfset1r: {=bool:?}, timfrst1r: {=bool:?}, timfset2r: {=bool:?}, timfrst2r: {=bool:?}, timfeefr1: {=bool:?}, timfeefr2: {=bool:?}, timfrstr: {=bool:?}, timfchpr: {=bool:?}, timfoutr: {=bool:?}, timffltr: {=bool:?}, timfcr2: {=bool:?}, timfeefr3: {=bool:?}, timfeeselr: {=bool:?}, timfcotrgr: {=bool:?}, timfcofer: {=bool:?}, timfcoder: {=bool:?} }}",
                self.timfcr(),
                self.timficr(),
                self.timfdier(),
                self.timfcnt(),
                self.timfper(),
                self.timfrep(),
                self.timfcmp1(),
                self.timfcmp2(),
                self.timfcmp3(),
                self.timfcmp4(),
                self.timfdtr(),
                self.timfset1r(),
                self.timfrst1r(),
                self.timfset2r(),
                self.timfrst2r(),
                self.timfeefr1(),
                self.timfeefr2(),
                self.timfrstr(),
                self.timfchpr(),
                self.timfoutr(),
                self.timffltr(),
                self.timfcr2(),
                self.timfeefr3(),
                self.timfeeselr(),
                self.timfcotrgr(),
                self.timfcofer(),
                self.timfcoder()
            )
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdtupr(pub u32);
    impl HrtimBdtupr {
        #[doc = "HRTIM_TIMxCR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxCR register update enable timer x."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HRTIM_TIMxICR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn icr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxICR register update enable timer x."]
        #[inline(always)]
        pub const fn set_icr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HRTIM_TIMxDIER register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dier(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxDIER register update enable timer x."]
        #[inline(always)]
        pub const fn set_dier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HRTIM_CNTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CNTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HRTIM_PERxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_PERxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HRTIM_REPxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_REPxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HRTIM_CMP1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HRTIM_CMP2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HRTIM_CMP3xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP3xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HRTIM_CMP4xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CMP4xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HRTIM_DTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timdtr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_DTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timdtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HRTIM_SET1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timset1r(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_SET1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timset1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HRTIM_RST1xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timrst1r(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RST1xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timrst1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HRTIM_SET2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timset2r(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_SET2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timset2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HRTIM_RST2xR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timrst2r(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RST2xR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timrst2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "HRTIM_EEFxR1 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timeefr1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR1 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timeefr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HRTIM_EEFxR2 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timeefr2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR2 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timeefr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HRTIM_RSTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timrstr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_RSTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timrstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HRTIM_CHPxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timchpr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CHPxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timchpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HRTIM_OUTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timoutr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_OUTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timoutr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HRTIM_FLTxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timfltr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_FLTxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timfltr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HRTIM_TIMxCR2 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcr2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_TIMxCR2 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "HRTIM_EEFxR3 register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timeefr3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EEFxR3 register update enable timer x."]
        #[inline(always)]
        pub const fn set_timeefr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "HRTIM_EESELxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timeeselr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_EESELxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timeeselr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HRTIM_COTRGxR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcotrgr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_COTRGxR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcotrgr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HRTIM_COFExR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcofer(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_COFExR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcofer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HRTIM_CODExR register update enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn timcoder(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM_CODExR register update enable timer x."]
        #[inline(always)]
        pub const fn set_timcoder(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for HrtimBdtupr {
        #[inline(always)]
        fn default() -> HrtimBdtupr {
            HrtimBdtupr(0)
        }
    }
    impl core::fmt::Debug for HrtimBdtupr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBdtupr")
                .field("cr", &self.cr())
                .field("icr", &self.icr())
                .field("dier", &self.dier())
                .field("cnt", &self.cnt())
                .field("per", &self.per())
                .field("rep", &self.rep())
                .field("timcmp1", &self.timcmp1())
                .field("timcmp2", &self.timcmp2())
                .field("timcmp3", &self.timcmp3())
                .field("timcmp4", &self.timcmp4())
                .field("timdtr", &self.timdtr())
                .field("timset1r", &self.timset1r())
                .field("timrst1r", &self.timrst1r())
                .field("timset2r", &self.timset2r())
                .field("timrst2r", &self.timrst2r())
                .field("timeefr1", &self.timeefr1())
                .field("timeefr2", &self.timeefr2())
                .field("timrstr", &self.timrstr())
                .field("timchpr", &self.timchpr())
                .field("timoutr", &self.timoutr())
                .field("timfltr", &self.timfltr())
                .field("timcr2", &self.timcr2())
                .field("timeefr3", &self.timeefr3())
                .field("timeeselr", &self.timeeselr())
                .field("timcotrgr", &self.timcotrgr())
                .field("timcofer", &self.timcofer())
                .field("timcoder", &self.timcoder())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdtupr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimBdtupr {{ cr: {=bool:?}, icr: {=bool:?}, dier: {=bool:?}, cnt: {=bool:?}, per: {=bool:?}, rep: {=bool:?}, timcmp1: {=bool:?}, timcmp2: {=bool:?}, timcmp3: {=bool:?}, timcmp4: {=bool:?}, timdtr: {=bool:?}, timset1r: {=bool:?}, timrst1r: {=bool:?}, timset2r: {=bool:?}, timrst2r: {=bool:?}, timeefr1: {=bool:?}, timeefr2: {=bool:?}, timrstr: {=bool:?}, timchpr: {=bool:?}, timoutr: {=bool:?}, timfltr: {=bool:?}, timcr2: {=bool:?}, timeefr3: {=bool:?}, timeeselr: {=bool:?}, timcotrgr: {=bool:?}, timcofer: {=bool:?}, timcoder: {=bool:?} }}",
                self.cr(),
                self.icr(),
                self.dier(),
                self.cnt(),
                self.per(),
                self.rep(),
                self.timcmp1(),
                self.timcmp2(),
                self.timcmp3(),
                self.timcmp4(),
                self.timdtr(),
                self.timset1r(),
                self.timrst1r(),
                self.timset2r(),
                self.timrst2r(),
                self.timeefr1(),
                self.timeefr2(),
                self.timrstr(),
                self.timchpr(),
                self.timoutr(),
                self.timfltr(),
                self.timcr2(),
                self.timeefr3(),
                self.timeeselr(),
                self.timcotrgr(),
                self.timcofer(),
                self.timcoder()
            )
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Compare Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmcmpr(pub u32);
    impl HrtimBmcmpr {
        #[doc = "Burst mode compare value."]
        #[must_use]
        #[inline(always)]
        pub const fn bmcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Burst mode compare value."]
        #[inline(always)]
        pub const fn set_bmcmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HrtimBmcmpr {
        #[inline(always)]
        fn default() -> HrtimBmcmpr {
            HrtimBmcmpr(0)
        }
    }
    impl core::fmt::Debug for HrtimBmcmpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBmcmpr").field("bmcmp", &self.bmcmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmcmpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HrtimBmcmpr {{ bmcmp: {=u16:?} }}", self.bmcmp())
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmcr(pub u32);
    impl HrtimBmcr {
        #[doc = "Burst mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bme(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode enable."]
        #[inline(always)]
        pub const fn set_bme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Burst mode operating mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bmom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode operating mode."]
        #[inline(always)]
        pub const fn set_bmom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Burst mode clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn bmclk(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Burst mode clock source."]
        #[inline(always)]
        pub const fn set_bmclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Burst mode prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn bmprsc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "Burst mode prescaler."]
        #[inline(always)]
        pub const fn set_bmprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "Burst mode preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpren(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode preload enable."]
        #[inline(always)]
        pub const fn set_bmpren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Master timer burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mtbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer burst mode."]
        #[inline(always)]
        pub const fn set_mtbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer A burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tabm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A burst mode."]
        #[inline(always)]
        pub const fn set_tabm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer B burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tbbm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B burst mode."]
        #[inline(always)]
        pub const fn set_tbbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer C burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tcbm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C burst mode."]
        #[inline(always)]
        pub const fn set_tcbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer D burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tdbm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D burst mode."]
        #[inline(always)]
        pub const fn set_tdbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer E burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tebm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E burst mode."]
        #[inline(always)]
        pub const fn set_tebm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer F burst mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tfbm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F burst mode."]
        #[inline(always)]
        pub const fn set_tfbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Burst mode status."]
        #[must_use]
        #[inline(always)]
        pub const fn bmstat(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode status."]
        #[inline(always)]
        pub const fn set_bmstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimBmcr {
        #[inline(always)]
        fn default() -> HrtimBmcr {
            HrtimBmcr(0)
        }
    }
    impl core::fmt::Debug for HrtimBmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBmcr")
                .field("bme", &self.bme())
                .field("bmom", &self.bmom())
                .field("bmclk", &self.bmclk())
                .field("bmprsc", &self.bmprsc())
                .field("bmpren", &self.bmpren())
                .field("mtbm", &self.mtbm())
                .field("tabm", &self.tabm())
                .field("tbbm", &self.tbbm())
                .field("tcbm", &self.tcbm())
                .field("tdbm", &self.tdbm())
                .field("tebm", &self.tebm())
                .field("tfbm", &self.tfbm())
                .field("bmstat", &self.bmstat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimBmcr {{ bme: {=bool:?}, bmom: {=bool:?}, bmclk: {=u8:?}, bmprsc: {=u8:?}, bmpren: {=bool:?}, mtbm: {=bool:?}, tabm: {=bool:?}, tbbm: {=bool:?}, tcbm: {=bool:?}, tdbm: {=bool:?}, tebm: {=bool:?}, tfbm: {=bool:?}, bmstat: {=bool:?} }}",
                self.bme(),
                self.bmom(),
                self.bmclk(),
                self.bmprsc(),
                self.bmpren(),
                self.mtbm(),
                self.tabm(),
                self.tbbm(),
                self.tcbm(),
                self.tdbm(),
                self.tebm(),
                self.tfbm(),
                self.bmstat()
            )
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmper(pub u32);
    impl HrtimBmper {
        #[doc = "Burst mode period."]
        #[must_use]
        #[inline(always)]
        pub const fn bmper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Burst mode period."]
        #[inline(always)]
        pub const fn set_bmper(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HrtimBmper {
        #[inline(always)]
        fn default() -> HrtimBmper {
            HrtimBmper(0)
        }
    }
    impl core::fmt::Debug for HrtimBmper {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBmper").field("bmper", &self.bmper()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmper {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HrtimBmper {{ bmper: {=u16:?} }}", self.bmper())
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Trigger Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmtrgr(pub u32);
    impl HrtimBmtrgr {
        #[doc = "Software start."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software start."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master reset or roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn mstrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master reset or roll-over."]
        #[inline(always)]
        pub const fn set_mstrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn mstrep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master repetition."]
        #[inline(always)]
        pub const fn set_mstrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1."]
        #[inline(always)]
        pub const fn set_mstcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2."]
        #[inline(always)]
        pub const fn set_mstcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3."]
        #[inline(always)]
        pub const fn set_mstcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4."]
        #[inline(always)]
        pub const fn set_mstcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer A reset or roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn tarst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A reset or roll-over."]
        #[inline(always)]
        pub const fn set_tarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer A repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn tarep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A repetition."]
        #[inline(always)]
        pub const fn set_tarep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer A compare 1 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tacmp1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A compare 1 event."]
        #[inline(always)]
        pub const fn set_tacmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer A compare 2 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tacmp2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A compare 2 event."]
        #[inline(always)]
        pub const fn set_tacmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer B reset or roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn tbrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B reset or roll-over."]
        #[inline(always)]
        pub const fn set_tbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timer B repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn tbrep(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B repetition."]
        #[inline(always)]
        pub const fn set_tbrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer B compare 1 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tbcmp1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 1 event."]
        #[inline(always)]
        pub const fn set_tbcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer B compare 2 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tbcmp2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 2 event."]
        #[inline(always)]
        pub const fn set_tbcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timer C reset or roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn tcrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C reset or roll-over."]
        #[inline(always)]
        pub const fn set_tcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer C repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn tcrep(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C repetition."]
        #[inline(always)]
        pub const fn set_tcrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer C compare 1 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tccmp1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 1 event."]
        #[inline(always)]
        pub const fn set_tccmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer F reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tfrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F reset."]
        #[inline(always)]
        pub const fn set_tfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer D reset or roll-over."]
        #[must_use]
        #[inline(always)]
        pub const fn tdrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D reset or roll-over."]
        #[inline(always)]
        pub const fn set_tdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer D repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn tdrep(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D repetition."]
        #[inline(always)]
        pub const fn set_tdrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer F repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn tfrep(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F repetition."]
        #[inline(always)]
        pub const fn set_tfrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer D compare 2 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcmp2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 2 event."]
        #[inline(always)]
        pub const fn set_tdcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Timer F compare 1 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tfcmp1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F compare 1 event."]
        #[inline(always)]
        pub const fn set_tfcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Timer E repetition."]
        #[must_use]
        #[inline(always)]
        pub const fn terep(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E repetition."]
        #[inline(always)]
        pub const fn set_terep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer E compare 1 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tecmp1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 1 event."]
        #[inline(always)]
        pub const fn set_tecmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Timer E compare 2 event."]
        #[must_use]
        #[inline(always)]
        pub const fn tecmp2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 2 event."]
        #[inline(always)]
        pub const fn set_tecmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Timer A period following external event G."]
        #[must_use]
        #[inline(always)]
        pub const fn taeevg(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A period following external event G."]
        #[inline(always)]
        pub const fn set_taeevg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Timer D period following external event H."]
        #[must_use]
        #[inline(always)]
        pub const fn tdeevh(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D period following external event H."]
        #[inline(always)]
        pub const fn set_tdeevh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External event G (TIMA filters applied)."]
        #[must_use]
        #[inline(always)]
        pub const fn eevg(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External event G (TIMA filters applied)."]
        #[inline(always)]
        pub const fn set_eevg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "External event H (TIMD filters applied)."]
        #[must_use]
        #[inline(always)]
        pub const fn eevh(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "External event H (TIMD filters applied)."]
        #[inline(always)]
        pub const fn set_eevh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "On-chip event."]
        #[must_use]
        #[inline(always)]
        pub const fn ochpev(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "On-chip event."]
        #[inline(always)]
        pub const fn set_ochpev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimBmtrgr {
        #[inline(always)]
        fn default() -> HrtimBmtrgr {
            HrtimBmtrgr(0)
        }
    }
    impl core::fmt::Debug for HrtimBmtrgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimBmtrgr")
                .field("sw", &self.sw())
                .field("mstrst", &self.mstrst())
                .field("mstrep", &self.mstrep())
                .field("mstcmp1", &self.mstcmp1())
                .field("mstcmp2", &self.mstcmp2())
                .field("mstcmp3", &self.mstcmp3())
                .field("mstcmp4", &self.mstcmp4())
                .field("tarst", &self.tarst())
                .field("tarep", &self.tarep())
                .field("tacmp1", &self.tacmp1())
                .field("tacmp2", &self.tacmp2())
                .field("tbrst", &self.tbrst())
                .field("tbrep", &self.tbrep())
                .field("tbcmp1", &self.tbcmp1())
                .field("tbcmp2", &self.tbcmp2())
                .field("tcrst", &self.tcrst())
                .field("tcrep", &self.tcrep())
                .field("tccmp1", &self.tccmp1())
                .field("tfrst", &self.tfrst())
                .field("tdrst", &self.tdrst())
                .field("tdrep", &self.tdrep())
                .field("tfrep", &self.tfrep())
                .field("tdcmp2", &self.tdcmp2())
                .field("tfcmp1", &self.tfcmp1())
                .field("terep", &self.terep())
                .field("tecmp1", &self.tecmp1())
                .field("tecmp2", &self.tecmp2())
                .field("taeevg", &self.taeevg())
                .field("tdeevh", &self.tdeevh())
                .field("eevg", &self.eevg())
                .field("eevh", &self.eevh())
                .field("ochpev", &self.ochpev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmtrgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimBmtrgr {{ sw: {=bool:?}, mstrst: {=bool:?}, mstrep: {=bool:?}, mstcmp1: {=bool:?}, mstcmp2: {=bool:?}, mstcmp3: {=bool:?}, mstcmp4: {=bool:?}, tarst: {=bool:?}, tarep: {=bool:?}, tacmp1: {=bool:?}, tacmp2: {=bool:?}, tbrst: {=bool:?}, tbrep: {=bool:?}, tbcmp1: {=bool:?}, tbcmp2: {=bool:?}, tcrst: {=bool:?}, tcrep: {=bool:?}, tccmp1: {=bool:?}, tfrst: {=bool:?}, tdrst: {=bool:?}, tdrep: {=bool:?}, tfrep: {=bool:?}, tdcmp2: {=bool:?}, tfcmp1: {=bool:?}, terep: {=bool:?}, tecmp1: {=bool:?}, tecmp2: {=bool:?}, taeevg: {=bool:?}, tdeevh: {=bool:?}, eevg: {=bool:?}, eevh: {=bool:?}, ochpev: {=bool:?} }}",
                self.sw(),
                self.mstrst(),
                self.mstrep(),
                self.mstcmp1(),
                self.mstcmp2(),
                self.mstcmp3(),
                self.mstcmp4(),
                self.tarst(),
                self.tarep(),
                self.tacmp1(),
                self.tacmp2(),
                self.tbrst(),
                self.tbrep(),
                self.tbcmp1(),
                self.tbcmp2(),
                self.tcrst(),
                self.tcrep(),
                self.tccmp1(),
                self.tfrst(),
                self.tdrst(),
                self.tdrep(),
                self.tfrep(),
                self.tdcmp2(),
                self.tfcmp1(),
                self.terep(),
                self.tecmp1(),
                self.tecmp2(),
                self.taeevg(),
                self.tdeevh(),
                self.eevg(),
                self.eevh(),
                self.ochpev()
            )
        }
    }
    #[doc = "High Resolution Timer: Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimCr1(pub u32);
    impl HrtimCr1 {
        #[doc = "Master update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn mudis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master update disable."]
        #[inline(always)]
        pub const fn set_mudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn taudis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A update disable."]
        #[inline(always)]
        pub const fn set_taudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer B update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbudis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B update disable."]
        #[inline(always)]
        pub const fn set_tbudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer C update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcudis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C update disable."]
        #[inline(always)]
        pub const fn set_tcudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer D update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tdudis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D update disable."]
        #[inline(always)]
        pub const fn set_tdudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer E update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn teudis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E update disable."]
        #[inline(always)]
        pub const fn set_teudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer F update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfudis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F update disable."]
        #[inline(always)]
        pub const fn set_tfudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Remote master update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rmudis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Remote master update disable."]
        #[inline(always)]
        pub const fn set_rmudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Remote timer A update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtaudis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer A update disable."]
        #[inline(always)]
        pub const fn set_rtaudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Remote Timer B update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtbudis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Timer B update disable."]
        #[inline(always)]
        pub const fn set_rtbudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Remote Timer C update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcudis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Timer C update disable."]
        #[inline(always)]
        pub const fn set_rtcudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Remote Timer D update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtdudis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Timer D update disable."]
        #[inline(always)]
        pub const fn set_rtdudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Remote Timer E update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rteudis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Timer E update disable."]
        #[inline(always)]
        pub const fn set_rteudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Remote Timer F update disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtfudis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Timer F update disable."]
        #[inline(always)]
        pub const fn set_rtfudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC trigger 1 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad1usrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 1 update source."]
        #[inline(always)]
        pub const fn set_ad1usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "ADC trigger 2 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad2usrc(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 2 update source."]
        #[inline(always)]
        pub const fn set_ad2usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
        #[doc = "ADC trigger 3 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad3usrc(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 3 update source."]
        #[inline(always)]
        pub const fn set_ad3usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[doc = "ADC trigger 4 update source."]
        #[must_use]
        #[inline(always)]
        pub const fn ad4usrc(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "ADC trigger 4 update source."]
        #[inline(always)]
        pub const fn set_ad4usrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
    }
    impl Default for HrtimCr1 {
        #[inline(always)]
        fn default() -> HrtimCr1 {
            HrtimCr1(0)
        }
    }
    impl core::fmt::Debug for HrtimCr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimCr1")
                .field("mudis", &self.mudis())
                .field("taudis", &self.taudis())
                .field("tbudis", &self.tbudis())
                .field("tcudis", &self.tcudis())
                .field("tdudis", &self.tdudis())
                .field("teudis", &self.teudis())
                .field("tfudis", &self.tfudis())
                .field("rmudis", &self.rmudis())
                .field("rtaudis", &self.rtaudis())
                .field("rtbudis", &self.rtbudis())
                .field("rtcudis", &self.rtcudis())
                .field("rtdudis", &self.rtdudis())
                .field("rteudis", &self.rteudis())
                .field("rtfudis", &self.rtfudis())
                .field("ad1usrc", &self.ad1usrc())
                .field("ad2usrc", &self.ad2usrc())
                .field("ad3usrc", &self.ad3usrc())
                .field("ad4usrc", &self.ad4usrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimCr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimCr1 {{ mudis: {=bool:?}, taudis: {=bool:?}, tbudis: {=bool:?}, tcudis: {=bool:?}, tdudis: {=bool:?}, teudis: {=bool:?}, tfudis: {=bool:?}, rmudis: {=bool:?}, rtaudis: {=bool:?}, rtbudis: {=bool:?}, rtcudis: {=bool:?}, rtdudis: {=bool:?}, rteudis: {=bool:?}, rtfudis: {=bool:?}, ad1usrc: {=u8:?}, ad2usrc: {=u8:?}, ad3usrc: {=u8:?}, ad4usrc: {=u8:?} }}",
                self.mudis(),
                self.taudis(),
                self.tbudis(),
                self.tcudis(),
                self.tdudis(),
                self.teudis(),
                self.tfudis(),
                self.rmudis(),
                self.rtaudis(),
                self.rtbudis(),
                self.rtcudis(),
                self.rtdudis(),
                self.rteudis(),
                self.rtfudis(),
                self.ad1usrc(),
                self.ad2usrc(),
                self.ad3usrc(),
                self.ad4usrc()
            )
        }
    }
    #[doc = "High Resolution Timer: Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimCr2(pub u32);
    impl HrtimCr2 {
        #[doc = "Master timer software update."]
        #[must_use]
        #[inline(always)]
        pub const fn mswu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer software update."]
        #[inline(always)]
        pub const fn set_mswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A software update."]
        #[must_use]
        #[inline(always)]
        pub const fn taswu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A software update."]
        #[inline(always)]
        pub const fn set_taswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer B software update."]
        #[must_use]
        #[inline(always)]
        pub const fn tbswu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B software update."]
        #[inline(always)]
        pub const fn set_tbswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer C software update."]
        #[must_use]
        #[inline(always)]
        pub const fn tcswu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C software update."]
        #[inline(always)]
        pub const fn set_tcswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer D software update."]
        #[must_use]
        #[inline(always)]
        pub const fn tdswu(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D software update."]
        #[inline(always)]
        pub const fn set_tdswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer E software update."]
        #[must_use]
        #[inline(always)]
        pub const fn teswu(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E software update."]
        #[inline(always)]
        pub const fn set_teswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer F software update."]
        #[must_use]
        #[inline(always)]
        pub const fn tfswu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F software update."]
        #[inline(always)]
        pub const fn set_tfswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn mrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master counter software reset."]
        #[inline(always)]
        pub const fn set_mrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer A counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tarst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A counter software reset."]
        #[inline(always)]
        pub const fn set_tarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer B counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tbrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B counter software reset."]
        #[inline(always)]
        pub const fn set_tbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer C counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tcrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C counter software reset."]
        #[inline(always)]
        pub const fn set_tcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timer D counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tdrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D counter software reset."]
        #[inline(always)]
        pub const fn set_tdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer E counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn terst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E counter software reset."]
        #[inline(always)]
        pub const fn set_terst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer F counter software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tfrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F counter software reset."]
        #[inline(always)]
        pub const fn set_tfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Swap timer A outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpa(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer A outputs."]
        #[inline(always)]
        pub const fn set_swpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Swap timer B outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer B outputs."]
        #[inline(always)]
        pub const fn set_swpb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Swap timer C outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer C outputs."]
        #[inline(always)]
        pub const fn set_swpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Swap timer D outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer D outputs."]
        #[inline(always)]
        pub const fn set_swpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Swap timer E outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer E outputs."]
        #[inline(always)]
        pub const fn set_swpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Swap timer F outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn swpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Swap timer F outputs."]
        #[inline(always)]
        pub const fn set_swpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA burst source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dbss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "DMA burst source selection."]
        #[inline(always)]
        pub const fn set_dbss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for HrtimCr2 {
        #[inline(always)]
        fn default() -> HrtimCr2 {
            HrtimCr2(0)
        }
    }
    impl core::fmt::Debug for HrtimCr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimCr2")
                .field("mswu", &self.mswu())
                .field("taswu", &self.taswu())
                .field("tbswu", &self.tbswu())
                .field("tcswu", &self.tcswu())
                .field("tdswu", &self.tdswu())
                .field("teswu", &self.teswu())
                .field("tfswu", &self.tfswu())
                .field("mrst", &self.mrst())
                .field("tarst", &self.tarst())
                .field("tbrst", &self.tbrst())
                .field("tcrst", &self.tcrst())
                .field("tdrst", &self.tdrst())
                .field("terst", &self.terst())
                .field("tfrst", &self.tfrst())
                .field("swpa", &self.swpa())
                .field("swpb", &self.swpb())
                .field("swpc", &self.swpc())
                .field("swpd", &self.swpd())
                .field("swpe", &self.swpe())
                .field("swpf", &self.swpf())
                .field("dbss", &self.dbss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimCr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimCr2 {{ mswu: {=bool:?}, taswu: {=bool:?}, tbswu: {=bool:?}, tcswu: {=bool:?}, tdswu: {=bool:?}, teswu: {=bool:?}, tfswu: {=bool:?}, mrst: {=bool:?}, tarst: {=bool:?}, tbrst: {=bool:?}, tcrst: {=bool:?}, tdrst: {=bool:?}, terst: {=bool:?}, tfrst: {=bool:?}, swpa: {=bool:?}, swpb: {=bool:?}, swpc: {=bool:?}, swpd: {=bool:?}, swpe: {=bool:?}, swpf: {=bool:?}, dbss: {=u8:?} }}",
                self.mswu(),
                self.taswu(),
                self.tbswu(),
                self.tcswu(),
                self.tdswu(),
                self.teswu(),
                self.tfswu(),
                self.mrst(),
                self.tarst(),
                self.tbrst(),
                self.tcrst(),
                self.tdrst(),
                self.terst(),
                self.tfrst(),
                self.swpa(),
                self.swpb(),
                self.swpc(),
                self.swpd(),
                self.swpe(),
                self.swpf(),
                self.dbss()
            )
        }
    }
    #[doc = "HRTIM control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimCr3(pub u32);
    impl HrtimCr3 {
        #[doc = "Remote master update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rmudisa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Remote master update disable authorization."]
        #[inline(always)]
        pub const fn set_rmudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Remote timer A update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rtaudisa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer A update disable authorization."]
        #[inline(always)]
        pub const fn set_rtaudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remote timer B update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rtbudisa(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer B update disable authorization."]
        #[inline(always)]
        pub const fn set_rtbudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Remote timer C update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcudisa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer C update disable authorization."]
        #[inline(always)]
        pub const fn set_rtcudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Remote timer D update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rtdudisa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer D update disable authorization."]
        #[inline(always)]
        pub const fn set_rtdudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Remote timer E update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rteudisa(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer E update disable authorization."]
        #[inline(always)]
        pub const fn set_rteudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Remote timer F update disable authorization."]
        #[must_use]
        #[inline(always)]
        pub const fn rtfudisa(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Remote timer F update disable authorization."]
        #[inline(always)]
        pub const fn set_rtfudisa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for HrtimCr3 {
        #[inline(always)]
        fn default() -> HrtimCr3 {
            HrtimCr3(0)
        }
    }
    impl core::fmt::Debug for HrtimCr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimCr3")
                .field("rmudisa", &self.rmudisa())
                .field("rtaudisa", &self.rtaudisa())
                .field("rtbudisa", &self.rtbudisa())
                .field("rtcudisa", &self.rtcudisa())
                .field("rtdudisa", &self.rtdudisa())
                .field("rteudisa", &self.rteudisa())
                .field("rtfudisa", &self.rtfudisa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimCr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimCr3 {{ rmudisa: {=bool:?}, rtaudisa: {=bool:?}, rtbudisa: {=bool:?}, rtcudisa: {=bool:?}, rtdudisa: {=bool:?}, rteudisa: {=bool:?}, rtfudisa: {=bool:?} }}",
                self.rmudisa(),
                self.rtaudisa(),
                self.rtbudisa(),
                self.rtcudisa(),
                self.rtdudisa(),
                self.rteudisa(),
                self.rtfudisa()
            )
        }
    }
    #[doc = "High Resolution Timer: DLL Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimDllcr(pub u32);
    impl HrtimDllcr {
        #[doc = "DLL Calibration Start."]
        #[must_use]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Calibration Start."]
        #[inline(always)]
        pub const fn set_cal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DLL Calibration Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn calen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Calibration Enable."]
        #[inline(always)]
        pub const fn set_calen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DLL Calibration rate."]
        #[must_use]
        #[inline(always)]
        pub const fn calrte(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "DLL Calibration rate."]
        #[inline(always)]
        pub const fn set_calrte(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for HrtimDllcr {
        #[inline(always)]
        fn default() -> HrtimDllcr {
            HrtimDllcr(0)
        }
    }
    impl core::fmt::Debug for HrtimDllcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimDllcr")
                .field("cal", &self.cal())
                .field("calen", &self.calen())
                .field("calrte", &self.calrte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimDllcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimDllcr {{ cal: {=bool:?}, calen: {=bool:?}, calrte: {=u8:?} }}",
                self.cal(),
                self.calen(),
                self.calrte()
            )
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr1(pub u32);
    impl HrtimEecr1 {
        #[doc = "External event 1 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "External event 1 source."]
        #[inline(always)]
        pub const fn set_ee1src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External event 1 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1pol(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event 1 polarity."]
        #[inline(always)]
        pub const fn set_ee1pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event 1 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1sns(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "External event 1 sensitivity."]
        #[inline(always)]
        pub const fn set_ee1sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "External event 1 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1fast(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External event 1 fast mode."]
        #[inline(always)]
        pub const fn set_ee1fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External event 2 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2src(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "External event 2 source."]
        #[inline(always)]
        pub const fn set_ee2src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "External event 2 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2pol(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event 2 polarity."]
        #[inline(always)]
        pub const fn set_ee2pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event 2 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2sns(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "External event 2 sensitivity."]
        #[inline(always)]
        pub const fn set_ee2sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "External event 2 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2fast(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External event 2 fast mode."]
        #[inline(always)]
        pub const fn set_ee2fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event 3 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3src(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External event 3 source."]
        #[inline(always)]
        pub const fn set_ee3src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "External event 3 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3pol(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event 3 polarity."]
        #[inline(always)]
        pub const fn set_ee3pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event 3 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3sns(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "External event 3 sensitivity."]
        #[inline(always)]
        pub const fn set_ee3sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "External event 3 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3fast(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External event 3 fast mode."]
        #[inline(always)]
        pub const fn set_ee3fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External event 4 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4src(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "External event 4 source."]
        #[inline(always)]
        pub const fn set_ee4src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "External event 4 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4pol(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External event 4 polarity."]
        #[inline(always)]
        pub const fn set_ee4pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event 4 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4sns(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "External event 4 sensitivity."]
        #[inline(always)]
        pub const fn set_ee4sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "External event 4 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4fast(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "External event 4 fast mode."]
        #[inline(always)]
        pub const fn set_ee4fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "External event 5 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5src(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "External event 5 source."]
        #[inline(always)]
        pub const fn set_ee5src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "External event 5 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5pol(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event 5 polarity."]
        #[inline(always)]
        pub const fn set_ee5pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event 5 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5sns(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "External event 5 sensitivity."]
        #[inline(always)]
        pub const fn set_ee5sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
        #[doc = "External event 5 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5fast(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External event 5 fast mode."]
        #[inline(always)]
        pub const fn set_ee5fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for HrtimEecr1 {
        #[inline(always)]
        fn default() -> HrtimEecr1 {
            HrtimEecr1(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr1")
                .field("ee1src", &self.ee1src())
                .field("ee1pol", &self.ee1pol())
                .field("ee1sns", &self.ee1sns())
                .field("ee1fast", &self.ee1fast())
                .field("ee2src", &self.ee2src())
                .field("ee2pol", &self.ee2pol())
                .field("ee2sns", &self.ee2sns())
                .field("ee2fast", &self.ee2fast())
                .field("ee3src", &self.ee3src())
                .field("ee3pol", &self.ee3pol())
                .field("ee3sns", &self.ee3sns())
                .field("ee3fast", &self.ee3fast())
                .field("ee4src", &self.ee4src())
                .field("ee4pol", &self.ee4pol())
                .field("ee4sns", &self.ee4sns())
                .field("ee4fast", &self.ee4fast())
                .field("ee5src", &self.ee5src())
                .field("ee5pol", &self.ee5pol())
                .field("ee5sns", &self.ee5sns())
                .field("ee5fast", &self.ee5fast())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr1 {{ ee1src: {=u8:?}, ee1pol: {=bool:?}, ee1sns: {=u8:?}, ee1fast: {=bool:?}, ee2src: {=u8:?}, ee2pol: {=bool:?}, ee2sns: {=u8:?}, ee2fast: {=bool:?}, ee3src: {=u8:?}, ee3pol: {=bool:?}, ee3sns: {=u8:?}, ee3fast: {=bool:?}, ee4src: {=u8:?}, ee4pol: {=bool:?}, ee4sns: {=u8:?}, ee4fast: {=bool:?}, ee5src: {=u8:?}, ee5pol: {=bool:?}, ee5sns: {=u8:?}, ee5fast: {=bool:?} }}",
                self.ee1src(),
                self.ee1pol(),
                self.ee1sns(),
                self.ee1fast(),
                self.ee2src(),
                self.ee2pol(),
                self.ee2sns(),
                self.ee2fast(),
                self.ee3src(),
                self.ee3pol(),
                self.ee3sns(),
                self.ee3fast(),
                self.ee4src(),
                self.ee4pol(),
                self.ee4sns(),
                self.ee4fast(),
                self.ee5src(),
                self.ee5pol(),
                self.ee5sns(),
                self.ee5fast()
            )
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr2(pub u32);
    impl HrtimEecr2 {
        #[doc = "External event 6 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "External event 6 source."]
        #[inline(always)]
        pub const fn set_ee6src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External event 6 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6pol(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event 6 polarity."]
        #[inline(always)]
        pub const fn set_ee6pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event 6 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6sns(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "External event 6 sensitivity."]
        #[inline(always)]
        pub const fn set_ee6sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "External event 7 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7src(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "External event 7 source."]
        #[inline(always)]
        pub const fn set_ee7src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "External event 7 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7pol(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event 7 polarity."]
        #[inline(always)]
        pub const fn set_ee7pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event 7 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7sns(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "External event 7 sensitivity."]
        #[inline(always)]
        pub const fn set_ee7sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "External event 8 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8src(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External event 8 source."]
        #[inline(always)]
        pub const fn set_ee8src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "External event 8 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8pol(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event 8 polarity."]
        #[inline(always)]
        pub const fn set_ee8pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event 8 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8sns(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "External event 8 sensitivity."]
        #[inline(always)]
        pub const fn set_ee8sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "External event 9 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9src(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "External event 9 source."]
        #[inline(always)]
        pub const fn set_ee9src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "External event 9 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9pol(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External event 9 polarity."]
        #[inline(always)]
        pub const fn set_ee9pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event 9 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9sns(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "External event 9 sensitivity."]
        #[inline(always)]
        pub const fn set_ee9sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "External event 10 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10src(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "External event 10 source."]
        #[inline(always)]
        pub const fn set_ee10src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "External event 10 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10pol(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event 10 polarity."]
        #[inline(always)]
        pub const fn set_ee10pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event 10 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10sns(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "External event 10 sensitivity."]
        #[inline(always)]
        pub const fn set_ee10sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
    }
    impl Default for HrtimEecr2 {
        #[inline(always)]
        fn default() -> HrtimEecr2 {
            HrtimEecr2(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr2")
                .field("ee6src", &self.ee6src())
                .field("ee6pol", &self.ee6pol())
                .field("ee6sns", &self.ee6sns())
                .field("ee7src", &self.ee7src())
                .field("ee7pol", &self.ee7pol())
                .field("ee7sns", &self.ee7sns())
                .field("ee8src", &self.ee8src())
                .field("ee8pol", &self.ee8pol())
                .field("ee8sns", &self.ee8sns())
                .field("ee9src", &self.ee9src())
                .field("ee9pol", &self.ee9pol())
                .field("ee9sns", &self.ee9sns())
                .field("ee10src", &self.ee10src())
                .field("ee10pol", &self.ee10pol())
                .field("ee10sns", &self.ee10sns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr2 {{ ee6src: {=u8:?}, ee6pol: {=bool:?}, ee6sns: {=u8:?}, ee7src: {=u8:?}, ee7pol: {=bool:?}, ee7sns: {=u8:?}, ee8src: {=u8:?}, ee8pol: {=bool:?}, ee8sns: {=u8:?}, ee9src: {=u8:?}, ee9pol: {=bool:?}, ee9sns: {=u8:?}, ee10src: {=u8:?}, ee10pol: {=bool:?}, ee10sns: {=u8:?} }}",
                self.ee6src(),
                self.ee6pol(),
                self.ee6sns(),
                self.ee7src(),
                self.ee7pol(),
                self.ee7sns(),
                self.ee8src(),
                self.ee8pol(),
                self.ee8sns(),
                self.ee9src(),
                self.ee9pol(),
                self.ee9sns(),
                self.ee10src(),
                self.ee10pol(),
                self.ee10sns()
            )
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr3(pub u32);
    impl HrtimEecr3 {
        #[doc = "External event 6 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6f(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 6 filter."]
        #[inline(always)]
        pub const fn set_ee6f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External event 7 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7f(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 7 filter."]
        #[inline(always)]
        pub const fn set_ee7f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "External event 8 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 8 filter."]
        #[inline(always)]
        pub const fn set_ee8f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "External event 9 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9f(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 9 filter."]
        #[inline(always)]
        pub const fn set_ee9f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "External event 10 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10f(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 10 filter."]
        #[inline(always)]
        pub const fn set_ee10f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "External event sampling clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn eevsd(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "External event sampling clock division."]
        #[inline(always)]
        pub const fn set_eevsd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for HrtimEecr3 {
        #[inline(always)]
        fn default() -> HrtimEecr3 {
            HrtimEecr3(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr3")
                .field("ee6f", &self.ee6f())
                .field("ee7f", &self.ee7f())
                .field("ee8f", &self.ee8f())
                .field("ee9f", &self.ee9f())
                .field("ee10f", &self.ee10f())
                .field("eevsd", &self.eevsd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr3 {{ ee6f: {=u8:?}, ee7f: {=u8:?}, ee8f: {=u8:?}, ee9f: {=u8:?}, ee10f: {=u8:?}, eevsd: {=u8:?} }}",
                self.ee6f(),
                self.ee7f(),
                self.ee8f(),
                self.ee9f(),
                self.ee10f(),
                self.eevsd()
            )
        }
    }
    #[doc = "HRTIM timer external event control register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr4(pub u32);
    impl HrtimEecr4 {
        #[doc = "External event 11 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee11src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "External event 11 source."]
        #[inline(always)]
        pub const fn set_ee11src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External event 11 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee11pol(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event 11 polarity."]
        #[inline(always)]
        pub const fn set_ee11pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event 11 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee11sns(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "External event 11 sensitivity."]
        #[inline(always)]
        pub const fn set_ee11sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "External event 11 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee11fast(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External event 11 fast mode."]
        #[inline(always)]
        pub const fn set_ee11fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External event 2 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee12src(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "External event 2 source."]
        #[inline(always)]
        pub const fn set_ee12src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "External event 2 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee12pol(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event 2 polarity."]
        #[inline(always)]
        pub const fn set_ee12pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event 2 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee12sns(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "External event 2 sensitivity."]
        #[inline(always)]
        pub const fn set_ee12sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "External event 2 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee12fast(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External event 2 fast mode."]
        #[inline(always)]
        pub const fn set_ee12fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event 3 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee13src(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External event 3 source."]
        #[inline(always)]
        pub const fn set_ee13src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "External event 3 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee13pol(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event 3 polarity."]
        #[inline(always)]
        pub const fn set_ee13pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event 3 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee13sns(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "External event 3 sensitivity."]
        #[inline(always)]
        pub const fn set_ee13sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
        #[doc = "External event 3 fast mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ee13fast(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External event 3 fast mode."]
        #[inline(always)]
        pub const fn set_ee13fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for HrtimEecr4 {
        #[inline(always)]
        fn default() -> HrtimEecr4 {
            HrtimEecr4(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr4")
                .field("ee11src", &self.ee11src())
                .field("ee11pol", &self.ee11pol())
                .field("ee11sns", &self.ee11sns())
                .field("ee11fast", &self.ee11fast())
                .field("ee12src", &self.ee12src())
                .field("ee12pol", &self.ee12pol())
                .field("ee12sns", &self.ee12sns())
                .field("ee12fast", &self.ee12fast())
                .field("ee13src", &self.ee13src())
                .field("ee13pol", &self.ee13pol())
                .field("ee13sns", &self.ee13sns())
                .field("ee13fast", &self.ee13fast())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr4 {{ ee11src: {=u8:?}, ee11pol: {=bool:?}, ee11sns: {=u8:?}, ee11fast: {=bool:?}, ee12src: {=u8:?}, ee12pol: {=bool:?}, ee12sns: {=u8:?}, ee12fast: {=bool:?}, ee13src: {=u8:?}, ee13pol: {=bool:?}, ee13sns: {=u8:?}, ee13fast: {=bool:?} }}",
                self.ee11src(),
                self.ee11pol(),
                self.ee11sns(),
                self.ee11fast(),
                self.ee12src(),
                self.ee12pol(),
                self.ee12sns(),
                self.ee12fast(),
                self.ee13src(),
                self.ee13pol(),
                self.ee13sns(),
                self.ee13fast()
            )
        }
    }
    #[doc = "HRTIM timer external event control register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr5(pub u32);
    impl HrtimEecr5 {
        #[doc = "External event 14 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee14src(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "External event 14 source."]
        #[inline(always)]
        pub const fn set_ee14src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External event 14 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee14pol(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event 14 polarity."]
        #[inline(always)]
        pub const fn set_ee14pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event 14 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee14sns(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "External event 14 sensitivity."]
        #[inline(always)]
        pub const fn set_ee14sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "External event 15 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee15src(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "External event 15 source."]
        #[inline(always)]
        pub const fn set_ee15src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "External event 15 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee15pol(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event 15 polarity."]
        #[inline(always)]
        pub const fn set_ee15pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event 15 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee15sns(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "External event 15 sensitivity."]
        #[inline(always)]
        pub const fn set_ee15sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "External event 16 source."]
        #[must_use]
        #[inline(always)]
        pub const fn ee16src(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External event 16 source."]
        #[inline(always)]
        pub const fn set_ee16src(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "External event 16 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee16pol(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event 16 polarity."]
        #[inline(always)]
        pub const fn set_ee16pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event 16 sensitivity."]
        #[must_use]
        #[inline(always)]
        pub const fn ee16sns(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[doc = "External event 16 sensitivity."]
        #[inline(always)]
        pub const fn set_ee16sns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
    }
    impl Default for HrtimEecr5 {
        #[inline(always)]
        fn default() -> HrtimEecr5 {
            HrtimEecr5(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr5")
                .field("ee14src", &self.ee14src())
                .field("ee14pol", &self.ee14pol())
                .field("ee14sns", &self.ee14sns())
                .field("ee15src", &self.ee15src())
                .field("ee15pol", &self.ee15pol())
                .field("ee15sns", &self.ee15sns())
                .field("ee16src", &self.ee16src())
                .field("ee16pol", &self.ee16pol())
                .field("ee16sns", &self.ee16sns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr5 {{ ee14src: {=u8:?}, ee14pol: {=bool:?}, ee14sns: {=u8:?}, ee15src: {=u8:?}, ee15pol: {=bool:?}, ee15sns: {=u8:?}, ee16src: {=u8:?}, ee16pol: {=bool:?}, ee16sns: {=u8:?} }}",
                self.ee14src(),
                self.ee14pol(),
                self.ee14sns(),
                self.ee15src(),
                self.ee15pol(),
                self.ee15sns(),
                self.ee16src(),
                self.ee16pol(),
                self.ee16sns()
            )
        }
    }
    #[doc = "HRTIM timer external event control register 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr6(pub u32);
    impl HrtimEecr6 {
        #[doc = "External event 14 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee14f(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 14 filter."]
        #[inline(always)]
        pub const fn set_ee14f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "External event 15 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee15f(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 15 filter."]
        #[inline(always)]
        pub const fn set_ee15f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "External event 16 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ee16f(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "External event 16 filter."]
        #[inline(always)]
        pub const fn set_ee16f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for HrtimEecr6 {
        #[inline(always)]
        fn default() -> HrtimEecr6 {
            HrtimEecr6(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr6")
                .field("ee14f", &self.ee14f())
                .field("ee15f", &self.ee15f())
                .field("ee16f", &self.ee16f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr6 {{ ee14f: {=u8:?}, ee15f: {=u8:?}, ee16f: {=u8:?} }}",
                self.ee14f(),
                self.ee15f(),
                self.ee16f()
            )
        }
    }
    #[doc = "HRTIM timer external event control register 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr7(pub u32);
    impl HrtimEecr7 {
        #[doc = "External event 1 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee1src(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External event 1 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee1src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External event 2 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee2src(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event 2 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee2src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event 3 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee3src(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External event 3 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee3src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External event 4 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee4src(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External event 4 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee4src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External event 5 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee5src(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event 5 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee5src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event 6 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee6src(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External event 6 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee6src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External event 7 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee7src(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "External event 7 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee7src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External event 8 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee8src(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event 8 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee8src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event 9 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee9src(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External event 9 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee9src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External event 10 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee10src(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External event 10 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee10src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "External event 11 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee11src(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External event 11 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee11src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event 12 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee12src(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "External event 12 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee12src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event 13 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee13src(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "External event 13 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee13src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External event 14 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee14src(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event 14 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee14src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event 15 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee15src(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External event 15 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee15src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External event 16 source - bit 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ee16src(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "External event 16 source - bit 2."]
        #[inline(always)]
        pub const fn set_ee16src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for HrtimEecr7 {
        #[inline(always)]
        fn default() -> HrtimEecr7 {
            HrtimEecr7(0)
        }
    }
    impl core::fmt::Debug for HrtimEecr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimEecr7")
                .field("ee1src", &self.ee1src())
                .field("ee2src", &self.ee2src())
                .field("ee3src", &self.ee3src())
                .field("ee4src", &self.ee4src())
                .field("ee5src", &self.ee5src())
                .field("ee6src", &self.ee6src())
                .field("ee7src", &self.ee7src())
                .field("ee8src", &self.ee8src())
                .field("ee9src", &self.ee9src())
                .field("ee10src", &self.ee10src())
                .field("ee11src", &self.ee11src())
                .field("ee12src", &self.ee12src())
                .field("ee13src", &self.ee13src())
                .field("ee14src", &self.ee14src())
                .field("ee15src", &self.ee15src())
                .field("ee16src", &self.ee16src())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimEecr7 {{ ee1src: {=bool:?}, ee2src: {=bool:?}, ee3src: {=bool:?}, ee4src: {=bool:?}, ee5src: {=bool:?}, ee6src: {=bool:?}, ee7src: {=bool:?}, ee8src: {=bool:?}, ee9src: {=bool:?}, ee10src: {=bool:?}, ee11src: {=bool:?}, ee12src: {=bool:?}, ee13src: {=bool:?}, ee14src: {=bool:?}, ee15src: {=bool:?}, ee16src: {=bool:?} }}",
                self.ee1src(),
                self.ee2src(),
                self.ee3src(),
                self.ee4src(),
                self.ee5src(),
                self.ee6src(),
                self.ee7src(),
                self.ee8src(),
                self.ee9src(),
                self.ee10src(),
                self.ee11src(),
                self.ee12src(),
                self.ee13src(),
                self.ee14src(),
                self.ee15src(),
                self.ee16src()
            )
        }
    }
    #[doc = "High Resolution Timer: Fault Input Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr1(pub u32);
    impl HrtimFltinr1 {
        #[doc = "Fault 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 enable."]
        #[inline(always)]
        pub const fn set_flt1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 1 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 polarity."]
        #[inline(always)]
        pub const fn set_flt1p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 1 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1src(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 source bit 0."]
        #[inline(always)]
        pub const fn set_flt1src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1f(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 1 filter."]
        #[inline(always)]
        pub const fn set_flt1f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Fault 1 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1lck(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 lock."]
        #[inline(always)]
        pub const fn set_flt1lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 enable."]
        #[inline(always)]
        pub const fn set_flt2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fault 2 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 polarity."]
        #[inline(always)]
        pub const fn set_flt2p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fault 2 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2src(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 source bit 0."]
        #[inline(always)]
        pub const fn set_flt2src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fault 2 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2f(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 2 filter."]
        #[inline(always)]
        pub const fn set_flt2f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "Fault 2 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2lck(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 lock."]
        #[inline(always)]
        pub const fn set_flt2lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fault 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3e(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 enable."]
        #[inline(always)]
        pub const fn set_flt3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fault 3 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3p(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 polarity."]
        #[inline(always)]
        pub const fn set_flt3p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fault 3 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3src(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 source bit 0."]
        #[inline(always)]
        pub const fn set_flt3src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fault 3 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3f(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 3 filter."]
        #[inline(always)]
        pub const fn set_flt3f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "Fault 3 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3lck(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 lock."]
        #[inline(always)]
        pub const fn set_flt3lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fault 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4e(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 enable."]
        #[inline(always)]
        pub const fn set_flt4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Fault 4 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4p(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 polarity."]
        #[inline(always)]
        pub const fn set_flt4p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Fault 4 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4src(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 source bit 0."]
        #[inline(always)]
        pub const fn set_flt4src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Fault 4 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4f(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 4 filter."]
        #[inline(always)]
        pub const fn set_flt4f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "Fault 4 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4lck(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 lock."]
        #[inline(always)]
        pub const fn set_flt4lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimFltinr1 {
        #[inline(always)]
        fn default() -> HrtimFltinr1 {
            HrtimFltinr1(0)
        }
    }
    impl core::fmt::Debug for HrtimFltinr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimFltinr1")
                .field("flt1e", &self.flt1e())
                .field("flt1p", &self.flt1p())
                .field("flt1src", &self.flt1src())
                .field("flt1f", &self.flt1f())
                .field("flt1lck", &self.flt1lck())
                .field("flt2e", &self.flt2e())
                .field("flt2p", &self.flt2p())
                .field("flt2src", &self.flt2src())
                .field("flt2f", &self.flt2f())
                .field("flt2lck", &self.flt2lck())
                .field("flt3e", &self.flt3e())
                .field("flt3p", &self.flt3p())
                .field("flt3src", &self.flt3src())
                .field("flt3f", &self.flt3f())
                .field("flt3lck", &self.flt3lck())
                .field("flt4e", &self.flt4e())
                .field("flt4p", &self.flt4p())
                .field("flt4src", &self.flt4src())
                .field("flt4f", &self.flt4f())
                .field("flt4lck", &self.flt4lck())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimFltinr1 {{ flt1e: {=bool:?}, flt1p: {=bool:?}, flt1src: {=bool:?}, flt1f: {=u8:?}, flt1lck: {=bool:?}, flt2e: {=bool:?}, flt2p: {=bool:?}, flt2src: {=bool:?}, flt2f: {=u8:?}, flt2lck: {=bool:?}, flt3e: {=bool:?}, flt3p: {=bool:?}, flt3src: {=bool:?}, flt3f: {=u8:?}, flt3lck: {=bool:?}, flt4e: {=bool:?}, flt4p: {=bool:?}, flt4src: {=bool:?}, flt4f: {=u8:?}, flt4lck: {=bool:?} }}",
                self.flt1e(),
                self.flt1p(),
                self.flt1src(),
                self.flt1f(),
                self.flt1lck(),
                self.flt2e(),
                self.flt2p(),
                self.flt2src(),
                self.flt2f(),
                self.flt2lck(),
                self.flt3e(),
                self.flt3p(),
                self.flt3src(),
                self.flt3f(),
                self.flt3lck(),
                self.flt4e(),
                self.flt4p(),
                self.flt4src(),
                self.flt4f(),
                self.flt4lck()
            )
        }
    }
    #[doc = "High Resolution Timer: Fault Input Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr2(pub u32);
    impl HrtimFltinr2 {
        #[doc = "Fault 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flte(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 enable."]
        #[inline(always)]
        pub const fn set_flte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 5 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn fltp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 polarity."]
        #[inline(always)]
        pub const fn set_fltp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FLT5SRC\\[0\\]: Fault 5 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn fltsrc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FLT5SRC\\[0\\]: Fault 5 source bit 0."]
        #[inline(always)]
        pub const fn set_fltsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 5 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn fltf(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 5 filter."]
        #[inline(always)]
        pub const fn set_fltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Fault 5 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn fltlck(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 lock."]
        #[inline(always)]
        pub const fn set_fltlck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 enable."]
        #[inline(always)]
        pub const fn set_flt6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fault 6 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 polarity."]
        #[inline(always)]
        pub const fn set_flt6p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FLT6SRC\\[0\\]: Fault 6 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6src(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FLT6SRC\\[0\\]: Fault 6 source bit 0."]
        #[inline(always)]
        pub const fn set_flt6src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fault 6 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6f(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 6 filter."]
        #[inline(always)]
        pub const fn set_flt6f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "Fault 6 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6lck(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 lock."]
        #[inline(always)]
        pub const fn set_flt6lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fault 1 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1src(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 source bit 1."]
        #[inline(always)]
        pub const fn set_flt1src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fault 2 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2src(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 source bit 1."]
        #[inline(always)]
        pub const fn set_flt2src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fault 3 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3src(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 source bit 1."]
        #[inline(always)]
        pub const fn set_flt3src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fault 4 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4src(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 source bit 1."]
        #[inline(always)]
        pub const fn set_flt4src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "FLT5SRC\\[1\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5src_1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FLT5SRC\\[1\\]."]
        #[inline(always)]
        pub const fn set_flt5src_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "FLT6SRC\\[1\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6src_1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "FLT6SRC\\[1\\]."]
        #[inline(always)]
        pub const fn set_flt6src_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Fault 7 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7src(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 source bit 1."]
        #[inline(always)]
        pub const fn set_flt7src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fault 8 source bit 1."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8src(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 source bit 1."]
        #[inline(always)]
        pub const fn set_flt8src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fault sampling clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn fltsd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Fault sampling clock division."]
        #[inline(always)]
        pub const fn set_fltsd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for HrtimFltinr2 {
        #[inline(always)]
        fn default() -> HrtimFltinr2 {
            HrtimFltinr2(0)
        }
    }
    impl core::fmt::Debug for HrtimFltinr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimFltinr2")
                .field("flte", &self.flte())
                .field("fltp", &self.fltp())
                .field("fltsrc", &self.fltsrc())
                .field("fltf", &self.fltf())
                .field("fltlck", &self.fltlck())
                .field("flt6e", &self.flt6e())
                .field("flt6p", &self.flt6p())
                .field("flt6src", &self.flt6src())
                .field("flt6f", &self.flt6f())
                .field("flt6lck", &self.flt6lck())
                .field("flt1src", &self.flt1src())
                .field("flt2src", &self.flt2src())
                .field("flt3src", &self.flt3src())
                .field("flt4src", &self.flt4src())
                .field("flt5src_1", &self.flt5src_1())
                .field("flt6src_1", &self.flt6src_1())
                .field("flt7src", &self.flt7src())
                .field("flt8src", &self.flt8src())
                .field("fltsd", &self.fltsd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimFltinr2 {{ flte: {=bool:?}, fltp: {=bool:?}, fltsrc: {=bool:?}, fltf: {=u8:?}, fltlck: {=bool:?}, flt6e: {=bool:?}, flt6p: {=bool:?}, flt6src: {=bool:?}, flt6f: {=u8:?}, flt6lck: {=bool:?}, flt1src: {=bool:?}, flt2src: {=bool:?}, flt3src: {=bool:?}, flt4src: {=bool:?}, flt5src_1: {=bool:?}, flt6src_1: {=bool:?}, flt7src: {=bool:?}, flt8src: {=bool:?}, fltsd: {=u8:?} }}",
                self.flte(),
                self.fltp(),
                self.fltsrc(),
                self.fltf(),
                self.fltlck(),
                self.flt6e(),
                self.flt6p(),
                self.flt6src(),
                self.flt6f(),
                self.flt6lck(),
                self.flt1src(),
                self.flt2src(),
                self.flt3src(),
                self.flt4src(),
                self.flt5src_1(),
                self.flt6src_1(),
                self.flt7src(),
                self.flt8src(),
                self.fltsd()
            )
        }
    }
    #[doc = "HRTIM fault input register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr3(pub u32);
    impl HrtimFltinr3 {
        #[doc = "Fault 1 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1blke(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 blanking enable."]
        #[inline(always)]
        pub const fn set_flt1blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 1 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1blks(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 blanking source."]
        #[inline(always)]
        pub const fn set_flt1blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 1 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1cnt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 1 counter."]
        #[inline(always)]
        pub const fn set_flt1cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Fault 1 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1cres(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 counter reset."]
        #[inline(always)]
        pub const fn set_flt1cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 1 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1rstm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 reset mode."]
        #[inline(always)]
        pub const fn set_flt1rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 2 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2blke(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 blanking enable."]
        #[inline(always)]
        pub const fn set_flt2blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fault 2 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2blks(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 blanking source."]
        #[inline(always)]
        pub const fn set_flt2blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fault 2 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2cnt(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 2 counter."]
        #[inline(always)]
        pub const fn set_flt2cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "Fault 2 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2cres(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 counter reset."]
        #[inline(always)]
        pub const fn set_flt2cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Fault 2 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2rstm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 reset mode."]
        #[inline(always)]
        pub const fn set_flt2rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fault 3 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3blke(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 blanking enable."]
        #[inline(always)]
        pub const fn set_flt3blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fault 3 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3blks(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 blanking source."]
        #[inline(always)]
        pub const fn set_flt3blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fault 3 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3cnt(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 3 counter."]
        #[inline(always)]
        pub const fn set_flt3cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Fault 3 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3cres(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 counter reset."]
        #[inline(always)]
        pub const fn set_flt3cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fault 3 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3rstm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 reset mode."]
        #[inline(always)]
        pub const fn set_flt3rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fault 4 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4blke(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 blanking enable."]
        #[inline(always)]
        pub const fn set_flt4blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Fault 4 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4blks(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 blanking source."]
        #[inline(always)]
        pub const fn set_flt4blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Fault 4 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4cnt(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 4 counter."]
        #[inline(always)]
        pub const fn set_flt4cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
        }
        #[doc = "Fault 4 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4cres(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 counter reset."]
        #[inline(always)]
        pub const fn set_flt4cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Fault 4 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4rstm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 reset mode."]
        #[inline(always)]
        pub const fn set_flt4rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimFltinr3 {
        #[inline(always)]
        fn default() -> HrtimFltinr3 {
            HrtimFltinr3(0)
        }
    }
    impl core::fmt::Debug for HrtimFltinr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimFltinr3")
                .field("flt1blke", &self.flt1blke())
                .field("flt1blks", &self.flt1blks())
                .field("flt1cnt", &self.flt1cnt())
                .field("flt1cres", &self.flt1cres())
                .field("flt1rstm", &self.flt1rstm())
                .field("flt2blke", &self.flt2blke())
                .field("flt2blks", &self.flt2blks())
                .field("flt2cnt", &self.flt2cnt())
                .field("flt2cres", &self.flt2cres())
                .field("flt2rstm", &self.flt2rstm())
                .field("flt3blke", &self.flt3blke())
                .field("flt3blks", &self.flt3blks())
                .field("flt3cnt", &self.flt3cnt())
                .field("flt3cres", &self.flt3cres())
                .field("flt3rstm", &self.flt3rstm())
                .field("flt4blke", &self.flt4blke())
                .field("flt4blks", &self.flt4blks())
                .field("flt4cnt", &self.flt4cnt())
                .field("flt4cres", &self.flt4cres())
                .field("flt4rstm", &self.flt4rstm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimFltinr3 {{ flt1blke: {=bool:?}, flt1blks: {=bool:?}, flt1cnt: {=u8:?}, flt1cres: {=bool:?}, flt1rstm: {=bool:?}, flt2blke: {=bool:?}, flt2blks: {=bool:?}, flt2cnt: {=u8:?}, flt2cres: {=bool:?}, flt2rstm: {=bool:?}, flt3blke: {=bool:?}, flt3blks: {=bool:?}, flt3cnt: {=u8:?}, flt3cres: {=bool:?}, flt3rstm: {=bool:?}, flt4blke: {=bool:?}, flt4blks: {=bool:?}, flt4cnt: {=u8:?}, flt4cres: {=bool:?}, flt4rstm: {=bool:?} }}",
                self.flt1blke(),
                self.flt1blks(),
                self.flt1cnt(),
                self.flt1cres(),
                self.flt1rstm(),
                self.flt2blke(),
                self.flt2blks(),
                self.flt2cnt(),
                self.flt2cres(),
                self.flt2rstm(),
                self.flt3blke(),
                self.flt3blks(),
                self.flt3cnt(),
                self.flt3cres(),
                self.flt3rstm(),
                self.flt4blke(),
                self.flt4blks(),
                self.flt4cnt(),
                self.flt4cres(),
                self.flt4rstm()
            )
        }
    }
    #[doc = "HRTIM fault input register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr4(pub u32);
    impl HrtimFltinr4 {
        #[doc = "Fault 5 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5blke(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 blanking enable."]
        #[inline(always)]
        pub const fn set_flt5blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 5 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5blks(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 blanking source."]
        #[inline(always)]
        pub const fn set_flt5blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 5 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5cnt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 5 counter."]
        #[inline(always)]
        pub const fn set_flt5cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Fault 5 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5cres(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 counter reset."]
        #[inline(always)]
        pub const fn set_flt5cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 5 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5rstm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 reset mode."]
        #[inline(always)]
        pub const fn set_flt5rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 6 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6blke(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 blanking enable."]
        #[inline(always)]
        pub const fn set_flt6blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fault 6 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6blks(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 blanking source."]
        #[inline(always)]
        pub const fn set_flt6blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fault 6 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6cnt(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 6 counter."]
        #[inline(always)]
        pub const fn set_flt6cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "Fault 6 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6cres(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 counter reset."]
        #[inline(always)]
        pub const fn set_flt6cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Fault 6 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6rstm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 reset mode."]
        #[inline(always)]
        pub const fn set_flt6rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Fault 7 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7blke(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 blanking enable."]
        #[inline(always)]
        pub const fn set_flt7blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fault 7 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7blks(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 blanking source."]
        #[inline(always)]
        pub const fn set_flt7blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fault 7 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7cnt(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 7 counter."]
        #[inline(always)]
        pub const fn set_flt7cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Fault 7 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7cres(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 counter reset."]
        #[inline(always)]
        pub const fn set_flt7cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fault 7 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7rstm(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 reset mode."]
        #[inline(always)]
        pub const fn set_flt7rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fault 8 blanking enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8blke(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 blanking enable."]
        #[inline(always)]
        pub const fn set_flt8blke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Fault 8 blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8blks(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 blanking source."]
        #[inline(always)]
        pub const fn set_flt8blks(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Fault 8 counter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8cnt(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 8 counter."]
        #[inline(always)]
        pub const fn set_flt8cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
        }
        #[doc = "Fault 8 counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8cres(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 counter reset."]
        #[inline(always)]
        pub const fn set_flt8cres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Fault 8 reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8rstm(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 reset mode."]
        #[inline(always)]
        pub const fn set_flt8rstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HrtimFltinr4 {
        #[inline(always)]
        fn default() -> HrtimFltinr4 {
            HrtimFltinr4(0)
        }
    }
    impl core::fmt::Debug for HrtimFltinr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimFltinr4")
                .field("flt5blke", &self.flt5blke())
                .field("flt5blks", &self.flt5blks())
                .field("flt5cnt", &self.flt5cnt())
                .field("flt5cres", &self.flt5cres())
                .field("flt5rstm", &self.flt5rstm())
                .field("flt6blke", &self.flt6blke())
                .field("flt6blks", &self.flt6blks())
                .field("flt6cnt", &self.flt6cnt())
                .field("flt6cres", &self.flt6cres())
                .field("flt6rstm", &self.flt6rstm())
                .field("flt7blke", &self.flt7blke())
                .field("flt7blks", &self.flt7blks())
                .field("flt7cnt", &self.flt7cnt())
                .field("flt7cres", &self.flt7cres())
                .field("flt7rstm", &self.flt7rstm())
                .field("flt8blke", &self.flt8blke())
                .field("flt8blks", &self.flt8blks())
                .field("flt8cnt", &self.flt8cnt())
                .field("flt8cres", &self.flt8cres())
                .field("flt8rstm", &self.flt8rstm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimFltinr4 {{ flt5blke: {=bool:?}, flt5blks: {=bool:?}, flt5cnt: {=u8:?}, flt5cres: {=bool:?}, flt5rstm: {=bool:?}, flt6blke: {=bool:?}, flt6blks: {=bool:?}, flt6cnt: {=u8:?}, flt6cres: {=bool:?}, flt6rstm: {=bool:?}, flt7blke: {=bool:?}, flt7blks: {=bool:?}, flt7cnt: {=u8:?}, flt7cres: {=bool:?}, flt7rstm: {=bool:?}, flt8blke: {=bool:?}, flt8blks: {=bool:?}, flt8cnt: {=u8:?}, flt8cres: {=bool:?}, flt8rstm: {=bool:?} }}",
                self.flt5blke(),
                self.flt5blks(),
                self.flt5cnt(),
                self.flt5cres(),
                self.flt5rstm(),
                self.flt6blke(),
                self.flt6blks(),
                self.flt6cnt(),
                self.flt6cres(),
                self.flt6rstm(),
                self.flt7blke(),
                self.flt7blks(),
                self.flt7cnt(),
                self.flt7cres(),
                self.flt7rstm(),
                self.flt8blke(),
                self.flt8blks(),
                self.flt8cnt(),
                self.flt8cres(),
                self.flt8rstm()
            )
        }
    }
    #[doc = "HRTIM fault input register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr5(pub u32);
    impl HrtimFltinr5 {
        #[doc = "Fault 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 enable."]
        #[inline(always)]
        pub const fn set_flt7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 7 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7p(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 polarity."]
        #[inline(always)]
        pub const fn set_flt7p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 7 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7src(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 source bit 0."]
        #[inline(always)]
        pub const fn set_flt7src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 7 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7f(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 7 filter."]
        #[inline(always)]
        pub const fn set_flt7f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Fault 7 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7lck(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 lock."]
        #[inline(always)]
        pub const fn set_flt7lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8e(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 enable."]
        #[inline(always)]
        pub const fn set_flt8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fault 8 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8p(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 polarity."]
        #[inline(always)]
        pub const fn set_flt8p(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fault 8 source bit 0."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8src(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 source bit 0."]
        #[inline(always)]
        pub const fn set_flt8src(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fault 8 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8f(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "Fault 8 filter."]
        #[inline(always)]
        pub const fn set_flt8f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "Fault 8 lock."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8lck(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 lock."]
        #[inline(always)]
        pub const fn set_flt8lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for HrtimFltinr5 {
        #[inline(always)]
        fn default() -> HrtimFltinr5 {
            HrtimFltinr5(0)
        }
    }
    impl core::fmt::Debug for HrtimFltinr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimFltinr5")
                .field("flt7e", &self.flt7e())
                .field("flt7p", &self.flt7p())
                .field("flt7src", &self.flt7src())
                .field("flt7f", &self.flt7f())
                .field("flt7lck", &self.flt7lck())
                .field("flt8e", &self.flt8e())
                .field("flt8p", &self.flt8p())
                .field("flt8src", &self.flt8src())
                .field("flt8f", &self.flt8f())
                .field("flt8lck", &self.flt8lck())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimFltinr5 {{ flt7e: {=bool:?}, flt7p: {=bool:?}, flt7src: {=bool:?}, flt7f: {=u8:?}, flt7lck: {=bool:?}, flt8e: {=bool:?}, flt8p: {=bool:?}, flt8src: {=bool:?}, flt8f: {=u8:?}, flt8lck: {=bool:?} }}",
                self.flt7e(),
                self.flt7p(),
                self.flt7src(),
                self.flt7f(),
                self.flt7lck(),
                self.flt8e(),
                self.flt8p(),
                self.flt8src(),
                self.flt8f(),
                self.flt8lck()
            )
        }
    }
    #[doc = "High Resolution Timer: Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIcr(pub u32);
    impl HrtimIcr {
        #[doc = "Fault 1 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1c(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 2 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2c(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 3 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3c(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt3c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 4 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4c(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt4c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fault 5 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5c(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt5c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "System fault interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System fault interrupt flag clear."]
        #[inline(always)]
        pub const fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Fault 6 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6c(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt6c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 7 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7c(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt7c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 8 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8c(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_flt8c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DLL Ready Interrupt flag Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt flag Clear."]
        #[inline(always)]
        pub const fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst mode period flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode period flag clear."]
        #[inline(always)]
        pub const fn set_bmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for HrtimIcr {
        #[inline(always)]
        fn default() -> HrtimIcr {
            HrtimIcr(0)
        }
    }
    impl core::fmt::Debug for HrtimIcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimIcr")
                .field("flt1c", &self.flt1c())
                .field("flt2c", &self.flt2c())
                .field("flt3c", &self.flt3c())
                .field("flt4c", &self.flt4c())
                .field("flt5c", &self.flt5c())
                .field("sysflt", &self.sysflt())
                .field("flt6c", &self.flt6c())
                .field("flt7c", &self.flt7c())
                .field("flt8c", &self.flt8c())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimIcr {{ flt1c: {=bool:?}, flt2c: {=bool:?}, flt3c: {=bool:?}, flt4c: {=bool:?}, flt5c: {=bool:?}, sysflt: {=bool:?}, flt6c: {=bool:?}, flt7c: {=bool:?}, flt8c: {=bool:?}, dllrdy: {=bool:?}, bmper: {=bool:?} }}",
                self.flt1c(),
                self.flt2c(),
                self.flt3c(),
                self.flt4c(),
                self.flt5c(),
                self.sysflt(),
                self.flt6c(),
                self.flt7c(),
                self.flt8c(),
                self.dllrdy(),
                self.bmper()
            )
        }
    }
    #[doc = "High Resolution Timer: Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIer(pub u32);
    impl HrtimIer {
        #[doc = "Fault 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fault 5 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "System fault interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System fault interrupt enable."]
        #[inline(always)]
        pub const fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Fault 6 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 7 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 8 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 interrupt enable."]
        #[inline(always)]
        pub const fn set_flt8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DLL Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst mode period interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode period interrupt enable."]
        #[inline(always)]
        pub const fn set_bmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for HrtimIer {
        #[inline(always)]
        fn default() -> HrtimIer {
            HrtimIer(0)
        }
    }
    impl core::fmt::Debug for HrtimIer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimIer")
                .field("flt1ie", &self.flt1ie())
                .field("flt2ie", &self.flt2ie())
                .field("flt3ie", &self.flt3ie())
                .field("flt4ie", &self.flt4ie())
                .field("flt5ie", &self.flt5ie())
                .field("sysflt", &self.sysflt())
                .field("flt6ie", &self.flt6ie())
                .field("flt7ie", &self.flt7ie())
                .field("flt8ie", &self.flt8ie())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimIer {{ flt1ie: {=bool:?}, flt2ie: {=bool:?}, flt3ie: {=bool:?}, flt4ie: {=bool:?}, flt5ie: {=bool:?}, sysflt: {=bool:?}, flt6ie: {=bool:?}, flt7ie: {=bool:?}, flt8ie: {=bool:?}, dllrdy: {=bool:?}, bmper: {=bool:?} }}",
                self.flt1ie(),
                self.flt2ie(),
                self.flt3ie(),
                self.flt4ie(),
                self.flt5ie(),
                self.sysflt(),
                self.flt6ie(),
                self.flt7ie(),
                self.flt8ie(),
                self.dllrdy(),
                self.bmper()
            )
        }
    }
    #[doc = "High Resolution Timer: Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIsr(pub u32);
    impl HrtimIsr {
        #[doc = "Fault 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fault 5 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "System fault interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System fault interrupt flag."]
        #[inline(always)]
        pub const fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Fault 6 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 7 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 8 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 interrupt flag."]
        #[inline(always)]
        pub const fn set_flt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DLL Ready Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst mode period interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst mode period interrupt flag."]
        #[inline(always)]
        pub const fn set_bmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for HrtimIsr {
        #[inline(always)]
        fn default() -> HrtimIsr {
            HrtimIsr(0)
        }
    }
    impl core::fmt::Debug for HrtimIsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimIsr")
                .field("flt1", &self.flt1())
                .field("flt2", &self.flt2())
                .field("flt3", &self.flt3())
                .field("flt4", &self.flt4())
                .field("flt5", &self.flt5())
                .field("sysflt", &self.sysflt())
                .field("flt6", &self.flt6())
                .field("flt7", &self.flt7())
                .field("flt8", &self.flt8())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimIsr {{ flt1: {=bool:?}, flt2: {=bool:?}, flt3: {=bool:?}, flt4: {=bool:?}, flt5: {=bool:?}, sysflt: {=bool:?}, flt6: {=bool:?}, flt7: {=bool:?}, flt8: {=bool:?}, dllrdy: {=bool:?}, bmper: {=bool:?} }}",
                self.flt1(),
                self.flt2(),
                self.flt3(),
                self.flt4(),
                self.flt5(),
                self.sysflt(),
                self.flt6(),
                self.flt7(),
                self.flt8(),
                self.dllrdy(),
                self.bmper()
            )
        }
    }
    #[doc = "High Resolution Timer: Output Disable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOdisr(pub u32);
    impl HrtimOdisr {
        #[doc = "Timer A output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ta1odis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 1 disable."]
        #[inline(always)]
        pub const fn set_ta1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ta2odis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 2 disable."]
        #[inline(always)]
        pub const fn set_ta2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer B output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tb1odis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 1 disable."]
        #[inline(always)]
        pub const fn set_tb1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer B output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tb2odis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 2 disable."]
        #[inline(always)]
        pub const fn set_tb2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer C output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tc1odis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 1 disable."]
        #[inline(always)]
        pub const fn set_tc1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer C output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tc2odis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 2 disable."]
        #[inline(always)]
        pub const fn set_tc2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer D output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn td1odis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 1 disable."]
        #[inline(always)]
        pub const fn set_td1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer D output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn td2odis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 2 disable."]
        #[inline(always)]
        pub const fn set_td2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer E output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn te1odis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 1 disable."]
        #[inline(always)]
        pub const fn set_te1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer E output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn te2odis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 2 disable."]
        #[inline(always)]
        pub const fn set_te2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer F output 1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tf1odis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 1 disable."]
        #[inline(always)]
        pub const fn set_tf1odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer F output 2 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tf2odis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 2 disable."]
        #[inline(always)]
        pub const fn set_tf2odis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for HrtimOdisr {
        #[inline(always)]
        fn default() -> HrtimOdisr {
            HrtimOdisr(0)
        }
    }
    impl core::fmt::Debug for HrtimOdisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimOdisr")
                .field("ta1odis", &self.ta1odis())
                .field("ta2odis", &self.ta2odis())
                .field("tb1odis", &self.tb1odis())
                .field("tb2odis", &self.tb2odis())
                .field("tc1odis", &self.tc1odis())
                .field("tc2odis", &self.tc2odis())
                .field("td1odis", &self.td1odis())
                .field("td2odis", &self.td2odis())
                .field("te1odis", &self.te1odis())
                .field("te2odis", &self.te2odis())
                .field("tf1odis", &self.tf1odis())
                .field("tf2odis", &self.tf2odis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOdisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimOdisr {{ ta1odis: {=bool:?}, ta2odis: {=bool:?}, tb1odis: {=bool:?}, tb2odis: {=bool:?}, tc1odis: {=bool:?}, tc2odis: {=bool:?}, td1odis: {=bool:?}, td2odis: {=bool:?}, te1odis: {=bool:?}, te2odis: {=bool:?}, tf1odis: {=bool:?}, tf2odis: {=bool:?} }}",
                self.ta1odis(),
                self.ta2odis(),
                self.tb1odis(),
                self.tb2odis(),
                self.tc1odis(),
                self.tc2odis(),
                self.td1odis(),
                self.td2odis(),
                self.te1odis(),
                self.te2odis(),
                self.tf1odis(),
                self.tf2odis()
            )
        }
    }
    #[doc = "High Resolution Timer: Output Disable Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOdsr(pub u32);
    impl HrtimOdsr {
        #[doc = "Timer A output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn ta1ods(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 1 disable status."]
        #[inline(always)]
        pub const fn set_ta1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn ta2ods(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 2 disable status."]
        #[inline(always)]
        pub const fn set_ta2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer B output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tb1ods(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 1 disable status."]
        #[inline(always)]
        pub const fn set_tb1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer B output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tb2ods(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 2 disable status."]
        #[inline(always)]
        pub const fn set_tb2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer C output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tc1ods(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 1 disable status."]
        #[inline(always)]
        pub const fn set_tc1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer C output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tc2ods(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 2 disable status."]
        #[inline(always)]
        pub const fn set_tc2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer D output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn td1ods(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 1 disable status."]
        #[inline(always)]
        pub const fn set_td1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer D output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn td2ods(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 2 disable status."]
        #[inline(always)]
        pub const fn set_td2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer E output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn te1ods(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 1 disable status."]
        #[inline(always)]
        pub const fn set_te1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer E output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn te2ods(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 2 disable status."]
        #[inline(always)]
        pub const fn set_te2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer F output 1 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tf1ods(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 1 disable status."]
        #[inline(always)]
        pub const fn set_tf1ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer F output 2 disable status."]
        #[must_use]
        #[inline(always)]
        pub const fn tf2ods(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 2 disable status."]
        #[inline(always)]
        pub const fn set_tf2ods(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for HrtimOdsr {
        #[inline(always)]
        fn default() -> HrtimOdsr {
            HrtimOdsr(0)
        }
    }
    impl core::fmt::Debug for HrtimOdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimOdsr")
                .field("ta1ods", &self.ta1ods())
                .field("ta2ods", &self.ta2ods())
                .field("tb1ods", &self.tb1ods())
                .field("tb2ods", &self.tb2ods())
                .field("tc1ods", &self.tc1ods())
                .field("tc2ods", &self.tc2ods())
                .field("td1ods", &self.td1ods())
                .field("td2ods", &self.td2ods())
                .field("te1ods", &self.te1ods())
                .field("te2ods", &self.te2ods())
                .field("tf1ods", &self.tf1ods())
                .field("tf2ods", &self.tf2ods())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimOdsr {{ ta1ods: {=bool:?}, ta2ods: {=bool:?}, tb1ods: {=bool:?}, tb2ods: {=bool:?}, tc1ods: {=bool:?}, tc2ods: {=bool:?}, td1ods: {=bool:?}, td2ods: {=bool:?}, te1ods: {=bool:?}, te2ods: {=bool:?}, tf1ods: {=bool:?}, tf2ods: {=bool:?} }}",
                self.ta1ods(),
                self.ta2ods(),
                self.tb1ods(),
                self.tb2ods(),
                self.tc1ods(),
                self.tc2ods(),
                self.td1ods(),
                self.td2ods(),
                self.te1ods(),
                self.te2ods(),
                self.tf1ods(),
                self.tf2ods()
            )
        }
    }
    #[doc = "High Resolution Timer: Output Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOenr(pub u32);
    impl HrtimOenr {
        #[doc = "Timer A output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ta1oen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 1 enable."]
        #[inline(always)]
        pub const fn set_ta1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ta2oen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A output 2 enable."]
        #[inline(always)]
        pub const fn set_ta2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer B output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tb1oen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 1 enable."]
        #[inline(always)]
        pub const fn set_tb1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer B output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tb2oen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 2 enable."]
        #[inline(always)]
        pub const fn set_tb2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer C output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tc1oen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 1 enable."]
        #[inline(always)]
        pub const fn set_tc1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer C output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tc2oen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 2 enable."]
        #[inline(always)]
        pub const fn set_tc2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer D output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn td1oen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 1 enable."]
        #[inline(always)]
        pub const fn set_td1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer D output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn td2oen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 2 enable."]
        #[inline(always)]
        pub const fn set_td2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer E output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te1oen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 1 enable."]
        #[inline(always)]
        pub const fn set_te1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer E output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te2oen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 2 enable."]
        #[inline(always)]
        pub const fn set_te2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer F output 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tf1oen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 1 enable."]
        #[inline(always)]
        pub const fn set_tf1oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer F output 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tf2oen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 2 enable."]
        #[inline(always)]
        pub const fn set_tf2oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for HrtimOenr {
        #[inline(always)]
        fn default() -> HrtimOenr {
            HrtimOenr(0)
        }
    }
    impl core::fmt::Debug for HrtimOenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimOenr")
                .field("ta1oen", &self.ta1oen())
                .field("ta2oen", &self.ta2oen())
                .field("tb1oen", &self.tb1oen())
                .field("tb2oen", &self.tb2oen())
                .field("tc1oen", &self.tc1oen())
                .field("tc2oen", &self.tc2oen())
                .field("td1oen", &self.td1oen())
                .field("td2oen", &self.td2oen())
                .field("te1oen", &self.te1oen())
                .field("te2oen", &self.te2oen())
                .field("tf1oen", &self.tf1oen())
                .field("tf2oen", &self.tf2oen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimOenr {{ ta1oen: {=bool:?}, ta2oen: {=bool:?}, tb1oen: {=bool:?}, tb2oen: {=bool:?}, tc1oen: {=bool:?}, tc2oen: {=bool:?}, td1oen: {=bool:?}, td2oen: {=bool:?}, te1oen: {=bool:?}, te2oen: {=bool:?}, tf1oen: {=bool:?}, tf2oen: {=bool:?} }}",
                self.ta1oen(),
                self.ta2oen(),
                self.tb1oen(),
                self.tb2oen(),
                self.tc1oen(),
                self.tc2oen(),
                self.td1oen(),
                self.td2oen(),
                self.te1oen(),
                self.te2oen(),
                self.tf1oen(),
                self.tf2oen()
            )
        }
    }
    #[doc = "HRTIM option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOr(pub u32);
    impl HrtimOr {
        #[doc = "Update Enable Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn bmtrgsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update Enable Selection."]
        #[inline(always)]
        pub const fn set_bmtrgsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update Enable Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn updensel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update Enable Selection."]
        #[inline(always)]
        pub const fn set_updensel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronization Input Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn insyncsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Input Selection."]
        #[inline(always)]
        pub const fn set_insyncsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for HrtimOr {
        #[inline(always)]
        fn default() -> HrtimOr {
            HrtimOr(0)
        }
    }
    impl core::fmt::Debug for HrtimOr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HrtimOr")
                .field("bmtrgsel", &self.bmtrgsel())
                .field("updensel", &self.updensel())
                .field("insyncsel", &self.insyncsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HrtimOr {{ bmtrgsel: {=bool:?}, updensel: {=bool:?}, insyncsel: {=bool:?} }}",
                self.bmtrgsel(),
                self.updensel(),
                self.insyncsel()
            )
        }
    }
    #[doc = "Master Timer Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcmp(pub u32);
    impl Mcmp {
        #[doc = "Master timer compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master timer compare 1 value."]
        #[inline(always)]
        pub const fn set_mcmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mcmp {
        #[inline(always)]
        fn default() -> Mcmp {
            Mcmp(0)
        }
    }
    impl core::fmt::Debug for Mcmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcmp").field("mcmp", &self.mcmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mcmp {{ mcmp: {=u16:?} }}", self.mcmp())
        }
    }
    #[doc = "Master Timer Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcntr(pub u32);
    impl Mcntr {
        #[doc = "Counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn mcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter value."]
        #[inline(always)]
        pub const fn set_mcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mcntr {
        #[inline(always)]
        fn default() -> Mcntr {
            Mcntr(0)
        }
    }
    impl core::fmt::Debug for Mcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcntr").field("mcnt", &self.mcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mcntr {{ mcnt: {=u16:?} }}", self.mcnt())
        }
    }
    #[doc = "Master Timer Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Clock prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Clock prescaler."]
        #[inline(always)]
        pub const fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Continuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode."]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Re-triggerable mode."]
        #[must_use]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Re-triggerable mode."]
        #[inline(always)]
        pub const fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode."]
        #[must_use]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode."]
        #[inline(always)]
        pub const fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interleaved mode."]
        #[must_use]
        #[inline(always)]
        pub const fn intlvd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Interleaved mode."]
        #[inline(always)]
        pub const fn set_intlvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Synchronization input."]
        #[must_use]
        #[inline(always)]
        pub const fn syncin(&self) -> super::vals::Syncin {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Syncin::from_bits(val as u8)
        }
        #[doc = "Synchronization input."]
        #[inline(always)]
        pub const fn set_syncin(&mut self, val: super::vals::Syncin) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Synchronization resets master."]
        #[must_use]
        #[inline(always)]
        pub const fn syncrstm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization resets master."]
        #[inline(always)]
        pub const fn set_syncrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization starts master."]
        #[must_use]
        #[inline(always)]
        pub const fn syncstrtm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization starts master."]
        #[inline(always)]
        pub const fn set_syncstrtm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Synchronization output."]
        #[must_use]
        #[inline(always)]
        pub const fn syncout(&self) -> super::vals::Syncout {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Syncout::from_bits(val as u8)
        }
        #[doc = "Synchronization output."]
        #[inline(always)]
        pub const fn set_syncout(&mut self, val: super::vals::Syncout) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Synchronization source."]
        #[must_use]
        #[inline(always)]
        pub const fn syncsrc(&self) -> super::vals::Syncsrc {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Syncsrc::from_bits(val as u8)
        }
        #[doc = "Synchronization source."]
        #[inline(always)]
        pub const fn set_syncsrc(&mut self, val: super::vals::Syncsrc) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Master timer counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer counter enable."]
        #[inline(always)]
        pub const fn set_mcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer A counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tacen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A counter enable."]
        #[inline(always)]
        pub const fn set_tacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer B counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbcen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B counter enable."]
        #[inline(always)]
        pub const fn set_tbcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer C counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tccen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C counter enable."]
        #[inline(always)]
        pub const fn set_tccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer D counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D counter enable."]
        #[inline(always)]
        pub const fn set_tdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer E counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tecen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E counter enable."]
        #[inline(always)]
        pub const fn set_tecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer F counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfcen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F counter enable."]
        #[inline(always)]
        pub const fn set_tfcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DAC synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "DAC synchronization."]
        #[inline(always)]
        pub const fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable."]
        #[inline(always)]
        pub const fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Master timer repetition update."]
        #[must_use]
        #[inline(always)]
        pub const fn mrepu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer repetition update."]
        #[inline(always)]
        pub const fn set_mrepu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Burst DMA update."]
        #[must_use]
        #[inline(always)]
        pub const fn brstdma(&self) -> super::vals::Brstdma {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Brstdma::from_bits(val as u8)
        }
        #[doc = "Burst DMA update."]
        #[inline(always)]
        pub const fn set_brstdma(&mut self, val: super::vals::Brstdma) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    impl core::fmt::Debug for Mcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr")
                .field("ckpsc", &self.ckpsc())
                .field("cont", &self.cont())
                .field("retrig", &self.retrig())
                .field("half", &self.half())
                .field("intlvd", &self.intlvd())
                .field("syncin", &self.syncin())
                .field("syncrstm", &self.syncrstm())
                .field("syncstrtm", &self.syncstrtm())
                .field("syncout", &self.syncout())
                .field("syncsrc", &self.syncsrc())
                .field("mcen", &self.mcen())
                .field("tacen", &self.tacen())
                .field("tbcen", &self.tbcen())
                .field("tccen", &self.tccen())
                .field("tdcen", &self.tdcen())
                .field("tecen", &self.tecen())
                .field("tfcen", &self.tfcen())
                .field("dacsync", &self.dacsync())
                .field("preen", &self.preen())
                .field("mrepu", &self.mrepu())
                .field("brstdma", &self.brstdma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr {{ ckpsc: {=u8:?}, cont: {=bool:?}, retrig: {=bool:?}, half: {=bool:?}, intlvd: {=u8:?}, syncin: {:?}, syncrstm: {=bool:?}, syncstrtm: {=bool:?}, syncout: {:?}, syncsrc: {:?}, mcen: {=bool:?}, tacen: {=bool:?}, tbcen: {=bool:?}, tccen: {=bool:?}, tdcen: {=bool:?}, tecen: {=bool:?}, tfcen: {=bool:?}, dacsync: {:?}, preen: {=bool:?}, mrepu: {=bool:?}, brstdma: {:?} }}",
                self.ckpsc(),
                self.cont(),
                self.retrig(),
                self.half(),
                self.intlvd(),
                self.syncin(),
                self.syncrstm(),
                self.syncstrtm(),
                self.syncout(),
                self.syncsrc(),
                self.mcen(),
                self.tacen(),
                self.tbcen(),
                self.tccen(),
                self.tdcen(),
                self.tecen(),
                self.tfcen(),
                self.dacsync(),
                self.preen(),
                self.mrepu(),
                self.brstdma()
            )
        }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdier(pub u32);
    impl Mdier {
        #[doc = "Master compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp1ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_mcmp1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master compare 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp2ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_mcmp2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master compare 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp3ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_mcmp3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master compare 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp4ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_mcmp4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Master repetition interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mrepie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master repetition interrupt enable."]
        #[inline(always)]
        pub const fn set_mrepie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync input interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync input interrupt enable."]
        #[inline(always)]
        pub const fn set_syncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master update interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mupdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master update interrupt enable."]
        #[inline(always)]
        pub const fn set_mupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp1de(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_mcmp1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Master compare 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp2de(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_mcmp2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Master compare 3 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp3de(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3 DMA request enable."]
        #[inline(always)]
        pub const fn set_mcmp3de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Master compare 4 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp4de(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4 DMA request enable."]
        #[inline(always)]
        pub const fn set_mcmp4de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Master repetition DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mrepde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Master repetition DMA request enable."]
        #[inline(always)]
        pub const fn set_mrepde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Sync input DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncde(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Sync input DMA request enable."]
        #[inline(always)]
        pub const fn set_syncde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Master update DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mupdde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Master update DMA request enable."]
        #[inline(always)]
        pub const fn set_mupdde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Mdier {
        #[inline(always)]
        fn default() -> Mdier {
            Mdier(0)
        }
    }
    impl core::fmt::Debug for Mdier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mdier")
                .field("mcmp1ie", &self.mcmp1ie())
                .field("mcmp2ie", &self.mcmp2ie())
                .field("mcmp3ie", &self.mcmp3ie())
                .field("mcmp4ie", &self.mcmp4ie())
                .field("mrepie", &self.mrepie())
                .field("syncie", &self.syncie())
                .field("mupdie", &self.mupdie())
                .field("mcmp1de", &self.mcmp1de())
                .field("mcmp2de", &self.mcmp2de())
                .field("mcmp3de", &self.mcmp3de())
                .field("mcmp4de", &self.mcmp4de())
                .field("mrepde", &self.mrepde())
                .field("syncde", &self.syncde())
                .field("mupdde", &self.mupdde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mdier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mdier {{ mcmp1ie: {=bool:?}, mcmp2ie: {=bool:?}, mcmp3ie: {=bool:?}, mcmp4ie: {=bool:?}, mrepie: {=bool:?}, syncie: {=bool:?}, mupdie: {=bool:?}, mcmp1de: {=bool:?}, mcmp2de: {=bool:?}, mcmp3de: {=bool:?}, mcmp4de: {=bool:?}, mrepde: {=bool:?}, syncde: {=bool:?}, mupdde: {=bool:?} }}",
                self.mcmp1ie(),
                self.mcmp2ie(),
                self.mcmp3ie(),
                self.mcmp4ie(),
                self.mrepie(),
                self.syncie(),
                self.mupdie(),
                self.mcmp1de(),
                self.mcmp2de(),
                self.mcmp3de(),
                self.mcmp4de(),
                self.mrepde(),
                self.syncde(),
                self.mupdde()
            )
        }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Micr(pub u32);
    impl Micr {
        #[doc = "Master compare 1 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp1c(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mcmp1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master compare 2 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp2c(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mcmp2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master compare 3 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp3c(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mcmp3c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master compare 4 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp4c(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mcmp4c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mrepc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mrepc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync input interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn syncc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync input interrupt flag clear."]
        #[inline(always)]
        pub const fn set_syncc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master update interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn mupdc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master update interrupt flag clear."]
        #[inline(always)]
        pub const fn set_mupdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Micr {
        #[inline(always)]
        fn default() -> Micr {
            Micr(0)
        }
    }
    impl core::fmt::Debug for Micr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Micr")
                .field("mcmp1c", &self.mcmp1c())
                .field("mcmp2c", &self.mcmp2c())
                .field("mcmp3c", &self.mcmp3c())
                .field("mcmp4c", &self.mcmp4c())
                .field("mrepc", &self.mrepc())
                .field("syncc", &self.syncc())
                .field("mupdc", &self.mupdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Micr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Micr {{ mcmp1c: {=bool:?}, mcmp2c: {=bool:?}, mcmp3c: {=bool:?}, mcmp4c: {=bool:?}, mrepc: {=bool:?}, syncc: {=bool:?}, mupdc: {=bool:?} }}",
                self.mcmp1c(),
                self.mcmp2c(),
                self.mcmp3c(),
                self.mcmp4c(),
                self.mrepc(),
                self.syncc(),
                self.mupdc()
            )
        }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Master compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_mcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master compare 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_mcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master compare 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_mcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master compare 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mcmp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_mcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Master repetition interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mrep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master repetition interrupt flag."]
        #[inline(always)]
        pub const fn set_mrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync input interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sync(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync input interrupt flag."]
        #[inline(always)]
        pub const fn set_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master update interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mupd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master update interrupt flag."]
        #[inline(always)]
        pub const fn set_mupd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("mcmp1", &self.mcmp1())
                .field("mcmp2", &self.mcmp2())
                .field("mcmp3", &self.mcmp3())
                .field("mcmp4", &self.mcmp4())
                .field("mrep", &self.mrep())
                .field("sync", &self.sync())
                .field("mupd", &self.mupd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misr {{ mcmp1: {=bool:?}, mcmp2: {=bool:?}, mcmp3: {=bool:?}, mcmp4: {=bool:?}, mrep: {=bool:?}, sync: {=bool:?}, mupd: {=bool:?} }}",
                self.mcmp1(),
                self.mcmp2(),
                self.mcmp3(),
                self.mcmp4(),
                self.mrep(),
                self.sync(),
                self.mupd()
            )
        }
    }
    #[doc = "Master Timer Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mper(pub u32);
    impl Mper {
        #[doc = "Master timer period value."]
        #[must_use]
        #[inline(always)]
        pub const fn mper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master timer period value."]
        #[inline(always)]
        pub const fn set_mper(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mper {
        #[inline(always)]
        fn default() -> Mper {
            Mper(0)
        }
    }
    impl core::fmt::Debug for Mper {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mper").field("mper", &self.mper()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mper {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mper {{ mper: {=u16:?} }}", self.mper())
        }
    }
    #[doc = "Master Timer Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrep(pub u32);
    impl Mrep {
        #[doc = "Master timer repetition period value."]
        #[must_use]
        #[inline(always)]
        pub const fn mrep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Master timer repetition period value."]
        #[inline(always)]
        pub const fn set_mrep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Mrep {
        #[inline(always)]
        fn default() -> Mrep {
            Mrep(0)
        }
    }
    impl core::fmt::Debug for Mrep {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrep").field("mrep", &self.mrep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mrep {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mrep {{ mrep: {=u8:?} }}", self.mrep())
        }
    }
    #[doc = "Timerx Capture 2 Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxccr(pub u32);
    impl Timxccr {
        #[doc = "Software capture."]
        #[must_use]
        #[inline(always)]
        pub const fn swcpt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software capture."]
        #[inline(always)]
        pub const fn set_swcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update capture."]
        #[must_use]
        #[inline(always)]
        pub const fn updcpt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update capture."]
        #[inline(always)]
        pub const fn set_updcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External event A capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevacpt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event A capture."]
        #[inline(always)]
        pub const fn set_exevacpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event B capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevbcpt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "External event B capture."]
        #[inline(always)]
        pub const fn set_exevbcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "External event C capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevccpt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External event C capture."]
        #[inline(always)]
        pub const fn set_exevccpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External event D capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevdcpt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External event D capture."]
        #[inline(always)]
        pub const fn set_exevdcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External event E capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevecpt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External event E capture."]
        #[inline(always)]
        pub const fn set_exevecpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External event F capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevfcpt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "External event F capture."]
        #[inline(always)]
        pub const fn set_exevfcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External event G capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevgcpt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event G capture."]
        #[inline(always)]
        pub const fn set_exevgcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event H capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevhcpt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External event H capture."]
        #[inline(always)]
        pub const fn set_exevhcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External event I capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevicpt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External event I capture."]
        #[inline(always)]
        pub const fn set_exevicpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External event J capture."]
        #[must_use]
        #[inline(always)]
        pub const fn exevjcpt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External event J capture."]
        #[inline(always)]
        pub const fn set_exevjcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timer F output 1 set."]
        #[must_use]
        #[inline(always)]
        pub const fn twset(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 1 set."]
        #[inline(always)]
        pub const fn set_twset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer F output 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn twrst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F output 1 reset."]
        #[inline(always)]
        pub const fn set_twrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer F compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn twcmp1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F compare 1."]
        #[inline(always)]
        pub const fn set_twcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timer E compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn twcmp2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 2."]
        #[inline(always)]
        pub const fn set_twcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer B output 1 set."]
        #[must_use]
        #[inline(always)]
        pub const fn txset(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 1 set."]
        #[inline(always)]
        pub const fn set_txset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer B output 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn txrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B output 1 reset."]
        #[inline(always)]
        pub const fn set_txrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer B compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn txcmp1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 1."]
        #[inline(always)]
        pub const fn set_txcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer B compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn txcmp2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 2."]
        #[inline(always)]
        pub const fn set_txcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer C output 1 set."]
        #[must_use]
        #[inline(always)]
        pub const fn tyset(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 1 set."]
        #[inline(always)]
        pub const fn set_tyset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer C output 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tyrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C output 1 reset."]
        #[inline(always)]
        pub const fn set_tyrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer C compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tccmp1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 1."]
        #[inline(always)]
        pub const fn set_tccmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Timer C compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn tccmp2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 2."]
        #[inline(always)]
        pub const fn set_tccmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Timer D output 1 set."]
        #[must_use]
        #[inline(always)]
        pub const fn tzset(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 1 set."]
        #[inline(always)]
        pub const fn set_tzset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer D output 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tzrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D output 1 reset."]
        #[inline(always)]
        pub const fn set_tzrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Timer D compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcmp1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 1."]
        #[inline(always)]
        pub const fn set_tdcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Timer D compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcmp2(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 2."]
        #[inline(always)]
        pub const fn set_tdcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Timer E output 1 set."]
        #[must_use]
        #[inline(always)]
        pub const fn ttset(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 1 set."]
        #[inline(always)]
        pub const fn set_ttset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Timer E output 1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ttrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E output 1 reset."]
        #[inline(always)]
        pub const fn set_ttrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Timer E compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tecmp1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 1."]
        #[inline(always)]
        pub const fn set_tecmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Timer E compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn tecmp2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 2."]
        #[inline(always)]
        pub const fn set_tecmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxccr {
        #[inline(always)]
        fn default() -> Timxccr {
            Timxccr(0)
        }
    }
    impl core::fmt::Debug for Timxccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxccr")
                .field("swcpt", &self.swcpt())
                .field("updcpt", &self.updcpt())
                .field("exevacpt", &self.exevacpt())
                .field("exevbcpt", &self.exevbcpt())
                .field("exevccpt", &self.exevccpt())
                .field("exevdcpt", &self.exevdcpt())
                .field("exevecpt", &self.exevecpt())
                .field("exevfcpt", &self.exevfcpt())
                .field("exevgcpt", &self.exevgcpt())
                .field("exevhcpt", &self.exevhcpt())
                .field("exevicpt", &self.exevicpt())
                .field("exevjcpt", &self.exevjcpt())
                .field("twset", &self.twset())
                .field("twrst", &self.twrst())
                .field("twcmp1", &self.twcmp1())
                .field("twcmp2", &self.twcmp2())
                .field("txset", &self.txset())
                .field("txrst", &self.txrst())
                .field("txcmp1", &self.txcmp1())
                .field("txcmp2", &self.txcmp2())
                .field("tyset", &self.tyset())
                .field("tyrst", &self.tyrst())
                .field("tccmp1", &self.tccmp1())
                .field("tccmp2", &self.tccmp2())
                .field("tzset", &self.tzset())
                .field("tzrst", &self.tzrst())
                .field("tdcmp1", &self.tdcmp1())
                .field("tdcmp2", &self.tdcmp2())
                .field("ttset", &self.ttset())
                .field("ttrst", &self.ttrst())
                .field("tecmp1", &self.tecmp1())
                .field("tecmp2", &self.tecmp2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxccr {{ swcpt: {=bool:?}, updcpt: {=bool:?}, exevacpt: {=bool:?}, exevbcpt: {=bool:?}, exevccpt: {=bool:?}, exevdcpt: {=bool:?}, exevecpt: {=bool:?}, exevfcpt: {=bool:?}, exevgcpt: {=bool:?}, exevhcpt: {=bool:?}, exevicpt: {=bool:?}, exevjcpt: {=bool:?}, twset: {=bool:?}, twrst: {=bool:?}, twcmp1: {=bool:?}, twcmp2: {=bool:?}, txset: {=bool:?}, txrst: {=bool:?}, txcmp1: {=bool:?}, txcmp2: {=bool:?}, tyset: {=bool:?}, tyrst: {=bool:?}, tccmp1: {=bool:?}, tccmp2: {=bool:?}, tzset: {=bool:?}, tzrst: {=bool:?}, tdcmp1: {=bool:?}, tdcmp2: {=bool:?}, ttset: {=bool:?}, ttrst: {=bool:?}, tecmp1: {=bool:?}, tecmp2: {=bool:?} }}",
                self.swcpt(),
                self.updcpt(),
                self.exevacpt(),
                self.exevbcpt(),
                self.exevccpt(),
                self.exevdcpt(),
                self.exevecpt(),
                self.exevfcpt(),
                self.exevgcpt(),
                self.exevhcpt(),
                self.exevicpt(),
                self.exevjcpt(),
                self.twset(),
                self.twrst(),
                self.twcmp1(),
                self.twcmp2(),
                self.txset(),
                self.txrst(),
                self.txcmp1(),
                self.txcmp2(),
                self.tyset(),
                self.tyrst(),
                self.tccmp1(),
                self.tccmp2(),
                self.tzset(),
                self.tzrst(),
                self.tdcmp1(),
                self.tdcmp2(),
                self.ttset(),
                self.ttrst(),
                self.tecmp1(),
                self.tecmp2()
            )
        }
    }
    #[doc = "Timerx Chopper Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxchp(pub u32);
    impl Timxchp {
        #[doc = "Timer x carrier frequency value."]
        #[must_use]
        #[inline(always)]
        pub const fn carfrq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Timer x carrier frequency value."]
        #[inline(always)]
        pub const fn set_carfrq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Timer x chopper duty cycle value."]
        #[must_use]
        #[inline(always)]
        pub const fn cardty(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Timer x chopper duty cycle value."]
        #[inline(always)]
        pub const fn set_cardty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Timer x start pulsewidth."]
        #[must_use]
        #[inline(always)]
        pub const fn strtpw(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "Timer x start pulsewidth."]
        #[inline(always)]
        pub const fn set_strtpw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for Timxchp {
        #[inline(always)]
        fn default() -> Timxchp {
            Timxchp(0)
        }
    }
    impl core::fmt::Debug for Timxchp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxchp")
                .field("carfrq", &self.carfrq())
                .field("cardty", &self.cardty())
                .field("strtpw", &self.strtpw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxchp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxchp {{ carfrq: {=u8:?}, cardty: {=u8:?}, strtpw: {=u8:?} }}",
                self.carfrq(),
                self.cardty(),
                self.strtpw()
            )
        }
    }
    #[doc = "Timerx Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmp(pub u32);
    impl Timxcmp {
        #[doc = "Timer x compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer x compare 1 value."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxcmp {
        #[inline(always)]
        fn default() -> Timxcmp {
            Timxcmp(0)
        }
    }
    impl core::fmt::Debug for Timxcmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcmp").field("cmp", &self.cmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxcmp {{ cmp: {=u16:?} }}", self.cmp())
        }
    }
    #[doc = "Timerx Compare X Compound Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmpc(pub u32);
    impl Timxcmpc {
        #[doc = "Timer x compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer x compare 1 value."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Timer x repetition value aliased from HRTIM_REPx register."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Timer x repetition value aliased from HRTIM_REPx register."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Timxcmpc {
        #[inline(always)]
        fn default() -> Timxcmpc {
            Timxcmpc(0)
        }
    }
    impl core::fmt::Debug for Timxcmpc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcmpc")
                .field("cmp", &self.cmp())
                .field("rep", &self.rep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcmpc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxcmpc {{ cmp: {=u16:?}, rep: {=u8:?} }}", self.cmp(), self.rep())
        }
    }
    #[doc = "Timerx Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcnt(pub u32);
    impl Timxcnt {
        #[doc = "Timer x counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer x counter value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxcnt {
        #[inline(always)]
        fn default() -> Timxcnt {
            Timxcnt(0)
        }
    }
    impl core::fmt::Debug for Timxcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxcnt {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
    #[doc = "HRTIM timer A crossbar override delayed fault event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcode(pub u32);
    impl Timxcode {
        #[doc = "Delayed 1 Fault duration."]
        #[must_use]
        #[inline(always)]
        pub const fn dly1fltd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Delayed 1 Fault duration."]
        #[inline(always)]
        pub const fn set_dly1fltd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Delayed 2 Fault duration."]
        #[must_use]
        #[inline(always)]
        pub const fn dly2fltd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Delayed 2 Fault duration."]
        #[inline(always)]
        pub const fn set_dly2fltd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Timxcode {
        #[inline(always)]
        fn default() -> Timxcode {
            Timxcode(0)
        }
    }
    impl core::fmt::Debug for Timxcode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcode")
                .field("dly1fltd", &self.dly1fltd())
                .field("dly2fltd", &self.dly2fltd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxcode {{ dly1fltd: {=u16:?}, dly2fltd: {=u16:?} }}",
                self.dly1fltd(),
                self.dly2fltd()
            )
        }
    }
    #[doc = "HRTIM timer A crossbar override forcing event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcofe(pub u32);
    impl Timxcofe {
        #[doc = "Crossbar override force mode output A."]
        #[must_use]
        #[inline(always)]
        pub const fn cofma(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Crossbar override force mode output A."]
        #[inline(always)]
        pub const fn set_cofma(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "External event A forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevafrca(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External event A forces output A."]
        #[inline(always)]
        pub const fn set_eevafrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External event B forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevbfrca(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External event B forces output A."]
        #[inline(always)]
        pub const fn set_eevbfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External event C forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevcfrca(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External event C forces output A."]
        #[inline(always)]
        pub const fn set_eevcfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External event D forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevdfrca(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "External event D forces output A."]
        #[inline(always)]
        pub const fn set_eevdfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External event E forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevefrca(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External event E forces output A."]
        #[inline(always)]
        pub const fn set_eevefrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event F forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevffrca(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External event F forces output A."]
        #[inline(always)]
        pub const fn set_eevffrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External event G forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevgfrca(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External event G forces output A."]
        #[inline(always)]
        pub const fn set_eevgfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External event H forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevhfrca(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External event H forces output A."]
        #[inline(always)]
        pub const fn set_eevhfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event I forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevifrca(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "External event I forces output A."]
        #[inline(always)]
        pub const fn set_eevifrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External event J forces output A."]
        #[must_use]
        #[inline(always)]
        pub const fn eevjfrca(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "External event J forces output A."]
        #[inline(always)]
        pub const fn set_eevjfrca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Crossbar override force mode output B."]
        #[must_use]
        #[inline(always)]
        pub const fn cofmb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Crossbar override force mode output B."]
        #[inline(always)]
        pub const fn set_cofmb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "External event A forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevafrcb(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "External event A forces output B."]
        #[inline(always)]
        pub const fn set_eevafrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event B forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevbfrcb(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "External event B forces output B."]
        #[inline(always)]
        pub const fn set_eevbfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "External event C forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevcfrcb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "External event C forces output B."]
        #[inline(always)]
        pub const fn set_eevcfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event D forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevdfrcb(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "External event D forces output B."]
        #[inline(always)]
        pub const fn set_eevdfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "External event E forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevefrcb(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "External event E forces output B."]
        #[inline(always)]
        pub const fn set_eevefrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External event F forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevffrcb(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "External event F forces output B."]
        #[inline(always)]
        pub const fn set_eevffrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "External event G forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevgfrcb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event G forces output B."]
        #[inline(always)]
        pub const fn set_eevgfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event H forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevhfrcb(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "External event H forces output B."]
        #[inline(always)]
        pub const fn set_eevhfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "External event I forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevifrcb(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External event I forces output B."]
        #[inline(always)]
        pub const fn set_eevifrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External event J forces output B."]
        #[must_use]
        #[inline(always)]
        pub const fn eevjfrcb(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External event J forces output B."]
        #[inline(always)]
        pub const fn set_eevjfrcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Timxcofe {
        #[inline(always)]
        fn default() -> Timxcofe {
            Timxcofe(0)
        }
    }
    impl core::fmt::Debug for Timxcofe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcofe")
                .field("cofma", &self.cofma())
                .field("eevafrca", &self.eevafrca())
                .field("eevbfrca", &self.eevbfrca())
                .field("eevcfrca", &self.eevcfrca())
                .field("eevdfrca", &self.eevdfrca())
                .field("eevefrca", &self.eevefrca())
                .field("eevffrca", &self.eevffrca())
                .field("eevgfrca", &self.eevgfrca())
                .field("eevhfrca", &self.eevhfrca())
                .field("eevifrca", &self.eevifrca())
                .field("eevjfrca", &self.eevjfrca())
                .field("cofmb", &self.cofmb())
                .field("eevafrcb", &self.eevafrcb())
                .field("eevbfrcb", &self.eevbfrcb())
                .field("eevcfrcb", &self.eevcfrcb())
                .field("eevdfrcb", &self.eevdfrcb())
                .field("eevefrcb", &self.eevefrcb())
                .field("eevffrcb", &self.eevffrcb())
                .field("eevgfrcb", &self.eevgfrcb())
                .field("eevhfrcb", &self.eevhfrcb())
                .field("eevifrcb", &self.eevifrcb())
                .field("eevjfrcb", &self.eevjfrcb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcofe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxcofe {{ cofma: {=u8:?}, eevafrca: {=bool:?}, eevbfrca: {=bool:?}, eevcfrca: {=bool:?}, eevdfrca: {=bool:?}, eevefrca: {=bool:?}, eevffrca: {=bool:?}, eevgfrca: {=bool:?}, eevhfrca: {=bool:?}, eevifrca: {=bool:?}, eevjfrca: {=bool:?}, cofmb: {=u8:?}, eevafrcb: {=bool:?}, eevbfrcb: {=bool:?}, eevcfrcb: {=bool:?}, eevdfrcb: {=bool:?}, eevefrcb: {=bool:?}, eevffrcb: {=bool:?}, eevgfrcb: {=bool:?}, eevhfrcb: {=bool:?}, eevifrcb: {=bool:?}, eevjfrcb: {=bool:?} }}",
                self.cofma(),
                self.eevafrca(),
                self.eevbfrca(),
                self.eevcfrca(),
                self.eevdfrca(),
                self.eevefrca(),
                self.eevffrca(),
                self.eevgfrca(),
                self.eevhfrca(),
                self.eevifrca(),
                self.eevjfrca(),
                self.cofmb(),
                self.eevafrcb(),
                self.eevbfrcb(),
                self.eevcfrcb(),
                self.eevdfrcb(),
                self.eevefrcb(),
                self.eevffrcb(),
                self.eevgfrcb(),
                self.eevhfrcb(),
                self.eevifrcb(),
                self.eevjfrcb()
            )
        }
    }
    #[doc = "HRTIM timer A crossbar override trigger register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcotrg(pub u32);
    impl Timxcotrg {
        #[doc = "Software trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn swt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software trigger."]
        #[inline(always)]
        pub const fn set_swt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLT1 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLT1 trigger."]
        #[inline(always)]
        pub const fn set_flt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FLT2 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FLT2 trigger."]
        #[inline(always)]
        pub const fn set_flt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FLT3 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FLT3 trigger."]
        #[inline(always)]
        pub const fn set_flt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLT4 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FLT4 trigger."]
        #[inline(always)]
        pub const fn set_flt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FLT5 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FLT5 trigger."]
        #[inline(always)]
        pub const fn set_flt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLT6 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FLT6 trigger."]
        #[inline(always)]
        pub const fn set_flt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FLT7 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FLT7 trigger."]
        #[inline(always)]
        pub const fn set_flt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FLT8 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLT8 trigger."]
        #[inline(always)]
        pub const fn set_flt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FLT1A2 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1a2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FLT1A2 trigger."]
        #[inline(always)]
        pub const fn set_flt1a2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FLT2A3 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2a3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FLT2A3 trigger."]
        #[inline(always)]
        pub const fn set_flt2a3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "FLT2A3 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3a4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "FLT2A3 trigger."]
        #[inline(always)]
        pub const fn set_flt3a4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "FLT4A5 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4a5(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FLT4A5 trigger."]
        #[inline(always)]
        pub const fn set_flt4a5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "FLT5A6 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5a6(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "FLT5A6 trigger."]
        #[inline(always)]
        pub const fn set_flt5a6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "FLT6A7 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6a7(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "FLT6A7 trigger."]
        #[inline(always)]
        pub const fn set_flt6a7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FLT7A8 trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7a8(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FLT7A8 trigger."]
        #[inline(always)]
        pub const fn set_flt7a8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "EEVA trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eeva(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EEVA trigger."]
        #[inline(always)]
        pub const fn set_eeva(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EEVB trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EEVB trigger."]
        #[inline(always)]
        pub const fn set_eevb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EEVC trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EEVC trigger."]
        #[inline(always)]
        pub const fn set_eevc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "EEVD trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "EEVD trigger."]
        #[inline(always)]
        pub const fn set_eevd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EEVE trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eeve(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "EEVE trigger."]
        #[inline(always)]
        pub const fn set_eeve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "EEVF trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EEVF trigger."]
        #[inline(always)]
        pub const fn set_eevf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EEVG trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevg(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EEVG trigger."]
        #[inline(always)]
        pub const fn set_eevg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EEVH trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevh(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EEVH trigger."]
        #[inline(always)]
        pub const fn set_eevh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EEVI trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevi(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EEVI trigger."]
        #[inline(always)]
        pub const fn set_eevi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EEVJ trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn eevj(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EEVJ trigger."]
        #[inline(always)]
        pub const fn set_eevj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "External trigger 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 0."]
        #[inline(always)]
        pub const fn set_ext_trg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External trigger 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 1."]
        #[inline(always)]
        pub const fn set_ext_trg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "External trigger 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 2."]
        #[inline(always)]
        pub const fn set_ext_trg2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External trigger 3."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 3."]
        #[inline(always)]
        pub const fn set_ext_trg3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "External trigger 4."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg4(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 4."]
        #[inline(always)]
        pub const fn set_ext_trg4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "External trigger 5."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_trg5(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger 5."]
        #[inline(always)]
        pub const fn set_ext_trg5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxcotrg {
        #[inline(always)]
        fn default() -> Timxcotrg {
            Timxcotrg(0)
        }
    }
    impl core::fmt::Debug for Timxcotrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcotrg")
                .field("swt", &self.swt())
                .field("flt1", &self.flt1())
                .field("flt2", &self.flt2())
                .field("flt3", &self.flt3())
                .field("flt4", &self.flt4())
                .field("flt5", &self.flt5())
                .field("flt6", &self.flt6())
                .field("flt7", &self.flt7())
                .field("flt8", &self.flt8())
                .field("flt1a2", &self.flt1a2())
                .field("flt2a3", &self.flt2a3())
                .field("flt3a4", &self.flt3a4())
                .field("flt4a5", &self.flt4a5())
                .field("flt5a6", &self.flt5a6())
                .field("flt6a7", &self.flt6a7())
                .field("flt7a8", &self.flt7a8())
                .field("eeva", &self.eeva())
                .field("eevb", &self.eevb())
                .field("eevc", &self.eevc())
                .field("eevd", &self.eevd())
                .field("eeve", &self.eeve())
                .field("eevf", &self.eevf())
                .field("eevg", &self.eevg())
                .field("eevh", &self.eevh())
                .field("eevi", &self.eevi())
                .field("eevj", &self.eevj())
                .field("ext_trg0", &self.ext_trg0())
                .field("ext_trg1", &self.ext_trg1())
                .field("ext_trg2", &self.ext_trg2())
                .field("ext_trg3", &self.ext_trg3())
                .field("ext_trg4", &self.ext_trg4())
                .field("ext_trg5", &self.ext_trg5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcotrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxcotrg {{ swt: {=bool:?}, flt1: {=bool:?}, flt2: {=bool:?}, flt3: {=bool:?}, flt4: {=bool:?}, flt5: {=bool:?}, flt6: {=bool:?}, flt7: {=bool:?}, flt8: {=bool:?}, flt1a2: {=bool:?}, flt2a3: {=bool:?}, flt3a4: {=bool:?}, flt4a5: {=bool:?}, flt5a6: {=bool:?}, flt6a7: {=bool:?}, flt7a8: {=bool:?}, eeva: {=bool:?}, eevb: {=bool:?}, eevc: {=bool:?}, eevd: {=bool:?}, eeve: {=bool:?}, eevf: {=bool:?}, eevg: {=bool:?}, eevh: {=bool:?}, eevi: {=bool:?}, eevj: {=bool:?}, ext_trg0: {=bool:?}, ext_trg1: {=bool:?}, ext_trg2: {=bool:?}, ext_trg3: {=bool:?}, ext_trg4: {=bool:?}, ext_trg5: {=bool:?} }}",
                self.swt(),
                self.flt1(),
                self.flt2(),
                self.flt3(),
                self.flt4(),
                self.flt5(),
                self.flt6(),
                self.flt7(),
                self.flt8(),
                self.flt1a2(),
                self.flt2a3(),
                self.flt3a4(),
                self.flt4a5(),
                self.flt5a6(),
                self.flt6a7(),
                self.flt7a8(),
                self.eeva(),
                self.eevb(),
                self.eevc(),
                self.eevd(),
                self.eeve(),
                self.eevf(),
                self.eevg(),
                self.eevh(),
                self.eevi(),
                self.eevj(),
                self.ext_trg0(),
                self.ext_trg1(),
                self.ext_trg2(),
                self.ext_trg3(),
                self.ext_trg4(),
                self.ext_trg5()
            )
        }
    }
    #[doc = "Timerx Capture X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcpt(pub u32);
    impl Timxcpt {
        #[doc = "Timer x capture 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer x capture 1 value."]
        #[inline(always)]
        pub const fn set_cpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Timer x capture 1 direction status."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x capture 1 direction status."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Timxcpt {
        #[inline(always)]
        fn default() -> Timxcpt {
            Timxcpt(0)
        }
    }
    impl core::fmt::Debug for Timxcpt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcpt")
                .field("cpt", &self.cpt())
                .field("dir", &self.dir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcpt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxcpt {{ cpt: {=u16:?}, dir: {=bool:?} }}", self.cpt(), self.dir())
        }
    }
    #[doc = "Timerx Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcr(pub u32);
    impl Timxcr {
        #[doc = "HRTIM timer x clock prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HRTIM timer x clock prescaler."]
        #[inline(always)]
        pub const fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Continuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode."]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Re-triggerable mode."]
        #[must_use]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Re-triggerable mode."]
        #[inline(always)]
        pub const fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode enable."]
        #[inline(always)]
        pub const fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Push-pull mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pshpll(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Push-pull mode enable."]
        #[inline(always)]
        pub const fn set_pshpll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interleaved mode."]
        #[must_use]
        #[inline(always)]
        pub const fn intlvd(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Interleaved mode."]
        #[inline(always)]
        pub const fn set_intlvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Re-synchronized update."]
        #[must_use]
        #[inline(always)]
        pub const fn rsyncu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Re-synchronized update."]
        #[inline(always)]
        pub const fn set_rsyncu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Synchronization resets timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn syncrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization resets timer x."]
        #[inline(always)]
        pub const fn set_syncrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization starts timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn syncstrt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization starts timer x."]
        #[inline(always)]
        pub const fn set_syncstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CMP2 auto-delayed mode."]
        #[must_use]
        #[inline(always)]
        pub const fn delcmp2(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "CMP2 auto-delayed mode."]
        #[inline(always)]
        pub const fn set_delcmp2(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "CMP4 auto-delayed mode."]
        #[must_use]
        #[inline(always)]
        pub const fn delcmp4(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "CMP4 auto-delayed mode."]
        #[inline(always)]
        pub const fn set_delcmp4(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Timer F update."]
        #[must_use]
        #[inline(always)]
        pub const fn tfu(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F update."]
        #[inline(always)]
        pub const fn set_tfu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer x repetition update."]
        #[must_use]
        #[inline(always)]
        pub const fn repu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x repetition update."]
        #[inline(always)]
        pub const fn set_repu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer x reset update."]
        #[must_use]
        #[inline(always)]
        pub const fn rstu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x reset update."]
        #[inline(always)]
        pub const fn set_rstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer A update."]
        #[must_use]
        #[inline(always)]
        pub const fn tau(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A update."]
        #[inline(always)]
        pub const fn set_tau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer B update."]
        #[must_use]
        #[inline(always)]
        pub const fn tbu(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B update."]
        #[inline(always)]
        pub const fn set_tbu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer C update."]
        #[must_use]
        #[inline(always)]
        pub const fn tcu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C update."]
        #[inline(always)]
        pub const fn set_tcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer D update."]
        #[must_use]
        #[inline(always)]
        pub const fn tdu(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D update."]
        #[inline(always)]
        pub const fn set_tdu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Timer E update."]
        #[must_use]
        #[inline(always)]
        pub const fn teu(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E update."]
        #[inline(always)]
        pub const fn set_teu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Master timer update."]
        #[must_use]
        #[inline(always)]
        pub const fn mstu(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer update."]
        #[inline(always)]
        pub const fn set_mstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DAC synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "DAC synchronization."]
        #[inline(always)]
        pub const fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable."]
        #[must_use]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable."]
        #[inline(always)]
        pub const fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Update gating."]
        #[must_use]
        #[inline(always)]
        pub const fn updgat(&self) -> super::vals::Updgat {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Updgat::from_bits(val as u8)
        }
        #[doc = "Update gating."]
        #[inline(always)]
        pub const fn set_updgat(&mut self, val: super::vals::Updgat) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Timxcr {
        #[inline(always)]
        fn default() -> Timxcr {
            Timxcr(0)
        }
    }
    impl core::fmt::Debug for Timxcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcr")
                .field("ckpsc", &self.ckpsc())
                .field("cont", &self.cont())
                .field("retrig", &self.retrig())
                .field("half", &self.half())
                .field("pshpll", &self.pshpll())
                .field("intlvd", &self.intlvd())
                .field("rsyncu", &self.rsyncu())
                .field("syncrst", &self.syncrst())
                .field("syncstrt", &self.syncstrt())
                .field("delcmp2", &self.delcmp2())
                .field("delcmp4", &self.delcmp4())
                .field("tfu", &self.tfu())
                .field("repu", &self.repu())
                .field("rstu", &self.rstu())
                .field("tau", &self.tau())
                .field("tbu", &self.tbu())
                .field("tcu", &self.tcu())
                .field("tdu", &self.tdu())
                .field("teu", &self.teu())
                .field("mstu", &self.mstu())
                .field("dacsync", &self.dacsync())
                .field("preen", &self.preen())
                .field("updgat", &self.updgat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxcr {{ ckpsc: {=u8:?}, cont: {=bool:?}, retrig: {=bool:?}, half: {=bool:?}, pshpll: {=bool:?}, intlvd: {=u8:?}, rsyncu: {=bool:?}, syncrst: {=bool:?}, syncstrt: {=bool:?}, delcmp2: {:?}, delcmp4: {:?}, tfu: {=bool:?}, repu: {=bool:?}, rstu: {=bool:?}, tau: {=bool:?}, tbu: {=bool:?}, tcu: {=bool:?}, tdu: {=bool:?}, teu: {=bool:?}, mstu: {=bool:?}, dacsync: {:?}, preen: {=bool:?}, updgat: {:?} }}",
                self.ckpsc(),
                self.cont(),
                self.retrig(),
                self.half(),
                self.pshpll(),
                self.intlvd(),
                self.rsyncu(),
                self.syncrst(),
                self.syncstrt(),
                self.delcmp2(),
                self.delcmp4(),
                self.tfu(),
                self.repu(),
                self.rstu(),
                self.tau(),
                self.tbu(),
                self.tcu(),
                self.tdu(),
                self.teu(),
                self.mstu(),
                self.dacsync(),
                self.preen(),
                self.updgat()
            )
        }
    }
    #[doc = "HRTIM Timer X Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcr2(pub u32);
    impl Timxcr2 {
        #[doc = "Dual channel DAC trigger enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Dual channel DAC trigger enable."]
        #[inline(always)]
        pub const fn set_dcde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Dual channel DAC Step trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn dcds(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Dual channel DAC Step trigger."]
        #[inline(always)]
        pub const fn set_dcds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Dual channel DAC reset trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Dual channel DAC reset trigger."]
        #[inline(always)]
        pub const fn set_dcdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Up-Down mode."]
        #[must_use]
        #[inline(always)]
        pub const fn udm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Up-Down mode."]
        #[inline(always)]
        pub const fn set_udm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Roll-over mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rom(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Roll-over mode."]
        #[inline(always)]
        pub const fn set_rom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Output roll-over mode."]
        #[must_use]
        #[inline(always)]
        pub const fn outrom(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Output roll-over mode."]
        #[inline(always)]
        pub const fn set_outrom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "ADC roll-over mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adrom(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "ADC roll-over mode."]
        #[inline(always)]
        pub const fn set_adrom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Burst mode roll-over mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bmrom(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Burst mode roll-over mode."]
        #[inline(always)]
        pub const fn set_bmrom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Fault and event roll-over mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ferom(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Fault and event roll-over mode."]
        #[inline(always)]
        pub const fn set_ferom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Greater than compare 1 PWM mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gtcmp1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Greater than compare 1 PWM mode."]
        #[inline(always)]
        pub const fn set_gtcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Greater than compare 3 PWM mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gtcmp3(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Greater than compare 3 PWM mode."]
        #[inline(always)]
        pub const fn set_gtcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Triggered-half mode."]
        #[must_use]
        #[inline(always)]
        pub const fn trghlf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Triggered-half mode."]
        #[inline(always)]
        pub const fn set_trghlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Triggered-half timeout mode."]
        #[must_use]
        #[inline(always)]
        pub const fn trghlft(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Triggered-half timeout mode."]
        #[inline(always)]
        pub const fn set_trghlft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC trigger for channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn adctrgch1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger for channel 1."]
        #[inline(always)]
        pub const fn set_adctrgch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADC trigger for channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn adctrgch2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger for channel 2."]
        #[inline(always)]
        pub const fn set_adctrgch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Crossbar override enable."]
        #[must_use]
        #[inline(always)]
        pub const fn coe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar override enable."]
        #[inline(always)]
        pub const fn set_coe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Crossbar override rearm."]
        #[must_use]
        #[inline(always)]
        pub const fn cor(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar override rearm."]
        #[inline(always)]
        pub const fn set_cor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Crossbar override mode."]
        #[must_use]
        #[inline(always)]
        pub const fn com(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar override mode."]
        #[inline(always)]
        pub const fn set_com(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Dual channel DAC crossbar override enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdcoe(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Dual channel DAC crossbar override enable."]
        #[inline(always)]
        pub const fn set_dcdcoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Timxcr2 {
        #[inline(always)]
        fn default() -> Timxcr2 {
            Timxcr2(0)
        }
    }
    impl core::fmt::Debug for Timxcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxcr2")
                .field("dcde", &self.dcde())
                .field("dcds", &self.dcds())
                .field("dcdr", &self.dcdr())
                .field("udm", &self.udm())
                .field("rom", &self.rom())
                .field("outrom", &self.outrom())
                .field("adrom", &self.adrom())
                .field("bmrom", &self.bmrom())
                .field("ferom", &self.ferom())
                .field("gtcmp1", &self.gtcmp1())
                .field("gtcmp3", &self.gtcmp3())
                .field("trghlf", &self.trghlf())
                .field("trghlft", &self.trghlft())
                .field("adctrgch1", &self.adctrgch1())
                .field("adctrgch2", &self.adctrgch2())
                .field("coe", &self.coe())
                .field("cor", &self.cor())
                .field("com", &self.com())
                .field("dcdcoe", &self.dcdcoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxcr2 {{ dcde: {=bool:?}, dcds: {=bool:?}, dcdr: {=bool:?}, udm: {=bool:?}, rom: {=u8:?}, outrom: {=u8:?}, adrom: {=u8:?}, bmrom: {=u8:?}, ferom: {=u8:?}, gtcmp1: {=bool:?}, gtcmp3: {=bool:?}, trghlf: {=bool:?}, trghlft: {=bool:?}, adctrgch1: {=bool:?}, adctrgch2: {=bool:?}, coe: {=bool:?}, cor: {=bool:?}, com: {=bool:?}, dcdcoe: {=bool:?} }}",
                self.dcde(),
                self.dcds(),
                self.dcdr(),
                self.udm(),
                self.rom(),
                self.outrom(),
                self.adrom(),
                self.bmrom(),
                self.ferom(),
                self.gtcmp1(),
                self.gtcmp3(),
                self.trghlf(),
                self.trghlft(),
                self.adctrgch1(),
                self.adctrgch2(),
                self.coe(),
                self.cor(),
                self.com(),
                self.dcdcoe()
            )
        }
    }
    #[doc = "Timerx DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdier(pub u32);
    impl Timxdier {
        #[doc = "Compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_cmp1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_cmp2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compare 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_cmp3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_cmp4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn repie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition interrupt enable."]
        #[inline(always)]
        pub const fn set_repie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Crossbar Override Trigger interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cotie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Trigger interrupt enable."]
        #[inline(always)]
        pub const fn set_cotie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn updie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable."]
        #[inline(always)]
        pub const fn set_updie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt1ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Capture interrupt enable."]
        #[inline(always)]
        pub const fn set_cpt1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt2ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture interrupt enable."]
        #[inline(always)]
        pub const fn set_cpt2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Output 1 set interrupt enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set1ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 set interrupt enable timer x."]
        #[inline(always)]
        pub const fn set_set1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Output 1 reset interrupt enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst1ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 reset interrupt enable timer x."]
        #[inline(always)]
        pub const fn set_rst1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Output 2 set interrupt enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set2ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 set interrupt enable timer x."]
        #[inline(always)]
        pub const fn set_set2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output 2 reset interrupt enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst2ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 reset interrupt enable timer x."]
        #[inline(always)]
        pub const fn set_rst2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset/roll-over interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over interrupt enable."]
        #[inline(always)]
        pub const fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed protection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprtie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed protection interrupt enable."]
        #[inline(always)]
        pub const fn set_dlyprtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Crossbar override rearm delay elapsed interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cordeie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar override rearm delay elapsed interrupt enable."]
        #[inline(always)]
        pub const fn set_cordeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1de(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_cmp1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Compare 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2de(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_cmp2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Compare 3 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3de(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 3 DMA request enable."]
        #[inline(always)]
        pub const fn set_cmp3de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Compare 4 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4de(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 4 DMA request enable."]
        #[inline(always)]
        pub const fn set_cmp4de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Repetition DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn repde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition DMA request enable."]
        #[inline(always)]
        pub const fn set_repde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Crossbar Override Trigger DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cotde(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Trigger DMA request enable."]
        #[inline(always)]
        pub const fn set_cotde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Update DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn updde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable."]
        #[inline(always)]
        pub const fn set_updde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Capture 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt1de(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_cpt1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Capture 2 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt2de(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 2 DMA request enable."]
        #[inline(always)]
        pub const fn set_cpt2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Output 1 set DMA request enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set1de(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 set DMA request enable timer x."]
        #[inline(always)]
        pub const fn set_set1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Output 1 reset DMA request enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst1de(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 reset DMA request enable timer x."]
        #[inline(always)]
        pub const fn set_rst1de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Output 2 set DMA request enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set2de(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 set DMA request enable timer x."]
        #[inline(always)]
        pub const fn set_set2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Output 2 reset DMA request enable timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst2de(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 reset DMA request enable timer x."]
        #[inline(always)]
        pub const fn set_rst2de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset/roll-over DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rstde(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over DMA request enable."]
        #[inline(always)]
        pub const fn set_rstde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Delayed protection DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprtde(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed protection DMA request enable."]
        #[inline(always)]
        pub const fn set_dlyprtde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Crossbar override rearm delay elapsed DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cordede(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar override rearm delay elapsed DMA request enable."]
        #[inline(always)]
        pub const fn set_cordede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxdier {
        #[inline(always)]
        fn default() -> Timxdier {
            Timxdier(0)
        }
    }
    impl core::fmt::Debug for Timxdier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxdier")
                .field("cmp1ie", &self.cmp1ie())
                .field("cmp2ie", &self.cmp2ie())
                .field("cmp3ie", &self.cmp3ie())
                .field("cmp4ie", &self.cmp4ie())
                .field("repie", &self.repie())
                .field("cotie", &self.cotie())
                .field("updie", &self.updie())
                .field("cpt1ie", &self.cpt1ie())
                .field("cpt2ie", &self.cpt2ie())
                .field("set1ie", &self.set1ie())
                .field("rst1ie", &self.rst1ie())
                .field("set2ie", &self.set2ie())
                .field("rst2ie", &self.rst2ie())
                .field("rstie", &self.rstie())
                .field("dlyprtie", &self.dlyprtie())
                .field("cordeie", &self.cordeie())
                .field("cmp1de", &self.cmp1de())
                .field("cmp2de", &self.cmp2de())
                .field("cmp3de", &self.cmp3de())
                .field("cmp4de", &self.cmp4de())
                .field("repde", &self.repde())
                .field("cotde", &self.cotde())
                .field("updde", &self.updde())
                .field("cpt1de", &self.cpt1de())
                .field("cpt2de", &self.cpt2de())
                .field("set1de", &self.set1de())
                .field("rst1de", &self.rst1de())
                .field("set2de", &self.set2de())
                .field("rst2de", &self.rst2de())
                .field("rstde", &self.rstde())
                .field("dlyprtde", &self.dlyprtde())
                .field("cordede", &self.cordede())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxdier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxdier {{ cmp1ie: {=bool:?}, cmp2ie: {=bool:?}, cmp3ie: {=bool:?}, cmp4ie: {=bool:?}, repie: {=bool:?}, cotie: {=bool:?}, updie: {=bool:?}, cpt1ie: {=bool:?}, cpt2ie: {=bool:?}, set1ie: {=bool:?}, rst1ie: {=bool:?}, set2ie: {=bool:?}, rst2ie: {=bool:?}, rstie: {=bool:?}, dlyprtie: {=bool:?}, cordeie: {=bool:?}, cmp1de: {=bool:?}, cmp2de: {=bool:?}, cmp3de: {=bool:?}, cmp4de: {=bool:?}, repde: {=bool:?}, cotde: {=bool:?}, updde: {=bool:?}, cpt1de: {=bool:?}, cpt2de: {=bool:?}, set1de: {=bool:?}, rst1de: {=bool:?}, set2de: {=bool:?}, rst2de: {=bool:?}, rstde: {=bool:?}, dlyprtde: {=bool:?}, cordede: {=bool:?} }}",
                self.cmp1ie(),
                self.cmp2ie(),
                self.cmp3ie(),
                self.cmp4ie(),
                self.repie(),
                self.cotie(),
                self.updie(),
                self.cpt1ie(),
                self.cpt2ie(),
                self.set1ie(),
                self.rst1ie(),
                self.set2ie(),
                self.rst2ie(),
                self.rstie(),
                self.dlyprtie(),
                self.cordeie(),
                self.cmp1de(),
                self.cmp2de(),
                self.cmp3de(),
                self.cmp4de(),
                self.repde(),
                self.cotde(),
                self.updde(),
                self.cpt1de(),
                self.cpt2de(),
                self.set1de(),
                self.rst1de(),
                self.set2de(),
                self.rst2de(),
                self.rstde(),
                self.dlyprtde(),
                self.cordede()
            )
        }
    }
    #[doc = "Timerx Deadtime Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdt(pub u32);
    impl Timxdt {
        #[doc = "Deadtime rising value timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime rising value timer x."]
        #[inline(always)]
        pub const fn set_dtr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Sign deadtime rising value timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn sdtr(&self) -> super::vals::Sdt {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Sdt::from_bits(val as u8)
        }
        #[doc = "Sign deadtime rising value timer x."]
        #[inline(always)]
        pub const fn set_sdtr(&mut self, val: super::vals::Sdt) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Deadtime prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn dtprsc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Deadtime prescaler."]
        #[inline(always)]
        pub const fn set_dtprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Deadtime rising sign lock timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtrslk(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime rising sign lock timer x."]
        #[inline(always)]
        pub const fn set_dtrslk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Deadtime rising lock timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtrlk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime rising lock timer x."]
        #[inline(always)]
        pub const fn set_dtrlk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Deadtime falling value timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtf(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime falling value timer x."]
        #[inline(always)]
        pub const fn set_dtf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "Sign deadtime falling value timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn sdtf(&self) -> super::vals::Sdt {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Sdt::from_bits(val as u8)
        }
        #[doc = "Sign deadtime falling value timer x."]
        #[inline(always)]
        pub const fn set_sdtf(&mut self, val: super::vals::Sdt) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Deadtime mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtm(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Deadtime mode."]
        #[inline(always)]
        pub const fn set_dtm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Deadtime falling sign lock timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtfslk(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime falling sign lock timer x."]
        #[inline(always)]
        pub const fn set_dtfslk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Deadtime falling lock timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn dtflk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime falling lock timer x."]
        #[inline(always)]
        pub const fn set_dtflk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxdt {
        #[inline(always)]
        fn default() -> Timxdt {
            Timxdt(0)
        }
    }
    impl core::fmt::Debug for Timxdt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxdt")
                .field("dtr", &self.dtr())
                .field("sdtr", &self.sdtr())
                .field("dtprsc", &self.dtprsc())
                .field("dtrslk", &self.dtrslk())
                .field("dtrlk", &self.dtrlk())
                .field("dtf", &self.dtf())
                .field("sdtf", &self.sdtf())
                .field("dtm", &self.dtm())
                .field("dtfslk", &self.dtfslk())
                .field("dtflk", &self.dtflk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxdt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxdt {{ dtr: {=u16:?}, sdtr: {:?}, dtprsc: {=u8:?}, dtrslk: {=bool:?}, dtrlk: {=bool:?}, dtf: {=u16:?}, sdtf: {:?}, dtm: {=u8:?}, dtfslk: {=bool:?}, dtflk: {=bool:?} }}",
                self.dtr(),
                self.sdtr(),
                self.dtprsc(),
                self.dtrslk(),
                self.dtrlk(),
                self.dtf(),
                self.sdtf(),
                self.dtm(),
                self.dtfslk(),
                self.dtflk()
            )
        }
    }
    #[doc = "Timer X External Event Filtering Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxeef(pub u32);
    impl Timxeef {
        #[doc = "External event A latch."]
        #[must_use]
        #[inline(always)]
        pub const fn eealtch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External event A latch."]
        #[inline(always)]
        pub const fn set_eealtch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External event A filter."]
        #[must_use]
        #[inline(always)]
        pub const fn eeafltr(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "External event A filter."]
        #[inline(always)]
        pub const fn set_eeafltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "External event B latch."]
        #[must_use]
        #[inline(always)]
        pub const fn eebltch(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External event B latch."]
        #[inline(always)]
        pub const fn set_eebltch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External event B filter."]
        #[must_use]
        #[inline(always)]
        pub const fn eebfltr(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "External event B filter."]
        #[inline(always)]
        pub const fn set_eebfltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "External event C latch."]
        #[must_use]
        #[inline(always)]
        pub const fn eecltch(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "External event C latch."]
        #[inline(always)]
        pub const fn set_eecltch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External event C filter."]
        #[must_use]
        #[inline(always)]
        pub const fn eecfltr(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x1f;
            val as u8
        }
        #[doc = "External event C filter."]
        #[inline(always)]
        pub const fn set_eecfltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
        }
        #[doc = "External event D latch."]
        #[must_use]
        #[inline(always)]
        pub const fn eedltch(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External event D latch."]
        #[inline(always)]
        pub const fn set_eedltch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "External event D filter."]
        #[must_use]
        #[inline(always)]
        pub const fn eedfltr(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "External event D filter."]
        #[inline(always)]
        pub const fn set_eedfltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "External event E latch."]
        #[must_use]
        #[inline(always)]
        pub const fn eeeltch(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "External event E latch."]
        #[inline(always)]
        pub const fn set_eeeltch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External event E filter."]
        #[must_use]
        #[inline(always)]
        pub const fn eeefltr(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "External event E filter."]
        #[inline(always)]
        pub const fn set_eeefltr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for Timxeef {
        #[inline(always)]
        fn default() -> Timxeef {
            Timxeef(0)
        }
    }
    impl core::fmt::Debug for Timxeef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxeef")
                .field("eealtch", &self.eealtch())
                .field("eeafltr", &self.eeafltr())
                .field("eebltch", &self.eebltch())
                .field("eebfltr", &self.eebfltr())
                .field("eecltch", &self.eecltch())
                .field("eecfltr", &self.eecfltr())
                .field("eedltch", &self.eedltch())
                .field("eedfltr", &self.eedfltr())
                .field("eeeltch", &self.eeeltch())
                .field("eeefltr", &self.eeefltr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxeef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxeef {{ eealtch: {=bool:?}, eeafltr: {=u8:?}, eebltch: {=bool:?}, eebfltr: {=u8:?}, eecltch: {=bool:?}, eecfltr: {=u8:?}, eedltch: {=bool:?}, eedfltr: {=u8:?}, eeeltch: {=bool:?}, eeefltr: {=u8:?} }}",
                self.eealtch(),
                self.eeafltr(),
                self.eebltch(),
                self.eebfltr(),
                self.eecltch(),
                self.eecfltr(),
                self.eedltch(),
                self.eedfltr(),
                self.eeeltch(),
                self.eeefltr()
            )
        }
    }
    #[doc = "HRTIM timer A external event filtering register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxeefr3(pub u32);
    impl Timxeefr3 {
        #[doc = "External event Y counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eevyce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External event Y counter enable."]
        #[inline(always)]
        pub const fn set_eevyce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External event Y counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn eevycres(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External event Y counter reset."]
        #[inline(always)]
        pub const fn set_eevycres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External event Y reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn eevyrstm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External event Y reset mode."]
        #[inline(always)]
        pub const fn set_eevyrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External event Y Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevysel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "External event Y Selection."]
        #[inline(always)]
        pub const fn set_eevysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "External event Y counter."]
        #[must_use]
        #[inline(always)]
        pub const fn eevycnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "External event Y counter."]
        #[inline(always)]
        pub const fn set_eevycnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "External event Z counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eevzce(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External event Z counter enable."]
        #[inline(always)]
        pub const fn set_eevzce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External event Z counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn eevzcres(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External event Z counter reset."]
        #[inline(always)]
        pub const fn set_eevzcres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External event Z reset mode."]
        #[must_use]
        #[inline(always)]
        pub const fn eevzrstm(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External event Z reset mode."]
        #[inline(always)]
        pub const fn set_eevzrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "External event Z selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevzsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "External event Z selection."]
        #[inline(always)]
        pub const fn set_eevzsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "External event Z counter."]
        #[must_use]
        #[inline(always)]
        pub const fn eevzcnt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "External event Z counter."]
        #[inline(always)]
        pub const fn set_eevzcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Timxeefr3 {
        #[inline(always)]
        fn default() -> Timxeefr3 {
            Timxeefr3(0)
        }
    }
    impl core::fmt::Debug for Timxeefr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxeefr3")
                .field("eevyce", &self.eevyce())
                .field("eevycres", &self.eevycres())
                .field("eevyrstm", &self.eevyrstm())
                .field("eevysel", &self.eevysel())
                .field("eevycnt", &self.eevycnt())
                .field("eevzce", &self.eevzce())
                .field("eevzcres", &self.eevzcres())
                .field("eevzrstm", &self.eevzrstm())
                .field("eevzsel", &self.eevzsel())
                .field("eevzcnt", &self.eevzcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxeefr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxeefr3 {{ eevyce: {=bool:?}, eevycres: {=bool:?}, eevyrstm: {=bool:?}, eevysel: {=u8:?}, eevycnt: {=u8:?}, eevzce: {=bool:?}, eevzcres: {=bool:?}, eevzrstm: {=bool:?}, eevzsel: {=u8:?}, eevzcnt: {=u8:?} }}",
                self.eevyce(),
                self.eevycres(),
                self.eevyrstm(),
                self.eevysel(),
                self.eevycnt(),
                self.eevzce(),
                self.eevzcres(),
                self.eevzrstm(),
                self.eevzsel(),
                self.eevzcnt()
            )
        }
    }
    #[doc = "HRTIM timer A external event selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxeesel(pub u32);
    impl Timxeesel {
        #[doc = "External event A selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevasel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "External event A selection."]
        #[inline(always)]
        pub const fn set_eevasel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External event B selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevbsel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "External event B selection."]
        #[inline(always)]
        pub const fn set_eevbsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "External event C selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevcsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "External event C selection."]
        #[inline(always)]
        pub const fn set_eevcsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "External event D selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevdsel(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "External event D selection."]
        #[inline(always)]
        pub const fn set_eevdsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "External event E selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevesel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "External event E selection."]
        #[inline(always)]
        pub const fn set_eevesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "External event F selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevfsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "External event F selection."]
        #[inline(always)]
        pub const fn set_eevfsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "External event G selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevgsel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[doc = "External event G selection."]
        #[inline(always)]
        pub const fn set_eevgsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
        #[doc = "External event H selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevhsel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "External event H selection."]
        #[inline(always)]
        pub const fn set_eevhsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "External event I selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevisel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "External event I selection."]
        #[inline(always)]
        pub const fn set_eevisel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "External event J selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eevjsel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "External event J selection."]
        #[inline(always)]
        pub const fn set_eevjsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Timxeesel {
        #[inline(always)]
        fn default() -> Timxeesel {
            Timxeesel(0)
        }
    }
    impl core::fmt::Debug for Timxeesel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxeesel")
                .field("eevasel", &self.eevasel())
                .field("eevbsel", &self.eevbsel())
                .field("eevcsel", &self.eevcsel())
                .field("eevdsel", &self.eevdsel())
                .field("eevesel", &self.eevesel())
                .field("eevfsel", &self.eevfsel())
                .field("eevgsel", &self.eevgsel())
                .field("eevhsel", &self.eevhsel())
                .field("eevisel", &self.eevisel())
                .field("eevjsel", &self.eevjsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxeesel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxeesel {{ eevasel: {=u8:?}, eevbsel: {=u8:?}, eevcsel: {=u8:?}, eevdsel: {=u8:?}, eevesel: {=u8:?}, eevfsel: {=u8:?}, eevgsel: {=u8:?}, eevhsel: {=u8:?}, eevisel: {=u8:?}, eevjsel: {=u8:?} }}",
                self.eevasel(),
                self.eevbsel(),
                self.eevcsel(),
                self.eevdsel(),
                self.eevesel(),
                self.eevfsel(),
                self.eevgsel(),
                self.eevhsel(),
                self.eevisel(),
                self.eevjsel()
            )
        }
    }
    #[doc = "Timerx Fault Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxflt(pub u32);
    impl Timxflt {
        #[doc = "Fault 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 enable."]
        #[inline(always)]
        pub const fn set_flt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fault 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 enable."]
        #[inline(always)]
        pub const fn set_flt2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Fault 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 enable."]
        #[inline(always)]
        pub const fn set_flt3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Fault 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 enable."]
        #[inline(always)]
        pub const fn set_flt4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fault 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 enable."]
        #[inline(always)]
        pub const fn set_flt5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Fault 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 enable."]
        #[inline(always)]
        pub const fn set_flt6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Fault 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 enable."]
        #[inline(always)]
        pub const fn set_flt7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fault 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt8en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 8 enable."]
        #[inline(always)]
        pub const fn set_flt8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fault 1 AND Fault 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt1a2en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 1 AND Fault 2 enable."]
        #[inline(always)]
        pub const fn set_flt1a2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fault 2 AND Fault 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt2a3en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 2 AND Fault 3 enable."]
        #[inline(always)]
        pub const fn set_flt2a3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fault 3 AND Fault 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt3a4en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 3 AND Fault 4 enable."]
        #[inline(always)]
        pub const fn set_flt3a4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fault 4 AND Fault 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt4a5en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 4 AND Fault 5 enable."]
        #[inline(always)]
        pub const fn set_flt4a5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Fault 5 AND Fault 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt5a6en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 5 AND Fault 6 enable."]
        #[inline(always)]
        pub const fn set_flt5a6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Fault 6 AND Fault 7 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt6a7en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 6 AND Fault 7 enable."]
        #[inline(always)]
        pub const fn set_flt6a7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Fault 7 AND Fault 8 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flt7a8en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fault 7 AND Fault 8 enable."]
        #[inline(always)]
        pub const fn set_flt7a8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fault sources lock."]
        #[must_use]
        #[inline(always)]
        pub const fn fltlck(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Fault sources lock."]
        #[inline(always)]
        pub const fn set_fltlck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxflt {
        #[inline(always)]
        fn default() -> Timxflt {
            Timxflt(0)
        }
    }
    impl core::fmt::Debug for Timxflt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxflt")
                .field("flt1en", &self.flt1en())
                .field("flt2en", &self.flt2en())
                .field("flt3en", &self.flt3en())
                .field("flt4en", &self.flt4en())
                .field("flt5en", &self.flt5en())
                .field("flt6en", &self.flt6en())
                .field("flt7en", &self.flt7en())
                .field("flt8en", &self.flt8en())
                .field("flt1a2en", &self.flt1a2en())
                .field("flt2a3en", &self.flt2a3en())
                .field("flt3a4en", &self.flt3a4en())
                .field("flt4a5en", &self.flt4a5en())
                .field("flt5a6en", &self.flt5a6en())
                .field("flt6a7en", &self.flt6a7en())
                .field("flt7a8en", &self.flt7a8en())
                .field("fltlck", &self.fltlck())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxflt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxflt {{ flt1en: {=bool:?}, flt2en: {=bool:?}, flt3en: {=bool:?}, flt4en: {=bool:?}, flt5en: {=bool:?}, flt6en: {=bool:?}, flt7en: {=bool:?}, flt8en: {=bool:?}, flt1a2en: {=bool:?}, flt2a3en: {=bool:?}, flt3a4en: {=bool:?}, flt4a5en: {=bool:?}, flt5a6en: {=bool:?}, flt6a7en: {=bool:?}, flt7a8en: {=bool:?}, fltlck: {=bool:?} }}",
                self.flt1en(),
                self.flt2en(),
                self.flt3en(),
                self.flt4en(),
                self.flt5en(),
                self.flt6en(),
                self.flt7en(),
                self.flt8en(),
                self.flt1a2en(),
                self.flt2a3en(),
                self.flt3a4en(),
                self.flt4a5en(),
                self.flt5a6en(),
                self.flt6a7en(),
                self.flt7a8en(),
                self.fltlck()
            )
        }
    }
    #[doc = "Timerx Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxicr(pub u32);
    impl Timxicr {
        #[doc = "Compare 1 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1c(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cmp1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 2 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2c(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 2 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cmp2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compare 3 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3c(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 3 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cmp3c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare 4 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4c(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 4 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cmp4c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn repc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition interrupt flag clear."]
        #[inline(always)]
        pub const fn set_repc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Crossbar Override Trigger flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cotc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Trigger flag clear."]
        #[inline(always)]
        pub const fn set_cotc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn updc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag clear."]
        #[inline(always)]
        pub const fn set_updc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture 1 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt1c(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 1 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cpt1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture 2 interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt2c(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 2 interrupt flag clear."]
        #[inline(always)]
        pub const fn set_cpt2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Output 1 set flag clear timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set1c(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 set flag clear timer x."]
        #[inline(always)]
        pub const fn set_set1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Output 1 reset flag clear timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst1c(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 reset flag clear timer x."]
        #[inline(always)]
        pub const fn set_rst1c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Output 2 set flag clear timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set2c(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 set flag clear timer x."]
        #[inline(always)]
        pub const fn set_set2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output 2 reset flag clear timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst2c(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 reset flag clear timer x."]
        #[inline(always)]
        pub const fn set_rst2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rstc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset interrupt flag clear."]
        #[inline(always)]
        pub const fn set_rstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed protection flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprtc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed protection flag clear."]
        #[inline(always)]
        pub const fn set_dlyprtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Crossbar Override Rearm Delay Elapsed flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cordec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Rearm Delay Elapsed flag clear."]
        #[inline(always)]
        pub const fn set_cordec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Timxicr {
        #[inline(always)]
        fn default() -> Timxicr {
            Timxicr(0)
        }
    }
    impl core::fmt::Debug for Timxicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxicr")
                .field("cmp1c", &self.cmp1c())
                .field("cmp2c", &self.cmp2c())
                .field("cmp3c", &self.cmp3c())
                .field("cmp4c", &self.cmp4c())
                .field("repc", &self.repc())
                .field("cotc", &self.cotc())
                .field("updc", &self.updc())
                .field("cpt1c", &self.cpt1c())
                .field("cpt2c", &self.cpt2c())
                .field("set1c", &self.set1c())
                .field("rst1c", &self.rst1c())
                .field("set2c", &self.set2c())
                .field("rst2c", &self.rst2c())
                .field("rstc", &self.rstc())
                .field("dlyprtc", &self.dlyprtc())
                .field("cordec", &self.cordec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxicr {{ cmp1c: {=bool:?}, cmp2c: {=bool:?}, cmp3c: {=bool:?}, cmp4c: {=bool:?}, repc: {=bool:?}, cotc: {=bool:?}, updc: {=bool:?}, cpt1c: {=bool:?}, cpt2c: {=bool:?}, set1c: {=bool:?}, rst1c: {=bool:?}, set2c: {=bool:?}, rst2c: {=bool:?}, rstc: {=bool:?}, dlyprtc: {=bool:?}, cordec: {=bool:?} }}",
                self.cmp1c(),
                self.cmp2c(),
                self.cmp3c(),
                self.cmp4c(),
                self.repc(),
                self.cotc(),
                self.updc(),
                self.cpt1c(),
                self.cpt2c(),
                self.set1c(),
                self.rst1c(),
                self.set2c(),
                self.rst2c(),
                self.rstc(),
                self.dlyprtc(),
                self.cordec()
            )
        }
    }
    #[doc = "Timerx Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxisr(pub u32);
    impl Timxisr {
        #[doc = "Compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_cmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_cmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compare 3 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 3 interrupt flag."]
        #[inline(always)]
        pub const fn set_cmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare 4 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 4 interrupt flag."]
        #[inline(always)]
        pub const fn set_cmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Repetition interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition interrupt flag."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Crossbar Override Trigger flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cot(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Trigger flag."]
        #[inline(always)]
        pub const fn set_cot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn upd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag."]
        #[inline(always)]
        pub const fn set_upd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_cpt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cpt2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_cpt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Output 1 set interrupt flag timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 set interrupt flag timer x."]
        #[inline(always)]
        pub const fn set_set1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Output 1 reset interrupt flag timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 reset interrupt flag timer x."]
        #[inline(always)]
        pub const fn set_rst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Output 2 set interrupt flag timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn set2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 set interrupt flag timer x."]
        #[inline(always)]
        pub const fn set_set2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output 2 reset interrupt flag timer x."]
        #[must_use]
        #[inline(always)]
        pub const fn rst2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 reset interrupt flag timer x."]
        #[inline(always)]
        pub const fn set_rst2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset and/or roll-over interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset and/or roll-over interrupt flag."]
        #[inline(always)]
        pub const fn set_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed protection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprt(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed protection flag."]
        #[inline(always)]
        pub const fn set_dlyprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Crossbar Override Rearm Delay Elapsed flag."]
        #[must_use]
        #[inline(always)]
        pub const fn corde(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Rearm Delay Elapsed flag."]
        #[inline(always)]
        pub const fn set_corde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Current push-pull status."]
        #[must_use]
        #[inline(always)]
        pub const fn cppstat(&self) -> super::vals::Cppstat {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Cppstat::from_bits(val as u8)
        }
        #[doc = "Current push-pull status."]
        #[inline(always)]
        pub const fn set_cppstat(&mut self, val: super::vals::Cppstat) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Idle push-pull Status."]
        #[must_use]
        #[inline(always)]
        pub const fn ippstat(&self) -> super::vals::Ippstat {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ippstat::from_bits(val as u8)
        }
        #[doc = "Idle push-pull Status."]
        #[inline(always)]
        pub const fn set_ippstat(&mut self, val: super::vals::Ippstat) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Output 1 status."]
        #[must_use]
        #[inline(always)]
        pub const fn o1stat(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 status."]
        #[inline(always)]
        pub const fn set_o1stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Output 2 status."]
        #[must_use]
        #[inline(always)]
        pub const fn o2stat(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 status."]
        #[inline(always)]
        pub const fn set_o2stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Output 1 copy."]
        #[must_use]
        #[inline(always)]
        pub const fn o1cpy(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 copy."]
        #[inline(always)]
        pub const fn set_o1cpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Output 2 copy."]
        #[must_use]
        #[inline(always)]
        pub const fn o2cpy(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 copy."]
        #[inline(always)]
        pub const fn set_o2cpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Crossbar Override status."]
        #[must_use]
        #[inline(always)]
        pub const fn cos(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override status."]
        #[inline(always)]
        pub const fn set_cos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Crossbar Override Trigger status."]
        #[must_use]
        #[inline(always)]
        pub const fn cots(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Crossbar Override Trigger status."]
        #[inline(always)]
        pub const fn set_cots(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Timxisr {
        #[inline(always)]
        fn default() -> Timxisr {
            Timxisr(0)
        }
    }
    impl core::fmt::Debug for Timxisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxisr")
                .field("cmp1", &self.cmp1())
                .field("cmp2", &self.cmp2())
                .field("cmp3", &self.cmp3())
                .field("cmp4", &self.cmp4())
                .field("rep", &self.rep())
                .field("cot", &self.cot())
                .field("upd", &self.upd())
                .field("cpt1", &self.cpt1())
                .field("cpt2", &self.cpt2())
                .field("set1", &self.set1())
                .field("rst1", &self.rst1())
                .field("set2", &self.set2())
                .field("rst2", &self.rst2())
                .field("rst", &self.rst())
                .field("dlyprt", &self.dlyprt())
                .field("corde", &self.corde())
                .field("cppstat", &self.cppstat())
                .field("ippstat", &self.ippstat())
                .field("o1stat", &self.o1stat())
                .field("o2stat", &self.o2stat())
                .field("o1cpy", &self.o1cpy())
                .field("o2cpy", &self.o2cpy())
                .field("cos", &self.cos())
                .field("cots", &self.cots())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxisr {{ cmp1: {=bool:?}, cmp2: {=bool:?}, cmp3: {=bool:?}, cmp4: {=bool:?}, rep: {=bool:?}, cot: {=bool:?}, upd: {=bool:?}, cpt1: {=bool:?}, cpt2: {=bool:?}, set1: {=bool:?}, rst1: {=bool:?}, set2: {=bool:?}, rst2: {=bool:?}, rst: {=bool:?}, dlyprt: {=bool:?}, corde: {=bool:?}, cppstat: {:?}, ippstat: {:?}, o1stat: {=bool:?}, o2stat: {=bool:?}, o1cpy: {=bool:?}, o2cpy: {=bool:?}, cos: {=bool:?}, cots: {=bool:?} }}",
                self.cmp1(),
                self.cmp2(),
                self.cmp3(),
                self.cmp4(),
                self.rep(),
                self.cot(),
                self.upd(),
                self.cpt1(),
                self.cpt2(),
                self.set1(),
                self.rst1(),
                self.set2(),
                self.rst2(),
                self.rst(),
                self.dlyprt(),
                self.corde(),
                self.cppstat(),
                self.ippstat(),
                self.o1stat(),
                self.o2stat(),
                self.o1cpy(),
                self.o2cpy(),
                self.cos(),
                self.cots()
            )
        }
    }
    #[doc = "Timerx Output Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxoutr(pub u32);
    impl Timxoutr {
        #[doc = "Output 1 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pol1(&self) -> super::vals::Pol {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output 1 polarity."]
        #[inline(always)]
        pub const fn set_pol1(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Output 1 Idle mode."]
        #[must_use]
        #[inline(always)]
        pub const fn idlem1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 Idle mode."]
        #[inline(always)]
        pub const fn set_idlem1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output 1 Idle State."]
        #[must_use]
        #[inline(always)]
        pub const fn idles1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 Idle State."]
        #[inline(always)]
        pub const fn set_idles1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Output 1 fault state."]
        #[must_use]
        #[inline(always)]
        pub const fn fault1(&self) -> super::vals::Fault {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Fault::from_bits(val as u8)
        }
        #[doc = "Output 1 fault state."]
        #[inline(always)]
        pub const fn set_fault1(&mut self, val: super::vals::Fault) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Output 1 chopper enable."]
        #[must_use]
        #[inline(always)]
        pub const fn chp1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 chopper enable."]
        #[inline(always)]
        pub const fn set_chp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Output 1 deadtime upon burst mode idle entry."]
        #[must_use]
        #[inline(always)]
        pub const fn didl1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1 deadtime upon burst mode idle entry."]
        #[inline(always)]
        pub const fn set_didl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Deadtime enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime enable."]
        #[inline(always)]
        pub const fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Delayed protection enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed protection enable."]
        #[inline(always)]
        pub const fn set_dlyprten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Delayed protection."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyprt(&self) -> super::vals::Dlyprt {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Dlyprt::from_bits(val as u8)
        }
        #[doc = "Delayed protection."]
        #[inline(always)]
        pub const fn set_dlyprt(&mut self, val: super::vals::Dlyprt) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "Balanced Idle Automatic Resume."]
        #[must_use]
        #[inline(always)]
        pub const fn biar(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Balanced Idle Automatic Resume."]
        #[inline(always)]
        pub const fn set_biar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Output 2 polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pol2(&self) -> super::vals::Pol {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output 2 polarity."]
        #[inline(always)]
        pub const fn set_pol2(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Output 2 idle mode."]
        #[must_use]
        #[inline(always)]
        pub const fn idlem2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 idle mode."]
        #[inline(always)]
        pub const fn set_idlem2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Output 2 idle state."]
        #[must_use]
        #[inline(always)]
        pub const fn idles2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 idle state."]
        #[inline(always)]
        pub const fn set_idles2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Output 2 fault state."]
        #[must_use]
        #[inline(always)]
        pub const fn fault2(&self) -> super::vals::Fault {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Fault::from_bits(val as u8)
        }
        #[doc = "Output 2 fault state."]
        #[inline(always)]
        pub const fn set_fault2(&mut self, val: super::vals::Fault) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Output 2 chopper enable."]
        #[must_use]
        #[inline(always)]
        pub const fn chp2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 chopper enable."]
        #[inline(always)]
        pub const fn set_chp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Output 2 deadtime upon burst mode Idle entry."]
        #[must_use]
        #[inline(always)]
        pub const fn didl2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2 deadtime upon burst mode Idle entry."]
        #[inline(always)]
        pub const fn set_didl2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Timxoutr {
        #[inline(always)]
        fn default() -> Timxoutr {
            Timxoutr(0)
        }
    }
    impl core::fmt::Debug for Timxoutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxoutr")
                .field("pol1", &self.pol1())
                .field("idlem1", &self.idlem1())
                .field("idles1", &self.idles1())
                .field("fault1", &self.fault1())
                .field("chp1", &self.chp1())
                .field("didl1", &self.didl1())
                .field("dten", &self.dten())
                .field("dlyprten", &self.dlyprten())
                .field("dlyprt", &self.dlyprt())
                .field("biar", &self.biar())
                .field("pol2", &self.pol2())
                .field("idlem2", &self.idlem2())
                .field("idles2", &self.idles2())
                .field("fault2", &self.fault2())
                .field("chp2", &self.chp2())
                .field("didl2", &self.didl2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxoutr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxoutr {{ pol1: {:?}, idlem1: {=bool:?}, idles1: {=bool:?}, fault1: {:?}, chp1: {=bool:?}, didl1: {=bool:?}, dten: {=bool:?}, dlyprten: {=bool:?}, dlyprt: {:?}, biar: {=bool:?}, pol2: {:?}, idlem2: {=bool:?}, idles2: {=bool:?}, fault2: {:?}, chp2: {=bool:?}, didl2: {=bool:?} }}",
                self.pol1(),
                self.idlem1(),
                self.idles1(),
                self.fault1(),
                self.chp1(),
                self.didl1(),
                self.dten(),
                self.dlyprten(),
                self.dlyprt(),
                self.biar(),
                self.pol2(),
                self.idlem2(),
                self.idles2(),
                self.fault2(),
                self.chp2(),
                self.didl2()
            )
        }
    }
    #[doc = "Timerx Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxper(pub u32);
    impl Timxper {
        #[doc = "Timer x period value."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer x period value."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timxper {
        #[inline(always)]
        fn default() -> Timxper {
            Timxper(0)
        }
    }
    impl core::fmt::Debug for Timxper {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxper").field("per", &self.per()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxper {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxper {{ per: {=u16:?} }}", self.per())
        }
    }
    #[doc = "Timerx Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrep(pub u32);
    impl Timxrep {
        #[doc = "Timer x repetition period value."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timer x repetition period value."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Timxrep {
        #[inline(always)]
        fn default() -> Timxrep {
            Timxrep(0)
        }
    }
    impl core::fmt::Debug for Timxrep {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxrep").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxrep {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timxrep {{ rep: {=u8:?} }}", self.rep())
        }
    }
    #[doc = "Timerx Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrst(pub u32);
    impl Timxrst {
        #[doc = "Timer F compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F compare 1."]
        #[inline(always)]
        pub const fn set_timfcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A update reset."]
        #[must_use]
        #[inline(always)]
        pub const fn updt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A update reset."]
        #[inline(always)]
        pub const fn set_updt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer A compare 2 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A compare 2 reset."]
        #[inline(always)]
        pub const fn set_cmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer A compare 4 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A compare 4 reset."]
        #[inline(always)]
        pub const fn set_cmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Master timer period."]
        #[must_use]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer period."]
        #[inline(always)]
        pub const fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1."]
        #[inline(always)]
        pub const fn set_mstcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2."]
        #[inline(always)]
        pub const fn set_mstcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3."]
        #[inline(always)]
        pub const fn set_mstcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4."]
        #[inline(always)]
        pub const fn set_mstcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External event A."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External event A."]
        #[inline(always)]
        pub const fn set_extevnta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External event B."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntb(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External event B."]
        #[inline(always)]
        pub const fn set_extevntb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External event C."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External event C."]
        #[inline(always)]
        pub const fn set_extevntc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "External event D."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "External event D."]
        #[inline(always)]
        pub const fn set_extevntd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "External event E."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnte(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "External event E."]
        #[inline(always)]
        pub const fn set_extevnte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "External event F."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External event F."]
        #[inline(always)]
        pub const fn set_extevntf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External event G."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "External event G."]
        #[inline(always)]
        pub const fn set_extevntg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "External event H."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnth(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External event H."]
        #[inline(always)]
        pub const fn set_extevnth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External event I."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnti(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External event I."]
        #[inline(always)]
        pub const fn set_extevnti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External event J."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntj(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External event J."]
        #[inline(always)]
        pub const fn set_extevntj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer B compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 1."]
        #[inline(always)]
        pub const fn set_timcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer B compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp2(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 2."]
        #[inline(always)]
        pub const fn set_timcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer B compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timcmp4(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer B compare 4."]
        #[inline(always)]
        pub const fn set_timcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer C compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timccmp1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 1."]
        #[inline(always)]
        pub const fn set_timccmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Timer C compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timccmp2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 2."]
        #[inline(always)]
        pub const fn set_timccmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Timer C compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timccmp4(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Timer C compare 4."]
        #[inline(always)]
        pub const fn set_timccmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer D compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timdcmp1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 1."]
        #[inline(always)]
        pub const fn set_timdcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Timer D compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timdcmp2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 2."]
        #[inline(always)]
        pub const fn set_timdcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Timer D compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timdcmp4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Timer D compare 4."]
        #[inline(always)]
        pub const fn set_timdcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Timer E compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timecmp1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 1."]
        #[inline(always)]
        pub const fn set_timecmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Timer E compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timecmp2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 2."]
        #[inline(always)]
        pub const fn set_timecmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Timer E compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timecmp4(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Timer E compare 4."]
        #[inline(always)]
        pub const fn set_timecmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Timer F compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timfcmp2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Timer F compare 2."]
        #[inline(always)]
        pub const fn set_timfcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxrst {
        #[inline(always)]
        fn default() -> Timxrst {
            Timxrst(0)
        }
    }
    impl core::fmt::Debug for Timxrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxrst")
                .field("timfcmp1", &self.timfcmp1())
                .field("updt", &self.updt())
                .field("cmp2", &self.cmp2())
                .field("cmp4", &self.cmp4())
                .field("mstper", &self.mstper())
                .field("mstcmp1", &self.mstcmp1())
                .field("mstcmp2", &self.mstcmp2())
                .field("mstcmp3", &self.mstcmp3())
                .field("mstcmp4", &self.mstcmp4())
                .field("extevnta", &self.extevnta())
                .field("extevntb", &self.extevntb())
                .field("extevntc", &self.extevntc())
                .field("extevntd", &self.extevntd())
                .field("extevnte", &self.extevnte())
                .field("extevntf", &self.extevntf())
                .field("extevntg", &self.extevntg())
                .field("extevnth", &self.extevnth())
                .field("extevnti", &self.extevnti())
                .field("extevntj", &self.extevntj())
                .field("timcmp1", &self.timcmp1())
                .field("timcmp2", &self.timcmp2())
                .field("timcmp4", &self.timcmp4())
                .field("timccmp1", &self.timccmp1())
                .field("timccmp2", &self.timccmp2())
                .field("timccmp4", &self.timccmp4())
                .field("timdcmp1", &self.timdcmp1())
                .field("timdcmp2", &self.timdcmp2())
                .field("timdcmp4", &self.timdcmp4())
                .field("timecmp1", &self.timecmp1())
                .field("timecmp2", &self.timecmp2())
                .field("timecmp4", &self.timecmp4())
                .field("timfcmp2", &self.timfcmp2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxrst {{ timfcmp1: {=bool:?}, updt: {=bool:?}, cmp2: {=bool:?}, cmp4: {=bool:?}, mstper: {=bool:?}, mstcmp1: {=bool:?}, mstcmp2: {=bool:?}, mstcmp3: {=bool:?}, mstcmp4: {=bool:?}, extevnta: {=bool:?}, extevntb: {=bool:?}, extevntc: {=bool:?}, extevntd: {=bool:?}, extevnte: {=bool:?}, extevntf: {=bool:?}, extevntg: {=bool:?}, extevnth: {=bool:?}, extevnti: {=bool:?}, extevntj: {=bool:?}, timcmp1: {=bool:?}, timcmp2: {=bool:?}, timcmp4: {=bool:?}, timccmp1: {=bool:?}, timccmp2: {=bool:?}, timccmp4: {=bool:?}, timdcmp1: {=bool:?}, timdcmp2: {=bool:?}, timdcmp4: {=bool:?}, timecmp1: {=bool:?}, timecmp2: {=bool:?}, timecmp4: {=bool:?}, timfcmp2: {=bool:?} }}",
                self.timfcmp1(),
                self.updt(),
                self.cmp2(),
                self.cmp4(),
                self.mstper(),
                self.mstcmp1(),
                self.mstcmp2(),
                self.mstcmp3(),
                self.mstcmp4(),
                self.extevnta(),
                self.extevntb(),
                self.extevntc(),
                self.extevntd(),
                self.extevnte(),
                self.extevntf(),
                self.extevntg(),
                self.extevnth(),
                self.extevnti(),
                self.extevntj(),
                self.timcmp1(),
                self.timcmp2(),
                self.timcmp4(),
                self.timccmp1(),
                self.timccmp2(),
                self.timccmp4(),
                self.timdcmp1(),
                self.timdcmp2(),
                self.timdcmp4(),
                self.timecmp1(),
                self.timecmp2(),
                self.timecmp4(),
                self.timfcmp2()
            )
        }
    }
    #[doc = "Timerx OutputX Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrstr(pub u32);
    impl Timxrstr {
        #[doc = "Software set trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn srt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software set trigger."]
        #[inline(always)]
        pub const fn set_srt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A resynchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn resync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A resynchronization."]
        #[inline(always)]
        pub const fn set_resync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer x period."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x period."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer x compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 1."]
        #[inline(always)]
        pub const fn set_cmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer x compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 2."]
        #[inline(always)]
        pub const fn set_cmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer x compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 3."]
        #[inline(always)]
        pub const fn set_cmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer x compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 4."]
        #[inline(always)]
        pub const fn set_cmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master period."]
        #[must_use]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master period."]
        #[inline(always)]
        pub const fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1."]
        #[inline(always)]
        pub const fn set_mstcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2."]
        #[inline(always)]
        pub const fn set_mstcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3."]
        #[inline(always)]
        pub const fn set_mstcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4."]
        #[inline(always)]
        pub const fn set_mstcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timer event 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 1."]
        #[inline(always)]
        pub const fn set_timevnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer event 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 2."]
        #[inline(always)]
        pub const fn set_timevnt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer event 3."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 3."]
        #[inline(always)]
        pub const fn set_timevnt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timer event 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 4."]
        #[inline(always)]
        pub const fn set_timevnt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer event 5."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 5."]
        #[inline(always)]
        pub const fn set_timevnt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer event 6."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt6(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 6."]
        #[inline(always)]
        pub const fn set_timevnt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer event 7."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt7(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 7."]
        #[inline(always)]
        pub const fn set_timevnt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer event 8."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt8(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 8."]
        #[inline(always)]
        pub const fn set_timevnt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer event 9."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt9(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 9."]
        #[inline(always)]
        pub const fn set_timevnt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event A."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnta(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "External event A."]
        #[inline(always)]
        pub const fn set_extevnta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "External event B."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "External event B."]
        #[inline(always)]
        pub const fn set_extevntb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event C."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "External event C."]
        #[inline(always)]
        pub const fn set_extevntc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "External event D."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "External event D."]
        #[inline(always)]
        pub const fn set_extevntd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External event E."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnte(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "External event E."]
        #[inline(always)]
        pub const fn set_extevnte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "External event F."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event F."]
        #[inline(always)]
        pub const fn set_extevntf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event G."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntg(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "External event G."]
        #[inline(always)]
        pub const fn set_extevntg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "External event H."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnth(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External event H."]
        #[inline(always)]
        pub const fn set_extevnth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External event I."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnti(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External event I."]
        #[inline(always)]
        pub const fn set_extevnti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "External event J."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntj(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "External event J."]
        #[inline(always)]
        pub const fn set_extevntj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Registers update (transfer from preload register to active register)."]
        #[must_use]
        #[inline(always)]
        pub const fn update(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update (transfer from preload register to active register)."]
        #[inline(always)]
        pub const fn set_update(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxrstr {
        #[inline(always)]
        fn default() -> Timxrstr {
            Timxrstr(0)
        }
    }
    impl core::fmt::Debug for Timxrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxrstr")
                .field("srt", &self.srt())
                .field("resync", &self.resync())
                .field("per", &self.per())
                .field("cmp1", &self.cmp1())
                .field("cmp2", &self.cmp2())
                .field("cmp3", &self.cmp3())
                .field("cmp4", &self.cmp4())
                .field("mstper", &self.mstper())
                .field("mstcmp1", &self.mstcmp1())
                .field("mstcmp2", &self.mstcmp2())
                .field("mstcmp3", &self.mstcmp3())
                .field("mstcmp4", &self.mstcmp4())
                .field("timevnt1", &self.timevnt1())
                .field("timevnt2", &self.timevnt2())
                .field("timevnt3", &self.timevnt3())
                .field("timevnt4", &self.timevnt4())
                .field("timevnt5", &self.timevnt5())
                .field("timevnt6", &self.timevnt6())
                .field("timevnt7", &self.timevnt7())
                .field("timevnt8", &self.timevnt8())
                .field("timevnt9", &self.timevnt9())
                .field("extevnta", &self.extevnta())
                .field("extevntb", &self.extevntb())
                .field("extevntc", &self.extevntc())
                .field("extevntd", &self.extevntd())
                .field("extevnte", &self.extevnte())
                .field("extevntf", &self.extevntf())
                .field("extevntg", &self.extevntg())
                .field("extevnth", &self.extevnth())
                .field("extevnti", &self.extevnti())
                .field("extevntj", &self.extevntj())
                .field("update", &self.update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxrstr {{ srt: {=bool:?}, resync: {=bool:?}, per: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, cmp3: {=bool:?}, cmp4: {=bool:?}, mstper: {=bool:?}, mstcmp1: {=bool:?}, mstcmp2: {=bool:?}, mstcmp3: {=bool:?}, mstcmp4: {=bool:?}, timevnt1: {=bool:?}, timevnt2: {=bool:?}, timevnt3: {=bool:?}, timevnt4: {=bool:?}, timevnt5: {=bool:?}, timevnt6: {=bool:?}, timevnt7: {=bool:?}, timevnt8: {=bool:?}, timevnt9: {=bool:?}, extevnta: {=bool:?}, extevntb: {=bool:?}, extevntc: {=bool:?}, extevntd: {=bool:?}, extevnte: {=bool:?}, extevntf: {=bool:?}, extevntg: {=bool:?}, extevnth: {=bool:?}, extevnti: {=bool:?}, extevntj: {=bool:?}, update: {=bool:?} }}",
                self.srt(),
                self.resync(),
                self.per(),
                self.cmp1(),
                self.cmp2(),
                self.cmp3(),
                self.cmp4(),
                self.mstper(),
                self.mstcmp1(),
                self.mstcmp2(),
                self.mstcmp3(),
                self.mstcmp4(),
                self.timevnt1(),
                self.timevnt2(),
                self.timevnt3(),
                self.timevnt4(),
                self.timevnt5(),
                self.timevnt6(),
                self.timevnt7(),
                self.timevnt8(),
                self.timevnt9(),
                self.extevnta(),
                self.extevntb(),
                self.extevntc(),
                self.extevntd(),
                self.extevnte(),
                self.extevntf(),
                self.extevntg(),
                self.extevnth(),
                self.extevnti(),
                self.extevntj(),
                self.update()
            )
        }
    }
    #[doc = "Timerx OutputX Set Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxsetr(pub u32);
    impl Timxsetr {
        #[doc = "Software set trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn sst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software set trigger."]
        #[inline(always)]
        pub const fn set_sst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer A resynchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn resync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A resynchronization."]
        #[inline(always)]
        pub const fn set_resync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer x period."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x period."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer x compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 1."]
        #[inline(always)]
        pub const fn set_cmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer x compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 2."]
        #[inline(always)]
        pub const fn set_cmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer x compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 3."]
        #[inline(always)]
        pub const fn set_cmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer x compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer x compare 4."]
        #[inline(always)]
        pub const fn set_cmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master period."]
        #[must_use]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master period."]
        #[inline(always)]
        pub const fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Master compare 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 1."]
        #[inline(always)]
        pub const fn set_mstcmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Master compare 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 2."]
        #[inline(always)]
        pub const fn set_mstcmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Master compare 3."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 3."]
        #[inline(always)]
        pub const fn set_mstcmp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Master compare 4."]
        #[must_use]
        #[inline(always)]
        pub const fn mstcmp4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Master compare 4."]
        #[inline(always)]
        pub const fn set_mstcmp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Timer event 1."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 1."]
        #[inline(always)]
        pub const fn set_timevnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer event 2."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 2."]
        #[inline(always)]
        pub const fn set_timevnt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer event 3."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 3."]
        #[inline(always)]
        pub const fn set_timevnt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timer event 4."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 4."]
        #[inline(always)]
        pub const fn set_timevnt4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer event 5."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 5."]
        #[inline(always)]
        pub const fn set_timevnt5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer event 6."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt6(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 6."]
        #[inline(always)]
        pub const fn set_timevnt6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer event 7."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt7(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 7."]
        #[inline(always)]
        pub const fn set_timevnt7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer event 8."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt8(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 8."]
        #[inline(always)]
        pub const fn set_timevnt8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer event 9."]
        #[must_use]
        #[inline(always)]
        pub const fn timevnt9(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer event 9."]
        #[inline(always)]
        pub const fn set_timevnt9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "External event A."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnta(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "External event A."]
        #[inline(always)]
        pub const fn set_extevnta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "External event B."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntb(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "External event B."]
        #[inline(always)]
        pub const fn set_extevntb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "External event C."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "External event C."]
        #[inline(always)]
        pub const fn set_extevntc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "External event D."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "External event D."]
        #[inline(always)]
        pub const fn set_extevntd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External event E."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnte(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "External event E."]
        #[inline(always)]
        pub const fn set_extevnte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "External event F."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External event F."]
        #[inline(always)]
        pub const fn set_extevntf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "External event G."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntg(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "External event G."]
        #[inline(always)]
        pub const fn set_extevntg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "External event H."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnth(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "External event H."]
        #[inline(always)]
        pub const fn set_extevnth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "External event I."]
        #[must_use]
        #[inline(always)]
        pub const fn extevnti(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "External event I."]
        #[inline(always)]
        pub const fn set_extevnti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "External event J."]
        #[must_use]
        #[inline(always)]
        pub const fn extevntj(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "External event J."]
        #[inline(always)]
        pub const fn set_extevntj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Registers update (transfer from preload register to active register)."]
        #[must_use]
        #[inline(always)]
        pub const fn update(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update (transfer from preload register to active register)."]
        #[inline(always)]
        pub const fn set_update(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Timxsetr {
        #[inline(always)]
        fn default() -> Timxsetr {
            Timxsetr(0)
        }
    }
    impl core::fmt::Debug for Timxsetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timxsetr")
                .field("sst", &self.sst())
                .field("resync", &self.resync())
                .field("per", &self.per())
                .field("cmp1", &self.cmp1())
                .field("cmp2", &self.cmp2())
                .field("cmp3", &self.cmp3())
                .field("cmp4", &self.cmp4())
                .field("mstper", &self.mstper())
                .field("mstcmp1", &self.mstcmp1())
                .field("mstcmp2", &self.mstcmp2())
                .field("mstcmp3", &self.mstcmp3())
                .field("mstcmp4", &self.mstcmp4())
                .field("timevnt1", &self.timevnt1())
                .field("timevnt2", &self.timevnt2())
                .field("timevnt3", &self.timevnt3())
                .field("timevnt4", &self.timevnt4())
                .field("timevnt5", &self.timevnt5())
                .field("timevnt6", &self.timevnt6())
                .field("timevnt7", &self.timevnt7())
                .field("timevnt8", &self.timevnt8())
                .field("timevnt9", &self.timevnt9())
                .field("extevnta", &self.extevnta())
                .field("extevntb", &self.extevntb())
                .field("extevntc", &self.extevntc())
                .field("extevntd", &self.extevntd())
                .field("extevnte", &self.extevnte())
                .field("extevntf", &self.extevntf())
                .field("extevntg", &self.extevntg())
                .field("extevnth", &self.extevnth())
                .field("extevnti", &self.extevnti())
                .field("extevntj", &self.extevntj())
                .field("update", &self.update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxsetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timxsetr {{ sst: {=bool:?}, resync: {=bool:?}, per: {=bool:?}, cmp1: {=bool:?}, cmp2: {=bool:?}, cmp3: {=bool:?}, cmp4: {=bool:?}, mstper: {=bool:?}, mstcmp1: {=bool:?}, mstcmp2: {=bool:?}, mstcmp3: {=bool:?}, mstcmp4: {=bool:?}, timevnt1: {=bool:?}, timevnt2: {=bool:?}, timevnt3: {=bool:?}, timevnt4: {=bool:?}, timevnt5: {=bool:?}, timevnt6: {=bool:?}, timevnt7: {=bool:?}, timevnt8: {=bool:?}, timevnt9: {=bool:?}, extevnta: {=bool:?}, extevntb: {=bool:?}, extevntc: {=bool:?}, extevntd: {=bool:?}, extevnte: {=bool:?}, extevntf: {=bool:?}, extevntg: {=bool:?}, extevnth: {=bool:?}, extevnti: {=bool:?}, extevntj: {=bool:?}, update: {=bool:?} }}",
                self.sst(),
                self.resync(),
                self.per(),
                self.cmp1(),
                self.cmp2(),
                self.cmp3(),
                self.cmp4(),
                self.mstper(),
                self.mstcmp1(),
                self.mstcmp2(),
                self.mstcmp3(),
                self.mstcmp4(),
                self.timevnt1(),
                self.timevnt2(),
                self.timevnt3(),
                self.timevnt4(),
                self.timevnt5(),
                self.timevnt6(),
                self.timevnt7(),
                self.timevnt8(),
                self.timevnt9(),
                self.extevnta(),
                self.extevntb(),
                self.extevntc(),
                self.extevntd(),
                self.extevnte(),
                self.extevntf(),
                self.extevntg(),
                self.extevnth(),
                self.extevnti(),
                self.extevntj(),
                self.update()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Brstdma {
        #[doc = "Update done independently from the DMA burst transfer completion"]
        Independent = 0x0,
        #[doc = "Update done when the DMA burst transfer is completed"]
        Completion = 0x01,
        #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
        Rollover = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Brstdma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Brstdma {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Brstdma {
        #[inline(always)]
        fn from(val: u8) -> Brstdma {
            Brstdma::from_bits(val)
        }
    }
    impl From<Brstdma> for u8 {
        #[inline(always)]
        fn from(val: Brstdma) -> u8 {
            Brstdma::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cppstat {
        #[doc = "Signal applied on output 1 and output 2 forced inactive"]
        Output1active = 0x0,
        #[doc = "Signal applied on output 2 and output 1 forced inactive"]
        Output2active = 0x01,
    }
    impl Cppstat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cppstat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cppstat {
        #[inline(always)]
        fn from(val: u8) -> Cppstat {
            Cppstat::from_bits(val)
        }
    }
    impl From<Cppstat> for u8 {
        #[inline(always)]
        fn from(val: Cppstat) -> u8 {
            Cppstat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacsync {
        #[doc = "No DAC trigger generated"]
        Disabled = 0x0,
        #[doc = "Trigger generated on DACSync1"]
        DacSync1 = 0x01,
        #[doc = "Trigger generated on DACSync2"]
        DacSync2 = 0x02,
        #[doc = "Trigger generated on DACSync3"]
        DacSync3 = 0x03,
    }
    impl Dacsync {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacsync {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacsync {
        #[inline(always)]
        fn from(val: u8) -> Dacsync {
            Dacsync::from_bits(val)
        }
    }
    impl From<Dacsync> for u8 {
        #[inline(always)]
        fn from(val: Dacsync) -> u8 {
            Dacsync::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Delcmp {
        #[doc = "CMP register is always active (standard compare mode)"]
        Standard = 0x0,
        #[doc = "CMP is recomputed and is active following a capture 1 event"]
        Capture1 = 0x01,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 1 match"]
        CaptureXCompare1 = 0x02,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 3 match"]
        CaptureXCompare3 = 0x03,
    }
    impl Delcmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Delcmp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Delcmp {
        #[inline(always)]
        fn from(val: u8) -> Delcmp {
            Delcmp::from_bits(val)
        }
    }
    impl From<Delcmp> for u8 {
        #[inline(always)]
        fn from(val: Delcmp) -> u8 {
            Delcmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dlyprt {
        #[doc = "Output 1 delayed idle on external event 6"]
        Output1Ee6 = 0x0,
        #[doc = "Output 2 delayed idle on external event 6"]
        Output2Ee6 = 0x01,
        #[doc = "Output 1 and 2 delayed idle on external event 6"]
        Output12Ee6 = 0x02,
        #[doc = "Balanced idle on external event 6"]
        BalancedEe6 = 0x03,
        #[doc = "Output 1 delayed idle on external event 7"]
        Output1Ee7 = 0x04,
        #[doc = "Output 2 delayed idle on external event 7"]
        Output2Ee7 = 0x05,
        #[doc = "Output 1 and 2 delayed idle on external event 7"]
        Output12Ee7 = 0x06,
        #[doc = "Balanced idle on external event 7"]
        BalancedEe7 = 0x07,
    }
    impl Dlyprt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dlyprt {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dlyprt {
        #[inline(always)]
        fn from(val: u8) -> Dlyprt {
            Dlyprt::from_bits(val)
        }
    }
    impl From<Dlyprt> for u8 {
        #[inline(always)]
        fn from(val: Dlyprt) -> u8 {
            Dlyprt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fault {
        #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
        Disabled = 0x0,
        #[doc = "Output goes to active state after a fault event"]
        SetActive = 0x01,
        #[doc = "Output goes to inactive state after a fault event"]
        SetInactive = 0x02,
        #[doc = "Output goes to high-z state after a fault event"]
        SetHighZ = 0x03,
    }
    impl Fault {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fault {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fault {
        #[inline(always)]
        fn from(val: u8) -> Fault {
            Fault::from_bits(val)
        }
    }
    impl From<Fault> for u8 {
        #[inline(always)]
        fn from(val: Fault) -> u8 {
            Fault::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ippstat {
        #[doc = "Protection occurred when the output 1 was active and output 2 forced inactive"]
        Output1active = 0x0,
        #[doc = "Protection occurred when the output 2 was active and output 1 forced inactive"]
        Output2active = 0x01,
    }
    impl Ippstat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ippstat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ippstat {
        #[inline(always)]
        fn from(val: u8) -> Ippstat {
            Ippstat::from_bits(val)
        }
    }
    impl From<Ippstat> for u8 {
        #[inline(always)]
        fn from(val: Ippstat) -> u8 {
            Ippstat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pol {
        #[doc = "Positive polarity (output active high)"]
        ActiveHigh = 0x0,
        #[doc = "Negative polarity (output active low)"]
        ActiveLow = 0x01,
    }
    impl Pol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pol {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdt {
        #[doc = "Positive deadtime (both outputs inactive during deadtime)"]
        Positive = 0x0,
        #[doc = "Negative deadtime (both outputs active during deadtime)"]
        Negative = 0x01,
    }
    impl Sdt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdt {
        #[inline(always)]
        fn from(val: u8) -> Sdt {
            Sdt::from_bits(val)
        }
    }
    impl From<Sdt> for u8 {
        #[inline(always)]
        fn from(val: Sdt) -> u8 {
            Sdt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncin {
        #[doc = "Disabled. HRTIM is not synchronized and runs in standalone mode"]
        Disabled = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
        Internal = 0x02,
        #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
        External = 0x03,
    }
    impl Syncin {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncin {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncin {
        #[inline(always)]
        fn from(val: u8) -> Syncin {
            Syncin::from_bits(val)
        }
    }
    impl From<Syncin> for u8 {
        #[inline(always)]
        fn from(val: Syncin) -> u8 {
            Syncin::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncout {
        #[doc = "Disabled"]
        Disabled = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        PositivePulse = 0x02,
        #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        NegativePulse = 0x03,
    }
    impl Syncout {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncout {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncout {
        #[inline(always)]
        fn from(val: u8) -> Syncout {
            Syncout::from_bits(val)
        }
    }
    impl From<Syncout> for u8 {
        #[inline(always)]
        fn from(val: Syncout) -> u8 {
            Syncout::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncsrc {
        #[doc = "Master timer Start"]
        MasterStart = 0x0,
        #[doc = "Master timer Compare 1 event"]
        MasterCompare1 = 0x01,
        #[doc = "Timer A start/reset"]
        TimerAStart = 0x02,
        #[doc = "Timer A Compare 1 event"]
        TimerACompare1 = 0x03,
    }
    impl Syncsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncsrc {
        #[inline(always)]
        fn from(val: u8) -> Syncsrc {
            Syncsrc::from_bits(val)
        }
    }
    impl From<Syncsrc> for u8 {
        #[inline(always)]
        fn from(val: Syncsrc) -> u8 {
            Syncsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Updgat {
        #[doc = "Update occurs independently from the DMA burst transfer"]
        Independent = 0x0,
        #[doc = "Update occurs when the DMA burst transfer is completed"]
        DmaBurst = 0x01,
        #[doc = "Update occurs on the update event following DMA burst transfer completion"]
        DmaBurstUpdate = 0x02,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
        Input1 = 0x03,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
        Input2 = 0x04,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
        Input3 = 0x05,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
        Input1Update = 0x06,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
        Input2Update = 0x07,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
        Input3Update = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Updgat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Updgat {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Updgat {
        #[inline(always)]
        fn from(val: u8) -> Updgat {
            Updgat::from_bits(val)
        }
    }
    impl From<Updgat> for u8 {
        #[inline(always)]
        fn from(val: Updgat) -> u8 {
            Updgat::to_bits(val)
        }
    }
}
