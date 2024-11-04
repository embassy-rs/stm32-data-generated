#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Operational amplifiers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OPAMP1 control/status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OPAMP1 offset trimming register in normal mode."]
    #[inline(always)]
    pub const fn otr(self) -> crate::common::Reg<regs::Otr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "OPAMP1 offset trimming register in low-power mode."]
    #[inline(always)]
    pub const fn hsotr(self) -> crate::common::Reg<regs::Hsotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPAMP1 control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Operational amplifier Enable."]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Operational amplifier Enable."]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force internal reference on VP (reserved for test."]
        #[inline(always)]
        pub const fn force_vp(&self) -> super::vals::ForceVp {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::ForceVp::from_bits(val as u8)
        }
        #[doc = "Force internal reference on VP (reserved for test."]
        #[inline(always)]
        pub fn set_force_vp(&mut self, val: super::vals::ForceVp) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Operational amplifier PGA mode."]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "Operational amplifier PGA mode."]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Inverting input selection."]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "Inverting input selection."]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Operational amplifier high-speed mode."]
        #[inline(always)]
        pub const fn opahsm(&self) -> super::vals::Opahsm {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Opahsm::from_bits(val as u8)
        }
        #[doc = "Operational amplifier high-speed mode."]
        #[inline(always)]
        pub fn set_opahsm(&mut self, val: super::vals::Opahsm) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Calibration mode enabled."]
        #[inline(always)]
        pub const fn calon(&self) -> super::vals::Calon {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Calon::from_bits(val as u8)
        }
        #[doc = "Calibration mode enabled."]
        #[inline(always)]
        pub fn set_calon(&mut self, val: super::vals::Calon) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Calibration selection."]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection."]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "allows to switch from AOP offset trimmed values to AOP offset."]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 14usize) & 0x0f;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "allows to switch from AOP offset trimmed values to AOP offset."]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
        }
        #[doc = "User trimming enable."]
        #[inline(always)]
        pub const fn usertrim(&self) -> super::vals::Usertrim {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Usertrim::from_bits(val as u8)
        }
        #[doc = "User trimming enable."]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: super::vals::Usertrim) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "OPAMP calibration reference voltage output control (reserved for test)."]
        #[inline(always)]
        pub const fn tstref(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP calibration reference voltage output control (reserved for test)."]
        #[inline(always)]
        pub fn set_tstref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Operational amplifier calibration output."]
        #[inline(always)]
        pub const fn calout(&self) -> super::vals::Calout {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Calout::from_bits(val as u8)
        }
        #[doc = "Operational amplifier calibration output."]
        #[inline(always)]
        pub fn set_calout(&mut self, val: super::vals::Calout) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "OPAMP1 offset trimming register in low-power mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsotr(pub u32);
    impl Hsotr {
        #[doc = "Trim for NMOS differential pairs."]
        #[inline(always)]
        pub const fn trimlpoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for NMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimlpoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[inline(always)]
        pub const fn trimlpoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimlpoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Hsotr {
        #[inline(always)]
        fn default() -> Hsotr {
            Hsotr(0)
        }
    }
    #[doc = "OPAMP1 offset trimming register in normal mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otr(pub u32);
    impl Otr {
        #[doc = "Trim for NMOS differential pairs."]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for NMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Otr {
        #[inline(always)]
        fn default() -> Otr {
            Otr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Calon {
        #[doc = "Normal mode"]
        NORMAL = 0x0,
        #[doc = "Calibration mode (all switches opened by HW)"]
        CALIBRATION = 0x01,
    }
    impl Calon {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calon {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calon {
        #[inline(always)]
        fn from(val: u8) -> Calon {
            Calon::from_bits(val)
        }
    }
    impl From<Calon> for u8 {
        #[inline(always)]
        fn from(val: Calon) -> u8 {
            Calon::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Calout {
        #[doc = "Non-inverting < inverting"]
        LESS = 0x0,
        #[doc = "Non-inverting > inverting"]
        GREATER = 0x01,
    }
    impl Calout {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calout {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calout {
        #[inline(always)]
        fn from(val: u8) -> Calout {
            Calout::from_bits(val)
        }
    }
    impl From<Calout> for u8 {
        #[inline(always)]
        fn from(val: Calout) -> u8 {
            Calout::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Calsel {
        #[doc = "VREFOPAMP=3.3% VDDA."]
        PERCENT3_3 = 0x0,
        #[doc = "VREFOPAMP=10% VDDA."]
        PERCENT10 = 0x01,
        #[doc = "VREFOPAMP=50% VDDA."]
        PERCENT50 = 0x02,
        #[doc = "VREFOPAMP=90% VDDA."]
        PERCENT90 = 0x03,
    }
    impl Calsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calsel {
        #[inline(always)]
        fn from(val: u8) -> Calsel {
            Calsel::from_bits(val)
        }
    }
    impl From<Calsel> for u8 {
        #[inline(always)]
        fn from(val: Calsel) -> u8 {
            Calsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ForceVp {
        #[doc = "Normal operating mode. Non-inverting input connected to inputs."]
        NORMALOPERATING = 0x0,
        #[doc = "Calibration verification mode. Non-inverting input connected to calibration reference voltage."]
        CALIBRATIONVERIFICATION = 0x01,
    }
    impl ForceVp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ForceVp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ForceVp {
        #[inline(always)]
        fn from(val: u8) -> ForceVp {
            ForceVp::from_bits(val)
        }
    }
    impl From<ForceVp> for u8 {
        #[inline(always)]
        fn from(val: ForceVp) -> u8 {
            ForceVp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Opahsm {
        #[doc = "operational amplifier in normal mode"]
        NORMAL = 0x0,
        #[doc = "operational amplifier in high-speed mode"]
        HIGHSPEED = 0x01,
    }
    impl Opahsm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opahsm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opahsm {
        #[inline(always)]
        fn from(val: u8) -> Opahsm {
            Opahsm::from_bits(val)
        }
    }
    impl From<Opahsm> for u8 {
        #[inline(always)]
        fn from(val: Opahsm) -> u8 {
            Opahsm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PgaGain {
        #[doc = "Non-inverting internal Gain 2, VREF- referenced"]
        GAIN2 = 0x0,
        #[doc = "Non-inverting internal Gain 4, VREF- referenced"]
        GAIN4 = 0x01,
        #[doc = "Non-inverting internal Gain 8, VREF- referenced"]
        GAIN8 = 0x02,
        #[doc = "Non-inverting internal Gain 16, VREF- referenced"]
        GAIN16 = 0x03,
        #[doc = "Non-inverting internal Gain 2 with filtering on INM0, VREF- referenced"]
        GAIN2_FILTERINGVINM0 = 0x04,
        #[doc = "Non-inverting internal Gain 4 with filtering on INM0, VREF- referenced"]
        GAIN4_FILTERINGVINM0 = 0x05,
        #[doc = "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced"]
        GAIN8_FILTERINGVINM0 = 0x06,
        #[doc = "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced"]
        GAIN16_FILTERINGVINM0 = 0x07,
        #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias"]
        GAIN2INVGAINNEG1_INPUTVINM0 = 0x08,
        #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias"]
        GAIN4INVGAINNEG3_INPUTVINM0 = 0x09,
        #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias"]
        GAIN8INVGAINNEG7_INPUTVINM0 = 0x0a,
        #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias"]
        GAIN16INVGAINNEG15_INPUTVINM0 = 0x0b,
        #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering"]
        GAIN2INVGAINNEG1_INPUTVINM0FILTERINGVINM1 = 0x0c,
        #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering"]
        GAIN4INVGAINNEG3_INPUTVINM0FILTERINGVINM1 = 0x0d,
        #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering"]
        GAIN8INVGAINNEG7_INPUTVINM0FILTERINGVINM1 = 0x0e,
        #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering"]
        GAIN16INVGAINNEG15_INPUTVINM0FILTERINGVINM1 = 0x0f,
    }
    impl PgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgaGain {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PgaGain {
        #[inline(always)]
        fn from(val: u8) -> PgaGain {
            PgaGain::from_bits(val)
        }
    }
    impl From<PgaGain> for u8 {
        #[inline(always)]
        fn from(val: PgaGain) -> u8 {
            PgaGain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usertrim {
        #[doc = "\\'factory\\' trim code used"]
        FACTORY = 0x0,
        #[doc = "\\'user\\' trim code used"]
        USER = 0x01,
    }
    impl Usertrim {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usertrim {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usertrim {
        #[inline(always)]
        fn from(val: u8) -> Usertrim {
            Usertrim::from_bits(val)
        }
    }
    impl From<Usertrim> for u8 {
        #[inline(always)]
        fn from(val: Usertrim) -> u8 {
            Usertrim::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum VmSel {
        #[doc = "INM0 connected to OPAMP_VINM input"]
        INM0 = 0x0,
        #[doc = "INM1 connected to OPAMP_VINM input"]
        INM1 = 0x01,
        #[doc = "Feedback resistor is connected to the OPAMP_VINM input (PGA mode), Inverting input selection depends on the PGA_GAIN setting"]
        PGA = 0x02,
        #[doc = "opamp_out connected to OPAMP_VINM input (Follower mode)"]
        FOLLOWER = 0x03,
    }
    impl VmSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VmSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VmSel {
        #[inline(always)]
        fn from(val: u8) -> VmSel {
            VmSel::from_bits(val)
        }
    }
    impl From<VmSel> for u8 {
        #[inline(always)]
        fn from(val: VmSel) -> u8 {
            VmSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum VpSel {
        #[doc = "GPIO connected to OPAMPx_VINP"]
        GPIO = 0x0,
        #[doc = "dac_outx connected to OPAMPx_VINP"]
        DACOUT = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl VpSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VpSel {
        #[inline(always)]
        fn from(val: u8) -> VpSel {
            VpSel::from_bits(val)
        }
    }
    impl From<VpSel> for u8 {
        #[inline(always)]
        fn from(val: VpSel) -> u8 {
            VpSel::to_bits(val)
        }
    }
}
