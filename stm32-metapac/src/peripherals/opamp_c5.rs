#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OPAMP register block."]
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
    pub const fn opamp_csr(self) -> crate::common::Reg<regs::OpampCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OPAMP timer-controlled mode register."]
    #[inline(always)]
    pub const fn opamp_tcmr(self) -> crate::common::Reg<regs::OpampTcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "OPAMP control/status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampCsr(pub u32);
    impl OpampCsr {
        #[doc = "Operational amplifier enable."]
        #[must_use]
        #[inline(always)]
        pub const fn opaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Operational amplifier enable."]
        #[inline(always)]
        pub const fn set_opaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force internal reference on noninverting input."]
        #[must_use]
        #[inline(always)]
        pub const fn force_vp(&self) -> super::vals::ForceVp {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::ForceVp::from_bits(val as u8)
        }
        #[doc = "Force internal reference on noninverting input."]
        #[inline(always)]
        pub const fn set_force_vp(&mut self, val: super::vals::ForceVp) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Noninverting input primary selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vp_sel(&self) -> super::vals::VpSel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::VpSel::from_bits(val as u8)
        }
        #[doc = "Noninverting input primary selection."]
        #[inline(always)]
        pub const fn set_vp_sel(&mut self, val: super::vals::VpSel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "User trimming enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usertrim(&self) -> super::vals::Usertrim {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Usertrim::from_bits(val as u8)
        }
        #[doc = "User trimming enable."]
        #[inline(always)]
        pub const fn set_usertrim(&mut self, val: super::vals::Usertrim) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Inverting input primary selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vm_sel(&self) -> super::vals::VmSel {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::VmSel::from_bits(val as u8)
        }
        #[doc = "Inverting input primary selection."]
        #[inline(always)]
        pub const fn set_vm_sel(&mut self, val: super::vals::VmSel) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Operational amplifier high-speed mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opahsm(&self) -> super::vals::Opahsm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Opahsm::from_bits(val as u8)
        }
        #[doc = "Operational amplifier high-speed mode."]
        #[inline(always)]
        pub const fn set_opahsm(&mut self, val: super::vals::Opahsm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Operational amplifier internal output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn opaintoen(&self) -> super::vals::Opaintoen {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Opaintoen::from_bits(val as u8)
        }
        #[doc = "Operational amplifier internal output enable."]
        #[inline(always)]
        pub const fn set_opaintoen(&mut self, val: super::vals::Opaintoen) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Calibration mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration mode enable."]
        #[inline(always)]
        pub const fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Calibration selection."]
        #[must_use]
        #[inline(always)]
        pub const fn calsel(&self) -> super::vals::Calsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Calsel::from_bits(val as u8)
        }
        #[doc = "Calibration selection."]
        #[inline(always)]
        pub const fn set_calsel(&mut self, val: super::vals::Calsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Operational amplifier programmable gain and PGA flavor primary control."]
        #[must_use]
        #[inline(always)]
        pub const fn pga_gain(&self) -> super::vals::PgaGain {
            let val = (self.0 >> 14usize) & 0x1f;
            super::vals::PgaGain::from_bits(val as u8)
        }
        #[doc = "Operational amplifier programmable gain and PGA flavor primary control."]
        #[inline(always)]
        pub const fn set_pga_gain(&mut self, val: super::vals::PgaGain) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val.to_bits() as u32) & 0x1f) << 14usize);
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[must_use]
        #[inline(always)]
        pub const fn trimoffsetp(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for PMOS differential pairs."]
        #[inline(always)]
        pub const fn set_trimoffsetp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "Trim for NMOS differential pairs."]
        #[must_use]
        #[inline(always)]
        pub const fn trimoffsetn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Trim for NMOS differential pairs."]
        #[inline(always)]
        pub const fn set_trimoffsetn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "OPAMP calibration reference voltage output control."]
        #[must_use]
        #[inline(always)]
        pub const fn tstref(&self) -> super::vals::Tstref {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Tstref::from_bits(val as u8)
        }
        #[doc = "OPAMP calibration reference voltage output control."]
        #[inline(always)]
        pub const fn set_tstref(&mut self, val: super::vals::Tstref) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Operational amplifier calibration output."]
        #[must_use]
        #[inline(always)]
        pub const fn calout(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Operational amplifier calibration output."]
        #[inline(always)]
        pub const fn set_calout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "OPAMP_CSR register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::OpampCsrLock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpampCsrLock::from_bits(val as u8)
        }
        #[doc = "OPAMP_CSR register lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: super::vals::OpampCsrLock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OpampCsr {
        #[inline(always)]
        fn default() -> OpampCsr {
            OpampCsr(0)
        }
    }
    impl core::fmt::Debug for OpampCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpampCsr")
                .field("opaen", &self.opaen())
                .field("force_vp", &self.force_vp())
                .field("vp_sel", &self.vp_sel())
                .field("usertrim", &self.usertrim())
                .field("vm_sel", &self.vm_sel())
                .field("opahsm", &self.opahsm())
                .field("opaintoen", &self.opaintoen())
                .field("calon", &self.calon())
                .field("calsel", &self.calsel())
                .field("pga_gain", &self.pga_gain())
                .field("trimoffsetp", &self.trimoffsetp())
                .field("trimoffsetn", &self.trimoffsetn())
                .field("tstref", &self.tstref())
                .field("calout", &self.calout())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpampCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpampCsr {{ opaen: {=bool:?}, force_vp: {:?}, vp_sel: {:?}, usertrim: {:?}, vm_sel: {:?}, opahsm: {:?}, opaintoen: {:?}, calon: {=bool:?}, calsel: {:?}, pga_gain: {:?}, trimoffsetp: {=u8:?}, trimoffsetn: {=u8:?}, tstref: {:?}, calout: {=bool:?}, lock: {:?} }}",
                self.opaen(),
                self.force_vp(),
                self.vp_sel(),
                self.usertrim(),
                self.vm_sel(),
                self.opahsm(),
                self.opaintoen(),
                self.calon(),
                self.calsel(),
                self.pga_gain(),
                self.trimoffsetp(),
                self.trimoffsetn(),
                self.tstref(),
                self.calout(),
                self.lock()
            )
        }
    }
    #[doc = "OPAMP timer-controlled mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpampTcmr(pub u32);
    impl OpampTcmr {
        #[doc = "OPAMP inverting input secondary selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vms_sel(&self) -> super::vals::VmsSel {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::VmsSel::from_bits(val as u8)
        }
        #[doc = "OPAMP inverting input secondary selection."]
        #[inline(always)]
        pub const fn set_vms_sel(&mut self, val: super::vals::VmsSel) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "OPAMP noninverting input secondary selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vps_sel(&self) -> super::vals::VpsSel {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::VpsSel::from_bits(val as u8)
        }
        #[doc = "OPAMP noninverting input secondary selection."]
        #[inline(always)]
        pub const fn set_vps_sel(&mut self, val: super::vals::VpsSel) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Timer toggle signal selection for operational amplifier input control."]
        #[must_use]
        #[inline(always)]
        pub const fn timcm_sel(&self) -> super::vals::TimcmSel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::TimcmSel::from_bits(val as u8)
        }
        #[doc = "Timer toggle signal selection for operational amplifier input control."]
        #[inline(always)]
        pub const fn set_timcm_sel(&mut self, val: super::vals::TimcmSel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "Operational amplifier programmable gain and PGA flavor secondary control."]
        #[must_use]
        #[inline(always)]
        pub const fn pgas_gain(&self) -> super::vals::PgasGain {
            let val = (self.0 >> 8usize) & 0x1f;
            super::vals::PgasGain::from_bits(val as u8)
        }
        #[doc = "Operational amplifier programmable gain and PGA flavor secondary control."]
        #[inline(always)]
        pub const fn set_pgas_gain(&mut self, val: super::vals::PgasGain) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
        }
        #[doc = "Timer toggle signal selection for programmable gain control."]
        #[must_use]
        #[inline(always)]
        pub const fn timpga_sel(&self) -> super::vals::TimpgaSel {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::TimpgaSel::from_bits(val as u8)
        }
        #[doc = "Timer toggle signal selection for programmable gain control."]
        #[inline(always)]
        pub const fn set_timpga_sel(&mut self, val: super::vals::TimpgaSel) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "OPAMP_TCMR register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::OpampTcmrLock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OpampTcmrLock::from_bits(val as u8)
        }
        #[doc = "OPAMP_TCMR register lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: super::vals::OpampTcmrLock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OpampTcmr {
        #[inline(always)]
        fn default() -> OpampTcmr {
            OpampTcmr(0)
        }
    }
    impl core::fmt::Debug for OpampTcmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpampTcmr")
                .field("vms_sel", &self.vms_sel())
                .field("vps_sel", &self.vps_sel())
                .field("timcm_sel", &self.timcm_sel())
                .field("pgas_gain", &self.pgas_gain())
                .field("timpga_sel", &self.timpga_sel())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpampTcmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpampTcmr {{ vms_sel: {:?}, vps_sel: {:?}, timcm_sel: {:?}, pgas_gain: {:?}, timpga_sel: {:?}, lock: {:?} }}",
                self.vms_sel(),
                self.vps_sel(),
                self.timcm_sel(),
                self.pgas_gain(),
                self.timpga_sel(),
                self.lock()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calsel {
        #[doc = "0."]
        B0x0 = 0x0,
        #[doc = "0."]
        B0x1 = 0x01,
        #[doc = "0."]
        B0x2 = 0x02,
        #[doc = "0."]
        B0x3 = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ForceVp {
        #[doc = "Do not force (normal operation)."]
        B0x0 = 0x0,
        #[doc = "Force."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Opahsm {
        #[doc = "Normal speed."]
        B0x0 = 0x0,
        #[doc = "High speed."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Opaintoen {
        #[doc = "OPAMP_VOUT pin."]
        B0x0 = 0x0,
        #[doc = "ADC/COMP channel."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpampCsrLock {
        #[doc = "OPAMP_CSR is read-write."]
        B0x0 = 0x0,
        #[doc = "OPAMP_CSR is read-only."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OpampTcmrLock {
        #[doc = "Read-write."]
        B0x0 = 0x0,
        #[doc = "Read-only (OPAMP_TCMR locked)."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PgaGain {
        #[doc = "gain 2."]
        B0x0 = 0x0,
        #[doc = "gain 4."]
        B0x1 = 0x01,
        #[doc = "gain 8."]
        B0x2 = 0x02,
        #[doc = "gain 16."]
        B0x3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "gain -1 / gain 2 with bias on OPAMP_VINM0."]
        B0x8 = 0x08,
        #[doc = "gain -3 / gain 4 with bias on OPAMP_VINM0."]
        B0x9 = 0x09,
        #[doc = "gain -7 / gain 8 with bias on OPAMP_VINM0."]
        B0xA = 0x0a,
        #[doc = "gain -15 / gain 16 with bias on OPAMP_VINM0."]
        B0xB = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "gain 2 with filtering on OPAMP_VINM0."]
        B0x10 = 0x10,
        #[doc = "gain 4 with filtering on OPAMP_VINM0."]
        B0x11 = 0x11,
        #[doc = "gain 8 with filtering on OPAMP_VINM0."]
        B0x12 = 0x12,
        #[doc = "gain 16 with filtering on OPAMP_VINM0."]
        B0x13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        #[doc = "gain -1 / gain 2 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x18 = 0x18,
        #[doc = "gain -3 / gain 4 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x19 = 0x19,
        #[doc = "gain -7 / gain 8 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x1a = 0x1a,
        #[doc = "gain -15 / gain 16 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PgasGain {
        #[doc = "gain 2."]
        B0x0 = 0x0,
        #[doc = "gain 4."]
        B0x1 = 0x01,
        #[doc = "gain 8."]
        B0x2 = 0x02,
        #[doc = "gain 16."]
        B0x3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "gain -1 / gain 2 with bias on OPAMP_VINM0."]
        B0x8 = 0x08,
        #[doc = "gain -3 / gain 4 with bias on OPAMP_VINM0."]
        B0x9 = 0x09,
        #[doc = "gain -7 / gain 8 with bias on OPAMP_VINM0."]
        B0xA = 0x0a,
        #[doc = "gain -15 / gain 16 with bias on OPAMP_VINM0."]
        B0xB = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        #[doc = "gain 2 with filtering on OPAMP_VINM0."]
        B0x10 = 0x10,
        #[doc = "gain 4 with filtering on OPAMP_VINM0."]
        B0x11 = 0x11,
        #[doc = "gain 8 with filtering on OPAMP_VINM0."]
        B0x12 = 0x12,
        #[doc = "gain 16 with filtering on OPAMP_VINM0."]
        B0x13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        #[doc = "gain -1 / gain 2 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x18 = 0x18,
        #[doc = "gain -3 / gain 4 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x19 = 0x19,
        #[doc = "gain -7 / gain 8 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x1a = 0x1a,
        #[doc = "gain -15 / gain of 16 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."]
        B0x1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl PgasGain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PgasGain {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PgasGain {
        #[inline(always)]
        fn from(val: u8) -> PgasGain {
            PgasGain::from_bits(val)
        }
    }
    impl From<PgasGain> for u8 {
        #[inline(always)]
        fn from(val: PgasGain) -> u8 {
            PgasGain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TimcmSel {
        #[doc = "None (input configuration permanently controlled through VP_SEL\\[1:0\\]
and VM_SEL)."]
        B0x0 = 0x0,
        #[doc = "opamp_tc1."]
        B0x1 = 0x01,
        #[doc = "opamp_tc2."]
        B0x2 = 0x02,
        #[doc = "opamp_tc3."]
        B0x3 = 0x03,
        #[doc = "opamp_tc4."]
        B0x4 = 0x04,
        #[doc = "opamp_tc5."]
        B0x5 = 0x05,
        #[doc = "opamp_tc6."]
        B0x6 = 0x06,
        #[doc = "opamp_tc7."]
        B0x7 = 0x07,
    }
    impl TimcmSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TimcmSel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TimcmSel {
        #[inline(always)]
        fn from(val: u8) -> TimcmSel {
            TimcmSel::from_bits(val)
        }
    }
    impl From<TimcmSel> for u8 {
        #[inline(always)]
        fn from(val: TimcmSel) -> u8 {
            TimcmSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TimpgaSel {
        #[doc = "None (gain permanently controlled through PGA_GAIN\\[4:0\\]
and VM_SEL\\[1:0\\])."]
        B0x0 = 0x0,
        #[doc = "opamp_tp1."]
        B0x1 = 0x01,
        #[doc = "opamp_tp2."]
        B0x2 = 0x02,
        #[doc = "opamp_tp3."]
        B0x3 = 0x03,
        #[doc = "opamp_tp4."]
        B0x4 = 0x04,
        #[doc = "opamp_tp5."]
        B0x5 = 0x05,
        #[doc = "opamp_tp6."]
        B0x6 = 0x06,
        #[doc = "opamp_tp7."]
        B0x7 = 0x07,
    }
    impl TimpgaSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TimpgaSel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TimpgaSel {
        #[inline(always)]
        fn from(val: u8) -> TimpgaSel {
            TimpgaSel::from_bits(val)
        }
    }
    impl From<TimpgaSel> for u8 {
        #[inline(always)]
        fn from(val: TimpgaSel) -> u8 {
            TimpgaSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tstref {
        #[doc = "Does not output calibration reference voltage."]
        B0x0 = 0x0,
        #[doc = "Outputs calibration reference voltage."]
        B0x1 = 0x01,
    }
    impl Tstref {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tstref {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tstref {
        #[inline(always)]
        fn from(val: u8) -> Tstref {
            Tstref::from_bits(val)
        }
    }
    impl From<Tstref> for u8 {
        #[inline(always)]
        fn from(val: Tstref) -> u8 {
            Tstref::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usertrim {
        #[doc = "Disable (trimming by user not possible)."]
        B0x0 = 0x0,
        #[doc = "Enable (trimming by user possible)."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VmSel {
        #[doc = "OPAMP_VINM0 pin."]
        B0x0 = 0x0,
        #[doc = "OPAMP_VINM1 pin."]
        B0x1 = 0x01,
        #[doc = "Internal resistor divider middle point."]
        B0x2 = 0x02,
        #[doc = "Operational amplifier output."]
        B0x3 = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VmsSel {
        #[doc = "Internal resistor divider middle point (operation as PGA)."]
        B0x0 = 0x0,
        #[doc = "Operational amplifier output (operation as follower)."]
        B0x1 = 0x01,
    }
    impl VmsSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> VmsSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for VmsSel {
        #[inline(always)]
        fn from(val: u8) -> VmsSel {
            VmsSel::from_bits(val)
        }
    }
    impl From<VmsSel> for u8 {
        #[inline(always)]
        fn from(val: VmsSel) -> u8 {
            VmsSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VpSel {
        #[doc = "OPAMP_VINP0 pin."]
        B0x0 = 0x0,
        #[doc = "OPAMP_VINP1 pin."]
        B0x1 = 0x01,
        #[doc = "OPAMP_VINP2 pin."]
        B0x2 = 0x02,
        #[doc = "OPAMP_VINP3 pin or DAC output channel."]
        B0x3 = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum VpsSel {
        #[doc = "OPAMP_VINP0."]
        B0x0 = 0x0,
        #[doc = "OPAMP_VINP1."]
        B0x1 = 0x01,
        #[doc = "OPAMP_VINP2."]
        B0x2 = 0x02,
        #[doc = "OPAMP_VINP3 pin or DAC output channel."]
        B0x3 = 0x03,
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
