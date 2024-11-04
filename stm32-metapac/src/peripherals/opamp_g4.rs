#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Operational Amplifier"]
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
    #[doc = "OPAMP control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OPAMP control/status register"]
    #[inline(always)]
    pub const fn tcmr(self) -> crate::common::Reg<regs::Tcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPAMP control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "OPAMP enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Forces a calibration reference voltage on non-inverting input and disables external connections."]
        #[inline(always)]
        pub const fn force_vp(&self) -> super::vals::ForceVp {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::ForceVp::from_bits(val as u8)
        }
        #[doc = "Forces a calibration reference voltage on non-inverting input and disables external connections."]
        #[inline(always)]
        pub fn set_force_vp(&mut self, val: super::vals::ForceVp) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "VP_SEL"]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "VP_SEL"]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USERTRIM"]
        #[inline(always)]
        pub const fn usertrim(&self) -> super::vals::Usertrim {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Usertrim::from_bits(val as u8)
        }
        #[doc = "USERTRIM"]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: super::vals::Usertrim) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "OPAMP inverting input selection"]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "OPAMP inverting input selection"]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "OPAHSM"]
        #[inline(always)]
        pub const fn opahsm(&self) -> super::vals::Opahsm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Opahsm::from_bits(val as u8)
        }
        #[doc = "OPAHSM"]
        #[inline(always)]
        pub fn set_opahsm(&mut self, val: super::vals::Opahsm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "OPAINTOEN"]
        #[inline(always)]
        pub const fn opaintoen(&self) -> super::vals::Opaintoen {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Opaintoen::from_bits(val as u8)
        }
        #[doc = "OPAINTOEN"]
        #[inline(always)]
        pub fn set_opaintoen(&mut self, val: super::vals::Opaintoen) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration mode enable"]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection"]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 14usize) & 0x1f;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "Gain in PGA mode"]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val.to_bits() as u32) & 0x1f) << 14usize);
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (PMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Offset trimming value (NMOS)"]
        #[inline(always)]
        pub fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "OPAMP ouput status flag"]
        #[inline(always)]
        pub const fn outcal(&self) -> super::vals::Outcal {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Outcal::from_bits(val as u8)
        }
        #[doc = "OPAMP ouput status flag"]
        #[inline(always)]
        pub fn set_outcal(&mut self, val: super::vals::Outcal) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "OPAMP timer controlled mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcmr(pub u32);
    impl Tcmr {
        #[doc = "VMS_SEL"]
        #[inline(always)]
        pub const fn vms_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VMS_SEL"]
        #[inline(always)]
        pub fn set_vms_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VPS_SEL"]
        #[inline(always)]
        pub const fn vps_sel(&self) -> super::vals::VpsSel {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::VpsSel::from_bits(val as u8)
        }
        #[doc = "VPS_SEL"]
        #[inline(always)]
        pub fn set_vps_sel(&mut self, val: super::vals::VpsSel) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "T1CM_EN"]
        #[inline(always)]
        pub const fn t1cm_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "T1CM_EN"]
        #[inline(always)]
        pub fn set_t1cm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "T8CM_EN"]
        #[inline(always)]
        pub const fn t8cm_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "T8CM_EN"]
        #[inline(always)]
        pub fn set_t8cm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "T20CM_EN"]
        #[inline(always)]
        pub const fn t20cm_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "T20CM_EN"]
        #[inline(always)]
        pub fn set_t20cm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TCMR LOCK"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TCMR LOCK"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Tcmr {
        #[inline(always)]
        fn default() -> Tcmr {
            Tcmr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Calsel {
        #[doc = "VREFOPAMP=3.3% VDDA"]
        PERCENT3_3 = 0x0,
        #[doc = "VREFOPAMP=10% VDDA"]
        PERCENT10 = 0x01,
        #[doc = "VREFOPAMP=50% VDDA"]
        PERCENT50 = 0x02,
        #[doc = "VREFOPAMP=90% VDDA"]
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
        #[doc = "Normal operating mode"]
        NORMAL = 0x0,
        #[doc = "Calibration mode. Non-inverting input connected to calibration reference"]
        CALIBRATION = 0x01,
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
        #[doc = "OpAmp in normal mode"]
        NORMAL = 0x0,
        #[doc = "OpAmp in high speed mode"]
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
    pub enum Opaintoen {
        #[doc = "Output is connected to the output Pin"]
        OUTPUTPIN = 0x0,
        #[doc = "Output is connected internally to ADC channel"]
        ADCCHANNEL = 0x01,
    }
    impl Opaintoen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opaintoen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opaintoen {
        #[inline(always)]
        fn from(val: u8) -> Opaintoen {
            Opaintoen::from_bits(val)
        }
    }
    impl From<Opaintoen> for u8 {
        #[inline(always)]
        fn from(val: Opaintoen) -> u8 {
            Opaintoen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Outcal {
        #[doc = "Non-inverting < inverting"]
        LOW = 0x0,
        #[doc = "Non-inverting > inverting"]
        HIGH = 0x01,
    }
    impl Outcal {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Outcal {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Outcal {
        #[inline(always)]
        fn from(val: u8) -> Outcal {
            Outcal::from_bits(val)
        }
    }
    impl From<Outcal> for u8 {
        #[inline(always)]
        fn from(val: Outcal) -> u8 {
            Outcal::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PgaGain {
        #[doc = "Gain 2"]
        GAIN2 = 0x0,
        #[doc = "Gain 4"]
        GAIN4 = 0x01,
        #[doc = "Gain 8"]
        GAIN8 = 0x02,
        #[doc = "Gain 16"]
        GAIN16 = 0x03,
        #[doc = "Gain 32"]
        GAIN32 = 0x04,
        #[doc = "Gain 64"]
        GAIN64 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Gain 2, input/bias connected to VINM0 or inverting gain"]
        GAIN2_INPUTVINM0 = 0x08,
        #[doc = "Gain 4, input/bias connected to VINM0 or inverting gain"]
        GAIN4_INPUTVINM0 = 0x09,
        #[doc = "Gain 8, input/bias connected to VINM0 or inverting gain"]
        GAIN8_INPUTVINM0 = 0x0a,
        #[doc = "Gain 16, input/bias connected to VINM0 or inverting gain"]
        GAIN16_INPUTVINM0 = 0x0b,
        #[doc = "Gain 32, input/bias connected to VINM0 or inverting gain"]
        GAIN32_INPUTVINM0 = 0x0c,
        #[doc = "Gain 64, input/bias connected to VINM0 or inverting gain"]
        GAIN64_INPUTVINM0 = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "Gain 2, with filtering on VINM0"]
        GAIN2_FILTERINGVINM0 = 0x10,
        #[doc = "Gain 4, with filtering on VINM0"]
        GAIN4_FILTERINGVINM0 = 0x11,
        #[doc = "Gain 8, with filtering on VINM0"]
        GAIN8_FILTERINGVINM0 = 0x12,
        #[doc = "Gain 16, with filtering on VINM0"]
        GAIN16_FILTERINGVINM0 = 0x13,
        #[doc = "Gain 32, with filtering on VINM0"]
        GAIN32_FILTERINGVINM0 = 0x14,
        #[doc = "Gain 64, with filtering on VINM0"]
        GAIN64_FILTERINGVINM0 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        #[doc = "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN2_INPUTVINM0FILTERINGVINM1 = 0x18,
        #[doc = "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN4_INPUTVINM0FILTERINGVINM1 = 0x19,
        #[doc = "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN8_INPUTVINM0FILTERINGVINM1 = 0x1a,
        #[doc = "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN16_INPUTVINM0FILTERINGVINM1 = 0x1b,
        #[doc = "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN32_INPUTVINM0FILTERINGVINM1 = 0x1c,
        #[doc = "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain"]
        GAIN64_INPUTVINM0FILTERINGVINM1 = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl PgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgaGain {
            unsafe { core::mem::transmute(val & 0x1f) }
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
        #[doc = "Factory trim used"]
        FACTORY = 0x0,
        #[doc = "User trim used"]
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
        #[doc = "VINM0 connected to VINM input"]
        VINM0 = 0x0,
        #[doc = "VINM1 connected to VINM input"]
        VINM1 = 0x01,
        #[doc = "Feedback resistor connected to VINM (PGA mode)"]
        PGA = 0x02,
        #[doc = "OpAmp output connected to VINM (Follower mode)"]
        OUTPUT = 0x03,
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
        #[doc = "VINP0 connected to VINP input"]
        VINP0 = 0x0,
        #[doc = "VINP1 connected to VINP input"]
        VINP1 = 0x01,
        #[doc = "VINP2 connected to VINP input"]
        VINP2 = 0x02,
        #[doc = "DAC3_CH1 connected to VINP input"]
        DAC3_CH1 = 0x03,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum VpsSel {
        #[doc = "VINP0 connected to VINP input"]
        VINP0 = 0x0,
        #[doc = "VINP1 connected to VINP input"]
        VINP1 = 0x01,
        #[doc = "VINP2 connected to VINP input"]
        VINP2 = 0x02,
        #[doc = "DAC3_CH1 connected to VINP input"]
        DAC3_CH1 = 0x03,
    }
    impl VpsSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpsSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VpsSel {
        #[inline(always)]
        fn from(val: u8) -> VpsSel {
            VpsSel::from_bits(val)
        }
    }
    impl From<VpsSel> for u8 {
        #[inline(always)]
        fn from(val: VpsSel) -> u8 {
            VpsSel::to_bits(val)
        }
    }
}
