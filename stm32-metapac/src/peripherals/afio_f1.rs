#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Alternate function I/O"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afio {
    ptr: *mut u8,
}
unsafe impl Send for Afio {}
unsafe impl Sync for Afio {}
impl Afio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[inline(always)]
    pub const fn evcr(self) -> crate::common::Reg<regs::Evcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[inline(always)]
    pub const fn mapr(self) -> crate::common::Reg<regs::Mapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[inline(always)]
    pub const fn mapr2(self) -> crate::common::Reg<regs::Mapr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evcr(pub u32);
    impl Evcr {
        #[doc = "Pin selection"]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pin selection"]
        #[inline(always)]
        pub fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Port selection"]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Port selection"]
        #[inline(always)]
        pub fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event Output Enable"]
        #[inline(always)]
        pub const fn evoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event Output Enable"]
        #[inline(always)]
        pub fn set_evoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Evcr {
        #[inline(always)]
        fn default() -> Evcr {
            Evcr(0)
        }
    }
    #[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI12 configuration"]
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
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr(pub u32);
    impl Mapr {
        #[doc = "SPI1 remapping"]
        #[inline(always)]
        pub const fn spi1_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 remapping"]
        #[inline(always)]
        pub fn set_spi1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C1 remapping"]
        #[inline(always)]
        pub const fn i2c1_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping"]
        #[inline(always)]
        pub fn set_i2c1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub const fn usart1_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub fn set_usart1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub const fn usart2_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub fn set_usart2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART3 remapping"]
        #[inline(always)]
        pub const fn usart3_remap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 remapping"]
        #[inline(always)]
        pub fn set_usart3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "TIM1 remapping"]
        #[inline(always)]
        pub const fn tim1_remap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 remapping"]
        #[inline(always)]
        pub fn set_tim1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "TIM2 remapping"]
        #[inline(always)]
        pub const fn tim2_remap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 remapping"]
        #[inline(always)]
        pub fn set_tim2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TIM3 remapping"]
        #[inline(always)]
        pub const fn tim3_remap(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TIM3 remapping"]
        #[inline(always)]
        pub fn set_tim3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIM4 remapping"]
        #[inline(always)]
        pub const fn tim4_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 remapping"]
        #[inline(always)]
        pub fn set_tim4_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN1 remapping"]
        #[inline(always)]
        pub const fn can1_remap(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CAN1 remapping"]
        #[inline(always)]
        pub fn set_can1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT"]
        #[inline(always)]
        pub const fn pd01_remap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D0/Port D1 mapping on OSCIN/OSCOUT"]
        #[inline(always)]
        pub fn set_pd01_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Set and cleared by software"]
        #[inline(always)]
        pub const fn tim5ch4_iremap(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Set and cleared by software"]
        #[inline(always)]
        pub fn set_tim5ch4_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC 1 External trigger injected conversion remapping"]
        #[inline(always)]
        pub const fn adc1_etrginj_remap(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 External trigger injected conversion remapping"]
        #[inline(always)]
        pub fn set_adc1_etrginj_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC 1 external trigger regular conversion remapping"]
        #[inline(always)]
        pub const fn adc1_etrgreg_remap(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 external trigger regular conversion remapping"]
        #[inline(always)]
        pub fn set_adc1_etrgreg_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADC 2 external trigger injected conversion remapping"]
        #[inline(always)]
        pub const fn adc2_etrginj_remap(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 external trigger injected conversion remapping"]
        #[inline(always)]
        pub fn set_adc2_etrginj_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADC 2 external trigger regular conversion remapping"]
        #[inline(always)]
        pub const fn adc2_etrgreg_remap(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 external trigger regular conversion remapping"]
        #[inline(always)]
        pub fn set_adc2_etrgreg_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[inline(always)]
        pub const fn eth_remap(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[inline(always)]
        pub fn set_eth_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CAN2 I/O remapping"]
        #[inline(always)]
        pub const fn can2_remap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 I/O remapping"]
        #[inline(always)]
        pub fn set_can2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MII or RMII selection"]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MII or RMII selection"]
        #[inline(always)]
        pub fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Serial wire JTAG configuration"]
        #[inline(always)]
        pub const fn swj_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration"]
        #[inline(always)]
        pub fn set_swj_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[inline(always)]
        pub const fn spi3_remap(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[inline(always)]
        pub fn set_spi3_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "TIM2 internal trigger 1 remapping"]
        #[inline(always)]
        pub const fn tim2itr1_iremap(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 internal trigger 1 remapping"]
        #[inline(always)]
        pub fn set_tim2itr1_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub const fn ptp_pps_remap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub fn set_ptp_pps_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Mapr {
        #[inline(always)]
        fn default() -> Mapr {
            Mapr(0)
        }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr2(pub u32);
    impl Mapr2 {
        #[doc = "TIM15 remapping"]
        #[inline(always)]
        pub const fn tim15_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 remapping"]
        #[inline(always)]
        pub fn set_tim15_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM16 remapping"]
        #[inline(always)]
        pub const fn tim16_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 remapping"]
        #[inline(always)]
        pub fn set_tim16_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM17 remapping"]
        #[inline(always)]
        pub const fn tim17_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 remapping"]
        #[inline(always)]
        pub fn set_tim17_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CEC remapping"]
        #[inline(always)]
        pub const fn cec_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CEC remapping"]
        #[inline(always)]
        pub fn set_cec_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM1 DMA remapping"]
        #[inline(always)]
        pub const fn tim1_dma_remap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 DMA remapping"]
        #[inline(always)]
        pub fn set_tim1_dma_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM9 remapping"]
        #[inline(always)]
        pub const fn tim9_remap(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 remapping"]
        #[inline(always)]
        pub fn set_tim9_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM10 remapping"]
        #[inline(always)]
        pub const fn tim10_remap(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 remapping"]
        #[inline(always)]
        pub fn set_tim10_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM11 remapping"]
        #[inline(always)]
        pub const fn tim11_remap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 remapping"]
        #[inline(always)]
        pub fn set_tim11_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM13 remapping"]
        #[inline(always)]
        pub const fn tim13_remap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 remapping"]
        #[inline(always)]
        pub fn set_tim13_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM14 remapping"]
        #[inline(always)]
        pub const fn tim14_remap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 remapping"]
        #[inline(always)]
        pub fn set_tim14_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "NADV connect/disconnect"]
        #[inline(always)]
        pub const fn fsmc_nadv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "NADV connect/disconnect"]
        #[inline(always)]
        pub fn set_fsmc_nadv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM67_DAC DMA remapping"]
        #[inline(always)]
        pub const fn tim67_dac_dma_remap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM67_DAC DMA remapping"]
        #[inline(always)]
        pub fn set_tim67_dac_dma_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM12 remapping"]
        #[inline(always)]
        pub const fn tim12_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 remapping"]
        #[inline(always)]
        pub fn set_tim12_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Miscellaneous features remapping"]
        #[inline(always)]
        pub const fn misc_remap(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Miscellaneous features remapping"]
        #[inline(always)]
        pub fn set_misc_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Mapr2 {
        #[inline(always)]
        fn default() -> Mapr2 {
            Mapr2(0)
        }
    }
}
