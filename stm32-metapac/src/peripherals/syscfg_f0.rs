#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize + n * 4usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
        #[inline(always)]
        pub const fn pa11_pa12_rmp(&self) -> super::vals::Pa11Pa12Rmp {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Pa11Pa12Rmp::from_bits(val as u8)
        }
        #[doc = "PA11 and PA12 remapping bit for small packages (28 and 20 pins)"]
        #[inline(always)]
        pub fn set_pa11_pa12_rmp(&mut self, val: super::vals::Pa11Pa12Rmp) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "IR Modulation Envelope signal selection"]
        #[inline(always)]
        pub const fn ir_mod(&self) -> super::vals::IrMod {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::IrMod::from_bits(val as u8)
        }
        #[doc = "IR Modulation Envelope signal selection"]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: super::vals::IrMod) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "ADC DMA remapping bit"]
        #[inline(always)]
        pub const fn adc_dma_rmp(&self) -> super::vals::AdcDmaRmp {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::AdcDmaRmp::from_bits(val as u8)
        }
        #[doc = "ADC DMA remapping bit"]
        #[inline(always)]
        pub fn set_adc_dma_rmp(&mut self, val: super::vals::AdcDmaRmp) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "USART1_TX DMA remapping bit"]
        #[inline(always)]
        pub const fn usart1_tx_dma_rmp(&self) -> super::vals::Usart1TxDmaRmp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Usart1TxDmaRmp::from_bits(val as u8)
        }
        #[doc = "USART1_TX DMA remapping bit"]
        #[inline(always)]
        pub fn set_usart1_tx_dma_rmp(&mut self, val: super::vals::Usart1TxDmaRmp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "USART1_RX DMA request remapping bit"]
        #[inline(always)]
        pub const fn usart1_rx_dma_rmp(&self) -> super::vals::Usart1RxDmaRmp {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Usart1RxDmaRmp::from_bits(val as u8)
        }
        #[doc = "USART1_RX DMA request remapping bit"]
        #[inline(always)]
        pub fn set_usart1_rx_dma_rmp(&mut self, val: super::vals::Usart1RxDmaRmp) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM16 DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim16_dma_rmp(&self) -> super::vals::Tim16DmaRmp {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Tim16DmaRmp::from_bits(val as u8)
        }
        #[doc = "TIM16 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim16_dma_rmp(&mut self, val: super::vals::Tim16DmaRmp) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM17 DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim17_dma_rmp(&self) -> super::vals::Tim17DmaRmp {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Tim17DmaRmp::from_bits(val as u8)
        }
        #[doc = "TIM17 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim17_dma_rmp(&mut self, val: super::vals::Tim17DmaRmp) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM16 alternate DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim16_dma_rmp2(&self) -> super::vals::Tim16DmaRmp2 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Tim16DmaRmp2::from_bits(val as u8)
        }
        #[doc = "TIM16 alternate DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim16_dma_rmp2(&mut self, val: super::vals::Tim16DmaRmp2) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM17 alternate DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim17_dma_rmp2(&self) -> super::vals::Tim17DmaRmp2 {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Tim17DmaRmp2::from_bits(val as u8)
        }
        #[doc = "TIM17 alternate DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim17_dma_rmp2(&mut self, val: super::vals::Tim17DmaRmp2) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Fast Mode Plus (FM plus) driving capability activation bits."]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> super::vals::I2cPb6Fmp {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::I2cPb6Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM plus) driving capability activation bits."]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: super::vals::I2cPb6Fmp) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> super::vals::I2cPb7Fmp {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::I2cPb7Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: super::vals::I2cPb7Fmp) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> super::vals::I2cPb8Fmp {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::I2cPb8Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: super::vals::I2cPb8Fmp) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> super::vals::I2cPb9Fmp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::I2cPb9Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits."]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: super::vals::I2cPb9Fmp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "FM+ driving capability activation for I2C1"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> super::vals::I2c1Fmp {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::I2c1Fmp::from_bits(val as u8)
        }
        #[doc = "FM+ driving capability activation for I2C1"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: super::vals::I2c1Fmp) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "FM+ driving capability activation for I2C2"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> super::vals::I2c2Fmp {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::I2c2Fmp::from_bits(val as u8)
        }
        #[doc = "FM+ driving capability activation for I2C2"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: super::vals::I2c2Fmp) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub const fn i2c_pa9_fmp(&self) -> super::vals::I2cPa9Fmp {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::I2cPa9Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub fn set_i2c_pa9_fmp(&mut self, val: super::vals::I2cPa9Fmp) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub const fn i2c_pa10_fmp(&self) -> super::vals::I2cPa10Fmp {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::I2cPa10Fmp::from_bits(val as u8)
        }
        #[doc = "Fast Mode Plus (FM+) driving capability activation bits"]
        #[inline(always)]
        pub fn set_i2c_pa10_fmp(&mut self, val: super::vals::I2cPa10Fmp) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "SPI2 DMA request remapping bit"]
        #[inline(always)]
        pub const fn spi2_dma_rmp(&self) -> super::vals::Spi2DmaRmp {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Spi2DmaRmp::from_bits(val as u8)
        }
        #[doc = "SPI2 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_spi2_dma_rmp(&mut self, val: super::vals::Spi2DmaRmp) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "USART2 DMA request remapping bit"]
        #[inline(always)]
        pub const fn usart2_dma_rmp(&self) -> super::vals::Usart2DmaRmp {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Usart2DmaRmp::from_bits(val as u8)
        }
        #[doc = "USART2 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_usart2_dma_rmp(&mut self, val: super::vals::Usart2DmaRmp) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "USART3 DMA request remapping bit"]
        #[inline(always)]
        pub const fn usart3_dma_rmp(&self) -> super::vals::Usart3DmaRmp {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Usart3DmaRmp::from_bits(val as u8)
        }
        #[doc = "USART3 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_usart3_dma_rmp(&mut self, val: super::vals::Usart3DmaRmp) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "I2C1 DMA request remapping bit"]
        #[inline(always)]
        pub const fn i2c1_dma_rmp(&self) -> super::vals::I2c1DmaRmp {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::I2c1DmaRmp::from_bits(val as u8)
        }
        #[doc = "I2C1 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_i2c1_dma_rmp(&mut self, val: super::vals::I2c1DmaRmp) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "TIM1 DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim1_dma_rmp(&self) -> super::vals::Tim1DmaRmp {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Tim1DmaRmp::from_bits(val as u8)
        }
        #[doc = "TIM1 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim1_dma_rmp(&mut self, val: super::vals::Tim1DmaRmp) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "TIM2 DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim2_dma_rmp(&self) -> super::vals::Tim2DmaRmp {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Tim2DmaRmp::from_bits(val as u8)
        }
        #[doc = "TIM2 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim2_dma_rmp(&mut self, val: super::vals::Tim2DmaRmp) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM3 DMA request remapping bit"]
        #[inline(always)]
        pub const fn tim3_dma_rmp(&self) -> super::vals::Tim3DmaRmp {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Tim3DmaRmp::from_bits(val as u8)
        }
        #[doc = "TIM3 DMA request remapping bit"]
        #[inline(always)]
        pub fn set_tim3_dma_rmp(&mut self, val: super::vals::Tim3DmaRmp) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M0 LOCKUP bit enable bit"]
        #[inline(always)]
        pub const fn lockup_lock(&self) -> super::vals::LockupLock {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::LockupLock::from_bits(val as u8)
        }
        #[doc = "Cortex-M0 LOCKUP bit enable bit"]
        #[inline(always)]
        pub fn set_lockup_lock(&mut self, val: super::vals::LockupLock) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub const fn sram_parity_lock(&self) -> super::vals::SramParityLock {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::SramParityLock::from_bits(val as u8)
        }
        #[doc = "SRAM parity lock bit"]
        #[inline(always)]
        pub fn set_sram_parity_lock(&mut self, val: super::vals::SramParityLock) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvd_lock(&self) -> super::vals::PvdLock {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::PvdLock::from_bits(val as u8)
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvd_lock(&mut self, val: super::vals::PvdLock) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM parity flag"]
        #[inline(always)]
        pub const fn sram_pef(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM parity flag"]
        #[inline(always)]
        pub fn set_sram_pef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum AdcDmaRmp {
        #[doc = "ADC DMA request mapped on DMA channel 1"]
        NOTREMAPPED = 0,
        #[doc = "ADC DMA request mapped on DMA channel 2"]
        REMAPPED = 0x01,
    }
    impl AdcDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AdcDmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AdcDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> AdcDmaRmp {
            AdcDmaRmp::from_bits(val)
        }
    }
    impl From<AdcDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: AdcDmaRmp) -> u8 {
            AdcDmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2c1DmaRmp {
        #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
        NOTREMAPPED = 0,
        #[doc = "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively"]
        REMAPPED = 0x01,
    }
    impl I2c1DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> I2c1DmaRmp {
            I2c1DmaRmp::from_bits(val)
        }
    }
    impl From<I2c1DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: I2c1DmaRmp) -> u8 {
            I2c1DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2c1Fmp {
        #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
        STANDARD = 0,
        #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers"]
        FMP = 0x01,
    }
    impl I2c1Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2c1Fmp {
            I2c1Fmp::from_bits(val)
        }
    }
    impl From<I2c1Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2c1Fmp) -> u8 {
            I2c1Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2c2Fmp {
        #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
        STANDARD = 0,
        #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers"]
        FMP = 0x01,
    }
    impl I2c2Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c2Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c2Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2c2Fmp {
            I2c2Fmp::from_bits(val)
        }
    }
    impl From<I2c2Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2c2Fmp) -> u8 {
            I2c2Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPa10Fmp {
        #[doc = "PA10 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PA10 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPa10Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPa10Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPa10Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPa10Fmp {
            I2cPa10Fmp::from_bits(val)
        }
    }
    impl From<I2cPa10Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPa10Fmp) -> u8 {
            I2cPa10Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPa9Fmp {
        #[doc = "PA9 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PA9 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPa9Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPa9Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPa9Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPa9Fmp {
            I2cPa9Fmp::from_bits(val)
        }
    }
    impl From<I2cPa9Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPa9Fmp) -> u8 {
            I2cPa9Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPb6Fmp {
        #[doc = "PB6 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPb6Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPb6Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPb6Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPb6Fmp {
            I2cPb6Fmp::from_bits(val)
        }
    }
    impl From<I2cPb6Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPb6Fmp) -> u8 {
            I2cPb6Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPb7Fmp {
        #[doc = "PB7 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPb7Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPb7Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPb7Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPb7Fmp {
            I2cPb7Fmp::from_bits(val)
        }
    }
    impl From<I2cPb7Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPb7Fmp) -> u8 {
            I2cPb7Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPb8Fmp {
        #[doc = "PB8 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPb8Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPb8Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPb8Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPb8Fmp {
            I2cPb8Fmp::from_bits(val)
        }
    }
    impl From<I2cPb8Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPb8Fmp) -> u8 {
            I2cPb8Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2cPb9Fmp {
        #[doc = "PB9 pin operate in standard mode"]
        STANDARD = 0,
        #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
        FMP = 0x01,
    }
    impl I2cPb9Fmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2cPb9Fmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2cPb9Fmp {
        #[inline(always)]
        fn from(val: u8) -> I2cPb9Fmp {
            I2cPb9Fmp::from_bits(val)
        }
    }
    impl From<I2cPb9Fmp> for u8 {
        #[inline(always)]
        fn from(val: I2cPb9Fmp) -> u8 {
            I2cPb9Fmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IrMod {
        #[doc = "TIM16 selected"]
        TIM16 = 0,
        #[doc = "USART1 selected"]
        USART1 = 0x01,
        #[doc = "USART4 selected"]
        USART4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl IrMod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMod {
        #[inline(always)]
        fn from(val: u8) -> IrMod {
            IrMod::from_bits(val)
        }
    }
    impl From<IrMod> for u8 {
        #[inline(always)]
        fn from(val: IrMod) -> u8 {
            IrMod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum LockupLock {
        #[doc = "Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input"]
        DISCONNECTED = 0,
        #[doc = "Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input"]
        CONNECTED = 0x01,
    }
    impl LockupLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LockupLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LockupLock {
        #[inline(always)]
        fn from(val: u8) -> LockupLock {
            LockupLock::from_bits(val)
        }
    }
    impl From<LockupLock> for u8 {
        #[inline(always)]
        fn from(val: LockupLock) -> u8 {
            LockupLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum MemMode {
        #[doc = "Main Flash memory mapped at 0x0000_0000"]
        MAINFLASH = 0,
        #[doc = "System Flash memory mapped at 0x0000_0000"]
        SYSTEMFLASH = 0x01,
        #[doc = "Main Flash memory mapped at 0x0000_0000"]
        MAINFLASH2 = 0x02,
        #[doc = "Embedded SRAM mapped at 0x0000_0000"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pa11Pa12Rmp {
        #[doc = "Pin pair PA9/PA10 mapped on the pins"]
        NOTREMAPPED = 0,
        #[doc = "Pin pair PA11/PA12 mapped instead of PA9/PA10"]
        REMAPPED = 0x01,
    }
    impl Pa11Pa12Rmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pa11Pa12Rmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pa11Pa12Rmp {
        #[inline(always)]
        fn from(val: u8) -> Pa11Pa12Rmp {
            Pa11Pa12Rmp::from_bits(val)
        }
    }
    impl From<Pa11Pa12Rmp> for u8 {
        #[inline(always)]
        fn from(val: Pa11Pa12Rmp) -> u8 {
            Pa11Pa12Rmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PvdLock {
        #[doc = "PVD interrupt disconnected from TIM1/15/16/17 Break input"]
        DISCONNECTED = 0,
        #[doc = "PVD interrupt connected to TIM1/15/16/17 Break input"]
        CONNECTED = 0x01,
    }
    impl PvdLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PvdLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PvdLock {
        #[inline(always)]
        fn from(val: u8) -> PvdLock {
            PvdLock::from_bits(val)
        }
    }
    impl From<PvdLock> for u8 {
        #[inline(always)]
        fn from(val: PvdLock) -> u8 {
            PvdLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spi2DmaRmp {
        #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively"]
        NOTREMAPPED = 0,
        #[doc = "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        REMAPPED = 0x01,
    }
    impl Spi2DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi2DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi2DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Spi2DmaRmp {
            Spi2DmaRmp::from_bits(val)
        }
    }
    impl From<Spi2DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Spi2DmaRmp) -> u8 {
            Spi2DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SramParityLock {
        #[doc = "SRAM parity error disconnected from TIM1/15/16/17 Break input"]
        DISCONNECTED = 0,
        #[doc = "SRAM parity error connected to TIM1/15/16/17 Break input"]
        CONNECTED = 0x01,
    }
    impl SramParityLock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SramParityLock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SramParityLock {
        #[inline(always)]
        fn from(val: u8) -> SramParityLock {
            SramParityLock::from_bits(val)
        }
    }
    impl From<SramParityLock> for u8 {
        #[inline(always)]
        fn from(val: SramParityLock) -> u8 {
            SramParityLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim16DmaRmp {
        #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3"]
        NOTREMAPPED = 0,
        #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4"]
        REMAPPED = 0x01,
    }
    impl Tim16DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim16DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim16DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Tim16DmaRmp {
            Tim16DmaRmp::from_bits(val)
        }
    }
    impl From<Tim16DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Tim16DmaRmp) -> u8 {
            Tim16DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim16DmaRmp2 {
        #[doc = "TIM16 DMA request mapped according to TIM16_DMA_RMP bit"]
        NOTALTERNATEREMAPPED = 0,
        #[doc = "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6"]
        ALTERNATEREMAPPED = 0x01,
    }
    impl Tim16DmaRmp2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim16DmaRmp2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim16DmaRmp2 {
        #[inline(always)]
        fn from(val: u8) -> Tim16DmaRmp2 {
            Tim16DmaRmp2::from_bits(val)
        }
    }
    impl From<Tim16DmaRmp2> for u8 {
        #[inline(always)]
        fn from(val: Tim16DmaRmp2) -> u8 {
            Tim16DmaRmp2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim17DmaRmp {
        #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1"]
        NOTREMAPPED = 0,
        #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2"]
        REMAPPED = 0x01,
    }
    impl Tim17DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim17DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim17DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Tim17DmaRmp {
            Tim17DmaRmp::from_bits(val)
        }
    }
    impl From<Tim17DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Tim17DmaRmp) -> u8 {
            Tim17DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim17DmaRmp2 {
        #[doc = "TIM17 DMA request mapped according to TIM16_DMA_RMP bit"]
        NOTALTERNATEREMAPPED = 0,
        #[doc = "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7"]
        ALTERNATEREMAPPED = 0x01,
    }
    impl Tim17DmaRmp2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim17DmaRmp2 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim17DmaRmp2 {
        #[inline(always)]
        fn from(val: u8) -> Tim17DmaRmp2 {
            Tim17DmaRmp2::from_bits(val)
        }
    }
    impl From<Tim17DmaRmp2> for u8 {
        #[inline(always)]
        fn from(val: Tim17DmaRmp2) -> u8 {
            Tim17DmaRmp2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim1DmaRmp {
        #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively"]
        NOTREMAPPED = 0,
        #[doc = "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6"]
        REMAPPED = 0x01,
    }
    impl Tim1DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim1DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim1DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Tim1DmaRmp {
            Tim1DmaRmp::from_bits(val)
        }
    }
    impl From<Tim1DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Tim1DmaRmp) -> u8 {
            Tim1DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim2DmaRmp {
        #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively"]
        NOTREMAPPED = 0,
        #[doc = "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7"]
        REMAPPED = 0x01,
    }
    impl Tim2DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim2DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim2DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Tim2DmaRmp {
            Tim2DmaRmp::from_bits(val)
        }
    }
    impl From<Tim2DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Tim2DmaRmp) -> u8 {
            Tim2DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tim3DmaRmp {
        #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4"]
        NOTREMAPPED = 0,
        #[doc = "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6"]
        REMAPPED = 0x01,
    }
    impl Tim3DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim3DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim3DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Tim3DmaRmp {
            Tim3DmaRmp::from_bits(val)
        }
    }
    impl From<Tim3DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Tim3DmaRmp) -> u8 {
            Tim3DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart1RxDmaRmp {
        #[doc = "USART1_RX DMA request mapped on DMA channel 3"]
        NOTREMAPPED = 0,
        #[doc = "USART1_RX DMA request mapped on DMA channel 5"]
        REMAPPED = 0x01,
    }
    impl Usart1RxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1RxDmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1RxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Usart1RxDmaRmp {
            Usart1RxDmaRmp::from_bits(val)
        }
    }
    impl From<Usart1RxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Usart1RxDmaRmp) -> u8 {
            Usart1RxDmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart1TxDmaRmp {
        #[doc = "USART1_TX DMA request mapped on DMA channel 2"]
        NOTREMAPPED = 0,
        #[doc = "USART1_TX DMA request mapped on DMA channel 4"]
        REMAPPED = 0x01,
    }
    impl Usart1TxDmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1TxDmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1TxDmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Usart1TxDmaRmp {
            Usart1TxDmaRmp::from_bits(val)
        }
    }
    impl From<Usart1TxDmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Usart1TxDmaRmp) -> u8 {
            Usart1TxDmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart2DmaRmp {
        #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively"]
        NOTREMAPPED = 0,
        #[doc = "USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively"]
        REMAPPED = 0x01,
    }
    impl Usart2DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart2DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart2DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Usart2DmaRmp {
            Usart2DmaRmp::from_bits(val)
        }
    }
    impl From<Usart2DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Usart2DmaRmp) -> u8 {
            Usart2DmaRmp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart3DmaRmp {
        #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)"]
        NOTREMAPPED = 0,
        #[doc = "USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively"]
        REMAPPED = 0x01,
    }
    impl Usart3DmaRmp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart3DmaRmp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart3DmaRmp {
        #[inline(always)]
        fn from(val: u8) -> Usart3DmaRmp {
            Usart3DmaRmp::from_bits(val)
        }
    }
    impl From<Usart3DmaRmp> for u8 {
        #[inline(always)]
        fn from(val: Usart3DmaRmp) -> u8 {
            Usart3DmaRmp::to_bits(val)
        }
    }
}
