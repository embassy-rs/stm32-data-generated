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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[inline(always)]
    pub const fn micr(self) -> crate::common::Reg<regs::Micr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mdier(self) -> crate::common::Reg<regs::Mdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Master Timer Counter Register"]
    #[inline(always)]
    pub const fn mcntr(self) -> crate::common::Reg<regs::Mcntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Master Timer Period Register"]
    #[inline(always)]
    pub const fn mper(self) -> crate::common::Reg<regs::Mper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Master Timer Repetition Register"]
    #[inline(always)]
    pub const fn mrep(self) -> crate::common::Reg<regs::Mrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Master Timer Compare X Register"]
    #[inline(always)]
    pub const fn mcmp(self, n: usize) -> crate::common::Reg<regs::Mcmpx, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr
                    .add(0x1cusize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _,
            )
        }
    }
    #[doc = "High Resolution Timer: Timing Unit"]
    #[inline(always)]
    pub const fn tim(self, n: usize) -> HrtimTimx {
        assert!(n < 6usize);
        unsafe { HrtimTimx::from_ptr(self.ptr.add(0x80usize + n * 128usize) as _) }
    }
    #[doc = "High Resolution Timer: Control Register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::HrtimCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "High Resolution Timer: Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::HrtimCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::HrtimIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::HrtimIcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[doc = "High Resolution Timer: Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::HrtimIer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Enable Register"]
    #[inline(always)]
    pub const fn oenr(self) -> crate::common::Reg<regs::HrtimOenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Disable Register"]
    #[inline(always)]
    pub const fn odisr(self) -> crate::common::Reg<regs::HrtimOdisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[doc = "High Resolution Timer: Output Disable Status Register"]
    #[inline(always)]
    pub const fn odsr(self) -> crate::common::Reg<regs::HrtimOdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Control Register"]
    #[inline(always)]
    pub const fn bmcr(self) -> crate::common::Reg<regs::HrtimBmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Trigger Register"]
    #[inline(always)]
    pub const fn bmtrgr(self) -> crate::common::Reg<regs::HrtimBmtrgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Compare Register"]
    #[inline(always)]
    pub const fn bmcmpr(self) -> crate::common::Reg<regs::HrtimBmcmpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst Mode Period Register"]
    #[inline(always)]
    pub const fn bmper(self) -> crate::common::Reg<regs::HrtimBmper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[doc = "High Resolution Timer: External Event Control Register 1"]
    #[inline(always)]
    pub const fn eecr1(self) -> crate::common::Reg<regs::HrtimEecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[doc = "High Resolution Timer: External Event Control Register 2"]
    #[inline(always)]
    pub const fn eecr2(self) -> crate::common::Reg<regs::HrtimEecr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[doc = "High Resolution Timer: External Event Control Register 3"]
    #[inline(always)]
    pub const fn eecr3(self) -> crate::common::Reg<regs::HrtimEecr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b8usize) as _) }
    }
    #[doc = "High Resolution Timer: ADC Trigger \\[1, 3\\]
Register"]
    #[inline(always)]
    pub const fn adc1r(self, n: usize) -> crate::common::Reg<regs::HrtimAdc1r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03bcusize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "High Resolution Timer: ADC Trigger \\[2, 4\\]
