#[doc = "Operational amplifiers"]
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
    pub const fn opamp_csr(self) -> crate::common::Reg<regs::OpampCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "OPAMP control/status register"]
    #[inline(always)]
    pub const fn opamp_tcmr(self) -> crate::common::Reg<regs::OpampTcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPAMP control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampCsr(pub u32);
    impl OpampCsr {
        #[doc = "Operational amplifier Enable"]
        #[inline(always)]
        pub const fn opaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Operational amplifier Enable"]
        #[inline(always)]
        pub fn set_opaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FORCE_VP"]
        #[inline(always)]
        pub const fn force_vp(&self) -> super::vals::OpampCsrForceVp {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::OpampCsrForceVp::from_bits(val as u8)
        }
        #[doc = "FORCE_VP"]
        #[inline(always)]
        pub fn set_force_vp(&mut self, val: super::vals::OpampCsrForceVp) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "VP_SEL"]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::OpampCsrVpSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::OpampCsrVpSel::from_bits(val as u8)
        }
        #[doc = "VP_SEL"]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::OpampCsrVpSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USERTRIM"]
        #[inline(always)]
        pub const fn usertrim(&self) -> super::vals::OpampCsrUsertrim {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OpampCsrUsertrim::from_bits(val as u8)
        }
        #[doc = "USERTRIM"]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: super::vals::OpampCsrUsertrim) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "VM_SEL"]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::OpampCsrVmSel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::OpampCsrVmSel::from_bits(val as u8)
        }
        #[doc = "VM_SEL"]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::OpampCsrVmSel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "OPAHSM"]
        #[inline(always)]
        pub const fn opahsm(&self) -> super::vals::OpampCsrOpahsm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OpampCsrOpahsm::from_bits(val as u8)
        }
        #[doc = "OPAHSM"]
        #[inline(always)]
        pub fn set_opahsm(&mut self, val: super::vals::OpampCsrOpahsm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "OPAINTOEN"]
        #[inline(always)]
        pub const fn opaintoen(&self) -> super::vals::OpampCsrOpaintoen {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::OpampCsrOpaintoen::from_bits(val as u8)
        }
        #[doc = "OPAINTOEN"]
        #[inline(always)]
        pub fn set_opaintoen(&mut self, val: super::vals::OpampCsrOpaintoen) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "CALON"]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CALON"]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CALSEL"]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::OpampCsrCalsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::OpampCsrCalsel::from_bits(val as u8)
        }
        #[doc = "CALSEL"]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::OpampCsrCalsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "PGA_GAIN"]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::OpampCsrPgaGain {
            let val = (self.0 >> 14usize) & 0x1f;
            super::vals::OpampCsrPgaGain::from_bits(val as u8)
        }
        #[doc = "PGA_GAIN"]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::OpampCsrPgaGain) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val.to_bits() as u32) & 0x1f) << 14usize);
        }
        #[doc = "TRIMOFFSETP"]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "TRIMOFFSETP"]
        #[inline(always)]
        pub fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "TRIMOFFSETN"]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "TRIMOFFSETN"]
        #[inline(always)]
        pub fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "CALOUT"]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "CALOUT"]
        #[inline(always)]
        pub fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::OpampCsrLock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpampCsrLock::from_bits(val as u8)
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: super::vals::OpampCsrLock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OpampCsr {
        #[inline(always)]
        fn default() -> OpampCsr {
            OpampCsr(0)
        }
    }
    #[doc = "OPAMP timer controlled mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampTcmr(pub u32);
    impl OpampTcmr {
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
        pub const fn vps_sel(&self) -> super::vals::OpampTcmrVpsSel {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::OpampTcmrVpsSel::from_bits(val as u8)
        }
        #[doc = "VPS_SEL"]
        #[inline(always)]
        pub fn set_vps_sel(&mut self, val: super::vals::OpampTcmrVpsSel) {
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
        #[doc = "LOCK"]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::OpampTcmrLock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpampTcmrLock::from_bits(val as u8)
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: super::vals::OpampTcmrLock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OpampTcmr {
        #[inline(always)]
        fn default() -> OpampTcmr {
            OpampTcmr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrCalsel {
        #[doc = "0.033*VDDA applied to OPAMP inputs during calibration"]
        PERCENT3_3 = 0,
        #[doc = "0.1*VDDA applied to OPAMP inputs during calibration"]
        PERCENT10 = 0x01,
        #[doc = "0.5*VDDA applied to OPAMP inputs during calibration"]
        PERCENT50 = 0x02,
        #[doc = "0.9*VDDA applied to OPAMP inputs during calibration"]
        PERCENT90 = 0x03,
    }
    impl OpampCsrCalsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrCalsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrCalsel {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrCalsel {
            OpampCsrCalsel::from_bits(val)
        }
    }
    impl From<OpampCsrCalsel> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrCalsel) -> u8 {
            OpampCsrCalsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrForceVp {
        #[doc = "Non-inverting input connected configured inputs"]
        NORMAL = 0,
        #[doc = "Non-inverting input connected to calibration reference voltage"]
        CALIBRATIONVERIFICATION = 0x01,
    }
    impl OpampCsrForceVp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrForceVp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrForceVp {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrForceVp {
            OpampCsrForceVp::from_bits(val)
        }
    }
    impl From<OpampCsrForceVp> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrForceVp) -> u8 {
            OpampCsrForceVp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrLock {
        #[doc = "CSR is read-write"]
        READWRITE = 0,
        #[doc = "CSR is read-only, can only be cleared by system reset"]
        READONLY = 0x01,
    }
    impl OpampCsrLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrLock {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrLock {
            OpampCsrLock::from_bits(val)
        }
    }
    impl From<OpampCsrLock> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrLock) -> u8 {
            OpampCsrLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrOpahsm {
        #[doc = "OpAmp in normal mode"]
        NORMAL = 0,
        #[doc = "OpAmp in high speed mode"]
        HIGHSPEED = 0x01,
    }
    impl OpampCsrOpahsm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrOpahsm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrOpahsm {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrOpahsm {
            OpampCsrOpahsm::from_bits(val)
        }
    }
    impl From<OpampCsrOpahsm> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrOpahsm) -> u8 {
            OpampCsrOpahsm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrOpaintoen {
        #[doc = "Output is connected to the output Pin"]
        OUTPUTPIN = 0,
        #[doc = "Output is connected internally to ADC channel"]
        ADCCHANNEL = 0x01,
    }
    impl OpampCsrOpaintoen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrOpaintoen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrOpaintoen {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrOpaintoen {
            OpampCsrOpaintoen::from_bits(val)
        }
    }
    impl From<OpampCsrOpaintoen> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrOpaintoen) -> u8 {
            OpampCsrOpaintoen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrPgaGain {
        #[doc = "Gain 2"]
        GAIN2 = 0,
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
    impl OpampCsrPgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrPgaGain {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrPgaGain {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrPgaGain {
            OpampCsrPgaGain::from_bits(val)
        }
    }
    impl From<OpampCsrPgaGain> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrPgaGain) -> u8 {
            OpampCsrPgaGain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrUsertrim {
        #[doc = "Factory trim used"]
        FACTORY = 0,
        #[doc = "User trim used"]
        USER = 0x01,
    }
    impl OpampCsrUsertrim {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrUsertrim {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrUsertrim {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrUsertrim {
            OpampCsrUsertrim::from_bits(val)
        }
    }
    impl From<OpampCsrUsertrim> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrUsertrim) -> u8 {
            OpampCsrUsertrim::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrVmSel {
        #[doc = "VINM0 connected to VINM input"]
        VINM0 = 0,
        #[doc = "VINM1 connected to VINM input"]
        VINM1 = 0x01,
        #[doc = "Feedback resistor connected to VINM (PGA mode)"]
        PGA = 0x02,
        #[doc = "OpAmp output connected to VINM (Follower mode)"]
        OUTPUT = 0x03,
    }
    impl OpampCsrVmSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrVmSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrVmSel {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrVmSel {
            OpampCsrVmSel::from_bits(val)
        }
    }
    impl From<OpampCsrVmSel> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrVmSel) -> u8 {
            OpampCsrVmSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampCsrVpSel {
        #[doc = "VINP0 connected to VINP input"]
        VINP0 = 0,
        #[doc = "VINP1 connected to VINP input"]
        VINP1 = 0x01,
        #[doc = "VINP2 connected to VINP input"]
        VINP2 = 0x02,
        #[doc = "DAC3_CH1 connected to VINP input"]
        DAC3_CH1 = 0x03,
    }
    impl OpampCsrVpSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampCsrVpSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampCsrVpSel {
        #[inline(always)]
        fn from(val: u8) -> OpampCsrVpSel {
            OpampCsrVpSel::from_bits(val)
        }
    }
    impl From<OpampCsrVpSel> for u8 {
        #[inline(always)]
        fn from(val: OpampCsrVpSel) -> u8 {
            OpampCsrVpSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampTcmrLock {
        #[doc = "TCMR is read-write"]
        READWRITE = 0,
        #[doc = "TCMR is read-only, can only be cleared by system reset"]
        READONLY = 0x01,
    }
    impl OpampTcmrLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampTcmrLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampTcmrLock {
        #[inline(always)]
        fn from(val: u8) -> OpampTcmrLock {
            OpampTcmrLock::from_bits(val)
        }
    }
    impl From<OpampTcmrLock> for u8 {
        #[inline(always)]
        fn from(val: OpampTcmrLock) -> u8 {
            OpampTcmrLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OpampTcmrVpsSel {
        #[doc = "VINP0 connected to VINP input"]
        VINP0 = 0,
        #[doc = "VINP1 connected to VINP input"]
        VINP1 = 0x01,
        #[doc = "VINP2 connected to VINP input"]
        VINP2 = 0x02,
        #[doc = "DAC3_CH1 connected to VINP input"]
        DAC3_CH1 = 0x03,
    }
    impl OpampTcmrVpsSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpampTcmrVpsSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpampTcmrVpsSel {
        #[inline(always)]
        fn from(val: u8) -> OpampTcmrVpsSel {
            OpampTcmrVpsSel::from_bits(val)
        }
    }
    impl From<OpampTcmrVpsSel> for u8 {
        #[inline(always)]
        fn from(val: OpampTcmrVpsSel) -> u8 {
            OpampTcmrVpsSel::to_bits(val)
        }
    }
}
