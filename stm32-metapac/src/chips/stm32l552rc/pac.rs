#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_PVM"]
    PVD_PVM = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - RTC_S"]
    RTC_S = 3,
    #[doc = "4 - TAMP"]
    TAMP = 4,
    #[doc = "5 - TAMP_S"]
    TAMP_S = 5,
    #[doc = "6 - FLASH"]
    FLASH = 6,
    #[doc = "7 - FLASH_S"]
    FLASH_S = 7,
    #[doc = "8 - GTZC"]
    GTZC = 8,
    #[doc = "9 - RCC"]
    RCC = 9,
    #[doc = "10 - RCC_S"]
    RCC_S = 10,
    #[doc = "11 - EXTI0"]
    EXTI0 = 11,
    #[doc = "12 - EXTI1"]
    EXTI1 = 12,
    #[doc = "13 - EXTI2"]
    EXTI2 = 13,
    #[doc = "14 - EXTI3"]
    EXTI3 = 14,
    #[doc = "15 - EXTI4"]
    EXTI4 = 15,
    #[doc = "16 - EXTI5"]
    EXTI5 = 16,
    #[doc = "17 - EXTI6"]
    EXTI6 = 17,
    #[doc = "18 - EXTI7"]
    EXTI7 = 18,
    #[doc = "19 - EXTI8"]
    EXTI8 = 19,
    #[doc = "20 - EXTI9"]
    EXTI9 = 20,
    #[doc = "21 - EXTI10"]
    EXTI10 = 21,
    #[doc = "22 - EXTI11"]
    EXTI11 = 22,
    #[doc = "23 - EXTI12"]
    EXTI12 = 23,
    #[doc = "24 - EXTI13"]
    EXTI13 = 24,
    #[doc = "25 - EXTI14"]
    EXTI14 = 25,
    #[doc = "26 - EXTI15"]
    EXTI15 = 26,
    #[doc = "27 - DMAMUX1"]
    DMAMUX1 = 27,
    #[doc = "28 - DMAMUX1_S"]
    DMAMUX1_S = 28,
    #[doc = "29 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 29,
    #[doc = "30 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 30,
    #[doc = "31 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 31,
    #[doc = "32 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 32,
    #[doc = "33 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 33,
    #[doc = "34 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 34,
    #[doc = "35 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 35,
    #[doc = "36 - DMA1_CHANNEL8"]
    DMA1_CHANNEL8 = 36,
    #[doc = "37 - ADC1_2"]
    ADC1_2 = 37,
    #[doc = "38 - DAC"]
    DAC = 38,
    #[doc = "39 - FDCAN1_IT0"]
    FDCAN1_IT0 = 39,
    #[doc = "40 - FDCAN1_IT1"]
    FDCAN1_IT1 = 40,
    #[doc = "41 - TIM1_BRK"]
    TIM1_BRK = 41,
    #[doc = "42 - TIM1_UP"]
    TIM1_UP = 42,
    #[doc = "43 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 43,
    #[doc = "44 - TIM1_CC"]
    TIM1_CC = 44,
    #[doc = "45 - TIM2"]
    TIM2 = 45,
    #[doc = "46 - TIM3"]
    TIM3 = 46,
    #[doc = "47 - TIM4"]
    TIM4 = 47,
    #[doc = "48 - TIM5"]
    TIM5 = 48,
    #[doc = "49 - TIM6"]
    TIM6 = 49,
    #[doc = "50 - TIM7"]
    TIM7 = 50,
    #[doc = "51 - TIM8_BRK"]
    TIM8_BRK = 51,
    #[doc = "52 - TIM8_UP"]
    TIM8_UP = 52,
    #[doc = "53 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 53,
    #[doc = "54 - TIM8_CC"]
    TIM8_CC = 54,
    #[doc = "55 - I2C1_EV"]
    I2C1_EV = 55,
    #[doc = "56 - I2C1_ER"]
    I2C1_ER = 56,
    #[doc = "57 - I2C2_EV"]
    I2C2_EV = 57,
    #[doc = "58 - I2C2_ER"]
    I2C2_ER = 58,
    #[doc = "59 - SPI1"]
    SPI1 = 59,
    #[doc = "60 - SPI2"]
    SPI2 = 60,
    #[doc = "61 - USART1"]
    USART1 = 61,
    #[doc = "62 - USART2"]
    USART2 = 62,
    #[doc = "63 - USART3"]
    USART3 = 63,
    #[doc = "64 - UART4"]
    UART4 = 64,
    #[doc = "65 - UART5"]
    UART5 = 65,
    #[doc = "66 - LPUART1"]
    LPUART1 = 66,
    #[doc = "67 - LPTIM1"]
    LPTIM1 = 67,
    #[doc = "68 - LPTIM2"]
    LPTIM2 = 68,
    #[doc = "69 - TIM15"]
    TIM15 = 69,
    #[doc = "70 - TIM16"]
    TIM16 = 70,
    #[doc = "71 - TIM17"]
    TIM17 = 71,
    #[doc = "72 - COMP"]
    COMP = 72,
    #[doc = "73 - USB_FS"]
    USB_FS = 73,
    #[doc = "74 - CRS"]
    CRS = 74,
    #[doc = "75 - FMC"]
    FMC = 75,
    #[doc = "76 - OCTOSPI1"]
    OCTOSPI1 = 76,
    #[doc = "78 - SDMMC1"]
    SDMMC1 = 78,
    #[doc = "80 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 80,
    #[doc = "81 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 81,
    #[doc = "82 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 82,
    #[doc = "83 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 83,
    #[doc = "84 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 84,
    #[doc = "85 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 85,
    #[doc = "86 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 86,
    #[doc = "87 - DMA2_CHANNEL8"]
    DMA2_CHANNEL8 = 87,
    #[doc = "88 - I2C3_EV"]
    I2C3_EV = 88,
    #[doc = "89 - I2C3_ER"]
    I2C3_ER = 89,
    #[doc = "90 - SAI1"]
    SAI1 = 90,
    #[doc = "91 - SAI2"]
    SAI2 = 91,
    #[doc = "92 - TSC"]
    TSC = 92,
    #[doc = "94 - RNG"]
    RNG = 94,
    #[doc = "95 - FPU"]
    FPU = 95,
    #[doc = "96 - HASH"]
    HASH = 96,
    #[doc = "98 - LPTIM3"]
    LPTIM3 = 98,
    #[doc = "99 - SPI3"]
    SPI3 = 99,
    #[doc = "100 - I2C4_ER"]
    I2C4_ER = 100,
    #[doc = "101 - I2C4_EV"]
    I2C4_EV = 101,
    #[doc = "102 - DFSDM1_FLT0"]
    DFSDM1_FLT0 = 102,
    #[doc = "103 - DFSDM1_FLT1"]
    DFSDM1_FLT1 = 103,
    #[doc = "104 - DFSDM1_FLT2"]
    DFSDM1_FLT2 = 104,
    #[doc = "105 - DFSDM1_FLT3"]
    DFSDM1_FLT3 = 105,
    #[doc = "106 - UCPD1"]
    UCPD1 = 106,
    #[doc = "107 - ICACHE"]
    ICACHE = 107,
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
        fn PVD_PVM();
        fn RTC();
        fn RTC_S();
        fn TAMP();
        fn TAMP_S();
        fn FLASH();
        fn FLASH_S();
        fn GTZC();
        fn RCC();
        fn RCC_S();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn EXTI5();
        fn EXTI6();
        fn EXTI7();
        fn EXTI8();
        fn EXTI9();
        fn EXTI10();
        fn EXTI11();
        fn EXTI12();
        fn EXTI13();
        fn EXTI14();
        fn EXTI15();
        fn DMAMUX1();
        fn DMAMUX1_S();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn DMA1_CHANNEL8();
        fn ADC1_2();
        fn DAC();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn TIM5();
        fn TIM6();
        fn TIM7();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn LPUART1();
        fn LPTIM1();
        fn LPTIM2();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn COMP();
        fn USB_FS();
        fn CRS();
        fn FMC();
        fn OCTOSPI1();
        fn SDMMC1();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn DMA2_CHANNEL5();
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn DMA2_CHANNEL8();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SAI1();
        fn SAI2();
        fn TSC();
        fn RNG();
        fn FPU();
        fn HASH();
        fn LPTIM3();
        fn SPI3();
        fn I2C4_ER();
        fn I2C4_EV();
        fn DFSDM1_FLT0();
        fn DFSDM1_FLT1();
        fn DFSDM1_FLT2();
        fn DFSDM1_FLT3();
        fn UCPD1();
        fn ICACHE();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 108] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_PVM },
        Vector { _handler: RTC },
        Vector { _handler: RTC_S },
        Vector { _handler: TAMP },
        Vector { _handler: TAMP_S },
        Vector { _handler: FLASH },
        Vector { _handler: FLASH_S },
        Vector { _handler: GTZC },
        Vector { _handler: RCC },
        Vector { _handler: RCC_S },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector { _handler: EXTI5 },
        Vector { _handler: EXTI6 },
        Vector { _handler: EXTI7 },
        Vector { _handler: EXTI8 },
        Vector { _handler: EXTI9 },
        Vector { _handler: EXTI10 },
        Vector { _handler: EXTI11 },
        Vector { _handler: EXTI12 },
        Vector { _handler: EXTI13 },
        Vector { _handler: EXTI14 },
        Vector { _handler: EXTI15 },
        Vector { _handler: DMAMUX1 },
        Vector { _handler: DMAMUX1_S },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector {
            _handler: DMA1_CHANNEL8,
        },
        Vector { _handler: ADC1_2 },
        Vector { _handler: DAC },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: TIM5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: COMP },
        Vector { _handler: USB_FS },
        Vector { _handler: CRS },
        Vector { _handler: FMC },
        Vector { _handler: OCTOSPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: SDMMC1 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA2_CHANNEL1,
        },
        Vector {
            _handler: DMA2_CHANNEL2,
        },
        Vector {
            _handler: DMA2_CHANNEL3,
        },
        Vector {
            _handler: DMA2_CHANNEL4,
        },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector {
            _handler: DMA2_CHANNEL8,
        },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SAI1 },
        Vector { _handler: SAI2 },
        Vector { _handler: TSC },
        Vector { _reserved: 0 },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: HASH },
        Vector { _reserved: 0 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: SPI3 },
        Vector { _handler: I2C4_ER },
        Vector { _handler: I2C4_EV },
        Vector { _handler: DFSDM1_FLT0 },
        Vector { _handler: DFSDM1_FLT1 },
        Vector { _handler: DFSDM1_FLT2 },
        Vector { _handler: DFSDM1_FLT3 },
        Vector { _handler: UCPD1 },
        Vector { _handler: ICACHE },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x0bfa_0590 as usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000 as usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400 as usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800 as usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00 as usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000 as usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400 as usize as _) };
