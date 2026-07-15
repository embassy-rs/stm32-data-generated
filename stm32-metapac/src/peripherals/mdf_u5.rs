#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Multi-function digital filter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdf {
    ptr: *mut u8,
}
unsafe impl Send for Mdf {}
unsafe impl Sync for Mdf {}
impl Mdf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MDF global control register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MDF clock generator control register."]
    #[inline(always)]
    pub const fn ckgcr(self) -> crate::common::Reg<regs::Ckgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "This register is used to control the serial interfaces (SITFx)."]
    #[inline(always)]
    pub const fn sitfcr(self, n: usize) -> crate::common::Reg<regs::Sitfcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD."]
    #[inline(always)]
    pub const fn bsmxcr(self, n: usize) -> crate::common::Reg<regs::Bsmxcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to control the digital filter x."]
    #[inline(always)]
    pub const fn dfltcr(self, n: usize) -> crate::common::Reg<regs::Dfltcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to control the main CIC filter."]
    #[inline(always)]
    pub const fn dfltcicr(self, n: usize) -> crate::common::Reg<regs::Dfltcicr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize + n * 128usize) as _) }
    }
    #[doc = "This register is used to control the reshape and HPF filters."]
    #[inline(always)]
    pub const fn dfltrsfr(self, n: usize) -> crate::common::Reg<regs::Dfltrsfr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to the integrator (INT) settings."]
    #[inline(always)]
    pub const fn dfltintr(self, n: usize) -> crate::common::Reg<regs::Dfltintr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to configure the Out-of Limit Detector function."]
    #[inline(always)]
    pub const fn oldcr(self, n: usize) -> crate::common::Reg<regs::Oldcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize + n * 128usize) as _) }
    }
    #[doc = "This register is used for the adjustment of the Out-off Limit low threshold."]
    #[inline(always)]
    pub const fn oldthlr(self, n: usize) -> crate::common::Reg<regs::Oldthlr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize + n * 128usize) as _) }
    }
    #[doc = "This register is used for the adjustment of the Out-off Limit high threshold."]
    #[inline(always)]
    pub const fn oldthhr(self, n: usize) -> crate::common::Reg<regs::Oldthhr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 128usize) as _) }
    }
    #[doc = "This register is used for the adjustment stream delays."]
    #[inline(always)]
    pub const fn dlycr(self, n: usize) -> crate::common::Reg<regs::Dlycr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize + n * 128usize) as _) }
    }
    #[doc = "This register is used for the adjustment stream delays."]
    #[inline(always)]
    pub const fn scdcr(self, n: usize) -> crate::common::Reg<regs::Scdcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize + n * 128usize) as _) }
    }
    #[doc = "This register is used for allowing or not the events to generate an interrupt."]
    #[inline(always)]
    pub const fn dfltier(self, n: usize) -> crate::common::Reg<regs::Dfltier, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize + n * 128usize) as _) }
    }
    #[doc = "MDF DFLT0 interrupt status register 0."]
    #[inline(always)]
    pub const fn dfltisr(self, n: usize) -> crate::common::Reg<regs::Dfltisr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize + n * 128usize) as _) }
    }
    #[doc = "This register contains the offset compensation value."]
    #[inline(always)]
    pub const fn oeccr(self, n: usize) -> crate::common::Reg<regs::Oeccr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize + n * 128usize) as _) }
    }
    #[doc = "This register is used to read the data processed by each digital filter in snapshot mode."]
    #[inline(always)]
    pub const fn snpsdr(self, n: usize) -> crate::common::Reg<regs::Snpsdr, crate::common::R> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize + n * 128usize) as _) }
    }
    #[doc = "This register is used to read the data processed by each digital filter."]
    #[inline(always)]
    pub const fn dfltdr(self, n: usize) -> crate::common::Reg<regs::Dfltdr, crate::common::R> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsmxcr(pub u32);
    impl Bsmxcr {
        #[doc = "Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn bssel(&self) -> super::vals::Bssel {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Bssel::from_bits(val as u8)
        }
        #[doc = "Bitstream Selection Set and cleared by software. This field is used to select the bitstream to be processed for the digital filter x and for the SCDx. The size of this field depends on the number of DFLTx instantiated. If the BSSEL is selecting an input which is not instantiated, the MDF will select the valid stream bs\\[x\\]_F having the higher index number. - 00000: The bitstream bs\\[0\\]_R is provided to DFLTx and SCDx - 00001: The bitstream bs\\[0\\]_F is provided to DFLTx and SCDx - 00010: The bitstream bs\\[1\\]_R is provided to DFLTx and SCDx (if instantiated) - 00011: The bitstream bs\\[1\\]_F is provided to DFLTx and SCDx (if instantiated) ... - 11110: The bitstream bs\\[15\\]_R is provided to DFLTx and SCDx (if instantiated) - 11111: The bitstream bs\\[15\\]_F is provided to DFLTx and SCDx (if instantiated) This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_bssel(&mut self, val: super::vals::Bssel) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when the BSMXACTIVE is set . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set in order update BSSEL\\[4:0\\]
field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured."]
        #[must_use]
        #[inline(always)]
        pub const fn bsmxactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BSMX Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the BSMX is effectively enabled (active) or not. BSSEL\\[4:0\\]