Register"]
    #[inline(always)]
    pub const fn adc2r(self, n: usize) -> crate::common::Reg<regs::HrtimAdc2r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "High Resolution Timer: DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(self) -> crate::common::Reg<regs::HrtimDllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ccusize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 1"]
    #[inline(always)]
    pub const fn fltinr1(self) -> crate::common::Reg<regs::HrtimFltinr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[doc = "High Resolution Timer: Fault Input Register 2"]
    #[inline(always)]
    pub const fn fltinr2(self) -> crate::common::Reg<regs::HrtimFltinr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[inline(always)]
    pub const fn bdmupr(self) -> crate::common::Reg<regs::HrtimBdmupr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d8usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Timer X update Register"]
    #[inline(always)]
    pub const fn bdtupr(self, n: usize) -> crate::common::Reg<regs::HrtimBdtupr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize + n * 4usize) as _) }
    }
    #[doc = "High Resolution Timer: Burst DMA Data Register"]
    #[inline(always)]
    pub const fn bdmadr(self) -> crate::common::Reg<regs::HrtimBdmadr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
}
#[doc = "High Resolution Timer: Timing Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrtimTimx {
    ptr: *mut u8,
}
unsafe impl Send for HrtimTimx {}
unsafe impl Sync for HrtimTimx {}
impl HrtimTimx {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Timer X Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Timxisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Timer X Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Timxicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer X DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Timxdier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Timer X Counter Register"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Timxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Timer X Period Register"]
    #[inline(always)]
    pub const fn per(self) -> crate::common::Reg<regs::Timxper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Timer X Repetition Register"]
    #[inline(always)]
    pub const fn rep(self) -> crate::common::Reg<regs::Timxrep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Timer X Compare X Register"]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Timxcmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr
                    .add(0x1cusize + ([0usize, 8usize, 12usize, 16usize][n] as usize)) as _,
            )
        }
    }
    #[doc = "Timer X Compare X Compound Register"]
    #[inline(always)]
    pub const fn cmpc(self, n: usize) -> crate::common::Reg<regs::Timxcmpc, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + ([0usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Capture X Register"]
    #[inline(always)]
    pub const fn cpt(self, n: usize) -> crate::common::Reg<regs::Timxcpt, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Timer X Deadtime Register"]
    #[inline(always)]
    pub const fn dt(self) -> crate::common::Reg<regs::Timxdt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Timer X Output X Set Register"]
    #[inline(always)]
    pub const fn setr(self, n: usize) -> crate::common::Reg<regs::Timxsetr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output X Reset Register"]
    #[inline(always)]
    pub const fn rstr(self, n: usize) -> crate::common::Reg<regs::Timxrstr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + ([0usize, 8usize][n] as usize)) as _) }
    }
    #[doc = "Timer X External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eef(self, n: usize) -> crate::common::Reg<regs::Timxeef, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Reset Register"]
    #[inline(always)]
    pub const fn rst(self) -> crate::common::Reg<regs::Timxrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Timer X Chopper Register"]
    #[inline(always)]
    pub const fn chp(self) -> crate::common::Reg<regs::Timxchp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Timer X Capture X Control Register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Timxccr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize + ([0usize, 4usize][n] as usize)) as _) }
    }
    #[doc = "Timer X Output Register"]
    #[inline(always)]
    pub const fn outr(self) -> crate::common::Reg<regs::Timxoutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Timer X Fault Register"]
    #[inline(always)]
    pub const fn flt(self) -> crate::common::Reg<regs::Timxflt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
}
pub mod regs {
    #[doc = "High Resolution Timer: ADC Trigger 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdc1r(pub u32);
    impl HrtimAdc1r {
        #[doc = "ADC trigger X on Master Compare Y"]
        #[inline(always)]
        pub const fn adcmc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Master Compare Y"]
        #[inline(always)]
        pub fn set_adcmc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Master Period"]
        #[inline(always)]
        pub const fn adcmper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Master Period"]
        #[inline(always)]
        pub fn set_adcmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC trigger X on External Event Y"]
        #[inline(always)]
        pub const fn adceev(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 5usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on External Event Y"]
        #[inline(always)]
        pub fn set_adceev(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 5usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 2"]
        #[inline(always)]
        pub const fn adctc2(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 10usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 2"]
        #[inline(always)]
        pub fn set_adctc2(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 10usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub const fn adctc3(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 11usize + ([0usize, 5usize, 10usize, 14usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub fn set_adctc3(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 11usize + ([0usize, 5usize, 10usize, 14usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub const fn adctc4(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 12usize + ([0usize, 5usize, 10usize, 14usize, 18usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub fn set_adctc4(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 12usize + ([0usize, 5usize, 10usize, 14usize, 18usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Period"]
        #[inline(always)]
        pub const fn adctper(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 13usize + ([0usize, 5usize, 10usize, 14usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Period"]
        #[inline(always)]
        pub fn set_adctper(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 13usize + ([0usize, 5usize, 10usize, 14usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Reset"]
        #[inline(always)]
        pub const fn adctrst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 14usize + ([0usize, 5usize, 14usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Reset"]
        #[inline(always)]
        pub fn set_adctrst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 14usize + ([0usize, 5usize, 14usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "adcmc",
                    &[
                        self.adcmc(0usize),
                        self.adcmc(1usize),
                        self.adcmc(2usize),
                        self.adcmc(3usize),
                    ],
                )
                .field("adcmper", &self.adcmper())
                .field(
                    "adceev",
                    &[
                        self.adceev(0usize),
                        self.adceev(1usize),
                        self.adceev(2usize),
                        self.adceev(3usize),
                        self.adceev(4usize),
                    ],
                )
                .field("adctc2", &[self.adctc2(0usize)])
                .field(
                    "adctc3",
                    &[
                        self.adctc3(0usize),
                        self.adctc3(1usize),
                        self.adctc3(2usize),
                        self.adctc3(3usize),
                        self.adctc3(4usize),
                    ],
                )
                .field(
                    "adctc4",
                    &[
                        self.adctc4(0usize),
                        self.adctc4(1usize),
                        self.adctc4(2usize),
                        self.adctc4(3usize),
                        self.adctc4(4usize),
                        self.adctc4(5usize),
                    ],
                )
                .field(
                    "adctper",
                    &[
                        self.adctper(0usize),
                        self.adctper(1usize),
                        self.adctper(2usize),
                        self.adctper(3usize),
                        self.adctper(4usize),
                    ],
                )
                .field(
                    "adctrst",
                    &[self.adctrst(0usize), self.adctrst(1usize), self.adctrst(2usize)],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdc1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimAdc1r {
                adcmc: [bool; 4usize],
                adcmper: bool,
                adceev: [bool; 5usize],
                adctc2: [bool; 1usize],
                adctc3: [bool; 5usize],
                adctc4: [bool; 6usize],
                adctper: [bool; 5usize],
                adctrst: [bool; 3usize],
            }
            let proxy = HrtimAdc1r {
                adcmc: [
                    self.adcmc(0usize),
                    self.adcmc(1usize),
                    self.adcmc(2usize),
                    self.adcmc(3usize),
                ],
                adcmper: self.adcmper(),
                adceev: [
                    self.adceev(0usize),
                    self.adceev(1usize),
                    self.adceev(2usize),
                    self.adceev(3usize),
                    self.adceev(4usize),
                ],
                adctc2: [self.adctc2(0usize)],
                adctc3: [
                    self.adctc3(0usize),
                    self.adctc3(1usize),
                    self.adctc3(2usize),
                    self.adctc3(3usize),
                    self.adctc3(4usize),
                ],
                adctc4: [
                    self.adctc4(0usize),
                    self.adctc4(1usize),
                    self.adctc4(2usize),
                    self.adctc4(3usize),
                    self.adctc4(4usize),
                    self.adctc4(5usize),
                ],
                adctper: [
                    self.adctper(0usize),
                    self.adctper(1usize),
                    self.adctper(2usize),
                    self.adctper(3usize),
                    self.adctper(4usize),
                ],
                adctrst: [self.adctrst(0usize), self.adctrst(1usize), self.adctrst(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: ADC Trigger 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimAdc2r(pub u32);
    impl HrtimAdc2r {
        #[doc = "ADC trigger X on Master Compare Y"]
        #[inline(always)]
        pub const fn adcmc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Master Compare Y"]
        #[inline(always)]
        pub fn set_adcmc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Master Period"]
        #[inline(always)]
        pub const fn adcmper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Master Period"]
        #[inline(always)]
        pub fn set_adcmper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ADC trigger X on External Event Y"]
        #[inline(always)]
        pub const fn adceev(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 5usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on External Event Y"]
        #[inline(always)]
        pub fn set_adceev(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 5usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 2"]
        #[inline(always)]
        pub const fn adctc2(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 10usize + ([0usize, 4usize, 8usize, 13usize, 18usize, 1usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 2"]
        #[inline(always)]
        pub fn set_adctc2(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 10usize + ([0usize, 4usize, 8usize, 13usize, 18usize, 1usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub const fn adctc4(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 12usize + ([0usize, 4usize, 8usize, 13usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub fn set_adctc4(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 12usize + ([0usize, 4usize, 8usize, 13usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Period"]
        #[inline(always)]
        pub const fn adctper(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 13usize + ([0usize, 4usize, 8usize, 13usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Period"]
        #[inline(always)]
        pub fn set_adctper(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 13usize + ([0usize, 4usize, 8usize, 13usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub const fn adctc3(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 15usize + ([14usize, 0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Compare 3"]
        #[inline(always)]
        pub fn set_adctc3(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 15usize + ([14usize, 0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC trigger X on Timer Y Reset"]
        #[inline(always)]
        pub const fn adctrst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 22usize + ([0usize, 5usize, 9usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ADC trigger X on Timer Y Reset"]
        #[inline(always)]
        pub fn set_adctrst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 22usize + ([0usize, 5usize, 9usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "adcmc",
                    &[
                        self.adcmc(0usize),
                        self.adcmc(1usize),
                        self.adcmc(2usize),
                        self.adcmc(3usize),
                    ],
                )
                .field("adcmper", &self.adcmper())
                .field(
                    "adceev",
                    &[
                        self.adceev(0usize),
                        self.adceev(1usize),
                        self.adceev(2usize),
                        self.adceev(3usize),
                        self.adceev(4usize),
                    ],
                )
                .field(
                    "adctc2",
                    &[
                        self.adctc2(0usize),
                        self.adctc2(1usize),
                        self.adctc2(2usize),
                        self.adctc2(3usize),
                        self.adctc2(4usize),
                        self.adctc2(5usize),
                    ],
                )
                .field(
                    "adctc4",
                    &[
                        self.adctc4(0usize),
                        self.adctc4(1usize),
                        self.adctc4(2usize),
                        self.adctc4(3usize),
                        self.adctc4(4usize),
                    ],
                )
                .field(
                    "adctper",
                    &[
                        self.adctper(0usize),
                        self.adctper(1usize),
                        self.adctper(2usize),
                        self.adctper(3usize),
                        self.adctper(4usize),
                    ],
                )
                .field("adctc3", &[self.adctc3(0usize), self.adctc3(1usize)])
                .field(
                    "adctrst",
                    &[self.adctrst(0usize), self.adctrst(1usize), self.adctrst(2usize)],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimAdc2r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimAdc2r {
                adcmc: [bool; 4usize],
                adcmper: bool,
                adceev: [bool; 5usize],
                adctc2: [bool; 6usize],
                adctc4: [bool; 5usize],
                adctper: [bool; 5usize],
                adctc3: [bool; 2usize],
                adctrst: [bool; 3usize],
            }
            let proxy = HrtimAdc2r {
                adcmc: [
                    self.adcmc(0usize),
                    self.adcmc(1usize),
                    self.adcmc(2usize),
                    self.adcmc(3usize),
                ],
                adcmper: self.adcmper(),
                adceev: [
                    self.adceev(0usize),
                    self.adceev(1usize),
                    self.adceev(2usize),
                    self.adceev(3usize),
                    self.adceev(4usize),
                ],
                adctc2: [
                    self.adctc2(0usize),
                    self.adctc2(1usize),
                    self.adctc2(2usize),
                    self.adctc2(3usize),
                    self.adctc2(4usize),
                    self.adctc2(5usize),
                ],
                adctc4: [
                    self.adctc4(0usize),
                    self.adctc4(1usize),
                    self.adctc4(2usize),
                    self.adctc4(3usize),
                    self.adctc4(4usize),
                ],
                adctper: [
                    self.adctper(0usize),
                    self.adctper(1usize),
                    self.adctper(2usize),
                    self.adctper(3usize),
                    self.adctper(4usize),
                ],
                adctc3: [self.adctc3(0usize), self.adctc3(1usize)],
                adctrst: [self.adctrst(0usize), self.adctrst(1usize), self.adctrst(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Data Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdmadr(pub u32);
    impl HrtimBdmadr {
        #[doc = "Burst DMA Data register"]
        #[inline(always)]
        pub const fn bdmadr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Burst DMA Data register"]
        #[inline(always)]
        pub fn set_bdmadr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
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
            #[derive(defmt :: Format)]
            struct HrtimBdmadr {
                bdmadr: u32,
            }
            let proxy = HrtimBdmadr { bdmadr: self.bdmadr() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdmupr(pub u32);
    impl HrtimBdmupr {
        #[doc = "MCR register update enable"]
        #[inline(always)]
        pub const fn mcr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MCR register update enable"]
        #[inline(always)]
        pub fn set_mcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MICR register update enable"]
        #[inline(always)]
        pub const fn micr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MICR register update enable"]
        #[inline(always)]
        pub fn set_micr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MDIER register update enable"]
        #[inline(always)]
        pub const fn mdier(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MDIER register update enable"]
        #[inline(always)]
        pub fn set_mdier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MCNT register update enable"]
        #[inline(always)]
        pub const fn mcnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MCNT register update enable"]
        #[inline(always)]
        pub fn set_mcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MPER register update enable"]
        #[inline(always)]
        pub const fn mper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MPER register update enable"]
        #[inline(always)]
        pub fn set_mper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MREP register update enable"]
        #[inline(always)]
        pub const fn mrep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MREP register update enable"]
        #[inline(always)]
        pub fn set_mrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MCMP register X update enable"]
        #[inline(always)]
        pub const fn mcmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "MCMP register X update enable"]
        #[inline(always)]
        pub fn set_mcmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "mcmp",
                    &[
                        self.mcmp(0usize),
                        self.mcmp(1usize),
                        self.mcmp(2usize),
                        self.mcmp(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdmupr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimBdmupr {
                mcr: bool,
                micr: bool,
                mdier: bool,
                mcnt: bool,
                mper: bool,
                mrep: bool,
                mcmp: [bool; 4usize],
            }
            let proxy = HrtimBdmupr {
                mcr: self.mcr(),
                micr: self.micr(),
                mdier: self.mdier(),
                mcnt: self.mcnt(),
                mper: self.mper(),
                mrep: self.mrep(),
                mcmp: [
                    self.mcmp(0usize),
                    self.mcmp(1usize),
                    self.mcmp(2usize),
                    self.mcmp(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst DMA Master timer update Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBdtupr(pub u32);
    impl HrtimBdtupr {
        #[doc = "CR register update enable"]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CR register update enable"]
        #[inline(always)]
        pub fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ICR register update enable"]
        #[inline(always)]
        pub const fn icr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ICR register update enable"]
        #[inline(always)]
        pub fn set_icr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DIER register update enable"]
        #[inline(always)]
        pub const fn dier(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DIER register update enable"]
        #[inline(always)]
        pub fn set_dier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CNT register update enable"]
        #[inline(always)]
        pub const fn cnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CNT register update enable"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PER register update enable"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PER register update enable"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "REP register update enable"]
        #[inline(always)]
        pub const fn rep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "REP register update enable"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMP register X update enable"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "CMP register X update enable"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "cmp",
                    &[self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBdtupr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimBdtupr {
                cr: bool,
                icr: bool,
                dier: bool,
                cnt: bool,
                per: bool,
                rep: bool,
                cmp: [bool; 4usize],
            }
            let proxy = HrtimBdtupr {
                cr: self.cr(),
                icr: self.icr(),
                dier: self.dier(),
                cnt: self.cnt(),
                per: self.per(),
                rep: self.rep(),
                cmp: [self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Compare Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmcmpr(pub u32);
    impl HrtimBmcmpr {
        #[doc = "Burst mode compare value"]
        #[inline(always)]
        pub const fn bmcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Burst mode compare value"]
        #[inline(always)]
        pub fn set_bmcmp(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct HrtimBmcmpr {
                bmcmp: u16,
            }
            let proxy = HrtimBmcmpr { bmcmp: self.bmcmp() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmcr(pub u32);
    impl HrtimBmcr {
        #[doc = "Burst Mode Enable"]
        #[inline(always)]
        pub const fn bme(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Enable"]
        #[inline(always)]
        pub fn set_bme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Burst Mode Operating Mode"]
        #[inline(always)]
        pub const fn bmom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Operating Mode"]
        #[inline(always)]
        pub fn set_bmom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Burst Mode Clock source"]
        #[inline(always)]
        pub const fn bmclk(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Mode Clock source"]
        #[inline(always)]
        pub fn set_bmclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "Burst Mode Prescaler"]
        #[inline(always)]
        pub const fn bmprsc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Mode Prescaler"]
        #[inline(always)]
        pub fn set_bmprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Burst Mode Preload Enable"]
        #[inline(always)]
        pub const fn bmpren(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Preload Enable"]
        #[inline(always)]
        pub fn set_bmpren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Master Timer Burst Mode"]
        #[inline(always)]
        pub const fn mtbm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer Burst Mode"]
        #[inline(always)]
        pub fn set_mtbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer X Burst Mode"]
        #[inline(always)]
        pub const fn tbm(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Burst Mode"]
        #[inline(always)]
        pub fn set_tbm(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[inline(always)]
        pub const fn bmstat(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bmstat(&mut self, val: bool) {
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
                .field(
                    "tbm",
                    &[
                        self.tbm(0usize),
                        self.tbm(1usize),
                        self.tbm(2usize),
                        self.tbm(3usize),
                        self.tbm(4usize),
                    ],
                )
                .field("bmstat", &self.bmstat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimBmcr {
                bme: bool,
                bmom: bool,
                bmclk: u8,
                bmprsc: u8,
                bmpren: bool,
                mtbm: bool,
                tbm: [bool; 5usize],
                bmstat: bool,
            }
            let proxy = HrtimBmcr {
                bme: self.bme(),
                bmom: self.bmom(),
                bmclk: self.bmclk(),
                bmprsc: self.bmprsc(),
                bmpren: self.bmpren(),
                mtbm: self.mtbm(),
                tbm: [
                    self.tbm(0usize),
                    self.tbm(1usize),
                    self.tbm(2usize),
                    self.tbm(3usize),
                    self.tbm(4usize),
                ],
                bmstat: self.bmstat(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmper(pub u32);
    impl HrtimBmper {
        #[doc = "Burst mode period value"]
        #[inline(always)]
        pub const fn bmper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Burst mode period value"]
        #[inline(always)]
        pub fn set_bmper(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct HrtimBmper {
                bmper: u16,
            }
            let proxy = HrtimBmper { bmper: self.bmper() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Burst Mode Trigger Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimBmtrgr(pub u32);
    impl HrtimBmtrgr {
        #[doc = "Software start"]
        #[inline(always)]
        pub const fn sw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software start"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master reset or roll-over"]
        #[inline(always)]
        pub const fn mstrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master reset or roll-over"]
        #[inline(always)]
        pub fn set_mstrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master repetition"]
        #[inline(always)]
        pub const fn mstrep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master repetition"]
        #[inline(always)]
        pub fn set_mstrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub const fn mstcmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub fn set_mstcmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X reset or roll-over"]
        #[inline(always)]
        pub const fn trst(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 7usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X reset or roll-over"]
        #[inline(always)]
        pub fn set_trst(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 7usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X repetition"]
        #[inline(always)]
        pub const fn trep(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 8usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X repetition"]
        #[inline(always)]
        pub fn set_trep(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 8usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X compare 1 event"]
        #[inline(always)]
        pub const fn tcmp1(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 9usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare 1 event"]
        #[inline(always)]
        pub fn set_tcmp1(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 9usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X compare 2 event"]
        #[inline(always)]
        pub const fn tcmp2(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 10usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare 2 event"]
        #[inline(always)]
        pub fn set_tcmp2(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 10usize + ([0usize, 4usize, 8usize, 12usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "mstcmp",
                    &[
                        self.mstcmp(0usize),
                        self.mstcmp(1usize),
                        self.mstcmp(2usize),
                        self.mstcmp(3usize),
                    ],
                )
                .field(
                    "trst",
                    &[
                        self.trst(0usize),
                        self.trst(1usize),
                        self.trst(2usize),
                        self.trst(3usize),
                        self.trst(4usize),
                    ],
                )
                .field(
                    "trep",
                    &[
                        self.trep(0usize),
                        self.trep(1usize),
                        self.trep(2usize),
                        self.trep(3usize),
                        self.trep(4usize),
                    ],
                )
                .field(
                    "tcmp1",
                    &[
                        self.tcmp1(0usize),
                        self.tcmp1(1usize),
                        self.tcmp1(2usize),
                        self.tcmp1(3usize),
                        self.tcmp1(4usize),
                    ],
                )
                .field(
                    "tcmp2",
                    &[
                        self.tcmp2(0usize),
                        self.tcmp2(1usize),
                        self.tcmp2(2usize),
                        self.tcmp2(3usize),
                        self.tcmp2(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimBmtrgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimBmtrgr {
                sw: bool,
                mstrst: bool,
                mstrep: bool,
                mstcmp: [bool; 4usize],
                trst: [bool; 5usize],
                trep: [bool; 5usize],
                tcmp1: [bool; 5usize],
                tcmp2: [bool; 5usize],
            }
            let proxy = HrtimBmtrgr {
                sw: self.sw(),
                mstrst: self.mstrst(),
                mstrep: self.mstrep(),
                mstcmp: [
                    self.mstcmp(0usize),
                    self.mstcmp(1usize),
                    self.mstcmp(2usize),
                    self.mstcmp(3usize),
                ],
                trst: [
                    self.trst(0usize),
                    self.trst(1usize),
                    self.trst(2usize),
                    self.trst(3usize),
                    self.trst(4usize),
                ],
                trep: [
                    self.trep(0usize),
                    self.trep(1usize),
                    self.trep(2usize),
                    self.trep(3usize),
                    self.trep(4usize),
                ],
                tcmp1: [
                    self.tcmp1(0usize),
                    self.tcmp1(1usize),
                    self.tcmp1(2usize),
                    self.tcmp1(3usize),
                    self.tcmp1(4usize),
                ],
                tcmp2: [
                    self.tcmp2(0usize),
                    self.tcmp2(1usize),
                    self.tcmp2(2usize),
                    self.tcmp2(3usize),
                    self.tcmp2(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimCr1(pub u32);
    impl HrtimCr1 {
        #[doc = "Master Update Disable"]
        #[inline(always)]
        pub const fn mudis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update Disable"]
        #[inline(always)]
        pub fn set_mudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X Update Disable"]
        #[inline(always)]
        pub const fn tudis(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Update Disable"]
        #[inline(always)]
        pub fn set_tudis(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ADC Trigger X Update Source"]
        #[inline(always)]
        pub const fn adusrc(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "ADC Trigger X Update Source"]
        #[inline(always)]
        pub fn set_adusrc(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 16usize + n * 2usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
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
                .field(
                    "tudis",
                    &[
                        self.tudis(0usize),
                        self.tudis(1usize),
                        self.tudis(2usize),
                        self.tudis(3usize),
                        self.tudis(4usize),
                    ],
                )
                .field(
                    "adusrc",
                    &[
                        self.adusrc(0usize),
                        self.adusrc(1usize),
                        self.adusrc(2usize),
                        self.adusrc(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimCr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimCr1 {
                mudis: bool,
                tudis: [bool; 5usize],
                adusrc: [u8; 4usize],
            }
            let proxy = HrtimCr1 {
                mudis: self.mudis(),
                tudis: [
                    self.tudis(0usize),
                    self.tudis(1usize),
                    self.tudis(2usize),
                    self.tudis(3usize),
                    self.tudis(4usize),
                ],
                adusrc: [
                    self.adusrc(0usize),
                    self.adusrc(1usize),
                    self.adusrc(2usize),
                    self.adusrc(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimCr2(pub u32);
    impl HrtimCr2 {
        #[doc = "Master Timer Software Update"]
        #[inline(always)]
        pub const fn mswu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer Software Update"]
        #[inline(always)]
        pub fn set_mswu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X Software Update"]
        #[inline(always)]
        pub const fn tswu(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Software Update"]
        #[inline(always)]
        pub fn set_tswu(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Counter Software Reset"]
        #[inline(always)]
        pub const fn mrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Master Counter Software Reset"]
        #[inline(always)]
        pub fn set_mrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer X Counter Software Reset"]
        #[inline(always)]
        pub const fn trst(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Counter Software Reset"]
        #[inline(always)]
        pub fn set_trst(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "tswu",
                    &[
                        self.tswu(0usize),
                        self.tswu(1usize),
                        self.tswu(2usize),
                        self.tswu(3usize),
                        self.tswu(4usize),
                    ],
                )
                .field("mrst", &self.mrst())
                .field(
                    "trst",
                    &[
                        self.trst(0usize),
                        self.trst(1usize),
                        self.trst(2usize),
                        self.trst(3usize),
                        self.trst(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimCr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimCr2 {
                mswu: bool,
                tswu: [bool; 5usize],
                mrst: bool,
                trst: [bool; 5usize],
            }
            let proxy = HrtimCr2 {
                mswu: self.mswu(),
                tswu: [
                    self.tswu(0usize),
                    self.tswu(1usize),
                    self.tswu(2usize),
                    self.tswu(3usize),
                    self.tswu(4usize),
                ],
                mrst: self.mrst(),
                trst: [
                    self.trst(0usize),
                    self.trst(1usize),
                    self.trst(2usize),
                    self.trst(3usize),
                    self.trst(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: DLL Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimDllcr(pub u32);
    impl HrtimDllcr {
        #[doc = "DLL Calibration Start"]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Calibration Start"]
        #[inline(always)]
        pub fn set_cal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DLL Calibration Enable"]
        #[inline(always)]
        pub const fn calen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Calibration Enable"]
        #[inline(always)]
        pub fn set_calen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DLL Calibration Rate"]
        #[inline(always)]
        pub const fn calrte(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "DLL Calibration Rate"]
        #[inline(always)]
        pub fn set_calrte(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct HrtimDllcr {
                cal: bool,
                calen: bool,
                calrte: u8,
            }
            let proxy = HrtimDllcr {
                cal: self.cal(),
                calen: self.calen(),
                calrte: self.calrte(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr1(pub u32);
    impl HrtimEecr1 {
        #[doc = "External Event X Source"]
        #[inline(always)]
        pub const fn eesrc(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "External Event X Source"]
        #[inline(always)]
        pub fn set_eesrc(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "External Event X Polarity"]
        #[inline(always)]
        pub const fn eepol(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 2usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X Polarity"]
        #[inline(always)]
        pub fn set_eepol(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 2usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X Sensitivity"]
        #[inline(always)]
        pub const fn eesns(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 3usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "External Event X Sensitivity"]
        #[inline(always)]
        pub fn set_eesns(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 3usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "External Event X Fast Mode"]
        #[inline(always)]
        pub const fn eefast(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 5usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "External Event X Fast Mode"]
        #[inline(always)]
        pub fn set_eefast(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 5usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
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
                .field(
                    "eesrc",
                    &[
                        self.eesrc(0usize),
                        self.eesrc(1usize),
                        self.eesrc(2usize),
                        self.eesrc(3usize),
                        self.eesrc(4usize),
                    ],
                )
                .field(
                    "eepol",
                    &[
                        self.eepol(0usize),
                        self.eepol(1usize),
                        self.eepol(2usize),
                        self.eepol(3usize),
                        self.eepol(4usize),
                    ],
                )
                .field(
                    "eesns",
                    &[
                        self.eesns(0usize),
                        self.eesns(1usize),
                        self.eesns(2usize),
                        self.eesns(3usize),
                        self.eesns(4usize),
                    ],
                )
                .field(
                    "eefast",
                    &[
                        self.eefast(0usize),
                        self.eefast(1usize),
                        self.eefast(2usize),
                        self.eefast(3usize),
                        self.eefast(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimEecr1 {
                eesrc: [u8; 5usize],
                eepol: [bool; 5usize],
                eesns: [u8; 5usize],
                eefast: [u8; 5usize],
            }
            let proxy = HrtimEecr1 {
                eesrc: [
                    self.eesrc(0usize),
                    self.eesrc(1usize),
                    self.eesrc(2usize),
                    self.eesrc(3usize),
                    self.eesrc(4usize),
                ],
                eepol: [
                    self.eepol(0usize),
                    self.eepol(1usize),
                    self.eepol(2usize),
                    self.eepol(3usize),
                    self.eepol(4usize),
                ],
                eesns: [
                    self.eesns(0usize),
                    self.eesns(1usize),
                    self.eesns(2usize),
                    self.eesns(3usize),
                    self.eesns(4usize),
                ],
                eefast: [
                    self.eefast(0usize),
                    self.eefast(1usize),
                    self.eefast(2usize),
                    self.eefast(3usize),
                    self.eefast(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr2(pub u32);
    impl HrtimEecr2 {
        #[doc = "External Event X Source"]
        #[inline(always)]
        pub const fn eesrc(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "External Event X Source"]
        #[inline(always)]
        pub fn set_eesrc(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "External Event X Polarity"]
        #[inline(always)]
        pub const fn eepol(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 2usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X Polarity"]
        #[inline(always)]
        pub fn set_eepol(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 2usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X Sensitivity"]
        #[inline(always)]
        pub const fn eesns(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 3usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "External Event X Sensitivity"]
        #[inline(always)]
        pub fn set_eesns(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 3usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
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
                .field(
                    "eesrc",
                    &[
                        self.eesrc(0usize),
                        self.eesrc(1usize),
                        self.eesrc(2usize),
                        self.eesrc(3usize),
                        self.eesrc(4usize),
                    ],
                )
                .field(
                    "eepol",
                    &[
                        self.eepol(0usize),
                        self.eepol(1usize),
                        self.eepol(2usize),
                        self.eepol(3usize),
                        self.eepol(4usize),
                    ],
                )
                .field(
                    "eesns",
                    &[
                        self.eesns(0usize),
                        self.eesns(1usize),
                        self.eesns(2usize),
                        self.eesns(3usize),
                        self.eesns(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimEecr2 {
                eesrc: [u8; 5usize],
                eepol: [bool; 5usize],
                eesns: [u8; 5usize],
            }
            let proxy = HrtimEecr2 {
                eesrc: [
                    self.eesrc(0usize),
                    self.eesrc(1usize),
                    self.eesrc(2usize),
                    self.eesrc(3usize),
                    self.eesrc(4usize),
                ],
                eepol: [
                    self.eepol(0usize),
                    self.eepol(1usize),
                    self.eepol(2usize),
                    self.eepol(3usize),
                    self.eepol(4usize),
                ],
                eesns: [
                    self.eesns(0usize),
                    self.eesns(1usize),
                    self.eesns(2usize),
                    self.eesns(3usize),
                    self.eesns(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: External Events Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimEecr3(pub u32);
    impl HrtimEecr3 {
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub const fn eef(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub fn set_eef(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 6usize, 12usize, 18usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
        #[doc = "External Event Sampling Clock Division"]
        #[inline(always)]
        pub const fn eevsd(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "External Event Sampling Clock Division"]
        #[inline(always)]
        pub fn set_eevsd(&mut self, val: u8) {
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
                .field(
                    "eef",
                    &[
                        self.eef(0usize),
                        self.eef(1usize),
                        self.eef(2usize),
                        self.eef(3usize),
                        self.eef(4usize),
                    ],
                )
                .field("eevsd", &self.eevsd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimEecr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimEecr3 {
                eef: [u8; 5usize],
                eevsd: u8,
            }
            let proxy = HrtimEecr3 {
                eef: [
                    self.eef(0usize),
                    self.eef(1usize),
                    self.eef(2usize),
                    self.eef(3usize),
                    self.eef(4usize),
                ],
                eevsd: self.eevsd(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Fault Input Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr1(pub u32);
    impl HrtimFltinr1 {
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub const fn flte(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub fn set_flte(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X polarity"]
        #[inline(always)]
        pub const fn fltp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X polarity"]
        #[inline(always)]
        pub fn set_fltp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X source"]
        #[inline(always)]
        pub const fn fltsrc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X source"]
        #[inline(always)]
        pub fn set_fltsrc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X filter"]
        #[inline(always)]
        pub const fn fltf(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Fault X filter"]
        #[inline(always)]
        pub fn set_fltf(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "Fault X Lock"]
        #[inline(always)]
        pub const fn fltlck(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 7usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X Lock"]
        #[inline(always)]
        pub fn set_fltlck(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 7usize + ([0usize, 8usize, 16usize, 24usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "flte",
                    &[
                        self.flte(0usize),
                        self.flte(1usize),
                        self.flte(2usize),
                        self.flte(3usize),
                    ],
                )
                .field(
                    "fltp",
                    &[
                        self.fltp(0usize),
                        self.fltp(1usize),
                        self.fltp(2usize),
                        self.fltp(3usize),
                    ],
                )
                .field(
                    "fltsrc",
                    &[
                        self.fltsrc(0usize),
                        self.fltsrc(1usize),
                        self.fltsrc(2usize),
                        self.fltsrc(3usize),
                    ],
                )
                .field(
                    "fltf",
                    &[
                        self.fltf(0usize),
                        self.fltf(1usize),
                        self.fltf(2usize),
                        self.fltf(3usize),
                    ],
                )
                .field(
                    "fltlck",
                    &[
                        self.fltlck(0usize),
                        self.fltlck(1usize),
                        self.fltlck(2usize),
                        self.fltlck(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimFltinr1 {
                flte: [bool; 4usize],
                fltp: [bool; 4usize],
                fltsrc: [bool; 4usize],
                fltf: [u8; 4usize],
                fltlck: [bool; 4usize],
            }
            let proxy = HrtimFltinr1 {
                flte: [
                    self.flte(0usize),
                    self.flte(1usize),
                    self.flte(2usize),
                    self.flte(3usize),
                ],
                fltp: [
                    self.fltp(0usize),
                    self.fltp(1usize),
                    self.fltp(2usize),
                    self.fltp(3usize),
                ],
                fltsrc: [
                    self.fltsrc(0usize),
                    self.fltsrc(1usize),
                    self.fltsrc(2usize),
                    self.fltsrc(3usize),
                ],
                fltf: [
                    self.fltf(0usize),
                    self.fltf(1usize),
                    self.fltf(2usize),
                    self.fltf(3usize),
                ],
                fltlck: [
                    self.fltlck(0usize),
                    self.fltlck(1usize),
                    self.fltlck(2usize),
                    self.fltlck(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Fault Input Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimFltinr2(pub u32);
    impl HrtimFltinr2 {
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub const fn flte(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub fn set_flte(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X polarity"]
        #[inline(always)]
        pub const fn fltp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X polarity"]
        #[inline(always)]
        pub fn set_fltp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X source"]
        #[inline(always)]
        pub const fn fltsrc(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X source"]
        #[inline(always)]
        pub fn set_fltsrc(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault X filter"]
        #[inline(always)]
        pub const fn fltf(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 3usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Fault X filter"]
        #[inline(always)]
        pub fn set_fltf(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 3usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "Fault X Lock"]
        #[inline(always)]
        pub const fn fltlck(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + ([0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X Lock"]
        #[inline(always)]
        pub fn set_fltlck(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + ([0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault Sampling clock division"]
        #[inline(always)]
        pub const fn fltsd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Fault Sampling clock division"]
        #[inline(always)]
        pub fn set_fltsd(&mut self, val: u8) {
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
                .field("flte", &[self.flte(0usize)])
                .field("fltp", &[self.fltp(0usize)])
                .field("fltsrc", &[self.fltsrc(0usize)])
                .field("fltf", &[self.fltf(0usize)])
                .field("fltlck", &[self.fltlck(0usize)])
                .field("fltsd", &self.fltsd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimFltinr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimFltinr2 {
                flte: [bool; 1usize],
                fltp: [bool; 1usize],
                fltsrc: [bool; 1usize],
                fltf: [u8; 1usize],
                fltlck: [bool; 1usize],
                fltsd: u8,
            }
            let proxy = HrtimFltinr2 {
                flte: [self.flte(0usize)],
                fltp: [self.fltp(0usize)],
                fltsrc: [self.fltsrc(0usize)],
                fltf: [self.fltf(0usize)],
                fltlck: [self.fltlck(0usize)],
                fltsd: self.fltsd(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIcr(pub u32);
    impl HrtimIcr {
        #[doc = "Fault X Interrupt Flag Clear"]
        #[inline(always)]
        pub const fn flt(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X Interrupt Flag Clear"]
        #[inline(always)]
        pub fn set_flt(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "System Fault Interrupt Flag Clear"]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System Fault Interrupt Flag Clear"]
        #[inline(always)]
        pub fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DLL Ready Interrupt Flag Clear"]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt Flag Clear"]
        #[inline(always)]
        pub fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst Mode Period Interrupt Flag Clear"]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Period Interrupt Flag Clear"]
        #[inline(always)]
        pub fn set_bmper(&mut self, val: bool) {
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
                .field(
                    "flt",
                    &[
                        self.flt(0usize),
                        self.flt(1usize),
                        self.flt(2usize),
                        self.flt(3usize),
                        self.flt(4usize),
                    ],
                )
                .field("sysflt", &self.sysflt())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimIcr {
                flt: [bool; 5usize],
                sysflt: bool,
                dllrdy: bool,
                bmper: bool,
            }
            let proxy = HrtimIcr {
                flt: [
                    self.flt(0usize),
                    self.flt(1usize),
                    self.flt(2usize),
                    self.flt(3usize),
                    self.flt(4usize),
                ],
                sysflt: self.sysflt(),
                dllrdy: self.dllrdy(),
                bmper: self.bmper(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIer(pub u32);
    impl HrtimIer {
        #[doc = "Fault X Interrupt Flag Enable"]
        #[inline(always)]
        pub const fn flt(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X Interrupt Flag Enable"]
        #[inline(always)]
        pub fn set_flt(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "System Fault Interrupt Flag Enable"]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System Fault Interrupt Flag Enable"]
        #[inline(always)]
        pub fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DLL Ready Interrupt Flag Enable"]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt Flag Enable"]
        #[inline(always)]
        pub fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst Mode Period Interrupt Flag Enable"]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Period Interrupt Flag Enable"]
        #[inline(always)]
        pub fn set_bmper(&mut self, val: bool) {
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
                .field(
                    "flt",
                    &[
                        self.flt(0usize),
                        self.flt(1usize),
                        self.flt(2usize),
                        self.flt(3usize),
                        self.flt(4usize),
                    ],
                )
                .field("sysflt", &self.sysflt())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIer {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimIer {
                flt: [bool; 5usize],
                sysflt: bool,
                dllrdy: bool,
                bmper: bool,
            }
            let proxy = HrtimIer {
                flt: [
                    self.flt(0usize),
                    self.flt(1usize),
                    self.flt(2usize),
                    self.flt(3usize),
                    self.flt(4usize),
                ],
                sysflt: self.sysflt(),
                dllrdy: self.dllrdy(),
                bmper: self.bmper(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimIsr(pub u32);
    impl HrtimIsr {
        #[doc = "Fault X Interrupt Flag"]
        #[inline(always)]
        pub const fn flt(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X Interrupt Flag"]
        #[inline(always)]
        pub fn set_flt(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "System Fault Interrupt Flag"]
        #[inline(always)]
        pub const fn sysflt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "System Fault Interrupt Flag"]
        #[inline(always)]
        pub fn set_sysflt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DLL Ready Interrupt Flag"]
        #[inline(always)]
        pub const fn dllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_dllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Burst Mode Period Interrupt Flag"]
        #[inline(always)]
        pub const fn bmper(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Mode Period Interrupt Flag"]
        #[inline(always)]
        pub fn set_bmper(&mut self, val: bool) {
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
                .field(
                    "flt",
                    &[
                        self.flt(0usize),
                        self.flt(1usize),
                        self.flt(2usize),
                        self.flt(3usize),
                        self.flt(4usize),
                    ],
                )
                .field("sysflt", &self.sysflt())
                .field("dllrdy", &self.dllrdy())
                .field("bmper", &self.bmper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimIsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimIsr {
                flt: [bool; 5usize],
                sysflt: bool,
                dllrdy: bool,
                bmper: bool,
            }
            let proxy = HrtimIsr {
                flt: [
                    self.flt(0usize),
                    self.flt(1usize),
                    self.flt(2usize),
                    self.flt(3usize),
                    self.flt(4usize),
                ],
                sysflt: self.sysflt(),
                dllrdy: self.dllrdy(),
                bmper: self.bmper(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Output Disable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOdisr(pub u32);
    impl HrtimOdisr {
        #[doc = "Timer X Output Disable"]
        #[inline(always)]
        pub const fn t1odis(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Output Disable"]
        #[inline(always)]
        pub fn set_t1odis(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X Complementary Output Disable"]
        #[inline(always)]
        pub const fn t2odis(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Complementary Output Disable"]
        #[inline(always)]
        pub fn set_t2odis(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "t1odis",
                    &[
                        self.t1odis(0usize),
                        self.t1odis(1usize),
                        self.t1odis(2usize),
                        self.t1odis(3usize),
                        self.t1odis(4usize),
                    ],
                )
                .field(
                    "t2odis",
                    &[
                        self.t2odis(0usize),
                        self.t2odis(1usize),
                        self.t2odis(2usize),
                        self.t2odis(3usize),
                        self.t2odis(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOdisr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimOdisr {
                t1odis: [bool; 5usize],
                t2odis: [bool; 5usize],
            }
            let proxy = HrtimOdisr {
                t1odis: [
                    self.t1odis(0usize),
                    self.t1odis(1usize),
                    self.t1odis(2usize),
                    self.t1odis(3usize),
                    self.t1odis(4usize),
                ],
                t2odis: [
                    self.t2odis(0usize),
                    self.t2odis(1usize),
                    self.t2odis(2usize),
                    self.t2odis(3usize),
                    self.t2odis(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Output Disable Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOdsr(pub u32);
    impl HrtimOdsr {
        #[doc = "Timer X Output Disable Status"]
        #[inline(always)]
        pub const fn t1odis(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Output Disable Status"]
        #[inline(always)]
        pub fn set_t1odis(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X Complementary Output Disable Status"]
        #[inline(always)]
        pub const fn t2odis(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Complementary Output Disable Status"]
        #[inline(always)]
        pub fn set_t2odis(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "t1odis",
                    &[
                        self.t1odis(0usize),
                        self.t1odis(1usize),
                        self.t1odis(2usize),
                        self.t1odis(3usize),
                        self.t1odis(4usize),
                    ],
                )
                .field(
                    "t2odis",
                    &[
                        self.t2odis(0usize),
                        self.t2odis(1usize),
                        self.t2odis(2usize),
                        self.t2odis(3usize),
                        self.t2odis(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOdsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimOdsr {
                t1odis: [bool; 5usize],
                t2odis: [bool; 5usize],
            }
            let proxy = HrtimOdsr {
                t1odis: [
                    self.t1odis(0usize),
                    self.t1odis(1usize),
                    self.t1odis(2usize),
                    self.t1odis(3usize),
                    self.t1odis(4usize),
                ],
                t2odis: [
                    self.t2odis(0usize),
                    self.t2odis(1usize),
                    self.t2odis(2usize),
                    self.t2odis(3usize),
                    self.t2odis(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "High Resolution Timer: Output Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrtimOenr(pub u32);
    impl HrtimOenr {
        #[doc = "Timer X Output Enable"]
        #[inline(always)]
        pub const fn t1oen(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Output Enable"]
        #[inline(always)]
        pub fn set_t1oen(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X Complementary Output Enable"]
        #[inline(always)]
        pub const fn t2oen(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Complementary Output Enable"]
        #[inline(always)]
        pub fn set_t2oen(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 1usize + ([0usize, 2usize, 4usize, 6usize, 8usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "t1oen",
                    &[
                        self.t1oen(0usize),
                        self.t1oen(1usize),
                        self.t1oen(2usize),
                        self.t1oen(3usize),
                        self.t1oen(4usize),
                    ],
                )
                .field(
                    "t2oen",
                    &[
                        self.t2oen(0usize),
                        self.t2oen(1usize),
                        self.t2oen(2usize),
                        self.t2oen(3usize),
                        self.t2oen(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HrtimOenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HrtimOenr {
                t1oen: [bool; 5usize],
                t2oen: [bool; 5usize],
            }
            let proxy = HrtimOenr {
                t1oen: [
                    self.t1oen(0usize),
                    self.t1oen(1usize),
                    self.t1oen(2usize),
                    self.t1oen(3usize),
                    self.t1oen(4usize),
                ],
                t2oen: [
                    self.t2oen(0usize),
                    self.t2oen(1usize),
                    self.t2oen(2usize),
                    self.t2oen(3usize),
                    self.t2oen(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcmpx(pub u32);
    impl Mcmpx {
        #[doc = "Master Timer Compare X value"]
        #[inline(always)]
        pub const fn mcmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master Timer Compare X value"]
        #[inline(always)]
        pub fn set_mcmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mcmpx {
        #[inline(always)]
        fn default() -> Mcmpx {
            Mcmpx(0)
        }
    }
    impl core::fmt::Debug for Mcmpx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcmpx").field("mcmp", &self.mcmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcmpx {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Mcmpx {
                mcmp: u16,
            }
            let proxy = Mcmpx { mcmp: self.mcmp() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcntr(pub u32);
    impl Mcntr {
        #[doc = "Counter value"]
        #[inline(always)]
        pub const fn mcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter value"]
        #[inline(always)]
        pub fn set_mcnt(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct Mcntr {
                mcnt: u16,
            }
            let proxy = Mcntr { mcnt: self.mcnt() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "HRTIM Master Clock prescaler"]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HRTIM Master Clock prescaler"]
        #[inline(always)]
        pub fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Master Continuous mode"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master Continuous mode"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Master Re-triggerable mode"]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master Re-triggerable mode"]
        #[inline(always)]
        pub fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Synchronization input"]
        #[inline(always)]
        pub const fn syncin(&self) -> super::vals::Syncin {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Syncin::from_bits(val as u8)
        }
        #[doc = "Synchronization input"]
        #[inline(always)]
        pub fn set_syncin(&mut self, val: super::vals::Syncin) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Synchronization Resets Master"]
        #[inline(always)]
        pub const fn syncrstm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Resets Master"]
        #[inline(always)]
        pub fn set_syncrstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization Starts Master"]
        #[inline(always)]
        pub const fn syncstrtm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Starts Master"]
        #[inline(always)]
        pub fn set_syncstrtm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Synchronization output"]
        #[inline(always)]
        pub const fn syncout(&self) -> super::vals::Syncout {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Syncout::from_bits(val as u8)
        }
        #[doc = "Synchronization output"]
        #[inline(always)]
        pub fn set_syncout(&mut self, val: super::vals::Syncout) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Synchronization source"]
        #[inline(always)]
        pub const fn syncsrc(&self) -> super::vals::Syncsrc {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Syncsrc::from_bits(val as u8)
        }
        #[doc = "Synchronization source"]
        #[inline(always)]
        pub fn set_syncsrc(&mut self, val: super::vals::Syncsrc) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Master Counter enable"]
        #[inline(always)]
        pub const fn mcen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Master Counter enable"]
        #[inline(always)]
        pub fn set_mcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer X counter enable"]
        #[inline(always)]
        pub const fn tcen(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X counter enable"]
        #[inline(always)]
        pub fn set_tcen(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 17usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Master Timer Repetition update"]
        #[inline(always)]
        pub const fn mrepu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer Repetition update"]
        #[inline(always)]
        pub fn set_mrepu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Burst DMA Update"]
        #[inline(always)]
        pub const fn brstdma(&self) -> super::vals::Brstdma {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Brstdma::from_bits(val as u8)
        }
        #[doc = "Burst DMA Update"]
        #[inline(always)]
        pub fn set_brstdma(&mut self, val: super::vals::Brstdma) {
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
                .field("syncin", &self.syncin())
                .field("syncrstm", &self.syncrstm())
                .field("syncstrtm", &self.syncstrtm())
                .field("syncout", &self.syncout())
                .field("syncsrc", &self.syncsrc())
                .field("mcen", &self.mcen())
                .field(
                    "tcen",
                    &[
                        self.tcen(0usize),
                        self.tcen(1usize),
                        self.tcen(2usize),
                        self.tcen(3usize),
                        self.tcen(4usize),
                    ],
                )
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
            #[derive(defmt :: Format)]
            struct Mcr {
                ckpsc: u8,
                cont: bool,
                retrig: bool,
                half: bool,
                syncin: super::vals::Syncin,
                syncrstm: bool,
                syncstrtm: bool,
                syncout: super::vals::Syncout,
                syncsrc: super::vals::Syncsrc,
                mcen: bool,
                tcen: [bool; 5usize],
                dacsync: super::vals::Dacsync,
                preen: bool,
                mrepu: bool,
                brstdma: super::vals::Brstdma,
            }
            let proxy = Mcr {
                ckpsc: self.ckpsc(),
                cont: self.cont(),
                retrig: self.retrig(),
                half: self.half(),
                syncin: self.syncin(),
                syncrstm: self.syncrstm(),
                syncstrtm: self.syncstrtm(),
                syncout: self.syncout(),
                syncsrc: self.syncsrc(),
                mcen: self.mcen(),
                tcen: [
                    self.tcen(0usize),
                    self.tcen(1usize),
                    self.tcen(2usize),
                    self.tcen(3usize),
                    self.tcen(4usize),
                ],
                dacsync: self.dacsync(),
                preen: self.preen(),
                mrepu: self.mrepu(),
                brstdma: self.brstdma(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdier(pub u32);
    impl Mdier {
        #[doc = "Master Compare X Interrupt Enable"]
        #[inline(always)]
        pub const fn mcmpie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X Interrupt Enable"]
        #[inline(always)]
        pub fn set_mcmpie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition Interrupt Enable"]
        #[inline(always)]
        pub const fn mrepie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master Repetition Interrupt Enable"]
        #[inline(always)]
        pub fn set_mrepie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt Enable"]
        #[inline(always)]
        pub const fn syncie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input Interrupt Enable"]
        #[inline(always)]
        pub fn set_syncie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master Update Interrupt Enable"]
        #[inline(always)]
        pub const fn mupdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update Interrupt Enable"]
        #[inline(always)]
        pub fn set_mupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Master Compare X DMA request Enable"]
        #[inline(always)]
        pub const fn mcmpde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X DMA request Enable"]
        #[inline(always)]
        pub fn set_mcmpde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition DMA request Enable"]
        #[inline(always)]
        pub const fn mrepde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Master Repetition DMA request Enable"]
        #[inline(always)]
        pub fn set_mrepde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Sync Input DMA request Enable"]
        #[inline(always)]
        pub const fn syncde(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input DMA request Enable"]
        #[inline(always)]
        pub fn set_syncde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Master Update DMA request Enable"]
        #[inline(always)]
        pub const fn mupdde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update DMA request Enable"]
        #[inline(always)]
        pub fn set_mupdde(&mut self, val: bool) {
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
                .field(
                    "mcmpie",
                    &[
                        self.mcmpie(0usize),
                        self.mcmpie(1usize),
                        self.mcmpie(2usize),
                        self.mcmpie(3usize),
                    ],
                )
                .field("mrepie", &self.mrepie())
                .field("syncie", &self.syncie())
                .field("mupdie", &self.mupdie())
                .field(
                    "mcmpde",
                    &[
                        self.mcmpde(0usize),
                        self.mcmpde(1usize),
                        self.mcmpde(2usize),
                        self.mcmpde(3usize),
                    ],
                )
                .field("mrepde", &self.mrepde())
                .field("syncde", &self.syncde())
                .field("mupdde", &self.mupdde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mdier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Mdier {
                mcmpie: [bool; 4usize],
                mrepie: bool,
                syncie: bool,
                mupdie: bool,
                mcmpde: [bool; 4usize],
                mrepde: bool,
                syncde: bool,
                mupdde: bool,
            }
            let proxy = Mdier {
                mcmpie: [
                    self.mcmpie(0usize),
                    self.mcmpie(1usize),
                    self.mcmpie(2usize),
                    self.mcmpie(3usize),
                ],
                mrepie: self.mrepie(),
                syncie: self.syncie(),
                mupdie: self.mupdie(),
                mcmpde: [
                    self.mcmpde(0usize),
                    self.mcmpde(1usize),
                    self.mcmpde(2usize),
                    self.mcmpde(3usize),
                ],
                mrepde: self.mrepde(),
                syncde: self.syncde(),
                mupdde: self.mupdde(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Micr(pub u32);
    impl Micr {
        #[doc = "Master Compare X Interrupt flag clear"]
        #[inline(always)]
        pub const fn mcmpc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mcmpc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt flag clear"]
        #[inline(always)]
        pub const fn mrepc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mrepc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt flag clear"]
        #[inline(always)]
        pub const fn syncc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input Interrupt flag clear"]
        #[inline(always)]
        pub fn set_syncc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master update Interrupt flag clear"]
        #[inline(always)]
        pub const fn mupdc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master update Interrupt flag clear"]
        #[inline(always)]
        pub fn set_mupdc(&mut self, val: bool) {
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
                .field(
                    "mcmpc",
                    &[
                        self.mcmpc(0usize),
                        self.mcmpc(1usize),
                        self.mcmpc(2usize),
                        self.mcmpc(3usize),
                    ],
                )
                .field("mrepc", &self.mrepc())
                .field("syncc", &self.syncc())
                .field("mupdc", &self.mupdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Micr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Micr {
                mcmpc: [bool; 4usize],
                mrepc: bool,
                syncc: bool,
                mupdc: bool,
            }
            let proxy = Micr {
                mcmpc: [
                    self.mcmpc(0usize),
                    self.mcmpc(1usize),
                    self.mcmpc(2usize),
                    self.mcmpc(3usize),
                ],
                mrepc: self.mrepc(),
                syncc: self.syncc(),
                mupdc: self.mupdc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Master Compare X Interrupt Flag"]
        #[inline(always)]
        pub const fn mcmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X Interrupt Flag"]
        #[inline(always)]
        pub fn set_mcmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Repetition Interrupt Flag"]
        #[inline(always)]
        pub const fn mrep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master Repetition Interrupt Flag"]
        #[inline(always)]
        pub fn set_mrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Sync Input Interrupt Flag"]
        #[inline(always)]
        pub const fn sync(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Input Interrupt Flag"]
        #[inline(always)]
        pub fn set_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Master Update Interrupt Flag"]
        #[inline(always)]
        pub const fn mupd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Master Update Interrupt Flag"]
        #[inline(always)]
        pub fn set_mupd(&mut self, val: bool) {
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
                .field(
                    "mcmp",
                    &[
                        self.mcmp(0usize),
                        self.mcmp(1usize),
                        self.mcmp(2usize),
                        self.mcmp(3usize),
                    ],
                )
                .field("mrep", &self.mrep())
                .field("sync", &self.sync())
                .field("mupd", &self.mupd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Misr {
                mcmp: [bool; 4usize],
                mrep: bool,
                sync: bool,
                mupd: bool,
            }
            let proxy = Misr {
                mcmp: [
                    self.mcmp(0usize),
                    self.mcmp(1usize),
                    self.mcmp(2usize),
                    self.mcmp(3usize),
                ],
                mrep: self.mrep(),
                sync: self.sync(),
                mupd: self.mupd(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mper(pub u32);
    impl Mper {
        #[doc = "Master Timer Period value"]
        #[inline(always)]
        pub const fn mper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Master Timer Period value"]
        #[inline(always)]
        pub fn set_mper(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct Mper {
                mper: u16,
            }
            let proxy = Mper { mper: self.mper() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Timer Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrep(pub u32);
    impl Mrep {
        #[doc = "Master Timer Repetition counter value"]
        #[inline(always)]
        pub const fn mrep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Master Timer Repetition counter value"]
        #[inline(always)]
        pub fn set_mrep(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct Mrep {
                mrep: u8,
            }
            let proxy = Mrep { mrep: self.mrep() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Capture 2 Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxccr(pub u32);
    impl Timxccr {
        #[doc = "Software Capture"]
        #[inline(always)]
        pub const fn swcpt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Capture"]
        #[inline(always)]
        pub fn set_swcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update Capture"]
        #[inline(always)]
        pub const fn updcpt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update Capture"]
        #[inline(always)]
        pub fn set_updcpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Event X Capture"]
        #[inline(always)]
        pub const fn exevcpt(&self, n: usize) -> bool {
            assert!(n < 10usize);
            let offs = 2usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X Capture"]
        #[inline(always)]
        pub fn set_exevcpt(&mut self, n: usize, val: bool) {
            assert!(n < 10usize);
            let offs = 2usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X output Set"]
        #[inline(always)]
        pub const fn txset(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X output Set"]
        #[inline(always)]
        pub fn set_txset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer X output Reset"]
        #[inline(always)]
        pub const fn txrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X output Reset"]
        #[inline(always)]
        pub fn set_txrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer X Compare X"]
        #[inline(always)]
        pub const fn txcmp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X Compare X"]
        #[inline(always)]
        pub fn set_txcmp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer Y output Set"]
        #[inline(always)]
        pub const fn tyset(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Y output Set"]
        #[inline(always)]
        pub fn set_tyset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer Y output Reset"]
        #[inline(always)]
        pub const fn tyrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Y output Reset"]
        #[inline(always)]
        pub fn set_tyrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Timer Y Compare X"]
        #[inline(always)]
        pub const fn tycmp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 22usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer Y Compare X"]
        #[inline(always)]
        pub fn set_tycmp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 22usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer Z output Set"]
        #[inline(always)]
        pub const fn tzset(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Z output Set"]
        #[inline(always)]
        pub fn set_tzset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer Z output Reset"]
        #[inline(always)]
        pub const fn tzrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Z output Reset"]
        #[inline(always)]
        pub fn set_tzrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Timer Z Compare X"]
        #[inline(always)]
        pub const fn tzcmp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer Z Compare X"]
        #[inline(always)]
        pub fn set_tzcmp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer T output Set"]
        #[inline(always)]
        pub const fn ttset(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Timer T output Set"]
        #[inline(always)]
        pub fn set_ttset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Timer T output Reset"]
        #[inline(always)]
        pub const fn ttrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Timer T output Reset"]
        #[inline(always)]
        pub fn set_ttrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Timer T Compare X"]
        #[inline(always)]
        pub const fn ttcmp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 30usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer T Compare X"]
        #[inline(always)]
        pub fn set_ttcmp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 30usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "exevcpt",
                    &[
                        self.exevcpt(0usize),
                        self.exevcpt(1usize),
                        self.exevcpt(2usize),
                        self.exevcpt(3usize),
                        self.exevcpt(4usize),
                        self.exevcpt(5usize),
                        self.exevcpt(6usize),
                        self.exevcpt(7usize),
                        self.exevcpt(8usize),
                        self.exevcpt(9usize),
                    ],
                )
                .field("txset", &self.txset())
                .field("txrst", &self.txrst())
                .field("txcmp", &[self.txcmp(0usize), self.txcmp(1usize)])
                .field("tyset", &self.tyset())
                .field("tyrst", &self.tyrst())
                .field("tycmp", &[self.tycmp(0usize), self.tycmp(1usize)])
                .field("tzset", &self.tzset())
                .field("tzrst", &self.tzrst())
                .field("tzcmp", &[self.tzcmp(0usize), self.tzcmp(1usize)])
                .field("ttset", &self.ttset())
                .field("ttrst", &self.ttrst())
                .field("ttcmp", &[self.ttcmp(0usize), self.ttcmp(1usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxccr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxccr {
                swcpt: bool,
                updcpt: bool,
                exevcpt: [bool; 10usize],
                txset: bool,
                txrst: bool,
                txcmp: [bool; 2usize],
                tyset: bool,
                tyrst: bool,
                tycmp: [bool; 2usize],
                tzset: bool,
                tzrst: bool,
                tzcmp: [bool; 2usize],
                ttset: bool,
                ttrst: bool,
                ttcmp: [bool; 2usize],
            }
            let proxy = Timxccr {
                swcpt: self.swcpt(),
                updcpt: self.updcpt(),
                exevcpt: [
                    self.exevcpt(0usize),
                    self.exevcpt(1usize),
                    self.exevcpt(2usize),
                    self.exevcpt(3usize),
                    self.exevcpt(4usize),
                    self.exevcpt(5usize),
                    self.exevcpt(6usize),
                    self.exevcpt(7usize),
                    self.exevcpt(8usize),
                    self.exevcpt(9usize),
                ],
                txset: self.txset(),
                txrst: self.txrst(),
                txcmp: [self.txcmp(0usize), self.txcmp(1usize)],
                tyset: self.tyset(),
                tyrst: self.tyrst(),
                tycmp: [self.tycmp(0usize), self.tycmp(1usize)],
                tzset: self.tzset(),
                tzrst: self.tzrst(),
                tzcmp: [self.tzcmp(0usize), self.tzcmp(1usize)],
                ttset: self.ttset(),
                ttrst: self.ttrst(),
                ttcmp: [self.ttcmp(0usize), self.ttcmp(1usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Chopper Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxchp(pub u32);
    impl Timxchp {
        #[doc = "Timerx carrier frequency value"]
        #[inline(always)]
        pub const fn carfrq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Timerx carrier frequency value"]
        #[inline(always)]
        pub fn set_carfrq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Timerx chopper duty cycle value"]
        #[inline(always)]
        pub const fn cardty(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Timerx chopper duty cycle value"]
        #[inline(always)]
        pub fn set_cardty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Timerx start pulsewidth"]
        #[inline(always)]
        pub const fn strtpw(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "Timerx start pulsewidth"]
        #[inline(always)]
        pub fn set_strtpw(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct Timxchp {
                carfrq: u8,
                cardty: u8,
                strtpw: u8,
            }
            let proxy = Timxchp {
                carfrq: self.carfrq(),
                cardty: self.cardty(),
                strtpw: self.strtpw(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmp(pub u32);
    impl Timxcmp {
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct Timxcmp {
                cmp: u16,
            }
            let proxy = Timxcmp { cmp: self.cmp() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Compare X Compound Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcmpc(pub u32);
    impl Timxcmpc {
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub const fn cmp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Compare X value"]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Timerx Repetition value (aliased from HRTIM_REPx register)"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Timerx Repetition value (aliased from HRTIM_REPx register)"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct Timxcmpc {
                cmp: u16,
                rep: u8,
            }
            let proxy = Timxcmpc {
                cmp: self.cmp(),
                rep: self.rep(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcnt(pub u32);
    impl Timxcnt {
        #[doc = "Timerx Counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct Timxcnt {
                cnt: u16,
            }
            let proxy = Timxcnt { cnt: self.cnt() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Capture X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcpt(pub u32);
    impl Timxcpt {
        #[doc = "Timerx Capture X value"]
        #[inline(always)]
        pub const fn cpt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Capture X value"]
        #[inline(always)]
        pub fn set_cpt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            f.debug_struct("Timxcpt").field("cpt", &self.cpt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxcpt {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxcpt {
                cpt: u16,
            }
            let proxy = Timxcpt { cpt: self.cpt() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxcr(pub u32);
    impl Timxcr {
        #[doc = "HRTIM Timer x Clock prescaler"]
        #[inline(always)]
        pub const fn ckpsc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HRTIM Timer x Clock prescaler"]
        #[inline(always)]
        pub fn set_ckpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Continuous mode"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Re-triggerable mode"]
        #[inline(always)]
        pub const fn retrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Re-triggerable mode"]
        #[inline(always)]
        pub fn set_retrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub const fn half(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Half mode enable"]
        #[inline(always)]
        pub fn set_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Push-Pull mode enable"]
        #[inline(always)]
        pub const fn pshpll(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Push-Pull mode enable"]
        #[inline(always)]
        pub fn set_pshpll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Synchronization Resets Timer X"]
        #[inline(always)]
        pub const fn syncrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Resets Timer X"]
        #[inline(always)]
        pub fn set_syncrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Synchronization Starts Timer X"]
        #[inline(always)]
        pub const fn syncstrt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization Starts Timer X"]
        #[inline(always)]
        pub fn set_syncstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Delayed CMP2 mode"]
        #[inline(always)]
        pub const fn delcmp2(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "Delayed CMP2 mode"]
        #[inline(always)]
        pub fn set_delcmp2(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Delayed CMP4 mode"]
        #[inline(always)]
        pub const fn delcmp4(&self) -> super::vals::Delcmp {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Delcmp::from_bits(val as u8)
        }
        #[doc = "Delayed CMP4 mode"]
        #[inline(always)]
        pub fn set_delcmp4(&mut self, val: super::vals::Delcmp) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Timer X Repetition update"]
        #[inline(always)]
        pub const fn repu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X Repetition update"]
        #[inline(always)]
        pub fn set_repu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timer X reset update"]
        #[inline(always)]
        pub const fn rstu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X reset update"]
        #[inline(always)]
        pub fn set_rstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Timer X update"]
        #[inline(always)]
        pub const fn tu(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 19usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X update"]
        #[inline(always)]
        pub fn set_tu(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 19usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Timer update"]
        #[inline(always)]
        pub const fn mstu(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Master Timer update"]
        #[inline(always)]
        pub fn set_mstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub const fn dacsync(&self) -> super::vals::Dacsync {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Dacsync::from_bits(val as u8)
        }
        #[doc = "AC Synchronization"]
        #[inline(always)]
        pub fn set_dacsync(&mut self, val: super::vals::Dacsync) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub const fn preen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Preload enable"]
        #[inline(always)]
        pub fn set_preen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Update Gating"]
        #[inline(always)]
        pub const fn updgat(&self) -> super::vals::Updgat {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Updgat::from_bits(val as u8)
        }
        #[doc = "Update Gating"]
        #[inline(always)]
        pub fn set_updgat(&mut self, val: super::vals::Updgat) {
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
                .field("syncrst", &self.syncrst())
                .field("syncstrt", &self.syncstrt())
                .field("delcmp2", &self.delcmp2())
                .field("delcmp4", &self.delcmp4())
                .field("repu", &self.repu())
                .field("rstu", &self.rstu())
                .field(
                    "tu",
                    &[
                        self.tu(0usize),
                        self.tu(1usize),
                        self.tu(2usize),
                        self.tu(3usize),
                        self.tu(4usize),
                    ],
                )
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
            #[derive(defmt :: Format)]
            struct Timxcr {
                ckpsc: u8,
                cont: bool,
                retrig: bool,
                half: bool,
                pshpll: bool,
                syncrst: bool,
                syncstrt: bool,
                delcmp2: super::vals::Delcmp,
                delcmp4: super::vals::Delcmp,
                repu: bool,
                rstu: bool,
                tu: [bool; 5usize],
                mstu: bool,
                dacsync: super::vals::Dacsync,
                preen: bool,
                updgat: super::vals::Updgat,
            }
            let proxy = Timxcr {
                ckpsc: self.ckpsc(),
                cont: self.cont(),
                retrig: self.retrig(),
                half: self.half(),
                pshpll: self.pshpll(),
                syncrst: self.syncrst(),
                syncstrt: self.syncstrt(),
                delcmp2: self.delcmp2(),
                delcmp4: self.delcmp4(),
                repu: self.repu(),
                rstu: self.rstu(),
                tu: [
                    self.tu(0usize),
                    self.tu(1usize),
                    self.tu(2usize),
                    self.tu(3usize),
                    self.tu(4usize),
                ],
                mstu: self.mstu(),
                dacsync: self.dacsync(),
                preen: self.preen(),
                updgat: self.updgat(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx DMA / Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdier(pub u32);
    impl Timxdier {
        #[doc = "Compare X Interrupt Enable"]
        #[inline(always)]
        pub const fn cmpie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X Interrupt Enable"]
        #[inline(always)]
        pub fn set_cmpie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt Enable"]
        #[inline(always)]
        pub const fn repie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt Enable"]
        #[inline(always)]
        pub fn set_repie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt Enable"]
        #[inline(always)]
        pub const fn updie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update Interrupt Enable"]
        #[inline(always)]
        pub fn set_updie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture Interrupt Enable"]
        #[inline(always)]
        pub const fn cptie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture Interrupt Enable"]
        #[inline(always)]
        pub fn set_cptie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set Interrupt Enable"]
        #[inline(always)]
        pub const fn setrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set Interrupt Enable"]
        #[inline(always)]
        pub fn set_setrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset Interrupt Enable"]
        #[inline(always)]
        pub const fn rstrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset Interrupt Enable"]
        #[inline(always)]
        pub fn set_rstrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset/roll-over Interrupt Enable"]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over Interrupt Enable"]
        #[inline(always)]
        pub fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Interrupt Enable"]
        #[inline(always)]
        pub const fn dlyprtie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Interrupt Enable"]
        #[inline(always)]
        pub fn set_dlyprtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Compare X DMA request Enable"]
        #[inline(always)]
        pub const fn cmpde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X DMA request Enable"]
        #[inline(always)]
        pub fn set_cmpde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition DMA request Enable"]
        #[inline(always)]
        pub const fn repde(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition DMA request Enable"]
        #[inline(always)]
        pub fn set_repde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Update DMA request Enable"]
        #[inline(always)]
        pub const fn updde(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request Enable"]
        #[inline(always)]
        pub fn set_updde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Capture X DMA request Enable"]
        #[inline(always)]
        pub const fn cptde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 23usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture X DMA request Enable"]
        #[inline(always)]
        pub fn set_cptde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 23usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set DMA request Enable"]
        #[inline(always)]
        pub const fn setrde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 25usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set DMA request Enable"]
        #[inline(always)]
        pub fn set_setrde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 25usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset DMA request Enable"]
        #[inline(always)]
        pub const fn rstrde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 26usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset DMA request Enable"]
        #[inline(always)]
        pub fn set_rstrde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 26usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset/roll-over DMA request Enable"]
        #[inline(always)]
        pub const fn rstde(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset/roll-over DMA request Enable"]
        #[inline(always)]
        pub fn set_rstde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Delayed Protection DMA request Enable"]
        #[inline(always)]
        pub const fn dlyprtde(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection DMA request Enable"]
        #[inline(always)]
        pub fn set_dlyprtde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field(
                    "cmpie",
                    &[
                        self.cmpie(0usize),
                        self.cmpie(1usize),
                        self.cmpie(2usize),
                        self.cmpie(3usize),
                    ],
                )
                .field("repie", &self.repie())
                .field("updie", &self.updie())
                .field("cptie", &[self.cptie(0usize), self.cptie(1usize)])
                .field("setrie", &[self.setrie(0usize), self.setrie(1usize)])
                .field("rstrie", &[self.rstrie(0usize), self.rstrie(1usize)])
                .field("rstie", &self.rstie())
                .field("dlyprtie", &self.dlyprtie())
                .field(
                    "cmpde",
                    &[
                        self.cmpde(0usize),
                        self.cmpde(1usize),
                        self.cmpde(2usize),
                        self.cmpde(3usize),
                    ],
                )
                .field("repde", &self.repde())
                .field("updde", &self.updde())
                .field("cptde", &[self.cptde(0usize), self.cptde(1usize)])
                .field("setrde", &[self.setrde(0usize), self.setrde(1usize)])
                .field("rstrde", &[self.rstrde(0usize), self.rstrde(1usize)])
                .field("rstde", &self.rstde())
                .field("dlyprtde", &self.dlyprtde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxdier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxdier {
                cmpie: [bool; 4usize],
                repie: bool,
                updie: bool,
                cptie: [bool; 2usize],
                setrie: [bool; 2usize],
                rstrie: [bool; 2usize],
                rstie: bool,
                dlyprtie: bool,
                cmpde: [bool; 4usize],
                repde: bool,
                updde: bool,
                cptde: [bool; 2usize],
                setrde: [bool; 2usize],
                rstrde: [bool; 2usize],
                rstde: bool,
                dlyprtde: bool,
            }
            let proxy = Timxdier {
                cmpie: [
                    self.cmpie(0usize),
                    self.cmpie(1usize),
                    self.cmpie(2usize),
                    self.cmpie(3usize),
                ],
                repie: self.repie(),
                updie: self.updie(),
                cptie: [self.cptie(0usize), self.cptie(1usize)],
                setrie: [self.setrie(0usize), self.setrie(1usize)],
                rstrie: [self.rstrie(0usize), self.rstrie(1usize)],
                rstie: self.rstie(),
                dlyprtie: self.dlyprtie(),
                cmpde: [
                    self.cmpde(0usize),
                    self.cmpde(1usize),
                    self.cmpde(2usize),
                    self.cmpde(3usize),
                ],
                repde: self.repde(),
                updde: self.updde(),
                cptde: [self.cptde(0usize), self.cptde(1usize)],
                setrde: [self.setrde(0usize), self.setrde(1usize)],
                rstrde: [self.rstrde(0usize), self.rstrde(1usize)],
                rstde: self.rstde(),
                dlyprtde: self.dlyprtde(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Deadtime Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxdt(pub u32);
    impl Timxdt {
        #[doc = "Deadtime Rising value"]
        #[inline(always)]
        pub const fn dtr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime Rising value"]
        #[inline(always)]
        pub fn set_dtr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Sign Deadtime Rising value"]
        #[inline(always)]
        pub const fn sdtr(&self) -> super::vals::Sdt {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Sdt::from_bits(val as u8)
        }
        #[doc = "Sign Deadtime Rising value"]
        #[inline(always)]
        pub fn set_sdtr(&mut self, val: super::vals::Sdt) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Deadtime Prescaler"]
        #[inline(always)]
        pub const fn dtprsc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Deadtime Prescaler"]
        #[inline(always)]
        pub fn set_dtprsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Deadtime Rising Sign Lock"]
        #[inline(always)]
        pub const fn dtrslk(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime Rising Sign Lock"]
        #[inline(always)]
        pub fn set_dtrslk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Deadtime Rising Lock"]
        #[inline(always)]
        pub const fn dtrlk(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime Rising Lock"]
        #[inline(always)]
        pub fn set_dtrlk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Deadtime Falling value"]
        #[inline(always)]
        pub const fn dtf(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Deadtime Falling value"]
        #[inline(always)]
        pub fn set_dtf(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "Sign Deadtime Falling value"]
        #[inline(always)]
        pub const fn sdtf(&self) -> super::vals::Sdt {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Sdt::from_bits(val as u8)
        }
        #[doc = "Sign Deadtime Falling value"]
        #[inline(always)]
        pub fn set_sdtf(&mut self, val: super::vals::Sdt) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Deadtime Falling Sign Lock"]
        #[inline(always)]
        pub const fn dtfslk(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime Falling Sign Lock"]
        #[inline(always)]
        pub fn set_dtfslk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Deadtime Falling Lock"]
        #[inline(always)]
        pub const fn dtflk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime Falling Lock"]
        #[inline(always)]
        pub fn set_dtflk(&mut self, val: bool) {
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
                .field("dtfslk", &self.dtfslk())
                .field("dtflk", &self.dtflk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxdt {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxdt {
                dtr: u16,
                sdtr: super::vals::Sdt,
                dtprsc: u8,
                dtrslk: bool,
                dtrlk: bool,
                dtf: u16,
                sdtf: super::vals::Sdt,
                dtfslk: bool,
                dtflk: bool,
            }
            let proxy = Timxdt {
                dtr: self.dtr(),
                sdtr: self.sdtr(),
                dtprsc: self.dtprsc(),
                dtrslk: self.dtrslk(),
                dtrlk: self.dtrlk(),
                dtf: self.dtf(),
                sdtf: self.sdtf(),
                dtfslk: self.dtfslk(),
                dtflk: self.dtflk(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timer X External Event Filtering Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxeef(pub u32);
    impl Timxeef {
        #[doc = "External Event X latch"]
        #[inline(always)]
        pub const fn ltch(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X latch"]
        #[inline(always)]
        pub fn set_ltch(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub const fn fltr(&self, n: usize) -> super::vals::Eefltr {
            assert!(n < 5usize);
            let offs = 1usize + n * 6usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::Eefltr::from_bits(val as u8)
        }
        #[doc = "External Event X filter"]
        #[inline(always)]
        pub fn set_fltr(&mut self, n: usize, val: super::vals::Eefltr) {
            assert!(n < 5usize);
            let offs = 1usize + n * 6usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
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
                .field(
                    "ltch",
                    &[
                        self.ltch(0usize),
                        self.ltch(1usize),
                        self.ltch(2usize),
                        self.ltch(3usize),
                        self.ltch(4usize),
                    ],
                )
                .field(
                    "fltr",
                    &[
                        self.fltr(0usize),
                        self.fltr(1usize),
                        self.fltr(2usize),
                        self.fltr(3usize),
                        self.fltr(4usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxeef {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxeef {
                ltch: [bool; 5usize],
                fltr: [super::vals::Eefltr; 5usize],
            }
            let proxy = Timxeef {
                ltch: [
                    self.ltch(0usize),
                    self.ltch(1usize),
                    self.ltch(2usize),
                    self.ltch(3usize),
                    self.ltch(4usize),
                ],
                fltr: [
                    self.fltr(0usize),
                    self.fltr(1usize),
                    self.fltr(2usize),
                    self.fltr(3usize),
                    self.fltr(4usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Fault Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxflt(pub u32);
    impl Timxflt {
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub const fn flten(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fault X enable"]
        #[inline(always)]
        pub fn set_flten(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Fault sources Lock"]
        #[inline(always)]
        pub const fn fltlck(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Fault sources Lock"]
        #[inline(always)]
        pub fn set_fltlck(&mut self, val: bool) {
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
                .field(
                    "flten",
                    &[
                        self.flten(0usize),
                        self.flten(1usize),
                        self.flten(2usize),
                        self.flten(3usize),
                        self.flten(4usize),
                    ],
                )
                .field("fltlck", &self.fltlck())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxflt {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxflt {
                flten: [bool; 5usize],
                fltlck: bool,
            }
            let proxy = Timxflt {
                flten: [
                    self.flten(0usize),
                    self.flten(1usize),
                    self.flten(2usize),
                    self.flten(3usize),
                    self.flten(4usize),
                ],
                fltlck: self.fltlck(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxicr(pub u32);
    impl Timxicr {
        #[doc = "Compare X Interrupt flag Clear"]
        #[inline(always)]
        pub const fn cmpc(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_cmpc(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt flag Clear"]
        #[inline(always)]
        pub const fn repc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_repc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt flag Clear"]
        #[inline(always)]
        pub const fn updc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_updc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture X Interrupt flag Clear"]
        #[inline(always)]
        pub const fn cptc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture X Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_cptc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set flag Clear"]
        #[inline(always)]
        pub const fn setrc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set flag Clear"]
        #[inline(always)]
        pub fn set_setrc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset flag Clear"]
        #[inline(always)]
        pub const fn rstrc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset flag Clear"]
        #[inline(always)]
        pub fn set_rstrc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset Interrupt flag Clear"]
        #[inline(always)]
        pub const fn rstc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Interrupt flag Clear"]
        #[inline(always)]
        pub fn set_rstc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Flag Clear"]
        #[inline(always)]
        pub const fn dlyprtc(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Flag Clear"]
        #[inline(always)]
        pub fn set_dlyprtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field(
                    "cmpc",
                    &[
                        self.cmpc(0usize),
                        self.cmpc(1usize),
                        self.cmpc(2usize),
                        self.cmpc(3usize),
                    ],
                )
                .field("repc", &self.repc())
                .field("updc", &self.updc())
                .field("cptc", &[self.cptc(0usize), self.cptc(1usize)])
                .field("setrc", &[self.setrc(0usize), self.setrc(1usize)])
                .field("rstrc", &[self.rstrc(0usize), self.rstrc(1usize)])
                .field("rstc", &self.rstc())
                .field("dlyprtc", &self.dlyprtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxicr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxicr {
                cmpc: [bool; 4usize],
                repc: bool,
                updc: bool,
                cptc: [bool; 2usize],
                setrc: [bool; 2usize],
                rstrc: [bool; 2usize],
                rstc: bool,
                dlyprtc: bool,
            }
            let proxy = Timxicr {
                cmpc: [
                    self.cmpc(0usize),
                    self.cmpc(1usize),
                    self.cmpc(2usize),
                    self.cmpc(3usize),
                ],
                repc: self.repc(),
                updc: self.updc(),
                cptc: [self.cptc(0usize), self.cptc(1usize)],
                setrc: [self.setrc(0usize), self.setrc(1usize)],
                rstrc: [self.rstrc(0usize), self.rstrc(1usize)],
                rstc: self.rstc(),
                dlyprtc: self.dlyprtc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxisr(pub u32);
    impl Timxisr {
        #[doc = "Compare X Interrupt Flag"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare X Interrupt Flag"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Repetition Interrupt Flag"]
        #[inline(always)]
        pub const fn rep(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Interrupt Flag"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Update Interrupt Flag"]
        #[inline(always)]
        pub const fn upd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Update Interrupt Flag"]
        #[inline(always)]
        pub fn set_upd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture X Interrupt Flag"]
        #[inline(always)]
        pub const fn cpt(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture X Interrupt Flag"]
        #[inline(always)]
        pub fn set_cpt(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Set Interrupt Flag"]
        #[inline(always)]
        pub const fn setr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Set Interrupt Flag"]
        #[inline(always)]
        pub fn set_setr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Reset Interrupt Flag"]
        #[inline(always)]
        pub const fn rstr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Reset Interrupt Flag"]
        #[inline(always)]
        pub fn set_rstr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 10usize + ([0usize, 2usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset Interrupt Flag"]
        #[inline(always)]
        pub const fn rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Interrupt Flag"]
        #[inline(always)]
        pub fn set_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed Protection Flag"]
        #[inline(always)]
        pub const fn dlyprt(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Flag"]
        #[inline(always)]
        pub fn set_dlyprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Current Push Pull Status"]
        #[inline(always)]
        pub const fn cppstat(&self) -> super::vals::Cppstat {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Cppstat::from_bits(val as u8)
        }
        #[doc = "Current Push Pull Status"]
        #[inline(always)]
        pub fn set_cppstat(&mut self, val: super::vals::Cppstat) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Idle Push Pull Status"]
        #[inline(always)]
        pub const fn ippstat(&self) -> super::vals::Ippstat {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ippstat::from_bits(val as u8)
        }
        #[doc = "Idle Push Pull Status"]
        #[inline(always)]
        pub fn set_ippstat(&mut self, val: super::vals::Ippstat) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Output X State"]
        #[inline(always)]
        pub const fn ostat(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X State"]
        #[inline(always)]
        pub fn set_ostat(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 18usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Copy"]
        #[inline(always)]
        pub const fn ocpy(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 20usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Copy"]
        #[inline(always)]
        pub fn set_ocpy(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 20usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "cmp",
                    &[self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                )
                .field("rep", &self.rep())
                .field("upd", &self.upd())
                .field("cpt", &[self.cpt(0usize), self.cpt(1usize)])
                .field("setr", &[self.setr(0usize), self.setr(1usize)])
                .field("rstr", &[self.rstr(0usize), self.rstr(1usize)])
                .field("rst", &self.rst())
                .field("dlyprt", &self.dlyprt())
                .field("cppstat", &self.cppstat())
                .field("ippstat", &self.ippstat())
                .field("ostat", &[self.ostat(0usize), self.ostat(1usize)])
                .field("ocpy", &[self.ocpy(0usize), self.ocpy(1usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxisr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxisr {
                cmp: [bool; 4usize],
                rep: bool,
                upd: bool,
                cpt: [bool; 2usize],
                setr: [bool; 2usize],
                rstr: [bool; 2usize],
                rst: bool,
                dlyprt: bool,
                cppstat: super::vals::Cppstat,
                ippstat: super::vals::Ippstat,
                ostat: [bool; 2usize],
                ocpy: [bool; 2usize],
            }
            let proxy = Timxisr {
                cmp: [self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                rep: self.rep(),
                upd: self.upd(),
                cpt: [self.cpt(0usize), self.cpt(1usize)],
                setr: [self.setr(0usize), self.setr(1usize)],
                rstr: [self.rstr(0usize), self.rstr(1usize)],
                rst: self.rst(),
                dlyprt: self.dlyprt(),
                cppstat: self.cppstat(),
                ippstat: self.ippstat(),
                ostat: [self.ostat(0usize), self.ostat(1usize)],
                ocpy: [self.ocpy(0usize), self.ocpy(1usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Output Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxoutr(pub u32);
    impl Timxoutr {
        #[doc = "Output 1 polarity"]
        #[inline(always)]
        pub const fn pol(&self, n: usize) -> super::vals::Pol {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output 1 polarity"]
        #[inline(always)]
        pub fn set_pol(&mut self, n: usize, val: super::vals::Pol) {
            assert!(n < 2usize);
            let offs = 1usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output X Idle mode"]
        #[inline(always)]
        pub const fn idlem(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Idle mode"]
        #[inline(always)]
        pub fn set_idlem(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Idle State"]
        #[inline(always)]
        pub const fn idles(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Idle State"]
        #[inline(always)]
        pub fn set_idles(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Fault state"]
        #[inline(always)]
        pub const fn fault(&self, n: usize) -> super::vals::Fault {
            assert!(n < 2usize);
            let offs = 4usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x03;
            super::vals::Fault::from_bits(val as u8)
        }
        #[doc = "Output X Fault state"]
        #[inline(always)]
        pub fn set_fault(&mut self, n: usize, val: super::vals::Fault) {
            assert!(n < 2usize);
            let offs = 4usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output X Chopper enable"]
        #[inline(always)]
        pub const fn chp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Chopper enable"]
        #[inline(always)]
        pub fn set_chp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output X Deadtime upon burst mode Idle entry"]
        #[inline(always)]
        pub const fn didl(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + ([0usize, 16usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output X Deadtime upon burst mode Idle entry"]
        #[inline(always)]
        pub fn set_didl(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + ([0usize, 16usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Deadtime enable"]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Deadtime enable"]
        #[inline(always)]
        pub fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Delayed Protection Enable"]
        #[inline(always)]
        pub const fn dlyprten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed Protection Enable"]
        #[inline(always)]
        pub fn set_dlyprten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Delayed Protection"]
        #[inline(always)]
        pub const fn dlyprt(&self) -> super::vals::Dlyprt {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Dlyprt::from_bits(val as u8)
        }
        #[doc = "Delayed Protection"]
        #[inline(always)]
        pub fn set_dlyprt(&mut self, val: super::vals::Dlyprt) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
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
                .field("pol", &[self.pol(0usize), self.pol(1usize)])
                .field("idlem", &[self.idlem(0usize), self.idlem(1usize)])
                .field("idles", &[self.idles(0usize), self.idles(1usize)])
                .field("fault", &[self.fault(0usize), self.fault(1usize)])
                .field("chp", &[self.chp(0usize), self.chp(1usize)])
                .field("didl", &[self.didl(0usize), self.didl(1usize)])
                .field("dten", &self.dten())
                .field("dlyprten", &self.dlyprten())
                .field("dlyprt", &self.dlyprt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxoutr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxoutr {
                pol: [super::vals::Pol; 2usize],
                idlem: [bool; 2usize],
                idles: [bool; 2usize],
                fault: [super::vals::Fault; 2usize],
                chp: [bool; 2usize],
                didl: [bool; 2usize],
                dten: bool,
                dlyprten: bool,
                dlyprt: super::vals::Dlyprt,
            }
            let proxy = Timxoutr {
                pol: [self.pol(0usize), self.pol(1usize)],
                idlem: [self.idlem(0usize), self.idlem(1usize)],
                idles: [self.idles(0usize), self.idles(1usize)],
                fault: [self.fault(0usize), self.fault(1usize)],
                chp: [self.chp(0usize), self.chp(1usize)],
                didl: [self.didl(0usize), self.didl(1usize)],
                dten: self.dten(),
                dlyprten: self.dlyprten(),
                dlyprt: self.dlyprt(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Period Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxper(pub u32);
    impl Timxper {
        #[doc = "Timerx Period value"]
        #[inline(always)]
        pub const fn per(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timerx Period value"]
        #[inline(always)]
        pub fn set_per(&mut self, val: u16) {
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
            #[derive(defmt :: Format)]
            struct Timxper {
                per: u16,
            }
            let proxy = Timxper { per: self.per() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Repetition Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrep(pub u32);
    impl Timxrep {
        #[doc = "Timerx Repetition counter value"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timerx Repetition counter value"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct Timxrep {
                rep: u8,
            }
            let proxy = Timxrep { rep: self.rep() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrst(pub u32);
    impl Timxrst {
        #[doc = "Timer X compare 1 event"]
        #[inline(always)]
        pub const fn tcmp1(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + ([19usize, 22usize, 25usize, 28usize, 0usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare 1 event"]
        #[inline(always)]
        pub fn set_tcmp1(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + ([19usize, 22usize, 25usize, 28usize, 0usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X Update reset"]
        #[inline(always)]
        pub const fn updt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X Update reset"]
        #[inline(always)]
        pub fn set_updt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X compare X reset"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare X reset"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master timer Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Master timer Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Master compare X"]
        #[inline(always)]
        pub const fn mstcmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 5usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master compare X"]
        #[inline(always)]
        pub fn set_mstcmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 5usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> bool {
            assert!(n < 10usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: bool) {
            assert!(n < 10usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X compare 2 event"]
        #[inline(always)]
        pub const fn tcmp2(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 20usize + ([0usize, 3usize, 6usize, 9usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare 2 event"]
        #[inline(always)]
        pub fn set_tcmp2(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 20usize + ([0usize, 3usize, 6usize, 9usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer X compare 4 event"]
        #[inline(always)]
        pub const fn tcmp4(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 21usize + ([0usize, 3usize, 6usize, 9usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare 4 event"]
        #[inline(always)]
        pub fn set_tcmp4(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 21usize + ([0usize, 3usize, 6usize, 9usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field(
                    "tcmp1",
                    &[
                        self.tcmp1(0usize),
                        self.tcmp1(1usize),
                        self.tcmp1(2usize),
                        self.tcmp1(3usize),
                        self.tcmp1(4usize),
                    ],
                )
                .field("updt", &self.updt())
                .field("cmp", &[self.cmp(0usize), self.cmp(1usize)])
                .field("mstper", &self.mstper())
                .field(
                    "mstcmp",
                    &[
                        self.mstcmp(0usize),
                        self.mstcmp(1usize),
                        self.mstcmp(2usize),
                        self.mstcmp(3usize),
                    ],
                )
                .field(
                    "extevnt",
                    &[
                        self.extevnt(0usize),
                        self.extevnt(1usize),
                        self.extevnt(2usize),
                        self.extevnt(3usize),
                        self.extevnt(4usize),
                        self.extevnt(5usize),
                        self.extevnt(6usize),
                        self.extevnt(7usize),
                        self.extevnt(8usize),
                        self.extevnt(9usize),
                    ],
                )
                .field(
                    "tcmp2",
                    &[
                        self.tcmp2(0usize),
                        self.tcmp2(1usize),
                        self.tcmp2(2usize),
                        self.tcmp2(3usize),
                        self.tcmp2(4usize),
                    ],
                )
                .field(
                    "tcmp4",
                    &[
                        self.tcmp4(0usize),
                        self.tcmp4(1usize),
                        self.tcmp4(2usize),
                        self.tcmp4(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxrst {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxrst {
                tcmp1: [bool; 5usize],
                updt: bool,
                cmp: [bool; 2usize],
                mstper: bool,
                mstcmp: [bool; 4usize],
                extevnt: [bool; 10usize],
                tcmp2: [bool; 5usize],
                tcmp4: [bool; 4usize],
            }
            let proxy = Timxrst {
                tcmp1: [
                    self.tcmp1(0usize),
                    self.tcmp1(1usize),
                    self.tcmp1(2usize),
                    self.tcmp1(3usize),
                    self.tcmp1(4usize),
                ],
                updt: self.updt(),
                cmp: [self.cmp(0usize), self.cmp(1usize)],
                mstper: self.mstper(),
                mstcmp: [
                    self.mstcmp(0usize),
                    self.mstcmp(1usize),
                    self.mstcmp(2usize),
                    self.mstcmp(3usize),
                ],
                extevnt: [
                    self.extevnt(0usize),
                    self.extevnt(1usize),
                    self.extevnt(2usize),
                    self.extevnt(3usize),
                    self.extevnt(4usize),
                    self.extevnt(5usize),
                    self.extevnt(6usize),
                    self.extevnt(7usize),
                    self.extevnt(8usize),
                    self.extevnt(9usize),
                ],
                tcmp2: [
                    self.tcmp2(0usize),
                    self.tcmp2(1usize),
                    self.tcmp2(2usize),
                    self.tcmp2(3usize),
                    self.tcmp2(4usize),
                ],
                tcmp4: [
                    self.tcmp4(0usize),
                    self.tcmp4(1usize),
                    self.tcmp4(2usize),
                    self.tcmp4(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx OutputX Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxrstr(pub u32);
    impl Timxrstr {
        #[doc = "Software Reset trigger"]
        #[inline(always)]
        pub const fn srt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset trigger"]
        #[inline(always)]
        pub fn set_srt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub const fn resync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub fn set_resync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub const fn mstcmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub fn set_mstcmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub const fn timevnt(&self, n: usize) -> bool {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub fn set_timevnt(&mut self, n: usize, val: bool) {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> bool {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: bool) {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub const fn update(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub fn set_update(&mut self, val: bool) {
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
                .field(
                    "cmp",
                    &[self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                )
                .field("mstper", &self.mstper())
                .field(
                    "mstcmp",
                    &[
                        self.mstcmp(0usize),
                        self.mstcmp(1usize),
                        self.mstcmp(2usize),
                        self.mstcmp(3usize),
                    ],
                )
                .field(
                    "timevnt",
                    &[
                        self.timevnt(0usize),
                        self.timevnt(1usize),
                        self.timevnt(2usize),
                        self.timevnt(3usize),
                        self.timevnt(4usize),
                        self.timevnt(5usize),
                        self.timevnt(6usize),
                        self.timevnt(7usize),
                        self.timevnt(8usize),
                    ],
                )
                .field(
                    "extevnt",
                    &[
                        self.extevnt(0usize),
                        self.extevnt(1usize),
                        self.extevnt(2usize),
                        self.extevnt(3usize),
                        self.extevnt(4usize),
                        self.extevnt(5usize),
                        self.extevnt(6usize),
                        self.extevnt(7usize),
                        self.extevnt(8usize),
                        self.extevnt(9usize),
                    ],
                )
                .field("update", &self.update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxrstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxrstr {
                srt: bool,
                resync: bool,
                per: bool,
                cmp: [bool; 4usize],
                mstper: bool,
                mstcmp: [bool; 4usize],
                timevnt: [bool; 9usize],
                extevnt: [bool; 10usize],
                update: bool,
            }
            let proxy = Timxrstr {
                srt: self.srt(),
                resync: self.resync(),
                per: self.per(),
                cmp: [self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                mstper: self.mstper(),
                mstcmp: [
                    self.mstcmp(0usize),
                    self.mstcmp(1usize),
                    self.mstcmp(2usize),
                    self.mstcmp(3usize),
                ],
                timevnt: [
                    self.timevnt(0usize),
                    self.timevnt(1usize),
                    self.timevnt(2usize),
                    self.timevnt(3usize),
                    self.timevnt(4usize),
                    self.timevnt(5usize),
                    self.timevnt(6usize),
                    self.timevnt(7usize),
                    self.timevnt(8usize),
                ],
                extevnt: [
                    self.extevnt(0usize),
                    self.extevnt(1usize),
                    self.extevnt(2usize),
                    self.extevnt(3usize),
                    self.extevnt(4usize),
                    self.extevnt(5usize),
                    self.extevnt(6usize),
                    self.extevnt(7usize),
                    self.extevnt(8usize),
                    self.extevnt(9usize),
                ],
                update: self.update(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timerx OutputX Set Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timxsetr(pub u32);
    impl Timxsetr {
        #[doc = "Software Set trigger"]
        #[inline(always)]
        pub const fn sst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Set trigger"]
        #[inline(always)]
        pub fn set_sst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub const fn resync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X resynchronizaton"]
        #[inline(always)]
        pub fn set_resync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer X Period"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub const fn cmp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer X compare X"]
        #[inline(always)]
        pub fn set_cmp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub const fn mstper(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Master Period"]
        #[inline(always)]
        pub fn set_mstper(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub const fn mstcmpx(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master Compare X"]
        #[inline(always)]
        pub fn set_mstcmpx(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub const fn timevnt(&self, n: usize) -> bool {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Timer Event X"]
        #[inline(always)]
        pub fn set_timevnt(&mut self, n: usize, val: bool) {
            assert!(n < 9usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub const fn extevnt(&self, n: usize) -> bool {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "External Event X"]
        #[inline(always)]
        pub fn set_extevnt(&mut self, n: usize, val: bool) {
            assert!(n < 10usize);
            let offs = 21usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub const fn update(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update (transfer preload to active)"]
        #[inline(always)]
        pub fn set_update(&mut self, val: bool) {
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
                .field(
                    "cmp",
                    &[self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                )
                .field("mstper", &self.mstper())
                .field(
                    "mstcmpx",
                    &[
                        self.mstcmpx(0usize),
                        self.mstcmpx(1usize),
                        self.mstcmpx(2usize),
                        self.mstcmpx(3usize),
                    ],
                )
                .field(
                    "timevnt",
                    &[
                        self.timevnt(0usize),
                        self.timevnt(1usize),
                        self.timevnt(2usize),
                        self.timevnt(3usize),
                        self.timevnt(4usize),
                        self.timevnt(5usize),
                        self.timevnt(6usize),
                        self.timevnt(7usize),
                        self.timevnt(8usize),
                    ],
                )
                .field(
                    "extevnt",
                    &[
                        self.extevnt(0usize),
                        self.extevnt(1usize),
                        self.extevnt(2usize),
                        self.extevnt(3usize),
                        self.extevnt(4usize),
                        self.extevnt(5usize),
                        self.extevnt(6usize),
                        self.extevnt(7usize),
                        self.extevnt(8usize),
                        self.extevnt(9usize),
                    ],
                )
                .field("update", &self.update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timxsetr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Timxsetr {
                sst: bool,
                resync: bool,
                per: bool,
                cmp: [bool; 4usize],
                mstper: bool,
                mstcmpx: [bool; 4usize],
                timevnt: [bool; 9usize],
                extevnt: [bool; 10usize],
                update: bool,
            }
            let proxy = Timxsetr {
                sst: self.sst(),
                resync: self.resync(),
                per: self.per(),
                cmp: [self.cmp(0usize), self.cmp(1usize), self.cmp(2usize), self.cmp(3usize)],
                mstper: self.mstper(),
                mstcmpx: [
                    self.mstcmpx(0usize),
                    self.mstcmpx(1usize),
                    self.mstcmpx(2usize),
                    self.mstcmpx(3usize),
                ],
                timevnt: [
                    self.timevnt(0usize),
                    self.timevnt(1usize),
                    self.timevnt(2usize),
                    self.timevnt(3usize),
                    self.timevnt(4usize),
                    self.timevnt(5usize),
                    self.timevnt(6usize),
                    self.timevnt(7usize),
                    self.timevnt(8usize),
                ],
                extevnt: [
                    self.extevnt(0usize),
                    self.extevnt(1usize),
                    self.extevnt(2usize),
                    self.extevnt(3usize),
                    self.extevnt(4usize),
                    self.extevnt(5usize),
                    self.extevnt(6usize),
                    self.extevnt(7usize),
                    self.extevnt(8usize),
                    self.extevnt(9usize),
                ],
                update: self.update(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Brstdma {
        #[doc = "Update done independently from the DMA burst transfer completion"]
        INDEPENDENT = 0x0,
        #[doc = "Update done when the DMA burst transfer is completed"]
        COMPLETION = 0x01,
        #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
        ROLLOVER = 0x02,
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
        OUTPUT1ACTIVE = 0x0,
        #[doc = "Signal applied on output 2 and output 1 forced inactive"]
        OUTPUT2ACTIVE = 0x01,
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
        DISABLED = 0x0,
        #[doc = "Trigger generated on DACSync1"]
        DACSYNC1 = 0x01,
        #[doc = "Trigger generated on DACSync2"]
        DACSYNC2 = 0x02,
        #[doc = "Trigger generated on DACSync3"]
        DACSYNC3 = 0x03,
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
        STANDARD = 0x0,
        #[doc = "CMP is recomputed and is active following a capture 1 event"]
        CAPTURE1 = 0x01,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 1 match"]
        CAPTURE_X_COMPARE1 = 0x02,
        #[doc = "CMP is recomputed and is active following a capture 1 event or a Compare 3 match"]
        CAPTURE_X_COMPARE3 = 0x03,
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
        OUTPUT1_EE6 = 0x0,
        #[doc = "Output 2 delayed idle on external event 6"]
        OUTPUT2_EE6 = 0x01,
        #[doc = "Output 1 and 2 delayed idle on external event 6"]
        OUTPUT1_2_EE6 = 0x02,
        #[doc = "Balanced idle on external event 6"]
        BALANCED_EE6 = 0x03,
        #[doc = "Output 1 delayed idle on external event 7"]
        OUTPUT1_EE7 = 0x04,
        #[doc = "Output 2 delayed idle on external event 7"]
        OUTPUT2_EE7 = 0x05,
        #[doc = "Output 1 and 2 delayed idle on external event 7"]
        OUTPUT1_2_EE7 = 0x06,
        #[doc = "Balanced idle on external event 7"]
        BALANCED_EE7 = 0x07,
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
    pub enum Eefltr {
        #[doc = "No filtering"]
        DISABLED = 0x0,
        #[doc = "Blanking from counter reset/roll-over to Compare 1"]
        BLANK_RESET_TO_COMPARE1 = 0x01,
        #[doc = "Blanking from counter reset/roll-over to Compare 2"]
        BLANK_RESET_TO_COMPARE2 = 0x02,
        #[doc = "Blanking from counter reset/roll-over to Compare 3"]
        BLANK_RESET_TO_COMPARE3 = 0x03,
        #[doc = "Blanking from counter reset/roll-over to Compare 4"]
        BLANK_RESET_TO_COMPARE4 = 0x04,
        #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
        BLANK_TIMFLTR1 = 0x05,
        #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
        BLANK_TIMFLTR2 = 0x06,
        #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
        BLANK_TIMFLTR3 = 0x07,
        #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
        BLANK_TIMFLTR4 = 0x08,
        #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
        BLANK_TIMFLTR5 = 0x09,
        #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
        BLANK_TIMFLTR6 = 0x0a,
        #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
        BLANK_TIMFLTR7 = 0x0b,
        #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
        BLANK_TIMFLTR8 = 0x0c,
        #[doc = "Windowing from counter reset/roll-over to compare 2"]
        WINDOW_RESET_TO_COMPARE2 = 0x0d,
        #[doc = "Windowing from counter reset/roll-over to compare 3"]
        WINDOW_RESET_TO_COMPARE3 = 0x0e,
        #[doc = "Windowing from another timing unit: TIMWIN source"]
        WINDOW_TIMWIN = 0x0f,
    }
    impl Eefltr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eefltr {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eefltr {
        #[inline(always)]
        fn from(val: u8) -> Eefltr {
            Eefltr::from_bits(val)
        }
    }
    impl From<Eefltr> for u8 {
        #[inline(always)]
        fn from(val: Eefltr) -> u8 {
            Eefltr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fault {
        #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
        DISABLED = 0x0,
        #[doc = "Output goes to active state after a fault event"]
        SET_ACTIVE = 0x01,
        #[doc = "Output goes to inactive state after a fault event"]
        SET_INACTIVE = 0x02,
        #[doc = "Output goes to high-z state after a fault event"]
        SET_HIGH_Z = 0x03,
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
        OUTPUT1ACTIVE = 0x0,
        #[doc = "Protection occurred when the output 2 was active and output 1 forced inactive"]
        OUTPUT2ACTIVE = 0x01,
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
        ACTIVE_HIGH = 0x0,
        #[doc = "Negative polarity (output active low)"]
        ACTIVE_LOW = 0x01,
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
        POSITIVE = 0x0,
        #[doc = "Negative deadtime (both outputs active during deadtime)"]
        NEGATIVE = 0x01,
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
        DISABLED = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
        INTERNAL = 0x02,
        #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
        EXTERNAL = 0x03,
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
        DISABLED = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        POSITIVE_PULSE = 0x02,
        #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
        NEGATIVE_PULSE = 0x03,
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
        MASTER_START = 0x0,
        #[doc = "Master timer Compare 1 event"]
        MASTER_COMPARE1 = 0x01,
        #[doc = "Timer A start/reset"]
        TIMER_ASTART = 0x02,
        #[doc = "Timer A Compare 1 event"]
        TIMER_ACOMPARE1 = 0x03,
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
        INDEPENDENT = 0x0,
        #[doc = "Update occurs when the DMA burst transfer is completed"]
        DMABURST = 0x01,
        #[doc = "Update occurs on the update event following DMA burst transfer completion"]
        DMABURST_UPDATE = 0x02,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 1"]
        INPUT1 = 0x03,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 2"]
        INPUT2 = 0x04,
        #[doc = "Update occurs on a rising edge of HRTIM update enable input 3"]
        INPUT3 = 0x05,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 1"]
        INPUT1_UPDATE = 0x06,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 2"]
        INPUT2_UPDATE = 0x07,
        #[doc = "Update occurs on the update event following a rising edge of HRTIM update enable input 3"]
        INPUT3_UPDATE = 0x08,
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