pub const RTC: *mut () = 0x4000_2800 as usize as _;
pub const WWDG: *mut () = 0x4000_2c00 as usize as _;
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000 as usize as _) };
pub const TAMP: *mut () = 0x4000_3400 as usize as _;
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800 as usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00 as usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400 as usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800 as usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00 as usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000 as usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400 as usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800 as usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00 as usize as _) };
pub const CRS: *mut () = 0x4000_6000 as usize as _;
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000 as usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400 as usize as _) };
pub const OPAMP1: *mut () = 0x4000_7800 as usize as _;
pub const OPAMP2: *mut () = 0x4000_7810 as usize as _;
pub const LPTIM1: *mut () = 0x4000_7c00 as usize as _;
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_8000 as usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_8400 as usize as _) };
pub const LPTIM2: *mut () = 0x4000_9400 as usize as _;
pub const LPTIM3: *mut () = 0x4000_9800 as usize as _;
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_a400 as usize as _) };
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4000_d400 as usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4000_d800 as usize as _) };
pub const UCPD1: *mut () = 0x4000_dc00 as usize as _;
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000 as usize as _) };
pub const VREFBUF: *mut () = 0x4001_0100 as usize as _;
pub const COMP1: *mut () = 0x4001_0200 as usize as _;
pub const COMP2: *mut () = 0x4001_0204 as usize as _;
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00 as usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000 as usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400 as usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800 as usize as _) };
pub const TIM15: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4000 as usize as _) };
pub const TIM16: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4400 as usize as _) };
pub const TIM17: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4001_4800 as usize as _) };
pub const SAI1: *mut () = 0x4001_5400 as usize as _;
pub const SAI2: *mut () = 0x4001_5800 as usize as _;
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000 as usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400 as usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800 as usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000 as usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000 as usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000 as usize as _) };
pub const TSC: *mut () = 0x4002_4000 as usize as _;
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4002_f400 as usize as _) };
pub const ICACHE: *mut () = 0x4003_0400 as usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000 as usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400 as usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800 as usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0c00 as usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1000 as usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1400 as usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1800 as usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00 as usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4202_8000 as usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4202_8100 as usize as _) };
pub const ADC_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4202_8300 as usize as _) };
pub const HASH: *mut () = 0x420c_0400 as usize as _;
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x420c_0800 as usize as _) };
pub const SDMMC1: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x420c_8000 as usize as _) };
pub const OCTOSPI1: *mut () = 0x9000_0000 as usize as _;
pub const DBGMCU: *mut () = 0xe004_4000 as usize as _;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1107427328 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_v3.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/can_fdcan.rs"]
pub mod can;
#[path = "../../peripherals/crc_v2.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v2.rs"]
pub mod dac;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/exti_l5.rs"]
pub mod exti;
#[path = "../../peripherals/flash_l5.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_l5.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_l5.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v2.rs"]
pub mod rng;
#[path = "../../peripherals/sdmmc_v2.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v2.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_l5.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v3.rs"]
pub mod usb;
#[path = "../../peripherals/usbram_16x2_1024.rs"]
pub mod usbram;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 262144;
pub const WRITE_SIZE: usize = 8;
