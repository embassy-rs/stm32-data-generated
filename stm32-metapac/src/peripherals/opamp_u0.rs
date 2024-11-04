#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPAMP address block description."]
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
    #[doc = "OPAMP control/status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OPAMP offset trimming register in normal mode."]
    #[inline(always)]
    pub const fn otr(self) -> crate::common::Reg<regs::Otr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "OPAMP offset trimming register in low-power mode."]
    #[inline(always)]
    pub const fn lpotr(self) -> crate::common::Reg<regs::Lpotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPAMP control/status register."]
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
        #[doc = "Operational amplifier Low Power Mode. The operational amplifier must be disable to change this configuration."]
        #[inline(always)]
        pub const fn opalpm(&self) -> super::vals::Opalpm {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Opalpm::from_bits(val as u8)
        }
        #[doc = "Operational amplifier Low Power Mode. The operational amplifier must be disable to change this configuration."]
        #[inline(always)]
        pub fn set_opalpm(&mut self, val: super::vals::Opalpm) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Operational amplifier PGA mode."]
        #[inline(always)]
        pub const fn opamode(&self) -> super::vals::Opamode {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Opamode::from_bits(val as u8)
        }
        #[doc = "Operational amplifier PGA mode."]
        #[inline(always)]
        pub fn set_opamode(&mut self, val: super::vals::Opamode) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Operational amplifier Programmable amplifier gain value."]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "Operational amplifier Programmable amplifier gain value."]
        #[inline(always)]
        pub fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Inverting input selection. These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)."]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "Inverting input selection. These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)."]
        #[inline(always)]
        pub fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Non inverted input selection."]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "Non inverted input selection."]
        #[inline(always)]
        pub fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Calibration mode enabled."]
        #[inline(always)]
        pub const fn calon(&self) -> super::vals::Calon {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Calon::from_bits(val as u8)
        }
        #[doc = "Calibration mode enabled."]
        #[inline(always)]
        pub fn set_calon(&mut self, val: super::vals::Calon) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Calibration selection."]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection."]
        #[inline(always)]
        pub fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
        #[inline(always)]
        pub const fn usertrim(&self) -> super::vals::Usertrim {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Usertrim::from_bits(val as u8)
        }
        #[doc = "allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power."]
        #[inline(always)]
        pub fn set_usertrim(&mut self, val: super::vals::Usertrim) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Operational amplifier calibration output During calibration mode offset is trimmed when this signal toggle."]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Operational amplifier calibration output During calibration mode offset is trimmed when this signal toggle."]
        #[inline(always)]
        pub fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
        #[inline(always)]
        pub const fn opa_range(&self) -> super::vals::OpaRange {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpaRange::from_bits(val as u8)
        }
        #[doc = "Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product."]
        #[inline(always)]
        pub fn set_opa_range(&mut self, val: super::vals::OpaRange) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "OPAMP offset trimming register in low-power mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpotr(pub u32);
    impl Lpotr {
        #[doc = "Low-power mode trim for NMOS differential pairs."]
        #[inline(always)]
        pub const fn trimlpoffsetn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Low-power mode trim for NMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimlpoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Low-power mode trim for PMOS differential pairs."]
        #[inline(always)]
        pub const fn trimlpoffsetp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Low-power mode trim for PMOS differential pairs."]
        #[inline(always)]
        pub fn set_trimlpoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Lpotr {
        #[inline(always)]
        fn default() -> Lpotr {
            Lpotr(0)
        }
    }
    #[doc = "OPAMP offset trimming register in normal mode."]
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
        #[doc = "Normal mode."]
        NORMAL = 0x0,
        #[doc = "Calibration mode (all switches opened by HW)."]
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
    pub enum Calsel {
        #[doc = "NMOS calibration (200mV applied on OPAMP inputs)."]
        NMOS = 0x0,
        #[doc = "PMOS calibration (VDDA-200mV applied on OPAMP inputs)."]
        PMOS = 0x01,
    }
    impl Calsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calsel {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum OpaRange {
        #[doc = "Low range (VDDA < 2.4V)."]
        LOW = 0x0,
        #[doc = "High range (VDDA > 2.4V)."]
        HIGH = 0x01,
    }
    impl OpaRange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OpaRange {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OpaRange {
        #[inline(always)]
        fn from(val: u8) -> OpaRange {
            OpaRange::from_bits(val)
        }
    }
    impl From<OpaRange> for u8 {
        #[inline(always)]
        fn from(val: OpaRange) -> u8 {
            OpaRange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Opalpm {
        #[doc = "operational amplifier in normal mode."]
        NORMAL = 0x0,
        #[doc = "operational amplifier in low-power mode."]
        LOWPOWER = 0x01,
    }
    impl Opalpm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opalpm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opalpm {
        #[inline(always)]
        fn from(val: u8) -> Opalpm {
            Opalpm::from_bits(val)
        }
    }
    impl From<Opalpm> for u8 {
        #[inline(always)]
        fn from(val: Opalpm) -> u8 {
            Opalpm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Opamode {
        #[doc = "internal PGA disable."]
        DISABLE = 0x0,
        #[doc = "internal PGA disable. (Duplicate)"]
        DISABLE2 = 0x01,
        #[doc = "internal PGA enable, gain programmed in PGA_GAIN."]
        ENABLE = 0x02,
        #[doc = "internal follower."]
        FOLLOWER = 0x03,
    }
    impl Opamode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opamode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opamode {
        #[inline(always)]
        fn from(val: u8) -> Opamode {
            Opamode::from_bits(val)
        }
    }
    impl From<Opamode> for u8 {
        #[inline(always)]
        fn from(val: Opamode) -> u8 {
            Opamode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PgaGain {
        #[doc = "internal PGA Gain 2."]
        GAIN2 = 0x0,
        #[doc = "internal PGA Gain 4."]
        GAIN4 = 0x01,
        #[doc = "internal PGA Gain 8."]
        GAIN8 = 0x02,
        #[doc = "internal PGA Gain 16."]
        GAIN16 = 0x03,
    }
    impl PgaGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgaGain {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "Factory trim code used."]
        FACTORY = 0x0,
        #[doc = "User trim code used."]
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
        #[doc = "GPIO connected to VINM (valid also in PGA mode for filtering)."]
        VINM = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)"]
        NOTCONNECTED = 0x02,
        _RESERVED_3 = 0x03,
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
        #[doc = "GPIO connected to VINP."]
        VINP = 0x0,
        #[doc = "DAC connected to VINP."]
        DAC = 0x01,
    }
    impl VpSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VpSel {
            unsafe { core::mem::transmute(val & 0x01) }
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
