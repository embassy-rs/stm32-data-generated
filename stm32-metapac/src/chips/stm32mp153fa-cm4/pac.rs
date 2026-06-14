#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG1"]
    WWDG1 = 0,
    #[doc = "1 - PVD_AVD"]
    PVD_AVD = 1,
    #[doc = "2 - TAMP"]
    TAMP = 2,
    #[doc = "3 - RTC_WKUP_ALARM"]
    RTC_WKUP_ALARM = 3,
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
    #[doc = "18 - ADC1"]
    ADC1 = 18,
    #[doc = "19 - FDCAN1_IT0"]
    FDCAN1_IT0 = 19,
    #[doc = "20 - FDCAN2_IT0"]
    FDCAN2_IT0 = 20,
    #[doc = "21 - FDCAN1_IT1"]
    FDCAN1_IT1 = 21,
    #[doc = "22 - FDCAN2_IT1"]
    FDCAN2_IT1 = 22,
    #[doc = "23 - EXTI5"]
    EXTI5 = 23,
    #[doc = "24 - TIM1_BRK"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1_UP"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 26,
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
    #[doc = "40 - EXTI10"]
    EXTI10 = 40,
    #[doc = "41 - RTC_TIMESTAMP"]
    RTC_TIMESTAMP = 41,
    #[doc = "42 - EXTI11"]
    EXTI11 = 42,
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1_STREAM7"]
    DMA1_STREAM7 = 47,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDMMC1"]
    SDMMC1 = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6"]
    TIM6 = 54,
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
    #[doc = "61 - ETH1"]
    ETH1 = 61,
    #[doc = "62 - ETH1_WKUP"]
    ETH1_WKUP = 62,
    #[doc = "63 - FDCAN_CAL"]
    FDCAN_CAL = 63,
    #[doc = "64 - EXTI6"]
    EXTI6 = 64,
    #[doc = "65 - EXTI7"]
    EXTI7 = 65,
    #[doc = "66 - EXTI8"]
    EXTI8 = 66,
    #[doc = "67 - EXTI9"]
    EXTI9 = 67,
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
    #[doc = "74 - USBH_OHCI"]
    USBH_OHCI = 74,
    #[doc = "75 - USBH_EHCI"]
    USBH_EHCI = 75,
    #[doc = "76 - EXTI12"]
    EXTI12 = 76,
    #[doc = "77 - EXTI13"]
    EXTI13 = 77,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "79 - CRYP1"]
    CRYP1 = 79,
    #[doc = "80 - HASH1"]
    HASH1 = 80,
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
    #[doc = "86 - SPI6"]
    SPI6 = 86,
    #[doc = "87 - SAI1"]
    SAI1 = 87,
    #[doc = "88 - LTDC"]
    LTDC = 88,
    #[doc = "89 - LTDC_ER"]
    LTDC_ER = 89,
    #[doc = "90 - ADC2"]
    ADC2 = 90,
    #[doc = "91 - SAI2"]
    SAI2 = 91,
    #[doc = "92 - QUADSPI"]
    QUADSPI = 92,
    #[doc = "93 - LPTIM1"]
    LPTIM1 = 93,
    #[doc = "94 - CEC"]
    CEC = 94,
    #[doc = "95 - I2C4_EV"]
    I2C4_EV = 95,
    #[doc = "96 - I2C4_ER"]
    I2C4_ER = 96,
    #[doc = "97 - SPDIF_RX"]
    SPDIF_RX = 97,
    #[doc = "98 - OTG"]
    OTG = 98,
    #[doc = "100 - IPCC_RX0"]
    IPCC_RX0 = 100,
    #[doc = "101 - IPCC_TX0"]
    IPCC_TX0 = 101,
    #[doc = "102 - DMAMUX1_OVR"]
    DMAMUX1_OVR = 102,
    #[doc = "103 - IPCC_RX1"]
    IPCC_RX1 = 103,
    #[doc = "104 - IPCC_TX1"]
    IPCC_TX1 = 104,
    #[doc = "105 - CRYP2"]
    CRYP2 = 105,
    #[doc = "106 - HASH2"]
    HASH2 = 106,
    #[doc = "107 - I2C5_EV"]
    I2C5_EV = 107,
    #[doc = "108 - I2C5_ER"]
    I2C5_ER = 108,
    #[doc = "110 - DFSDM1_FLT0"]
    DFSDM1_FLT0 = 110,
    #[doc = "111 - DFSDM1_FLT1"]
    DFSDM1_FLT1 = 111,
    #[doc = "112 - DFSDM1_FLT2"]
    DFSDM1_FLT2 = 112,
    #[doc = "113 - DFSDM1_FLT3"]
    DFSDM1_FLT3 = 113,
    #[doc = "114 - SAI3"]
    SAI3 = 114,
    #[doc = "115 - DFSDM1_FLT4"]
    DFSDM1_FLT4 = 115,
    #[doc = "116 - TIM15"]
    TIM15 = 116,
    #[doc = "117 - TIM16"]
    TIM16 = 117,
    #[doc = "118 - TIM17"]
    TIM17 = 118,
    #[doc = "119 - TIM12"]
    TIM12 = 119,
    #[doc = "120 - MDIOS"]
    MDIOS = 120,
    #[doc = "121 - EXTI14"]
    EXTI14 = 121,
    #[doc = "122 - MDMA"]
    MDMA = 122,
    #[doc = "124 - SDMMC2"]
    SDMMC2 = 124,
    #[doc = "125 - HSEM_IT2"]
    HSEM_IT2 = 125,
    #[doc = "126 - DFSDM1_FLT5"]
    DFSDM1_FLT5 = 126,
    #[doc = "127 - EXTI15"]
    EXTI15 = 127,
    #[doc = "128 - NCTIIRQ1"]
    NCTIIRQ1 = 128,
    #[doc = "129 - NCTIIRQ2"]
    NCTIIRQ2 = 129,
    #[doc = "130 - TIM13"]
    TIM13 = 130,
    #[doc = "131 - TIM14"]
    TIM14 = 131,
    #[doc = "132 - DAC"]
    DAC = 132,
    #[doc = "133 - RNG1"]
    RNG1 = 133,
    #[doc = "134 - RNG2"]
    RNG2 = 134,
    #[doc = "135 - I2C6_EV"]
    I2C6_EV = 135,
    #[doc = "136 - I2C6_ER"]
    I2C6_ER = 136,
    #[doc = "137 - SDMMC3"]
    SDMMC3 = 137,
    #[doc = "138 - LPTIM2"]
    LPTIM2 = 138,
    #[doc = "139 - LPTIM3"]
    LPTIM3 = 139,
    #[doc = "140 - LPTIM4"]
    LPTIM4 = 140,
    #[doc = "141 - LPTIM5"]
    LPTIM5 = 141,
    #[doc = "142 - ETH1_LPI"]
    ETH1_LPI = 142,
    #[doc = "144 - MPU_SEV"]
    MPU_SEV = 144,
    #[doc = "145 - RCC_WAKEUP"]
    RCC_WAKEUP = 145,
    #[doc = "146 - SAI4"]
    SAI4 = 146,
    #[doc = "147 - DTS"]
    DTS = 147,
    #[doc = "149 - WAKEUP_PIN"]
    WAKEUP_PIN = 149,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG1();
        fn PVD_AVD();
        fn TAMP();
        fn RTC_WKUP_ALARM();
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
        fn ADC1();
        fn FDCAN1_IT0();
        fn FDCAN2_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT1();
        fn EXTI5();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
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
        fn EXTI10();
        fn RTC_TIMESTAMP();
        fn EXTI11();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn DMA1_STREAM7();
        fn FMC();
        fn SDMMC1();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6();
        fn TIM7();
        fn DMA2_STREAM0();
        fn DMA2_STREAM1();
        fn DMA2_STREAM2();
        fn DMA2_STREAM3();
        fn DMA2_STREAM4();
        fn ETH1();
        fn ETH1_WKUP();
        fn FDCAN_CAL();
        fn EXTI6();
        fn EXTI7();
        fn EXTI8();
        fn EXTI9();
        fn DMA2_STREAM5();
        fn DMA2_STREAM6();
        fn DMA2_STREAM7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn USBH_OHCI();
        fn USBH_EHCI();
        fn EXTI12();
        fn EXTI13();
        fn DCMI();
        fn CRYP1();
        fn HASH1();
        fn FPU();
        fn UART7();
        fn UART8();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn SAI1();
        fn LTDC();
        fn LTDC_ER();
        fn ADC2();
        fn SAI2();
        fn QUADSPI();
        fn LPTIM1();
        fn CEC();
        fn I2C4_EV();
        fn I2C4_ER();
        fn SPDIF_RX();
        fn OTG();
        fn IPCC_RX0();
        fn IPCC_TX0();
        fn DMAMUX1_OVR();
        fn IPCC_RX1();
        fn IPCC_TX1();
        fn CRYP2();
        fn HASH2();
        fn I2C5_EV();
        fn I2C5_ER();
        fn DFSDM1_FLT0();
        fn DFSDM1_FLT1();
        fn DFSDM1_FLT2();
        fn DFSDM1_FLT3();
        fn SAI3();
        fn DFSDM1_FLT4();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn TIM12();
        fn MDIOS();
        fn EXTI14();
        fn MDMA();
        fn SDMMC2();
        fn HSEM_IT2();
        fn DFSDM1_FLT5();
        fn EXTI15();
        fn NCTIIRQ1();
        fn NCTIIRQ2();
        fn TIM13();
        fn TIM14();
        fn DAC();
        fn RNG1();
        fn RNG2();
        fn I2C6_EV();
        fn I2C6_ER();
        fn SDMMC3();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn ETH1_LPI();
        fn MPU_SEV();
        fn RCC_WAKEUP();
        fn SAI4();
        fn DTS();
        fn WAKEUP_PIN();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 150] = [
        Vector { _handler: WWDG1 },
        Vector { _handler: PVD_AVD },
        Vector { _handler: TAMP },
        Vector {
            _handler: RTC_WKUP_ALARM,
        },
        Vector { _reserved: 0 },
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
        Vector { _handler: ADC1 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: EXTI5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
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
        Vector { _handler: EXTI10 },
        Vector {
            _handler: RTC_TIMESTAMP,
        },
        Vector { _handler: EXTI11 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: DMA1_STREAM7 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: DMA2_STREAM0 },
        Vector { _handler: DMA2_STREAM1 },
        Vector { _handler: DMA2_STREAM2 },
        Vector { _handler: DMA2_STREAM3 },
        Vector { _handler: DMA2_STREAM4 },
        Vector { _handler: ETH1 },
        Vector { _handler: ETH1_WKUP },
        Vector { _handler: FDCAN_CAL },
        Vector { _handler: EXTI6 },
        Vector { _handler: EXTI7 },
        Vector { _handler: EXTI8 },
        Vector { _handler: EXTI9 },
        Vector { _handler: DMA2_STREAM5 },
        Vector { _handler: DMA2_STREAM6 },
        Vector { _handler: DMA2_STREAM7 },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: USBH_OHCI },
        Vector { _handler: USBH_EHCI },
        Vector { _handler: EXTI12 },
        Vector { _handler: EXTI13 },
        Vector { _handler: DCMI },
        Vector { _handler: CRYP1 },
        Vector { _handler: HASH1 },
        Vector { _handler: FPU },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: SAI1 },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
        Vector { _handler: ADC2 },
        Vector { _handler: SAI2 },
        Vector { _handler: QUADSPI },
        Vector { _handler: LPTIM1 },
        Vector { _handler: CEC },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: SPDIF_RX },
        Vector { _handler: OTG },
        Vector { _reserved: 0 },
        Vector { _handler: IPCC_RX0 },
        Vector { _handler: IPCC_TX0 },
        Vector { _handler: DMAMUX1_OVR },
        Vector { _handler: IPCC_RX1 },
        Vector { _handler: IPCC_TX1 },
        Vector { _handler: CRYP2 },
        Vector { _handler: HASH2 },
        Vector { _handler: I2C5_EV },
        Vector { _handler: I2C5_ER },
        Vector { _reserved: 0 },
        Vector { _handler: DFSDM1_FLT0 },
        Vector { _handler: DFSDM1_FLT1 },
        Vector { _handler: DFSDM1_FLT2 },
        Vector { _handler: DFSDM1_FLT3 },
        Vector { _handler: SAI3 },
        Vector { _handler: DFSDM1_FLT4 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: TIM12 },
        Vector { _handler: MDIOS },
        Vector { _handler: EXTI14 },
        Vector { _handler: MDMA },
        Vector { _reserved: 0 },
        Vector { _handler: SDMMC2 },
        Vector { _handler: HSEM_IT2 },
        Vector { _handler: DFSDM1_FLT5 },
        Vector { _handler: EXTI15 },
        Vector { _handler: NCTIIRQ1 },
        Vector { _handler: NCTIIRQ2 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: DAC },
        Vector { _handler: RNG1 },
        Vector { _handler: RNG2 },
        Vector { _handler: I2C6_EV },
        Vector { _handler: I2C6_ER },
        Vector { _handler: SDMMC3 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: ETH1_LPI },
        Vector { _reserved: 0 },
        Vector { _handler: MPU_SEV },
        Vector { _handler: RCC_WAKEUP },
        Vector { _handler: SAI4 },
        Vector { _handler: DTS },
        Vector { _reserved: 0 },
        Vector { _handler: WAKEUP_PIN },
    ];
}
pub const TIM2: *mut () = 0x4000_0000usize as _;
pub const TIM3: *mut () = 0x4000_1000usize as _;
pub const TIM4: *mut () = 0x4000_2000usize as _;
pub const TIM5: *mut () = 0x4000_3000usize as _;
pub const TIM6: *mut () = 0x4000_4000usize as _;
pub const TIM7: *mut () = 0x4000_5000usize as _;
pub const TIM12: *mut () = 0x4000_6000usize as _;
pub const TIM13: *mut () = 0x4000_7000usize as _;
pub const TIM14: *mut () = 0x4000_8000usize as _;
pub const LPTIM1: *mut () = 0x4000_9000usize as _;
pub const WWDG1: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_a000usize as _) };
pub const SPI2: *mut () = 0x4000_b000usize as _;
pub const SPI3: *mut () = 0x4000_c000usize as _;
pub const SPDIFRX1: *mut () = 0x4000_d000usize as _;
pub const USART2: *mut () = 0x4000_e000usize as _;
pub const USART3: *mut () = 0x4000_f000usize as _;
pub const UART4: *mut () = 0x4001_0000usize as _;
pub const UART5: *mut () = 0x4001_1000usize as _;
pub const I2C1: *mut () = 0x4001_2000usize as _;
pub const I2C2: *mut () = 0x4001_3000usize as _;
pub const I2C3: *mut () = 0x4001_4000usize as _;
pub const I2C5: *mut () = 0x4001_5000usize as _;
pub const CEC: cec::Cec = unsafe { cec::Cec::from_ptr(0x4001_6000usize as _) };
pub const DAC1: *mut () = 0x4001_7000usize as _;
pub const UART7: *mut () = 0x4001_8000usize as _;
pub const UART8: *mut () = 0x4001_9000usize as _;
pub const TIM1: *mut () = 0x4400_0000usize as _;
pub const TIM8: *mut () = 0x4400_1000usize as _;
pub const USART6: *mut () = 0x4400_3000usize as _;
pub const SPI1: *mut () = 0x4400_4000usize as _;
pub const SPI4: *mut () = 0x4400_5000usize as _;
pub const TIM15: *mut () = 0x4400_6000usize as _;
pub const TIM16: *mut () = 0x4400_7000usize as _;
pub const TIM17: *mut () = 0x4400_8000usize as _;
pub const SPI5: *mut () = 0x4400_9000usize as _;
pub const SAI1: *mut () = 0x4400_a000usize as _;
pub const SAI2: *mut () = 0x4400_b000usize as _;
pub const SAI3: *mut () = 0x4400_c000usize as _;
pub const DFSDM1: *mut () = 0x4400_d000usize as _;
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4400_e000usize as _) };
pub const FDCAN2: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4400_f000usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4401_1000usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4401_1350usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4800_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4800_1000usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4800_2000usize as _) };
pub const ADC1: *mut () = 0x4800_3000usize as _;
pub const ADC2: *mut () = 0x4800_3100usize as _;
pub const ADC12_COMMON: *mut () = 0x4800_3300usize as _;
pub const SDMMC3: *mut () = 0x4800_4000usize as _;
pub const USB_OTG_HS: *mut () = 0x4900_0000usize as _;
pub const HSEM: *mut () = 0x4c00_0000usize as _;
pub const IPCC: *mut () = 0x4c00_1000usize as _;
pub const HASH2: *mut () = 0x4c00_2000usize as _;
pub const RNG2: *mut () = 0x4c00_3000usize as _;
pub const CRC2: *mut () = 0x4c00_4000usize as _;
pub const CRYP2: cryp::Cryp = unsafe { cryp::Cryp::from_ptr(0x4c00_5000usize as _) };
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x4c00_6000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5000_0000usize as _) };
pub const PWR: *mut () = 0x5000_1000usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_2000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_3000usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_4000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_5000usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_6000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_7000usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_8000usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_9000usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_a000usize as _) };
pub const GPIOJ: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_b000usize as _) };
pub const GPIOK: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_c000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5000_d000usize as _) };
pub const SYSCFG: *mut () = 0x5002_0000usize as _;
pub const LPTIM2: *mut () = 0x5002_1000usize as _;
pub const LPTIM3: *mut () = 0x5002_2000usize as _;
pub const LPTIM4: *mut () = 0x5002_3000usize as _;
pub const LPTIM5: *mut () = 0x5002_4000usize as _;
pub const VREFBUF: *mut () = 0x5002_5000usize as _;
pub const SAI4: *mut () = 0x5002_7000usize as _;
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x5002_8000usize as _) };
pub const HDP: *mut () = 0x5002_a000usize as _;
pub const DBGMCU: *mut () = 0x5008_1000usize as _;
pub const CRYP1: cryp::Cryp = unsafe { cryp::Cryp::from_ptr(0x5400_1000usize as _) };
pub const HASH1: *mut () = 0x5400_2000usize as _;
pub const RNG1: *mut () = 0x5400_3000usize as _;
pub const GPIOZ: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5400_4000usize as _) };
pub const MDMA: *mut () = 0x5800_0000usize as _;
pub const FMC: *mut () = 0x5800_2000usize as _;
pub const QUADSPI: quadspi::Quadspi = unsafe { quadspi::Quadspi::from_ptr(0x5800_3000usize as _) };
pub const SDMMC1: *mut () = 0x5800_5000usize as _;
pub const SDMMC2: *mut () = 0x5800_7000usize as _;
pub const CRC1: *mut () = 0x5800_9000usize as _;
pub const LTDC: *mut () = 0x5a00_1000usize as _;
pub const IWDG2: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5a00_2000usize as _) };
pub const USART1: *mut () = 0x5c00_0000usize as _;
pub const SPI6: *mut () = 0x5c00_1000usize as _;
pub const I2C4: *mut () = 0x5c00_2000usize as _;
pub const IWDG1: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5c00_3000usize as _) };
pub const RTC: *mut () = 0x5c00_4000usize as _;
pub const BSEC: bsec::Bsec = unsafe { bsec::Bsec::from_ptr(0x5c00_5000usize as _) };
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x5c00_5234usize as _) };
pub const I2C6: *mut () = 0x5c00_9000usize as _;
pub const TAMP: *mut () = 0x5c00_a000usize as _;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/bsec_v2.rs"]
pub mod bsec;
#[path = "../../peripherals/can_fdcan_v2.rs"]
pub mod can;
#[path = "../../peripherals/cec_v2.rs"]
pub mod cec;
#[path = "../../peripherals/cryp_v2.rs"]
pub mod cryp;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../../peripherals/rcc_mp1.rs"]
pub mod rcc;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