can only be updated when the BSMXACTIVE is set . The BSMXACTIVE flag is a logical between OLDACTIVE, DFLTACTIVE, and SCDACTIVE flags. Both of them must be set in order update BSSEL\\[4:0\\]
field. - 0: The BSMX is not active, and can be configured if needed - 1: The BSMX is active, and protected fields cannot be configured."]
        #[inline(always)]
        pub const fn set_bsmxactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bsmxcr {
        #[inline(always)]
        fn default() -> Bsmxcr {
            Bsmxcr(0)
        }
    }
    impl core::fmt::Debug for Bsmxcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsmxcr")
                .field("bssel", &self.bssel())
                .field("bsmxactive", &self.bsmxactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsmxcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsmxcr {{ bssel: {:?}, bsmxactive: {=bool:?} }}",
                self.bssel(),
                self.bsmxactive()
            )
        }
    }
    #[doc = "MDF clock generator control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckgcr(pub u32);
    impl Ckgcr {
        #[doc = "CKGDEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ckgden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CKGDEN."]
        #[inline(always)]
        pub const fn set_ckgden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CCK0EN."]
        #[must_use]
        #[inline(always)]
        pub const fn cck0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CCK0EN."]
        #[inline(always)]
        pub const fn set_cck0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CCK1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn cck1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CCK1EN."]
        #[inline(always)]
        pub const fn set_cck1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CKGMOD."]
        #[must_use]
        #[inline(always)]
        pub const fn ckgmod(&self) -> super::vals::Ckgmod {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Ckgmod::from_bits(val as u8)
        }
        #[doc = "CKGMOD."]
        #[inline(always)]
        pub const fn set_ckgmod(&mut self, val: super::vals::Ckgmod) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "CCK0DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn cck0dir(&self) -> super::vals::Cckdir {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cckdir::from_bits(val as u8)
        }
        #[doc = "CCK0DIR."]
        #[inline(always)]
        pub const fn set_cck0dir(&mut self, val: super::vals::Cckdir) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "CCK1DIR."]
        #[must_use]
        #[inline(always)]
        pub const fn cck1dir(&self) -> super::vals::Cckdir {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cckdir::from_bits(val as u8)
        }
        #[doc = "CCK1DIR."]
        #[inline(always)]
        pub const fn set_cck1dir(&mut self, val: super::vals::Cckdir) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "TRGSENS."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsens(&self) -> super::vals::Trgsens {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Trgsens::from_bits(val as u8)
        }
        #[doc = "TRGSENS."]
        #[inline(always)]
        pub const fn set_trgsens(&mut self, val: super::vals::Trgsens) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "TRGSRC."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsrc(&self) -> super::vals::Trgsrc {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Trgsrc::from_bits(val as u8)
        }
        #[doc = "TRGSRC."]
        #[inline(always)]
        pub const fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "CCKDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn cckdiv(&self) -> super::vals::Cckdiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Cckdiv::from_bits(val as u8)
        }
        #[doc = "CCKDIV."]
        #[inline(always)]
        pub const fn set_cckdiv(&mut self, val: super::vals::Cckdiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "PROCDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn procdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "PROCDIV."]
        #[inline(always)]
        pub const fn set_procdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "CKGACTIVE."]
        #[must_use]
        #[inline(always)]
        pub const fn ckgactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CKGACTIVE."]
        #[inline(always)]
        pub const fn set_ckgactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ckgcr {
        #[inline(always)]
        fn default() -> Ckgcr {
            Ckgcr(0)
        }
    }
    impl core::fmt::Debug for Ckgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckgcr")
                .field("ckgden", &self.ckgden())
                .field("cck0en", &self.cck0en())
                .field("cck1en", &self.cck1en())
                .field("ckgmod", &self.ckgmod())
                .field("cck0dir", &self.cck0dir())
                .field("cck1dir", &self.cck1dir())
                .field("trgsens", &self.trgsens())
                .field("trgsrc", &self.trgsrc())
                .field("cckdiv", &self.cckdiv())
                .field("procdiv", &self.procdiv())
                .field("ckgactive", &self.ckgactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ckgcr {{ ckgden: {=bool:?}, cck0en: {=bool:?}, cck1en: {=bool:?}, ckgmod: {:?}, cck0dir: {:?}, cck1dir: {:?}, trgsens: {:?}, trgsrc: {:?}, cckdiv: {:?}, procdiv: {=u8:?}, ckgactive: {=bool:?} }}",
                self.ckgden(),
                self.cck0en(),
                self.cck1en(),
                self.ckgmod(),
                self.cck0dir(),
                self.cck1dir(),
                self.trgsens(),
                self.trgsrc(),
                self.cckdiv(),
                self.procdiv(),
                self.ckgactive()
            )
        }
    }
    #[doc = "This register is used to control the main CIC filter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltcicr(pub u32);
    impl Dfltcicr {
        #[doc = "Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn datsrc(&self) -> super::vals::Datsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Datsrc::from_bits(val as u8)
        }
        #[doc = "Source data for the digital filter Set and cleared by software. 0x: Select the stream coming from the BSMX - 10: Select the stream coming from the ADCITF1 - 11: Select the stream coming from the ADCITF2 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_datsrc(&mut self, val: super::vals::Datsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\\[2:0\\]
is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn cicmod(&self) -> super::vals::Cicmod {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Cicmod::from_bits(val as u8)
        }
        #[doc = "Select the CIC mode Set and cleared by software. This field allows the application to select the configuration and the order of the MCIC. When CICMOD\\[2:0\\]
is equal to 0xx , the CIC is split into two filters: - The main CIC (MCIC) - The auxiliary CIC (ACIC), used for the out-off limit detector - 000: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in FastSinc filter - 001: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc1 filter - 010: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc2 filter - 011: The CIC is split into 2 filters, and the main CIC (MCIC) is configured in Sinc3 filter - 100: The CIC is configured in single sinc4 filter others: The CIC is configured in single sinc5 filter This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_cicmod(&mut self, val: super::vals::Cicmod) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn mcicd(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "CIC decimation ratio selection Set and cleared by software. This bit is used to allow the application to select the decimation ratio of the CIC. Decimation ratio smaller than 2 is not allowed. The decimation ratio is given by (CICDEC+1). - 0: Decimation ratio is 2 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 511: Decimation ratio is 512 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_mcicd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[doc = "Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\\[5:0\\]
field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn scale(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "Scaling factor selection Set and cleared by software. This field is used to allow the application to select the gain to be applied at CIC output. Please refer to Table 13: Possible gain values for details. If the application attempts to write a new gain value while the previous one is not yet applied, this new gain value is ignored. Reading back the SCALE\\[5:0\\]
field will inform the application on the current gain value. - 100000: - 48.2 dB, or shift right by 8 bits (default value) - 100001: - 44.6 dB, - 100010: - 42.1 dB, or shift right by 7 bits - 100011: - 38.6 dB, ... - 101110: -6 dB, or shift right by 1 bit - 101111: -2.5 dB, - 000000: 0 dB - 000001: + 3.5 dB, - 000010: + 6 dB, or shift left by 1 bit ... - 011000: + 72 dB, or shift left by 12 bits."]
        #[inline(always)]
        pub const fn set_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
    }
    impl Default for Dfltcicr {
        #[inline(always)]
        fn default() -> Dfltcicr {
            Dfltcicr(0)
        }
    }
    impl core::fmt::Debug for Dfltcicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltcicr")
                .field("datsrc", &self.datsrc())
                .field("cicmod", &self.cicmod())
                .field("mcicd", &self.mcicd())
                .field("scale", &self.scale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltcicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltcicr {{ datsrc: {:?}, cicmod: {:?}, mcicd: {=u16:?}, scale: {=u8:?} }}",
                self.datsrc(),
                self.cicmod(),
                self.mcicd(),
                self.scale()
            )
        }
    }
    #[doc = "This register is used to control the digital filter x."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltcr(pub u32);
    impl Dfltcr {
        #[doc = "Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,."]
        #[must_use]
        #[inline(always)]
        pub const fn dflten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Filter Enable Set and cleared by software. This bit is used to control the start of acquisition of the corresponding digital filter path. The behavior of this bit depends on ACQMOD and external events. or the acquisition starts when the proper trigger event occurs if ACQMOD = 01x . The serial or parallel interface delivering the samples shall be enabled as well. - 0: The acquisition is stopped immediately - 1: The acquisition is immediately started if ACQMOD = 00x or 1xx ,."]
        #[inline(always)]
        pub const fn set_dflten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Requests Enable Set and cleared by software. This bit is used to control the generation of DMA request in order to transfer the processed samples into the memory. - 0: The DMA interface for the corresponding digital filter is disabled - 1: The DMA interface for the corresponding digital filter is enabled This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RXFIFO Threshold selection Set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn fth(&self) -> super::vals::Rxfifo {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Rxfifo::from_bits(val as u8)
        }
        #[doc = "RXFIFO Threshold selection Set and cleared by software."]
        #[inline(always)]
        pub const fn set_fth(&mut self, val: super::vals::Rxfifo) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn acqmod(&self) -> super::vals::Acqmod {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Acqmod::from_bits(val as u8)
        }
        #[doc = "Digital filter Trigger mode Set and cleared by software. This field is used to select the filter trigger mode. - 000: Asynchronous, continuous acquisition mode - 001: Asynchronous, single-shot acquisition mode - 010: Synchronous, continuous acquisition mode - 011: Synchronous, single-shot acquisition mode - 100: Window, continuous acquisition mode - 101: Synchronous, snapshot mode others: same a 000 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_acqmod(&mut self, val: super::vals::Acqmod) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsens(&self) -> super::vals::Trgsens {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Trgsens::from_bits(val as u8)
        }
        #[doc = "Digital filter Trigger sensitivity selection Set and cleared by software. This field is used to select the trigger sensitivity of the external signals - 0: A rising edge event triggers the acquisition - 1: A falling edge even triggers the acquisition Note that when the trigger source is TRGO or OLDx event, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx event is selected, the sensitivity is forced to rising edge. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_trgsens(&mut self, val: super::vals::Trgsens) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\\[0\\]
