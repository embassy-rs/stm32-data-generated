#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD"]
    PVD = 1,
    #[doc = "2 - TAMP_STAMP"]
    TAMP_STAMP = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_STREAM0"]
    DMA1_STREAM0 = 11,
    #[doc = "12 - DMA1_STREAM1"]
    DMA1_STREAM1 = 12,
    #[doc = "13 - DMA1_STREAM2"]
    DMA1_STREAM2 = 13,
    #[doc = "14 - DMA1_STREAM3"]
    DMA1_STREAM3 = 14,
    #[doc = "15 - DMA1_STREAM4"]
    DMA1_STREAM4 = 15,
    #[doc = "16 - DMA1_STREAM5"]
    DMA1_STREAM5 = 16,
    #[doc = "17 - DMA1_STREAM6"]
    DMA1_STREAM6 = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
    #[doc = "19 - CAN1_TX"]
    CAN1_TX = 19,
    #[doc = "20 - CAN1_RX0"]
    CAN1_RX0 = 20,
    #[doc = "21 - CAN1_RX1"]
    CAN1_RX1 = 21,
    #[doc = "22 - CAN1_SCE"]
    CAN1_SCE = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK_TIM9"]
    TIM1_BRK_TIM9 = 24,
    #[doc = "25 - TIM1_UP_TIM10"]
    TIM1_UP_TIM10 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM11"]
    TIM1_TRG_COM_TIM11 = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - OTG_FS_WKUP"]
    OTG_FS_WKUP = 42,
    #[doc = "43 - TIM8_BRK_TIM12"]
    TIM8_BRK_TIM12 = 43,
    #[doc = "44 - TIM8_UP_TIM13"]
    TIM8_UP_TIM13 = 44,
    #[doc = "45 - TIM8_TRG_COM_TIM14"]
    TIM8_TRG_COM_TIM14 = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1_STREAM7"]
    DMA1_STREAM7 = 47,
    #[doc = "49 - SDIO"]
    SDIO = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6_DAC"]
    TIM6_DAC = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_STREAM0"]
    DMA2_STREAM0 = 56,
    #[doc = "57 - DMA2_STREAM1"]
    DMA2_STREAM1 = 57,
    #[doc = "58 - DMA2_STREAM2"]
    DMA2_STREAM2 = 58,
    #[doc = "59 - DMA2_STREAM3"]
    DMA2_STREAM3 = 59,
    #[doc = "60 - DMA2_STREAM4"]
    DMA2_STREAM4 = 60,
    #[doc = "61 - DFSDM1_FLT0"]
    DFSDM1_FLT0 = 61,
    #[doc = "62 - DFSDM1_FLT1"]
    DFSDM1_FLT1 = 62,
    #[doc = "63 - CAN2_TX"]
    CAN2_TX = 63,
    #[doc = "64 - CAN2_RX0"]
    CAN2_RX0 = 64,
    #[doc = "65 - CAN2_RX1"]
    CAN2_RX1 = 65,
    #[doc = "66 - CAN2_SCE"]
    CAN2_SCE = 66,
    #[doc = "67 - OTG_FS"]
    OTG_FS = 67,
    #[doc = "68 - DMA2_STREAM5"]
    DMA2_STREAM5 = 68,
    #[doc = "69 - DMA2_STREAM6"]
    DMA2_STREAM6 = 69,
    #[doc = "70 - DMA2_STREAM7"]
    DMA2_STREAM7 = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "74 - CAN3_TX"]
    CAN3_TX = 74,
    #[doc = "75 - CAN3_RX0"]
    CAN3_RX0 = 75,
    #[doc = "76 - CAN3_RX1"]
    CAN3_RX1 = 76,
    #[doc = "77 - CAN3_SCE"]
    CAN3_SCE = 77,
    #[doc = "79 - AES"]
    AES = 79,
    #[doc = "80 - RNG"]
    RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "82 - UART7"]
    UART7 = 82,
    #[doc = "83 - UART8"]
    UART8 = 83,
    #[doc = "84 - SPI4"]
    SPI4 = 84,
    #[doc = "85 - SPI5"]
    SPI5 = 85,
    #[doc = "87 - SAI1"]
    SAI1 = 87,
    #[doc = "88 - UART9"]
    UART9 = 88,
    #[doc = "89 - UART10"]
    UART10 = 89,
    #[doc = "92 - QUADSPI"]
    QUADSPI = 92,
    #[doc = "95 - FMPI2C1_EV"]
    FMPI2C1_EV = 95,
    #[doc = "96 - FMPI2C1_ER"]
    FMPI2C1_ER = 96,
    #[doc = "97 - LPTIM1"]
    LPTIM1 = 97,
    #[doc = "98 - DFSDM2_FLT0"]
    DFSDM2_FLT0 = 98,
    #[doc = "99 - DFSDM2_FLT1"]
    DFSDM2_FLT1 = 99,
    #[doc = "100 - DFSDM2_FLT2"]
    DFSDM2_FLT2 = 100,
    #[doc = "101 - DFSDM2_FLT3"]
    DFSDM2_FLT3 = 101,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn WWDG();
        fn PVD();
        fn TAMP_STAMP();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_STREAM0();
        fn DMA1_STREAM1();
        fn DMA1_STREAM2();
        fn DMA1_STREAM3();
        fn DMA1_STREAM4();
        fn DMA1_STREAM5();
        fn DMA1_STREAM6();
        fn ADC();
        fn CAN1_TX();
        fn CAN1_RX0();
        fn CAN1_RX1();
        fn CAN1_SCE();
        fn EXTI9_5();
        fn TIM1_BRK_TIM9();
        fn TIM1_UP_TIM10();
        fn TIM1_TRG_COM_TIM11();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI15_10();
        fn RTC_ALARM();
        fn OTG_FS_WKUP();
        fn TIM8_BRK_TIM12();
        fn TIM8_UP_TIM13();
        fn TIM8_TRG_COM_TIM14();
        fn TIM8_CC();
        fn DMA1_STREAM7();
        fn SDIO();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC();
        fn TIM7();
        fn DMA2_STREAM0();
        fn DMA2_STREAM1();
        fn DMA2_STREAM2();
        fn DMA2_STREAM3();
        fn DMA2_STREAM4();
        fn DFSDM1_FLT0();
        fn DFSDM1_FLT1();
        fn CAN2_TX();
        fn CAN2_RX0();
        fn CAN2_RX1();
        fn CAN2_SCE();
        fn OTG_FS();
        fn DMA2_STREAM5();
        fn DMA2_STREAM6();
        fn DMA2_STREAM7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn CAN3_TX();
        fn CAN3_RX0();
        fn CAN3_RX1();
        fn CAN3_SCE();
        fn AES();
        fn RNG();
        fn FPU();
        fn UART7();
        fn UART8();
        fn SPI4();
        fn SPI5();
        fn SAI1();
        fn UART9();
        fn UART10();
        fn QUADSPI();
        fn FMPI2C1_EV();
        fn FMPI2C1_ER();
        fn LPTIM1();
        fn DFSDM2_FLT0();
        fn DFSDM2_FLT1();
        fn DFSDM2_FLT2();
        fn DFSDM2_FLT3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 102] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
        Vector { _handler: TAMP_STAMP },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: DMA1_STREAM0 },
        Vector { _handler: DMA1_STREAM1 },
        Vector { _handler: DMA1_STREAM2 },
        Vector { _handler: DMA1_STREAM3 },
        Vector { _handler: DMA1_STREAM4 },
        Vector { _handler: DMA1_STREAM5 },
        Vector { _handler: DMA1_STREAM6 },
        Vector { _handler: ADC },
        Vector { _handler: CAN1_TX },
        Vector { _handler: CAN1_RX0 },
        Vector { _handler: CAN1_RX1 },
        Vector { _handler: CAN1_SCE },
        Vector { _handler: EXTI9_5 },
        Vector {
            _handler: TIM1_BRK_TIM9,
        },
        Vector {
            _handler: TIM1_UP_TIM10,
        },
        Vector {
            _handler: TIM1_TRG_COM_TIM11,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: EXTI15_10 },
        Vector { _handler: RTC_ALARM },
        Vector { _handler: OTG_FS_WKUP },
        Vector {
            _handler: TIM8_BRK_TIM12,
        },
        Vector {
            _handler: TIM8_UP_TIM13,
        },
        Vector {
            _handler: TIM8_TRG_COM_TIM14,
        },
        Vector { _handler: TIM8_CC },
        Vector { _handler: DMA1_STREAM7 },
        Vector { _reserved: 0 },
        Vector { _handler: SDIO },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC },
        Vector { _handler: TIM7 },
        Vector { _handler: DMA2_STREAM0 },
        Vector { _handler: DMA2_STREAM1 },
        Vector { _handler: DMA2_STREAM2 },
        Vector { _handler: DMA2_STREAM3 },
        Vector { _handler: DMA2_STREAM4 },
        Vector { _handler: DFSDM1_FLT0 },
        Vector { _handler: DFSDM1_FLT1 },
        Vector { _handler: CAN2_TX },
        Vector { _handler: CAN2_RX0 },
        Vector { _handler: CAN2_RX1 },
        Vector { _handler: CAN2_SCE },
        Vector { _handler: OTG_FS },
        Vector { _handler: DMA2_STREAM5 },
        Vector { _handler: DMA2_STREAM6 },
        Vector { _handler: DMA2_STREAM7 },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: CAN3_TX },
        Vector { _handler: CAN3_RX0 },
        Vector { _handler: CAN3_RX1 },
        Vector { _handler: CAN3_SCE },
        Vector { _reserved: 0 },
        Vector { _handler: AES },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _reserved: 0 },
        Vector { _handler: SAI1 },
        Vector { _handler: UART9 },
        Vector { _handler: UART10 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: QUADSPI },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FMPI2C1_EV },
        Vector { _handler: FMPI2C1_ER },
        Vector { _handler: LPTIM1 },
        Vector { _handler: DFSDM2_FLT0 },
        Vector { _handler: DFSDM2_FLT1 },
        Vector { _handler: DFSDM2_FLT2 },
        Vector { _handler: DFSDM2_FLT3 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7a10usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_2400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const FMPI2C1: fmpi2c::Fmpi2c = unsafe { fmpi2c::Fmpi2c::from_ptr(0x4000_6000usize as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_6400usize as _) };
pub const CAN2: can::Can = unsafe { can::Can::from_ptr(0x4000_6800usize as _) };
pub const CAN3: can::Can = unsafe { can::Can::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const UART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_7800usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1000usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2000usize as _) };
pub const ADC1_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4001_2300usize as _) };
pub const SDIO: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3400usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_3800usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_3c00usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_4000usize as _) };
pub const TIM10: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4400usize as _) };
pub const TIM11: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4800usize as _) };
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_5000usize as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5800usize as _) };
pub const DFSDM1: *mut () = 0x4001_6000usize as _;
pub const DFSDM2: *mut () = 0x4001_6400usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1c00usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_3800usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_3c00usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6400usize as _) };
pub const USB_OTG_FS: otg::Otg = unsafe { otg::Otg::from_ptr(0x5000_0000usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x5006_0000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x5006_0800usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_v2.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v2.rs"]
pub mod adccommon;
#[path = "../../peripherals/aes_v1.rs"]
pub mod aes;
#[path = "../../peripherals/can_bxcan.rs"]
pub mod can;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v2.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_f4.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dma_v2.rs"]
pub mod dma;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/flash_f4.rs"]
pub mod flash;
#[path = "../../peripherals/fmpi2c_v2.rs"]
pub mod fmpi2c;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v1.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v1a.rs"]
pub mod lptim;
#[path = "../../peripherals/otg_v1.rs"]
pub mod otg;
#[path = "../../peripherals/pwr_f4.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f4.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v1.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v2f4.rs"]
pub mod rtc;
#[path = "../../peripherals/sai_v2.rs"]
pub mod sai;
#[path = "../../peripherals/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v1.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v2.rs"]
pub mod usart;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
