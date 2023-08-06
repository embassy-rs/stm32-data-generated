#[doc = "ADC common registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCommon {
    ptr: *mut u8,
}
unsafe impl Send for AdcCommon {}
unsafe impl Sync for AdcCommon {}
impl AdcCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC Common status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Dual ADC mode selection"]
        #[inline(always)]
        pub const fn dual(&self) -> super::vals::Dual {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Dual::from_bits(val as u8)
        }
        #[doc = "Dual ADC mode selection"]
        #[inline(always)]
        pub fn set_dual(&mut self, val: super::vals::Dual) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub const fn delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DMA configuration (for multi-ADC mode)"]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "DMA configuration (for multi-ADC mode)"]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub const fn mdma(&self) -> super::vals::Mdma {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Mdma::from_bits(val as u8)
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub fn set_mdma(&mut self, val: super::vals::Mdma) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub const fn ckmode(&self) -> super::vals::Ckmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ckmode::from_bits(val as u8)
        }
        #[doc = "ADC clock mode"]
        #[inline(always)]
        pub fn set_ckmode(&mut self, val: super::vals::Ckmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub const fn vbaten(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub fn set_vbaten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr(pub u32);
    impl Cdr {
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub const fn rdata_mst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub fn set_rdata_mst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub const fn rdata_slv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data of the master ADC"]
        #[inline(always)]
        pub fn set_rdata_slv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cdr {
        #[inline(always)]
        fn default() -> Cdr {
            Cdr(0)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Master ADC ready"]
        #[inline(always)]
        pub const fn adrdy_mst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master ADC ready"]
        #[inline(always)]
        pub fn set_adrdy_mst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling phase flag of the master ADC"]
        #[inline(always)]
        pub const fn eosmp_mst(&self) -> super::vals::Ended {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of sampling phase flag of the master ADC"]
        #[inline(always)]
        pub fn set_eosmp_mst(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "End of regular conversion of the master ADC"]
        #[inline(always)]
        pub const fn eoc_mst(&self) -> super::vals::Ended {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of regular conversion of the master ADC"]
        #[inline(always)]
        pub fn set_eoc_mst(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence flag of the master ADC"]
        #[inline(always)]
        pub const fn eos_mst(&self) -> super::vals::Ended {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of regular sequence flag of the master ADC"]
        #[inline(always)]
        pub fn set_eos_mst(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun flag of the master ADC"]
        #[inline(always)]
        pub const fn ovr_mst(&self) -> super::vals::Ovr {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Ovr::from_bits(val as u8)
        }
        #[doc = "Overrun flag of the master ADC"]
        #[inline(always)]
        pub fn set_ovr_mst(&mut self, val: super::vals::Ovr) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "End of injected conversion of the master ADC"]
        #[inline(always)]
        pub const fn jeoc_mst(&self) -> super::vals::Ended {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of injected conversion of the master ADC"]
        #[inline(always)]
        pub fn set_jeoc_mst(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "End of injected sequence flag of the master ADC"]
        #[inline(always)]
        pub const fn jeos(&self) -> super::vals::Ended {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of injected sequence flag of the master ADC"]
        #[inline(always)]
        pub fn set_jeos(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog 1 flag of the master ADC"]
        #[inline(always)]
        pub const fn awd1_mst(&self) -> super::vals::Awd {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 1 flag of the master ADC"]
        #[inline(always)]
        pub fn set_awd1_mst(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Analog watchdog 2 flag of the master ADC"]
        #[inline(always)]
        pub const fn awd2_mst(&self) -> super::vals::Awd {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 2 flag of the master ADC"]
        #[inline(always)]
        pub fn set_awd2_mst(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Analog watchdog 3 flag of the master ADC"]
        #[inline(always)]
        pub const fn awd3_mst(&self) -> super::vals::Awd {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 3 flag of the master ADC"]
        #[inline(always)]
        pub fn set_awd3_mst(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Injected context queue overflow flag of the master ADC"]
        #[inline(always)]
        pub const fn jqovf_mst(&self) -> super::vals::Jqovf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Jqovf::from_bits(val as u8)
        }
        #[doc = "Injected context queue overflow flag of the master ADC"]
        #[inline(always)]
        pub fn set_jqovf_mst(&mut self, val: super::vals::Jqovf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Slave ADC ready"]
        #[inline(always)]
        pub const fn adrdy_slv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Slave ADC ready"]
        #[inline(always)]
        pub fn set_adrdy_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "End of sampling phase flag of the slave ADC"]
        #[inline(always)]
        pub const fn eosmp_slv(&self) -> super::vals::Ended {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of sampling phase flag of the slave ADC"]
        #[inline(always)]
        pub fn set_eosmp_slv(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "End of regular conversion of the slave ADC"]
        #[inline(always)]
        pub const fn eoc_slv(&self) -> super::vals::Ended {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of regular conversion of the slave ADC"]
        #[inline(always)]
        pub fn set_eoc_slv(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "End of regular sequence flag of the slave ADC"]
        #[inline(always)]
        pub const fn eos_slv(&self) -> super::vals::Ended {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of regular sequence flag of the slave ADC"]
        #[inline(always)]
        pub fn set_eos_slv(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Overrun flag of the slave ADC"]
        #[inline(always)]
        pub const fn ovr_slv(&self) -> super::vals::Ovr {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ovr::from_bits(val as u8)
        }
        #[doc = "Overrun flag of the slave ADC"]
        #[inline(always)]
        pub fn set_ovr_slv(&mut self, val: super::vals::Ovr) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "End of injected conversion of the slave ADC"]
        #[inline(always)]
        pub const fn jeoc_slv(&self) -> super::vals::Ended {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of injected conversion of the slave ADC"]
        #[inline(always)]
        pub fn set_jeoc_slv(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "End of injected sequence flag of the slave ADC"]
        #[inline(always)]
        pub const fn jeos_slv(&self) -> super::vals::Ended {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ended::from_bits(val as u8)
        }
        #[doc = "End of injected sequence flag of the slave ADC"]
        #[inline(always)]
        pub fn set_jeos_slv(&mut self, val: super::vals::Ended) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC"]
        #[inline(always)]
        pub const fn awd1_slv(&self) -> super::vals::Awd {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 1 flag of the slave ADC"]
        #[inline(always)]
        pub fn set_awd1_slv(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Analog watchdog 2 flag of the slave ADC"]
        #[inline(always)]
        pub const fn awd2_slv(&self) -> super::vals::Awd {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 2 flag of the slave ADC"]
        #[inline(always)]
        pub fn set_awd2_slv(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Analog watchdog 3 flag of the slave ADC"]
        #[inline(always)]
        pub const fn awd3_slv(&self) -> super::vals::Awd {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Awd::from_bits(val as u8)
        }
        #[doc = "Analog watchdog 3 flag of the slave ADC"]
        #[inline(always)]
        pub fn set_awd3_slv(&mut self, val: super::vals::Awd) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Injected context queue overflow flag of the slave ADC"]
        #[inline(always)]
        pub const fn jqovf_slv(&self) -> super::vals::Jqovf {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Jqovf::from_bits(val as u8)
        }
        #[doc = "Injected context queue overflow flag of the slave ADC"]
        #[inline(always)]
        pub fn set_jqovf_slv(&mut self, val: super::vals::Jqovf) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
}
pub mod vals {
    #[doc = "Analog watchdog flag"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Awd {
        #[doc = "No analog watchdog event occurred"]
        NOEVENT = 0,
        #[doc = "Analog watchdog event occurred"]
        EVENT = 0x01,
    }
    impl Awd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Awd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Awd {
        #[inline(always)]
        fn from(val: u8) -> Awd {
            Awd::from_bits(val)
        }
    }
    impl From<Awd> for u8 {
        #[inline(always)]
        fn from(val: Awd) -> u8 {
            Awd::to_bits(val)
        }
    }
    #[doc = "ADC clock mode"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckmode {
        #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous mode"]
        ASYNCHRONOUS = 0,
        #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck."]
        SYNCDIV1 = 0x01,
        #[doc = "Use AHB clock rcc_hclk3 divided by 2."]
        SYNCDIV2 = 0x02,
        #[doc = "Use AHB clock rcc_hclk3 divided by 4."]
        SYNCDIV4 = 0x03,
    }
    impl Ckmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckmode {
        #[inline(always)]
        fn from(val: u8) -> Ckmode {
            Ckmode::from_bits(val)
        }
    }
    impl From<Ckmode> for u8 {
        #[inline(always)]
        fn from(val: Ckmode) -> u8 {
            Ckmode::to_bits(val)
        }
    }
    #[doc = "DMA configuration (for multi-ADC mode)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dmacfg {
        #[doc = "DMA one shot mode selected"]
        ONESHOT = 0,
        #[doc = "DMA circular mode selected"]
        CIRCULATOR = 0x01,
    }
    impl Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Dmacfg {
            Dmacfg::from_bits(val)
        }
    }
    impl From<Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Dmacfg) -> u8 {
            Dmacfg::to_bits(val)
        }
    }
    #[doc = "Dual ADC mode selection"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dual {
        #[doc = "Independent mode"]
        INDEPENDENT = 0,
        #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
        DUALRJ = 0x01,
        #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
        DUALRA = 0x02,
        #[doc = "Dual, combined injected simultaneous + fast interleaved mode"]
        DUALIJ = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "Dual, injected simultaneous mode only"]
        DUALJ = 0x05,
        #[doc = "Dual, regular simultaneous mode only"]
        DUALR = 0x06,
        #[doc = "dual, interleaved mode only"]
        DUALI = 0x07,
        _RESERVED_8 = 0x08,
        #[doc = "Dual, alternate trigger mode only"]
        DUALA = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Dual {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dual {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dual {
        #[inline(always)]
        fn from(val: u8) -> Dual {
            Dual::from_bits(val)
        }
    }
    impl From<Dual> for u8 {
        #[inline(always)]
        fn from(val: Dual) -> u8 {
            Dual::to_bits(val)
        }
    }
    #[doc = "End of operation"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ended {
        #[doc = "Operation is not ended"]
        NOTENDED = 0,
        #[doc = "Operation is ended"]
        ENDED = 0x01,
    }
    impl Ended {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ended {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ended {
        #[inline(always)]
        fn from(val: u8) -> Ended {
            Ended::from_bits(val)
        }
    }
    impl From<Ended> for u8 {
        #[inline(always)]
        fn from(val: Ended) -> u8 {
            Ended::to_bits(val)
        }
    }
    #[doc = "Injected context queue overflow flag"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Jqovf {
        #[doc = "No injected context queue overflow"]
        NOOVERFLOW = 0,
        #[doc = "Injected context queue overflow"]
        OVERFLOW = 0x01,
    }
    impl Jqovf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Jqovf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Jqovf {
        #[inline(always)]
        fn from(val: u8) -> Jqovf {
            Jqovf::from_bits(val)
        }
    }
    impl From<Jqovf> for u8 {
        #[inline(always)]
        fn from(val: Jqovf) -> u8 {
            Jqovf::to_bits(val)
        }
    }
    #[doc = "Direct memory access mode for multi ADC mode"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mdma {
        #[doc = "MDMA mode disabled"]
        DISABLED = 0,
        _RESERVED_1 = 0x01,
        #[doc = "MDMA mode enabled for 12 and 10-bit resolution"]
        BITS12_10 = 0x02,
        #[doc = "MDMA mode enabled for 8 and 6-bit resolution"]
        BIT8_6 = 0x03,
    }
    impl Mdma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mdma {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mdma {
        #[inline(always)]
        fn from(val: u8) -> Mdma {
            Mdma::from_bits(val)
        }
    }
    impl From<Mdma> for u8 {
        #[inline(always)]
        fn from(val: Mdma) -> u8 {
            Mdma::to_bits(val)
        }
    }
    #[doc = "Overrun flag"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ovr {
        #[doc = "No overrun occurred"]
        NOOVERRUN = 0,
        #[doc = "Overrun occurred"]
        OVERRUN = 0x01,
    }
    impl Ovr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovr {
        #[inline(always)]
        fn from(val: u8) -> Ovr {
            Ovr::from_bits(val)
        }
    }
    impl From<Ovr> for u8 {
        #[inline(always)]
        fn from(val: Ovr) -> u8 {
            Ovr::to_bits(val)
        }
    }
}