is selected ... - 1111: mdf_trg\\[13\\]
is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn trgsrc(&self) -> super::vals::Trgsrc {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Trgsrc::from_bits(val as u8)
        }
        #[doc = "Digital filter Trigger signal selection, Set and cleared by software. This field is used to select which external signals is used as trigger for the corresponding filter. - 0000: TRGO is selected - 0001: OLDx event is selected - 0010: mdf_trg\\[0\\]
is selected ... - 1111: mdf_trg\\[13\\]
is selected This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \\[15:9\\]
of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn snpsfmt(&self) -> super::vals::Snpsfmt {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Snpsfmt::from_bits(val as u8)
        }
        #[doc = "Snapshot data format Set and cleared by software. This field is used to select the data format for the snapshot mode. - 0: The integrator counter (INT_CNT) is not inserted into the MDF_SNPSxDR register, leaving a data resolution of 23 bits. - 1: The integrator counter (INT_CNT) is inserted at position \\[15:9\\]
of MDF_SNPSxDR register, leaving a data resolution of 16 bits. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_snpsfmt(&mut self, val: super::vals::Snpsfmt) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn nbdis(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Number of samples to be discarded Set and cleared by software. This field is used to define the number of samples to be discarded every time the DFLTx is re-started. - 0: no sample discarded - 1: 1 sample discarded - 2: 2 samples discarded ... - 255: 255 samples discarded This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_nbdis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running."]
        #[must_use]
        #[inline(always)]
        pub const fn dfltrun(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Digital filter Run Status Flag Set and cleared by hardware. This bit indicates if the digital filter is running or not. - 0: The digital filter is not running, and ready to accept a new trigger event - 1: The digital filter is running."]
        #[inline(always)]
        pub const fn set_dfltrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active."]
        #[must_use]
        #[inline(always)]
        pub const fn dfltactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Digital filter Active Flag Set and cleared by hardware. This bit indicates if the digital filter is active: can be running or waiting for events. - 0: The digital filter is not active, and can be re-enabled again (via DFLTEN bit) if needed - 1: The digital filter is active."]
        #[inline(always)]
        pub const fn set_dfltactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dfltcr {
        #[inline(always)]
        fn default() -> Dfltcr {
            Dfltcr(0)
        }
    }
    impl core::fmt::Debug for Dfltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltcr")
                .field("dflten", &self.dflten())
                .field("dmaen", &self.dmaen())
                .field("fth", &self.fth())
                .field("acqmod", &self.acqmod())
                .field("trgsens", &self.trgsens())
                .field("trgsrc", &self.trgsrc())
                .field("snpsfmt", &self.snpsfmt())
                .field("nbdis", &self.nbdis())
                .field("dfltrun", &self.dfltrun())
                .field("dfltactive", &self.dfltactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltcr {{ dflten: {=bool:?}, dmaen: {=bool:?}, fth: {:?}, acqmod: {:?}, trgsens: {:?}, trgsrc: {:?}, snpsfmt: {:?}, nbdis: {=u8:?}, dfltrun: {=bool:?}, dfltactive: {=bool:?} }}",
                self.dflten(),
                self.dmaen(),
                self.fth(),
                self.acqmod(),
                self.trgsens(),
                self.trgsrc(),
                self.snpsfmt(),
                self.nbdis(),
                self.dfltrun(),
                self.dfltactive()
            )
        }
    }
    #[doc = "This register is used to read the data processed by each digital filter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltdr(pub u32);
    impl Dfltdr {
        #[doc = "Data processed by digital filter."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Data processed by digital filter."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Dfltdr {
        #[inline(always)]
        fn default() -> Dfltdr {
            Dfltdr(0)
        }
    }
    impl core::fmt::Debug for Dfltdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltdr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dfltdr {{ dr: {=u32:?} }}", self.dr())
        }
    }
    #[doc = "This register is used for allowing or not the events to generate an interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltier(pub u32);
    impl Dfltier {
        #[doc = "RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn fthie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXFIFO threshold interrupt enable Set and cleared by software. - 0: RXFIFO threshold interrupt disabled - 1: RXFIFO threshold interrupt enabled."]
        #[inline(always)]
        pub const fn set_fthie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dovrie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data overflow interrupt enable Set and cleared by software. - 0: Data overflow interrupt disabled - 1: Data overflow interrupt enabled."]
        #[inline(always)]
        pub const fn set_dovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ssdrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Snapshot data ready interrupt enable Set and cleared by software. - 0: Snapshot data ready interrupt disabled - 1: Snapshot data ready interrupt enabled."]
        #[inline(always)]
        pub const fn set_ssdrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn oldie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Out-of Limit interrupt enable Set and cleared by software. - 0: OLD event interrupt disabled - 1: OLD event interrupt enabled."]
        #[inline(always)]
        pub const fn set_oldie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ssovrie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Snapshot overrun interrupt enable Set and cleared by software. - 0: Snapshot overrun interrupt disabled - 1: Snapshot overrun interrupt enabled."]
        #[inline(always)]
        pub const fn set_ssovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn scdie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Short-Circuit Detector interrupt enable Set and cleared by software. - 0: SCD interrupt disabled - 1: SCD interrupt enabled."]
        #[inline(always)]
        pub const fn set_scdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn satie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation detection interrupt enable Set and cleared by software. - 0: Saturation interrupt disabled - 1: Saturation interrupt enabled."]
        #[inline(always)]
        pub const fn set_satie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence detection interrupt enable Set and cleared by software. - 0: Clock absence interrupt disabled - 1: Clock absence interrupt enabled."]
        #[inline(always)]
        pub const fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rfovrie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Reshape Filter Overrun interrupt enable Set and cleared by software. - 0: Reshape filter overrun interrupt disabled - 1: Reshape filter overrun interrupt enabled."]
        #[inline(always)]
        pub const fn set_rfovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dfltier {
        #[inline(always)]
        fn default() -> Dfltier {
            Dfltier(0)
        }
    }
    impl core::fmt::Debug for Dfltier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltier")
                .field("fthie", &self.fthie())
                .field("dovrie", &self.dovrie())
                .field("ssdrie", &self.ssdrie())
                .field("oldie", &self.oldie())
                .field("ssovrie", &self.ssovrie())
                .field("scdie", &self.scdie())
                .field("satie", &self.satie())
                .field("ckabie", &self.ckabie())
                .field("rfovrie", &self.rfovrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltier {{ fthie: {=bool:?}, dovrie: {=bool:?}, ssdrie: {=bool:?}, oldie: {=bool:?}, ssovrie: {=bool:?}, scdie: {=bool:?}, satie: {=bool:?}, ckabie: {=bool:?}, rfovrie: {=bool:?} }}",
                self.fthie(),
                self.dovrie(),
                self.ssdrie(),
                self.oldie(),
                self.ssovrie(),
                self.scdie(),
                self.satie(),
                self.ckabie(),
                self.rfovrie()
            )
        }
    }
    #[doc = "This register is used to the integrator (INT) settings."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltintr(pub u32);
    impl Dfltintr {
        #[doc = "Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn intdiv(&self) -> super::vals::Intdiv {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Intdiv::from_bits(val as u8)
        }
        #[doc = "Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_intdiv(&mut self, val: super::vals::Intdiv) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn intval(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x7f;
            val as u8
        }
        #[doc = "Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_intval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
        }
    }
    impl Default for Dfltintr {
        #[inline(always)]
        fn default() -> Dfltintr {
            Dfltintr(0)
        }
    }
    impl core::fmt::Debug for Dfltintr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltintr")
                .field("intdiv", &self.intdiv())
                .field("intval", &self.intval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltintr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltintr {{ intdiv: {:?}, intval: {=u8:?} }}",
                self.intdiv(),
                self.intval()
            )
        }
    }
    #[doc = "MDF DFLT0 interrupt status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltisr(pub u32);
    impl Dfltisr {
        #[doc = "FTHF."]
        #[must_use]
        #[inline(always)]
        pub const fn fthf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FTHF."]
        #[inline(always)]
        pub const fn set_fthf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dovrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data overflow flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no overflow is detected, writing 0 has no effect. - 1: Reading 1 means that an overflow is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_dovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ssdrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Snapshot data ready flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no data is available on , writing 0 has no effect. - 1: Reading 1 means that a new data is available on , writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_ssdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rxnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RXFIFO Not Empty flag Set and cleared by hardware according to the RXFIFO level. - 0: Reading 0 means that the RXFIFO is empty. - 1: Reading 1 means that the RXFIFO is not empty."]
        #[inline(always)]
        pub const fn set_rxnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags."]
        #[must_use]
        #[inline(always)]
        pub const fn oldf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Out-of Limit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no OLD event is detected, writing 0 has no effect. - 1: Reading 1 means that an OLD event is detected, writing 1 clears THHF, THLF and OLDF flags."]
        #[inline(always)]
        pub const fn set_oldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn thlf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the low threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was higher than OLDTHL when the last OLD event occurred. - 1: The signal was lower than OLDTHL when the last OLD event occurred."]
        #[inline(always)]
        pub const fn set_thlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn thhf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "High threshold status flag Set by hardware, and cleared by software by writing this bit to 1 . This flag indicates the status of the high threshold comparator when the last OLD event occurred. This bit gives additional information on the conditions triggering the last OLD event. It can be cleared by writing OLDF flag to a 1. - 0: The signal was lower than OLDTHH when the last OLD event occurred. - 1: The signal was higher than OLDTHH when the last OLD event occurred."]
        #[inline(always)]
        pub const fn set_thhf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ssovrf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Snapshot overrun flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no snapshot overrun event is detected, writing 0 has no effect. - 1: Reading 1 means that a snapshot overrun event is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_ssovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn scdf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Short-Circuit Detector flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no SCD event is detected, writing 0 has no effect. - 1: Reading 1 means that a SCD event is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_scdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn satf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no saturation is detected, writing 0 has no effect. - 1: Reading 1 means that a saturation is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_satf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ckabf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no clock absence is detected, writing 0 has no effect. - 1: Reading 1 means that a clock absence is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_ckabf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rfovrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Reshape Filter Overrun detection flag Set by hardware, and cleared by software by writing this bit to 1 . - 0: Reading 0 means that no reshape filter overrun is detected, writing 0 has no effect. - 1: Reading 1 means that reshape filter overrun is detected, writing 1 clears this flag."]
        #[inline(always)]
        pub const fn set_rfovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dfltisr {
        #[inline(always)]
        fn default() -> Dfltisr {
            Dfltisr(0)
        }
    }
    impl core::fmt::Debug for Dfltisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltisr")
                .field("fthf", &self.fthf())
                .field("dovrf", &self.dovrf())
                .field("ssdrf", &self.ssdrf())
                .field("rxnef", &self.rxnef())
                .field("oldf", &self.oldf())
                .field("thlf", &self.thlf())
                .field("thhf", &self.thhf())
                .field("ssovrf", &self.ssovrf())
                .field("scdf", &self.scdf())
                .field("satf", &self.satf())
                .field("ckabf", &self.ckabf())
                .field("rfovrf", &self.rfovrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltisr {{ fthf: {=bool:?}, dovrf: {=bool:?}, ssdrf: {=bool:?}, rxnef: {=bool:?}, oldf: {=bool:?}, thlf: {=bool:?}, thhf: {=bool:?}, ssovrf: {=bool:?}, scdf: {=bool:?}, satf: {=bool:?}, ckabf: {=bool:?}, rfovrf: {=bool:?} }}",
                self.fthf(),
                self.dovrf(),
                self.ssdrf(),
                self.rxnef(),
                self.oldf(),
                self.thlf(),
                self.thhf(),
                self.ssovrf(),
                self.scdf(),
                self.satf(),
                self.ckabf(),
                self.rfovrf()
            )
        }
    }
    #[doc = "This register is used to control the reshape and HPF filters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfltrsfr(pub u32);
    impl Dfltrsfr {
        #[doc = "Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn rsfltbyp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reshaper filter bypass Set and cleared by software. This bit is used to bypass the reshape filter and its decimation block. - 0: The reshape filter is not bypassed (Default value) - 1: The reshape filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_rsfltbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn rsfltd(&self) -> super::vals::Rsfltd {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Rsfltd::from_bits(val as u8)
        }
        #[doc = "Reshaper filter decimation ratio Set and cleared by software. This bit is used to select the decimation ratio for the reshape filter - 0: Decimation ratio is 4 (Default value) - 1: Decimation ratio is 1 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_rsfltd(&mut self, val: super::vals::Rsfltd) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn hpfbyp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "High-Pass Filter bypass Set and cleared by software. This bit is used to bypass the high-pass filter. - 0: The high pass filter is not bypassed (Default value) - 1: The high pass filter is bypassed This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_hpfbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn hpfc(&self) -> super::vals::Hpfc {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hpfc::from_bits(val as u8)
        }
        #[doc = "High-pass filter cut-off frequency Set and cleared by software. This field is used to select the cut-off frequency of the high-pass filter. FPCM represents the sampling frequency at HPF input. - 00: Cut-off frequency = 0.000625 x FPCM - 01: Cut-off frequency = 0.00125 x FPCM - 10: Cut-off frequency = 0.00250 x FPCM - 11: Cut-off frequency = 0.00950 x FPCM This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_hpfc(&mut self, val: super::vals::Hpfc) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Dfltrsfr {
        #[inline(always)]
        fn default() -> Dfltrsfr {
            Dfltrsfr(0)
        }
    }
    impl core::fmt::Debug for Dfltrsfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfltrsfr")
                .field("rsfltbyp", &self.rsfltbyp())
                .field("rsfltd", &self.rsfltd())
                .field("hpfbyp", &self.hpfbyp())
                .field("hpfc", &self.hpfc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfltrsfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfltrsfr {{ rsfltbyp: {=bool:?}, rsfltd: {:?}, hpfbyp: {=bool:?}, hpfc: {:?} }}",
                self.rsfltbyp(),
                self.rsfltd(),
                self.hpfbyp(),
                self.hpfc()
            )
        }
    }
    #[doc = "This register is used for the adjustment stream delays."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlycr(pub u32);
    impl Dlycr {
        #[doc = "Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,."]
        #[must_use]
        #[inline(always)]
        pub const fn skpdly(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,."]
        #[inline(always)]
        pub const fn set_skpdly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\\[6:0\\]. - 1: Reading 1 means that last valid SKPDLY\\[6:0\\]
is still under precessing."]
        #[must_use]
        #[inline(always)]
        pub const fn skpbf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\\[6:0\\]. - 1: Reading 1 means that last valid SKPDLY\\[6:0\\]
is still under precessing."]
        #[inline(always)]
        pub const fn set_skpbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dlycr {
        #[inline(always)]
        fn default() -> Dlycr {
            Dlycr(0)
        }
    }
    impl core::fmt::Debug for Dlycr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlycr")
                .field("skpdly", &self.skpdly())
                .field("skpbf", &self.skpbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlycr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlycr {{ skpdly: {=u8:?}, skpbf: {=bool:?} }}",
                self.skpdly(),
                self.skpbf()
            )
        }
    }
    #[doc = "MDF global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "TRGO."]
        #[must_use]
        #[inline(always)]
        pub const fn trgo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TRGO."]
        #[inline(always)]
        pub const fn set_trgo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ILVNB."]
        #[must_use]
        #[inline(always)]
        pub const fn ilvnb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "ILVNB."]
        #[inline(always)]
        pub const fn set_ilvnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("trgo", &self.trgo())
                .field("ilvnb", &self.ilvnb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gcr {{ trgo: {=bool:?}, ilvnb: {=u8:?} }}",
                self.trgo(),
                self.ilvnb()
            )
        }
    }
    #[doc = "This register contains the offset compensation value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oeccr(pub u32);
    impl Oeccr {
        #[doc = "Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\\[25:0\\]
field will inform the application on the current offset value. OFFSET\\[25:0\\]
represents the value to be subtracted to the signal before going to the SCALE."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\\[25:0\\]
field will inform the application on the current offset value. OFFSET\\[25:0\\]
represents the value to be subtracted to the signal before going to the SCALE."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for Oeccr {
        #[inline(always)]
        fn default() -> Oeccr {
            Oeccr(0)
        }
    }
    impl core::fmt::Debug for Oeccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oeccr").field("offset", &self.offset()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oeccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oeccr {{ offset: {=u32:?} }}", self.offset())
        }
    }
    #[doc = "This register is used to configure the Out-of Limit Detector function."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oldcr(pub u32);
    impl Oldcr {
        #[doc = "Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn olden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Over-Current Detector Enable Set and cleared by software. - 0: The OLD is disabled (Default value) - 1: The OLD is enabled, including the ACIC filter working in continuous mode."]
        #[inline(always)]
        pub const fn set_olden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn thinb(&self) -> super::vals::Thinb {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Thinb::from_bits(val as u8)
        }
        #[doc = "Threshold In band Set and cleared by software. - 0: The OLD generates an event if the signal is lower than OLDTHL OR higher than OLDTHH (Default value) - 1: The OLD generates an event if the signal is lower than OLDTHH AND higher than OLDTHL This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_thinb(&mut self, val: super::vals::Thinb) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\\[i\\]
= 0: Break signal (mdf_break\\[i\\]) is not assigned to threshold event BKOLD\\[i\\]
= 1: Break signal (mdf_break\\[i\\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn bkold(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment for out-of limit detector Set and cleared by software. BKOLD\\[i\\]
= 0: Break signal (mdf_break\\[i\\]) is not assigned to threshold event BKOLD\\[i\\]
= 1: Break signal (mdf_break\\[i\\]) is assigned to threshold event This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_bkold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\\[2:0\\]
= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn acicn(&self) -> super::vals::Acicn {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Acicn::from_bits(val as u8)
        }
        #[doc = "OLD CIC order selection Set and cleared by software. This field allows the application to select the type, and the order of the ACIC. This field is only taken into account by the MDF when CICMOD\\[2:0\\]
= 0xx . - 00: FastSinc filter type - 01: Sinc1 filter type - 10: Sinc2 filter type - 11: Sinc3 filter type This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_acicn(&mut self, val: super::vals::Acicn) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\\[2:0\\]
= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn acicd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x1f;
            val as u8
        }
        #[doc = "OLD CIC decimation ratio selection Set and cleared by software. This field is used to allow the application to select the decimation ratio of the ACIC. This field is only taken into account by the MDF when CICMOD\\[2:0\\]
= 0xx . The decimation ratio is given by (ACICD+1). - 0: Decimation ratio is 1 - 1: Decimation ratio is 2 - 2: Decimation ratio is 3 - 3: Decimation ratio is 4 ... - 31: Decimation ratio is 32 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_acicd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
        }
        #[doc = "OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured."]
        #[must_use]
        #[inline(always)]
        pub const fn oldactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "OLD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the OLD is effectively enabled (active) or not. The protected fields and registers of this function can only be updated when the OLDACTIVE is set to , please refer to Section 1.4.15: Register protection for details. The delay between a transition on OLDEN and a transition on OLDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The OLD is not active, and can be configured if needed - 1: The OLD is active, and protected fields cannot be configured."]
        #[inline(always)]
        pub const fn set_oldactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Oldcr {
        #[inline(always)]
        fn default() -> Oldcr {
            Oldcr(0)
        }
    }
    impl core::fmt::Debug for Oldcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oldcr")
                .field("olden", &self.olden())
                .field("thinb", &self.thinb())
                .field("bkold", &self.bkold())
                .field("acicn", &self.acicn())
                .field("acicd", &self.acicd())
                .field("oldactive", &self.oldactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oldcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Oldcr {{ olden: {=bool:?}, thinb: {:?}, bkold: {=u8:?}, acicn: {:?}, acicd: {=u8:?}, oldactive: {=bool:?} }}",
                self.olden(),
                self.thinb(),
                self.bkold(),
                self.acicn(),
                self.acicd(),
                self.oldactive()
            )
        }
    }
    #[doc = "This register is used for the adjustment of the Out-off Limit high threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oldthhr(pub u32);
    impl Oldthhr {
        #[doc = "OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn oldthh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_oldthh(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for Oldthhr {
        #[inline(always)]
        fn default() -> Oldthhr {
            Oldthhr(0)
        }
    }
    impl core::fmt::Debug for Oldthhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oldthhr").field("oldthh", &self.oldthh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oldthhr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oldthhr {{ oldthh: {=u32:?} }}", self.oldthh())
        }
    }
    #[doc = "This register is used for the adjustment of the Out-off Limit low threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oldthlr(pub u32);
    impl Oldthlr {
        #[doc = "OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn oldthl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_oldthl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for Oldthlr {
        #[inline(always)]
        fn default() -> Oldthlr {
            Oldthlr(0)
        }
    }
    impl core::fmt::Debug for Oldthlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oldthlr").field("oldthl", &self.oldthl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oldthlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oldthlr {{ oldthl: {=u32:?} }}", self.oldthl())
        }
    }
    #[doc = "This register is used for the adjustment stream delays."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scdcr(pub u32);
    impl Scdcr {
        #[doc = "Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,."]
        #[must_use]
        #[inline(always)]
        pub const fn scden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,."]
        #[inline(always)]
        pub const fn set_scden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Break signal assignment for short circuit detector Set and cleared by software. BKSCD\\[i\\]
= 0: Break signal (mdf_break\\[i\\]) is not assigned to this SCD event BKSCD\\[i\\]
= 1: Break signal (mdf_break\\[i\\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn bkscd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment for short circuit detector Set and cleared by software. BKSCD\\[i\\]
= 0: Break signal (mdf_break\\[i\\]) is not assigned to this SCD event BKSCD\\[i\\]
= 1: Break signal (mdf_break\\[i\\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_bkscd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn scdt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_scdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured."]
        #[must_use]
        #[inline(always)]
        pub const fn scdactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured."]
        #[inline(always)]
        pub const fn set_scdactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Scdcr {
        #[inline(always)]
        fn default() -> Scdcr {
            Scdcr(0)
        }
    }
    impl core::fmt::Debug for Scdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scdcr")
                .field("scden", &self.scden())
                .field("bkscd", &self.bkscd())
                .field("scdt", &self.scdt())
                .field("scdactive", &self.scdactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scdcr {{ scden: {=bool:?}, bkscd: {=u8:?}, scdt: {=u8:?}, scdactive: {=bool:?} }}",
                self.scden(),
                self.bkscd(),
                self.scdt(),
                self.scdactive()
            )
        }
    }
    #[doc = "This register is used to control the serial interfaces (SITFx)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sitfcr(pub u32);
    impl Sitfcr {
        #[doc = "Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sitfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Serial interface enable Set and cleared by software. This bit is used to enable/disable the serial interface. - 0: Serial interface disabled - 1: Serial interface enabled."]
        #[inline(always)]
        pub const fn set_sitfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn scksrc(&self) -> super::vals::Scksrc {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Scksrc::from_bits(val as u8)
        }
        #[doc = "Serial clock source Set and cleared by software. This bit is used to select the clock source of the serial interface. - 00: Serial clock source is MDF_CCK0 - 01: Serial clock source is MDF_CCK1 1x: Serial clock source is MDF_CKIx, not allowed in LF_MASTER SPI mode This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_scksrc(&mut self, val: super::vals::Scksrc) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn sitfmod(&self) -> super::vals::Sitfmod {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Sitfmod::from_bits(val as u8)
        }
        #[doc = "Serial interface type Set and cleared by software. This field is used to defined the serial interface type. - 00: LF_MASTER (Low-Frequency MASTER) SPI mode - 01: Normal SPI mode - 10: Manchester mode: rising edge = logic 0, falling edge = logic 1 - 11: Manchester mode: rising edge = logic 1, falling edge = logic 0 This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_sitfmod(&mut self, val: super::vals::Sitfmod) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[must_use]
        #[inline(always)]
        pub const fn sth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Manchester Symbol threshold / SPI threshold Set and cleared by software. This field is used for Manchester mode, in order to define the expected symbol threshold levels. Please refer to Section : Manchester mode for details on computation. In addition this field is used to define the timeout value for the clock absence detection in Normal SPI mode. Values of STH\\[4:0\\]
lower than 4 are invalid. This field can be write-protected, please refer to Section 1.4.15: Register protection for details."]
        #[inline(always)]
        pub const fn set_sth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured."]
        #[must_use]
        #[inline(always)]
        pub const fn sitfactive(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Serial interface Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the serial interface is effectively enabled (active) or not. The protected fields of this function can only be updated when the SITFACTIVE is set , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SITFEN and a transition on SITFACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The serial interface is not active, and can be configured if needed - 1: The serial interface is active, and protected fields cannot be configured."]
        #[inline(always)]
        pub const fn set_sitfactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sitfcr {
        #[inline(always)]
        fn default() -> Sitfcr {
            Sitfcr(0)
        }
    }
    impl core::fmt::Debug for Sitfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sitfcr")
                .field("sitfen", &self.sitfen())
                .field("scksrc", &self.scksrc())
                .field("sitfmod", &self.sitfmod())
                .field("sth", &self.sth())
                .field("sitfactive", &self.sitfactive())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sitfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sitfcr {{ sitfen: {=bool:?}, scksrc: {:?}, sitfmod: {:?}, sth: {=u8:?}, sitfactive: {=bool:?} }}",
                self.sitfen(),
                self.scksrc(),
                self.sitfmod(),
                self.sth(),
                self.sitfactive()
            )
        }
    }
    #[doc = "This register is used to read the data processed by each digital filter in snapshot mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Snpsdr(pub u32);
    impl Snpsdr {
        #[doc = "Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mcicdc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)."]
        #[inline(always)]
        pub const fn set_mcicdc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Extended data size If SNPSFMT = 0 , EXTSDR\\[6:0\\]
contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT)."]
        #[must_use]
        #[inline(always)]
        pub const fn extsdr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[doc = "Extended data size If SNPSFMT = 0 , EXTSDR\\[6:0\\]
contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT)."]
        #[inline(always)]
        pub const fn set_extsdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
        #[doc = "Contains the 16 MSB of the last valid data processed by the digital filter."]
        #[must_use]
        #[inline(always)]
        pub const fn sdr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Contains the 16 MSB of the last valid data processed by the digital filter."]
        #[inline(always)]
        pub const fn set_sdr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Snpsdr {
        #[inline(always)]
        fn default() -> Snpsdr {
            Snpsdr(0)
        }
    }
    impl core::fmt::Debug for Snpsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Snpsdr")
                .field("mcicdc", &self.mcicdc())
                .field("extsdr", &self.extsdr())
                .field("sdr", &self.sdr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Snpsdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Snpsdr {{ mcicdc: {=u16:?}, extsdr: {=u8:?}, sdr: {=u16:?} }}",
                self.mcicdc(),
                self.extsdr(),
                self.sdr()
            )
        }
    }
}
pub mod vals {
    #[doc = "Auxiliary CIC order selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Acicn {
        #[doc = "FastSinc filter type."]
        FastSinc = 0x0,
        #[doc = "Sinc1 filter type."]
        Sinc1 = 0x01,
        #[doc = "Sinc2 filter type."]
        Sinc2 = 0x02,
        #[doc = "Sinc3 filter type."]
        Sinc3 = 0x03,
    }
    impl Acicn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Acicn {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Acicn {
        #[inline(always)]
        fn from(val: u8) -> Acicn {
            Acicn::from_bits(val)
        }
    }
    impl From<Acicn> for u8 {
        #[inline(always)]
        fn from(val: Acicn) -> u8 {
            Acicn::to_bits(val)
        }
    }
    #[doc = "Digital filter trigger mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Acqmod {
        #[doc = "Asynchronous continuous acquisition mode."]
        AsynchronousContinuous = 0x0,
        #[doc = "Asynchronous single-shot acquisition mode."]
        AsynchronousSingleShot = 0x01,
        #[doc = "Synchronous continuous acquisition mode."]
        SynchronousContinuous = 0x02,
        #[doc = "Synchronous single-shot acquisition mode."]
        SynchronousSingleShot = 0x03,
        #[doc = "Window continuous acquisition mode."]
        WindowContinuous = 0x04,
        #[doc = "Synchronous snapshot acquisition mode."]
        SynchronousSnapshot = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Acqmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Acqmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Acqmod {
        #[inline(always)]
        fn from(val: u8) -> Acqmod {
            Acqmod::from_bits(val)
        }
    }
    impl From<Acqmod> for u8 {
        #[inline(always)]
        fn from(val: Acqmod) -> u8 {
            Acqmod::to_bits(val)
        }
    }
    #[doc = "Bitstream selection. This bitfield is set and cleared by software. It is used to select the bitstream to be used by the DFLT0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bssel {
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs0R = 0x0,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs0F = 0x01,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs1R = 0x02,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs1F = 0x03,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs2R = 0x04,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs2F = 0x05,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs3R = 0x06,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs3F = 0x07,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs4R = 0x08,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs4F = 0x09,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs5R = 0x0a,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs5F = 0x0b,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs6R = 0x0c,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs6F = 0x0d,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs7R = 0x0e,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs7F = 0x0f,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs8R = 0x10,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs8F = 0x11,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs9R = 0x12,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs9F = 0x13,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs10R = 0x14,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs10F = 0x15,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs11R = 0x16,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs11F = 0x17,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs12R = 0x18,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs12F = 0x19,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs13R = 0x1a,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs13F = 0x1b,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs14R = 0x1c,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs14F = 0x1d,
        #[doc = "bsx_r provided to DFLTy (and SCDy)."]
        Bs15R = 0x1e,
        #[doc = "bsx_f provided to DFLTy (and SCDy)."]
        Bs15F = 0x1f,
    }
    impl Bssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bssel {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bssel {
        #[inline(always)]
        fn from(val: u8) -> Bssel {
            Bssel::from_bits(val)
        }
    }
    impl From<Bssel> for u8 {
        #[inline(always)]
        fn from(val: Bssel) -> u8 {
            Bssel::to_bits(val)
        }
    }
    #[doc = "CCK1 direction. This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cckdir {
        #[doc = "CCK is an input."]
        Input = 0x0,
        #[doc = "CCK is an output."]
        Output = 0x01,
    }
    impl Cckdir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cckdir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cckdir {
        #[inline(always)]
        fn from(val: u8) -> Cckdir {
            Cckdir::from_bits(val)
        }
    }
    impl From<Cckdir> for u8 {
        #[inline(always)]
        fn from(val: Cckdir) -> u8 {
            Cckdir::to_bits(val)
        }
    }
    #[doc = "Divider to control the CCK clock. This bit is set and reset by software. It is used to control the frequency of the bitstream clock on the CCK pin."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cckdiv {
        #[doc = "The ADF_CCK clock is adf_proc_ck."]
        Div1 = 0x0,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 2."]
        Div2 = 0x01,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 3."]
        Div3 = 0x02,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 4."]
        Div4 = 0x03,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 5."]
        Div5 = 0x04,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 6."]
        Div6 = 0x05,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 7."]
        Div7 = 0x06,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 8."]
        Div8 = 0x07,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 9."]
        Div9 = 0x08,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 10."]
        Div10 = 0x09,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 11."]
        Div11 = 0x0a,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 12."]
        Div12 = 0x0b,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 13."]
        Div13 = 0x0c,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 14."]
        Div14 = 0x0d,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 15."]
        Div15 = 0x0e,
        #[doc = "The ADF_CCK clock is adf_proc_ck divided by 16."]
        Div16 = 0x0f,
    }
    impl Cckdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cckdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cckdiv {
        #[inline(always)]
        fn from(val: u8) -> Cckdiv {
            Cckdiv::from_bits(val)
        }
    }
    impl From<Cckdiv> for u8 {
        #[inline(always)]
        fn from(val: Cckdiv) -> u8 {
            Cckdiv::to_bits(val)
        }
    }
    #[doc = "CIC filter mode selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cicmod {
        #[doc = "Split filters with main CIC in FastSinc mode."]
        FastSincSplit = 0x0,
        #[doc = "Split filters with main CIC in Sinc1 mode."]
        Sinc1split = 0x01,
        #[doc = "Split filters with main CIC in Sinc2 mode."]
        Sinc2split = 0x02,
        #[doc = "Split filters with main CIC in Sinc3 mode."]
        Sinc3split = 0x03,
        #[doc = "Single Sinc4 filter."]
        Sinc4single = 0x04,
        #[doc = "Single Sinc5 filter."]
        Sinc5single = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cicmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cicmod {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cicmod {
        #[inline(always)]
        fn from(val: u8) -> Cicmod {
            Cicmod::from_bits(val)
        }
    }
    impl From<Cicmod> for u8 {
        #[inline(always)]
        fn from(val: Cicmod) -> u8 {
            Cicmod::to_bits(val)
        }
    }
    #[doc = "Clock generator mode. This bit is set and reset by software. It is used to define the way the clock generator is enabled. This bit must not be changed if the filter is enabled (DFTEN = 1)."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckgmod {
        #[doc = "The kernel clock is provided to the dividers as soon as CKGDEN is set to 1."]
        Immediate = 0x0,
        #[doc = "The kernel clock is provided to the dividers when CKGDEN is set to 1 and the trigger condition met."]
        Trigger = 0x01,
    }
    impl Ckgmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckgmod {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckgmod {
        #[inline(always)]
        fn from(val: u8) -> Ckgmod {
            Ckgmod::from_bits(val)
        }
    }
    impl From<Ckgmod> for u8 {
        #[inline(always)]
        fn from(val: Ckgmod) -> u8 {
            Ckgmod::to_bits(val)
        }
    }
    #[doc = "Source data for the digital filter."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Datsrc {
        #[doc = "Stream coming from the BSMX selected"]
        Bsmx = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Stream coming from the ADCITF1 selected"]
        Adcitf1 = 0x02,
        #[doc = "Stream coming from the ADCITF2 selected"]
        Adcitf2 = 0x03,
    }
    impl Datsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datsrc {
        #[inline(always)]
        fn from(val: u8) -> Datsrc {
            Datsrc::from_bits(val)
        }
    }
    impl From<Datsrc> for u8 {
        #[inline(always)]
        fn from(val: Datsrc) -> u8 {
            Datsrc::to_bits(val)
        }
    }
    #[doc = "High-pass filter cut-off frequency. This bitfield is set and cleared by software. it is used to select the cut-off frequency of the high-pass filter. F PCM represents the sampling frequency at HPF input."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpfc {
        #[doc = "Cut-off frequency = 0.000625 x FPCM."]
        Low = 0x0,
        #[doc = "Cut-off frequency = 0.00125 x FPCM."]
        Medium = 0x01,
        #[doc = "Cut-off frequency = 0.00250 x FPCM"]
        High = 0x02,
        #[doc = "Cut-off frequency = 0.00950 x FPCM"]
        Maximum = 0x03,
    }
    impl Hpfc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpfc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpfc {
        #[inline(always)]
        fn from(val: u8) -> Hpfc {
            Hpfc::from_bits(val)
        }
    }
    impl From<Hpfc> for u8 {
        #[inline(always)]
        fn from(val: Hpfc) -> u8 {
            Hpfc::to_bits(val)
        }
    }
    #[doc = "Integrator output division."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Intdiv {
        #[doc = "Integrator output divided by 128."]
        Div128 = 0x0,
        #[doc = "Integrator output divided by 32."]
        Div32 = 0x01,
        #[doc = "Integrator output divided by 4."]
        Div4 = 0x02,
        #[doc = "Integrator output not divided."]
        None = 0x03,
    }
    impl Intdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Intdiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Intdiv {
        #[inline(always)]
        fn from(val: u8) -> Intdiv {
            Intdiv::from_bits(val)
        }
    }
    impl From<Intdiv> for u8 {
        #[inline(always)]
        fn from(val: Intdiv) -> u8 {
            Intdiv::to_bits(val)
        }
    }
    #[doc = "Reshaper filter decimation ratio. This bitfield is set and cleared by software. It is used to select the decimation ratio of the reshaper filter."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rsfltd {
        #[doc = "Decimation ratio is 4 (default value)."]
        Decimation4 = 0x0,
        #[doc = "Decimation ratio is 1."]
        Decimation1 = 0x01,
    }
    impl Rsfltd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rsfltd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rsfltd {
        #[inline(always)]
        fn from(val: u8) -> Rsfltd {
            Rsfltd::from_bits(val)
        }
    }
    impl From<Rsfltd> for u8 {
        #[inline(always)]
        fn from(val: Rsfltd) -> u8 {
            Rsfltd::to_bits(val)
        }
    }
    #[doc = "RXFIFO threshold selection. This bitfield is set and cleared by software. It is used to select the RXFIFO threshold."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxfifo {
        #[doc = "RXFIFO threshold event generated when the RXFIFO is not empty"]
        NotEmpty = 0x0,
        #[doc = "RXFIFO threshold event generated when the RXFIFO is half-full"]
        HalfFull = 0x01,
    }
    impl Rxfifo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxfifo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxfifo {
        #[inline(always)]
        fn from(val: u8) -> Rxfifo {
            Rxfifo::from_bits(val)
        }
    }
    impl From<Rxfifo> for u8 {
        #[inline(always)]
        fn from(val: Rxfifo) -> u8 {
            Rxfifo::to_bits(val)
        }
    }
    #[doc = "Serial clock source. This bitfield is set and cleared by software. It is used to select the clock source of the serial interface."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scksrc {
        #[doc = "Serial clock source is CCK0."]
        Cck0 = 0x0,
        #[doc = "Serial clock source is CCK1."]
        Cck1 = 0x01,
        #[doc = "Serial clock source is CCI0."]
        Cki0 = 0x02,
        #[doc = "Serial clock source is CCI1."]
        Cki1 = 0x03,
    }
    impl Scksrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scksrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scksrc {
        #[inline(always)]
        fn from(val: u8) -> Scksrc {
            Scksrc::from_bits(val)
        }
    }
    impl From<Scksrc> for u8 {
        #[inline(always)]
        fn from(val: Scksrc) -> u8 {
            Scksrc::to_bits(val)
        }
    }
    #[doc = "Serial interface mode. This bitfield is set and cleared by software. It is used to select the serial interface mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sitfmod {
        #[doc = "LF_MASTER SPI mode."]
        MasterSpi = 0x0,
        #[doc = "Normal SPI mode."]
        NormalSpi = 0x01,
        #[doc = "Manchester mode rising edge = logic 0, falling edge = logic 1."]
        ManchesterFalling = 0x02,
        #[doc = "Manchester mode rising edge = logic 1, falling edge = logic 0."]
        ManchesterRising = 0x03,
    }
    impl Sitfmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sitfmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sitfmod {
        #[inline(always)]
        fn from(val: u8) -> Sitfmod {
            Sitfmod::from_bits(val)
        }
    }
    impl From<Sitfmod> for u8 {
        #[inline(always)]
        fn from(val: Sitfmod) -> u8 {
            Sitfmod::to_bits(val)
        }
    }
    #[doc = "Snapshot data format."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Snpsfmt {
        #[doc = "23-bit data resolution without integrator counter."]
        Resolution23bit = 0x0,
        #[doc = "16-bit data resolution with integrator counter."]
        Resolution16bit = 0x01,
    }
    impl Snpsfmt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Snpsfmt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Snpsfmt {
        #[inline(always)]
        fn from(val: u8) -> Snpsfmt {
            Snpsfmt::from_bits(val)
        }
    }
    impl From<Snpsfmt> for u8 {
        #[inline(always)]
        fn from(val: Snpsfmt) -> u8 {
            Snpsfmt::to_bits(val)
        }
    }
    #[doc = "Out-of-limit threshold band mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Thinb {
        #[doc = "Event when signal is below low or above high threshold."]
        OutsideBand = 0x0,
        #[doc = "Event when signal is between low and high thresholds."]
        InsideBand = 0x01,
    }
    impl Thinb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Thinb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Thinb {
        #[inline(always)]
        fn from(val: u8) -> Thinb {
            Thinb::from_bits(val)
        }
    }
    impl From<Thinb> for u8 {
        #[inline(always)]
        fn from(val: Thinb) -> u8 {
            Thinb::to_bits(val)
        }
    }
    #[doc = "CKGEN trigger sensitivity selection. This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger signals. This bit is not significant if the CKGMOD = 0."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trgsens {
        #[doc = "A rising edge event triggers the activation of CKGEN dividers."]
        RisingEdge = 0x0,
        #[doc = "A falling edge even triggers the activation of CKGEN dividers."]
        FallingEdge = 0x01,
    }
    impl Trgsens {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trgsens {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trgsens {
        #[inline(always)]
        fn from(val: u8) -> Trgsens {
            Trgsens::from_bits(val)
        }
    }
    impl From<Trgsens> for u8 {
        #[inline(always)]
        fn from(val: Trgsens) -> u8 {
            Trgsens::to_bits(val)
        }
    }
    #[doc = "Digital filter trigger signal selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trgsrc {
        #[doc = "TRGO is selected."]
        Trgo = 0x0,
        #[doc = "OLDx event is selected."]
        Oldx = 0x01,
        #[doc = "mdf_trg\\[0\\]
is selected."]
        MdfTrg0 = 0x02,
        #[doc = "mdf_trg\\[1\\]
is selected."]
        MdfTrg1 = 0x03,
        #[doc = "mdf_trg\\[2\\]
is selected."]
        MdfTrg2 = 0x04,
        #[doc = "mdf_trg\\[3\\]
is selected."]
        MdfTrg3 = 0x05,
        #[doc = "mdf_trg\\[4\\]
is selected."]
        MdfTrg4 = 0x06,
        #[doc = "mdf_trg\\[5\\]
is selected."]
        MdfTrg5 = 0x07,
        #[doc = "mdf_trg\\[6\\]
is selected."]
        MdfTrg6 = 0x08,
        #[doc = "mdf_trg\\[7\\]
is selected."]
        MdfTrg7 = 0x09,
        #[doc = "mdf_trg\\[8\\]
is selected."]
        MdfTrg8 = 0x0a,
        #[doc = "mdf_trg\\[9\\]
is selected."]
        MdfTrg9 = 0x0b,
        #[doc = "mdf_trg\\[10\\]
is selected."]
        MdfTrg10 = 0x0c,
        #[doc = "mdf_trg\\[11\\]
is selected."]
        MdfTrg11 = 0x0d,
        #[doc = "mdf_trg\\[12\\]
is selected."]
        MdfTrg12 = 0x0e,
        #[doc = "mdf_trg\\[13\\]
is selected."]
        MdfTrg13 = 0x0f,
    }
    impl Trgsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trgsrc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trgsrc {
        #[inline(always)]
        fn from(val: u8) -> Trgsrc {
            Trgsrc::from_bits(val)
        }
    }
    impl From<Trgsrc> for u8 {
        #[inline(always)]
        fn from(val: Trgsrc) -> u8 {
            Trgsrc::to_bits(val)
        }
    }
}
