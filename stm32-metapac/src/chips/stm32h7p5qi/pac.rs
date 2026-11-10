#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - PWR_PVD_AVD"]
    PWR_PVD_AVD = 0,
    #[doc = "2 - DTS"]
    DTS = 2,
    #[doc = "3 - IWDG"]
    IWDG = 3,
    #[doc = "4 - WWDG"]
    WWDG = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "8 - FLASH"]
    FLASH = 8,
    #[doc = "9 - RAMECC"]
    RAMECC = 9,
    #[doc = "10 - FPU"]
    FPU = 10,
    #[doc = "12 - LSECSS"]
    LSECSS = 12,
    #[doc = "13 - TAMP"]
    TAMP = 13,
    #[doc = "14 - CTI0"]
    CTI0 = 14,
    #[doc = "15 - CTI1"]
    CTI1 = 15,
    #[doc = "16 - EXTI0"]
    EXTI0 = 16,
    #[doc = "17 - EXTI1"]
    EXTI1 = 17,
    #[doc = "18 - EXTI2"]
    EXTI2 = 18,
    #[doc = "19 - EXTI3"]
    EXTI3 = 19,
    #[doc = "20 - EXTI4"]
    EXTI4 = 20,
    #[doc = "21 - EXTI5"]
    EXTI5 = 21,
    #[doc = "22 - EXTI6"]
    EXTI6 = 22,
    #[doc = "23 - EXTI7"]
    EXTI7 = 23,
    #[doc = "24 - EXTI8"]
    EXTI8 = 24,
    #[doc = "25 - EXTI9"]
    EXTI9 = 25,
    #[doc = "26 - EXTI10"]
    EXTI10 = 26,
    #[doc = "27 - EXTI11"]
    EXTI11 = 27,
    #[doc = "28 - EXTI12"]
    EXTI12 = 28,
    #[doc = "29 - EXTI13"]
    EXTI13 = 29,
    #[doc = "30 - EXTI14"]
    EXTI14 = 30,
    #[doc = "31 - EXTI15"]
    EXTI15 = 31,
    #[doc = "32 - RTC"]
    RTC = 32,
    #[doc = "34 - ADC2"]
    ADC2 = 34,
    #[doc = "35 - ADC4"]
    ADC4 = 35,
    #[doc = "36 - HASH"]
    HASH = 36,
    #[doc = "37 - RNG"]
    RNG = 37,
    #[doc = "38 - ADC1"]
    ADC1 = 38,
    #[doc = "39 - GPDMA1_CH0"]
    GPDMA1_CH0 = 39,
    #[doc = "40 - GPDMA1_CH1"]
    GPDMA1_CH1 = 40,
    #[doc = "41 - GPDMA1_CH2"]
    GPDMA1_CH2 = 41,
    #[doc = "42 - GPDMA1_CH3"]
    GPDMA1_CH3 = 42,
    #[doc = "43 - GPDMA1_CH4"]
    GPDMA1_CH4 = 43,
    #[doc = "44 - GPDMA1_CH5"]
    GPDMA1_CH5 = 44,
    #[doc = "45 - GPDMA1_CH6"]
    GPDMA1_CH6 = 45,
    #[doc = "46 - GPDMA1_CH7"]
    GPDMA1_CH7 = 46,
    #[doc = "47 - TIM1_BRK"]
    TIM1_BRK = 47,
    #[doc = "48 - TIM1_UP"]
    TIM1_UP = 48,
    #[doc = "49 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 49,
    #[doc = "50 - TIM1_CC"]
    TIM1_CC = 50,
    #[doc = "51 - TIM2"]
    TIM2 = 51,
    #[doc = "52 - TIM3"]
    TIM3 = 52,
    #[doc = "53 - TIM4"]
    TIM4 = 53,
    #[doc = "54 - TIM5"]
    TIM5 = 54,
    #[doc = "55 - TIM6"]
    TIM6 = 55,
    #[doc = "56 - TIM7"]
    TIM7 = 56,
    #[doc = "57 - TIM9"]
    TIM9 = 57,
    #[doc = "58 - SPI1"]
    SPI1 = 58,
    #[doc = "59 - SPI2"]
    SPI2 = 59,
    #[doc = "60 - SPI3"]
    SPI3 = 60,
    #[doc = "61 - SPI4"]
    SPI4 = 61,
    #[doc = "62 - SPI5"]
    SPI5 = 62,
    #[doc = "63 - SPI6"]
    SPI6 = 63,
    #[doc = "64 - HPDMA1_CH0"]
    HPDMA1_CH0 = 64,
    #[doc = "65 - HPDMA1_CH1"]
    HPDMA1_CH1 = 65,
    #[doc = "66 - HPDMA1_CH2"]
    HPDMA1_CH2 = 66,
    #[doc = "67 - HPDMA1_CH3"]
    HPDMA1_CH3 = 67,
    #[doc = "68 - HPDMA1_CH4"]
    HPDMA1_CH4 = 68,
    #[doc = "69 - HPDMA1_CH5"]
    HPDMA1_CH5 = 69,
    #[doc = "70 - HPDMA1_CH6"]
    HPDMA1_CH6 = 70,
    #[doc = "71 - HPDMA1_CH7"]
    HPDMA1_CH7 = 71,
    #[doc = "72 - TIM8_BRK"]
    TIM8_BRK = 72,
    #[doc = "73 - TIM8_UP"]
    TIM8_UP = 73,
    #[doc = "74 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 74,
    #[doc = "75 - TIM8_CC"]
    TIM8_CC = 75,
    #[doc = "76 - I2C1_EV"]
    I2C1_EV = 76,
    #[doc = "77 - I2C1_ERR"]
    I2C1_ERR = 77,
    #[doc = "78 - I2C2_EV"]
    I2C2_EV = 78,
    #[doc = "79 - I2C2_ERR"]
    I2C2_ERR = 79,
    #[doc = "80 - I2C3_EV"]
    I2C3_EV = 80,
    #[doc = "81 - I2C3_ERR"]
    I2C3_ERR = 81,
    #[doc = "82 - USART1"]
    USART1 = 82,
    #[doc = "83 - USART2"]
    USART2 = 83,
    #[doc = "84 - USART3"]
    USART3 = 84,
    #[doc = "85 - UART4"]
    UART4 = 85,
    #[doc = "86 - UART5"]
    UART5 = 86,
    #[doc = "87 - UART7"]
    UART7 = 87,
    #[doc = "88 - UART8"]
    UART8 = 88,
    #[doc = "89 - I3C1_EV"]
    I3C1_EV = 89,
    #[doc = "90 - I3C1_ERR"]
    I3C1_ERR = 90,
    #[doc = "92 - FMAC1"]
    FMAC1 = 92,
    #[doc = "93 - CORDIC"]
    CORDIC = 93,
    #[doc = "94 - ADC3"]
    ADC3 = 94,
    #[doc = "95 - TIM20_BRK_TERR_IERR"]
    TIM20_BRK_TERR_IERR = 95,
    #[doc = "96 - TIM20_UPD"]
    TIM20_UPD = 96,
    #[doc = "97 - TIM20_TRGI_COM_DIR_IDX"]
    TIM20_TRGI_COM_DIR_IDX = 97,
    #[doc = "98 - TIM20_CC"]
    TIM20_CC = 98,
    #[doc = "99 - TIM10"]
    TIM10 = 99,
    #[doc = "100 - TIM11"]
    TIM11 = 100,
    #[doc = "101 - I3C1_WKUP"]
    I3C1_WKUP = 101,
    #[doc = "102 - NPU_IT0"]
    NPU_IT0 = 102,
    #[doc = "103 - NPU_IT1"]
    NPU_IT1 = 103,
    #[doc = "104 - NPU_IT2"]
    NPU_IT2 = 104,
    #[doc = "105 - XSPI1"]
    XSPI1 = 105,
    #[doc = "106 - HSP1"]
    HSP1 = 106,
    #[doc = "107 - FMC"]
    FMC = 107,
    #[doc = "108 - SDMMC1"]
    SDMMC1 = 108,
    #[doc = "109 - HSP1_ERR"]
    HSP1_ERR = 109,
    #[doc = "110 - HSP1_EVT"]
    HSP1_EVT = 110,
    #[doc = "111 - HSP1_FWEVT"]
    HSP1_FWEVT = 111,
    #[doc = "112 - USB_OTG_FS"]
    USB_OTG_FS = 112,
    #[doc = "113 - TIM12"]
    TIM12 = 113,
    #[doc = "114 - TIM13"]
    TIM13 = 114,
    #[doc = "115 - TIM14"]
    TIM14 = 115,
    #[doc = "116 - TIM15"]
    TIM15 = 116,
    #[doc = "117 - TIM16"]
    TIM16 = 117,
    #[doc = "118 - TIM17"]
    TIM17 = 118,
    #[doc = "119 - LPTIM1"]
    LPTIM1 = 119,
    #[doc = "120 - LPTIM2"]
    LPTIM2 = 120,
    #[doc = "121 - LPTIM3"]
    LPTIM3 = 121,
    #[doc = "122 - LPTIM4"]
    LPTIM4 = 122,
    #[doc = "123 - LPTIM5"]
    LPTIM5 = 123,
    #[doc = "124 - UART9"]
    UART9 = 124,
    #[doc = "125 - USART10"]
    USART10 = 125,
    #[doc = "126 - ADF1_FLT0"]
    ADF1_FLT0 = 126,
    #[doc = "127 - CRS"]
    CRS = 127,
    #[doc = "128 - UCPD1"]
    UCPD1 = 128,
    #[doc = "129 - WKUP5_8"]
    WKUP5_8 = 129,
    #[doc = "130 - USART6"]
    USART6 = 130,
    #[doc = "131 - LPUART1"]
    LPUART1 = 131,
    #[doc = "132 - WKUP1_4"]
    WKUP1_4 = 132,
    #[doc = "133 - GPDMA1_CH8"]
    GPDMA1_CH8 = 133,
    #[doc = "134 - GPDMA1_CH9"]
    GPDMA1_CH9 = 134,
    #[doc = "135 - GPDMA1_CH10"]
    GPDMA1_CH10 = 135,
    #[doc = "136 - GPDMA1_CH11"]
    GPDMA1_CH11 = 136,
    #[doc = "137 - GPDMA1_CH12"]
    GPDMA1_CH12 = 137,
    #[doc = "138 - GPDMA1_CH13"]
    GPDMA1_CH13 = 138,
    #[doc = "139 - GPDMA1_CH14"]
    GPDMA1_CH14 = 139,
    #[doc = "140 - GPDMA1_CH15"]
    GPDMA1_CH15 = 140,
    #[doc = "141 - HPDMA1_CH8"]
    HPDMA1_CH8 = 141,
    #[doc = "142 - HPDMA1_CH9"]
    HPDMA1_CH9 = 142,
    #[doc = "143 - HPDMA1_CH10"]
    HPDMA1_CH10 = 143,
    #[doc = "144 - HPDMA1_CH11"]
    HPDMA1_CH11 = 144,
    #[doc = "145 - HPDMA1_CH12"]
    HPDMA1_CH12 = 145,
    #[doc = "146 - HPDMA1_CH13"]
    HPDMA1_CH13 = 146,
    #[doc = "147 - HPDMA1_CH14"]
    HPDMA1_CH14 = 147,
    #[doc = "148 - HPDMA1_CH15"]
    HPDMA1_CH15 = 148,
    #[doc = "150 - FDCAN3_IT0"]
    FDCAN3_IT0 = 150,
    #[doc = "151 - FDCAN3_IT1"]
    FDCAN3_IT1 = 151,
    #[doc = "152 - FDCAN1_IT0"]
    FDCAN1_IT0 = 152,
    #[doc = "153 - FDCAN1_IT1"]
    FDCAN1_IT1 = 153,
    #[doc = "154 - FDCAN2_IT0"]
    FDCAN2_IT0 = 154,
    #[doc = "155 - FDCAN2_IT1"]
    FDCAN2_IT1 = 155,
    #[doc = "156 - DAC1"]
    DAC1 = 156,
    #[doc = "157 - DAC2"]
    DAC2 = 157,
    #[doc = "158 - DAC3"]
    DAC3 = 158,
    #[doc = "159 - DAC4"]
    DAC4 = 159,
    #[doc = "160 - COMP1"]
    COMP1 = 160,
    #[doc = "161 - COMP2"]
    COMP2 = 161,
    #[doc = "162 - COMP3"]
    COMP3 = 162,
    #[doc = "163 - COMP4"]
    COMP4 = 163,
    #[doc = "164 - HRTIM1_M"]
    HRTIM1_M = 164,
    #[doc = "165 - HRTIM1_TIMA"]
    HRTIM1_TIMA = 165,
    #[doc = "166 - HRTIM1_TIMB"]
    HRTIM1_TIMB = 166,
    #[doc = "167 - HRTIM1_TIMC"]
    HRTIM1_TIMC = 167,
    #[doc = "168 - HRTIM1_TIMD"]
    HRTIM1_TIMD = 168,
    #[doc = "169 - HRTIM1_TIME"]
    HRTIM1_TIME = 169,
    #[doc = "170 - HRTIM1_TIMF"]
    HRTIM1_TIMF = 170,
    #[doc = "171 - HRTIM1_FLT"]
    HRTIM1_FLT = 171,
    #[doc = "172 - HRTIM2_M"]
    HRTIM2_M = 172,
    #[doc = "173 - HRTIM2_TIMA"]
    HRTIM2_TIMA = 173,
    #[doc = "174 - HRTIM2_TIMB"]
    HRTIM2_TIMB = 174,
    #[doc = "175 - HRTIM2_TIMC"]
    HRTIM2_TIMC = 175,
    #[doc = "176 - HRTIM2_TIMD"]
    HRTIM2_TIMD = 176,
    #[doc = "177 - HRTIM2_TIME"]
    HRTIM2_TIME = 177,
    #[doc = "178 - HRTIM2_TIMF"]
    HRTIM2_TIMF = 178,
    #[doc = "179 - HRTIM2_FLT"]
    HRTIM2_FLT = 179,
    #[doc = "180 - COMP5"]
    COMP5 = 180,
    #[doc = "181 - COMP6"]
    COMP6 = 181,
    #[doc = "182 - COMP7"]
    COMP7 = 182,
    #[doc = "183 - COMP8"]
    COMP8 = 183,
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
        fn PWR_PVD_AVD();
        fn DTS();
        fn IWDG();
        fn WWDG();
        fn RCC();
        fn FLASH();
        fn RAMECC();
        fn FPU();
        fn LSECSS();
        fn TAMP();
        fn CTI0();
        fn CTI1();
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
        fn RTC();
        fn ADC2();
        fn ADC4();
        fn HASH();
        fn RNG();
        fn ADC1();
        fn GPDMA1_CH0();
        fn GPDMA1_CH1();
        fn GPDMA1_CH2();
        fn GPDMA1_CH3();
        fn GPDMA1_CH4();
        fn GPDMA1_CH5();
        fn GPDMA1_CH6();
        fn GPDMA1_CH7();
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
        fn TIM9();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn HPDMA1_CH0();
        fn HPDMA1_CH1();
        fn HPDMA1_CH2();
        fn HPDMA1_CH3();
        fn HPDMA1_CH4();
        fn HPDMA1_CH5();
        fn HPDMA1_CH6();
        fn HPDMA1_CH7();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn I2C1_EV();
        fn I2C1_ERR();
        fn I2C2_EV();
        fn I2C2_ERR();
        fn I2C3_EV();
        fn I2C3_ERR();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn UART7();
        fn UART8();
        fn I3C1_EV();
        fn I3C1_ERR();
        fn FMAC1();
        fn CORDIC();
        fn ADC3();
        fn TIM20_BRK_TERR_IERR();
        fn TIM20_UPD();
        fn TIM20_TRGI_COM_DIR_IDX();
        fn TIM20_CC();
        fn TIM10();
        fn TIM11();
        fn I3C1_WKUP();
        fn NPU_IT0();
        fn NPU_IT1();
        fn NPU_IT2();
        fn XSPI1();
        fn HSP1();
        fn FMC();
        fn SDMMC1();
        fn HSP1_ERR();
        fn HSP1_EVT();
        fn HSP1_FWEVT();
        fn USB_OTG_FS();
        fn TIM12();
        fn TIM13();
        fn TIM14();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn LPTIM1();
        fn LPTIM2();
        fn LPTIM3();
        fn LPTIM4();
        fn LPTIM5();
        fn UART9();
        fn USART10();
        fn ADF1_FLT0();
        fn CRS();
        fn UCPD1();
        fn WKUP5_8();
        fn USART6();
        fn LPUART1();
        fn WKUP1_4();
        fn GPDMA1_CH8();
        fn GPDMA1_CH9();
        fn GPDMA1_CH10();
        fn GPDMA1_CH11();
        fn GPDMA1_CH12();
        fn GPDMA1_CH13();
        fn GPDMA1_CH14();
        fn GPDMA1_CH15();
        fn HPDMA1_CH8();
        fn HPDMA1_CH9();
        fn HPDMA1_CH10();
        fn HPDMA1_CH11();
        fn HPDMA1_CH12();
        fn HPDMA1_CH13();
        fn HPDMA1_CH14();
        fn HPDMA1_CH15();
        fn FDCAN3_IT0();
        fn FDCAN3_IT1();
        fn FDCAN1_IT0();
        fn FDCAN1_IT1();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn DAC1();
        fn DAC2();
        fn DAC3();
        fn DAC4();
        fn COMP1();
        fn COMP2();
        fn COMP3();
        fn COMP4();
        fn HRTIM1_M();
        fn HRTIM1_TIMA();
        fn HRTIM1_TIMB();
        fn HRTIM1_TIMC();
        fn HRTIM1_TIMD();
        fn HRTIM1_TIME();
        fn HRTIM1_TIMF();
        fn HRTIM1_FLT();
        fn HRTIM2_M();
        fn HRTIM2_TIMA();
        fn HRTIM2_TIMB();
        fn HRTIM2_TIMC();
        fn HRTIM2_TIMD();
        fn HRTIM2_TIME();
        fn HRTIM2_TIMF();
        fn HRTIM2_FLT();
        fn COMP5();
        fn COMP6();
        fn COMP7();
        fn COMP8();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 184] = [
        Vector { _handler: PWR_PVD_AVD },
        Vector { _reserved: 0 },
        Vector { _handler: DTS },
        Vector { _handler: IWDG },
        Vector { _handler: WWDG },
        Vector { _handler: RCC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FLASH },
        Vector { _handler: RAMECC },
        Vector { _handler: FPU },
        Vector { _reserved: 0 },
        Vector { _handler: LSECSS },
        Vector { _handler: TAMP },
        Vector { _handler: CTI0 },
        Vector { _handler: CTI1 },
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
        Vector { _handler: RTC },
        Vector { _reserved: 0 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC4 },
        Vector { _handler: HASH },
        Vector { _handler: RNG },
        Vector { _handler: ADC1 },
        Vector { _handler: GPDMA1_CH0 },
        Vector { _handler: GPDMA1_CH1 },
        Vector { _handler: GPDMA1_CH2 },
        Vector { _handler: GPDMA1_CH3 },
        Vector { _handler: GPDMA1_CH4 },
        Vector { _handler: GPDMA1_CH5 },
        Vector { _handler: GPDMA1_CH6 },
        Vector { _handler: GPDMA1_CH7 },
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
        Vector { _handler: TIM9 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: HPDMA1_CH0 },
        Vector { _handler: HPDMA1_CH1 },
        Vector { _handler: HPDMA1_CH2 },
        Vector { _handler: HPDMA1_CH3 },
        Vector { _handler: HPDMA1_CH4 },
        Vector { _handler: HPDMA1_CH5 },
        Vector { _handler: HPDMA1_CH6 },
        Vector { _handler: HPDMA1_CH7 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ERR },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ERR },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ERR },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ERR },
        Vector { _reserved: 0 },
        Vector { _handler: FMAC1 },
        Vector { _handler: CORDIC },
        Vector { _handler: ADC3 },
        Vector {
            _handler: TIM20_BRK_TERR_IERR,
        },
        Vector { _handler: TIM20_UPD },
        Vector {
            _handler: TIM20_TRGI_COM_DIR_IDX,
        },
        Vector { _handler: TIM20_CC },
        Vector { _handler: TIM10 },
        Vector { _handler: TIM11 },
        Vector { _handler: I3C1_WKUP },
        Vector { _handler: NPU_IT0 },
        Vector { _handler: NPU_IT1 },
        Vector { _handler: NPU_IT2 },
        Vector { _handler: XSPI1 },
        Vector { _handler: HSP1 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC1 },
        Vector { _handler: HSP1_ERR },
        Vector { _handler: HSP1_EVT },
        Vector { _handler: HSP1_FWEVT },
        Vector { _handler: USB_OTG_FS },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPTIM3 },
        Vector { _handler: LPTIM4 },
        Vector { _handler: LPTIM5 },
        Vector { _handler: UART9 },
        Vector { _handler: USART10 },
        Vector { _handler: ADF1_FLT0 },
        Vector { _handler: CRS },
        Vector { _handler: UCPD1 },
        Vector { _handler: WKUP5_8 },
        Vector { _handler: USART6 },
        Vector { _handler: LPUART1 },
        Vector { _handler: WKUP1_4 },
        Vector { _handler: GPDMA1_CH8 },
        Vector { _handler: GPDMA1_CH9 },
        Vector { _handler: GPDMA1_CH10 },
        Vector { _handler: GPDMA1_CH11 },
        Vector { _handler: GPDMA1_CH12 },
        Vector { _handler: GPDMA1_CH13 },
        Vector { _handler: GPDMA1_CH14 },
        Vector { _handler: GPDMA1_CH15 },
        Vector { _handler: HPDMA1_CH8 },
        Vector { _handler: HPDMA1_CH9 },
        Vector { _handler: HPDMA1_CH10 },
        Vector { _handler: HPDMA1_CH11 },
        Vector { _handler: HPDMA1_CH12 },
        Vector { _handler: HPDMA1_CH13 },
        Vector { _handler: HPDMA1_CH14 },
        Vector { _handler: HPDMA1_CH15 },
        Vector { _reserved: 0 },
        Vector { _handler: FDCAN3_IT0 },
        Vector { _handler: FDCAN3_IT1 },
        Vector { _handler: FDCAN1_IT0 },
        Vector { _handler: FDCAN1_IT1 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: DAC1 },
        Vector { _handler: DAC2 },
        Vector { _handler: DAC3 },
        Vector { _handler: DAC4 },
        Vector { _handler: COMP1 },
        Vector { _handler: COMP2 },
        Vector { _handler: COMP3 },
        Vector { _handler: COMP4 },
        Vector { _handler: HRTIM1_M },
        Vector { _handler: HRTIM1_TIMA },
        Vector { _handler: HRTIM1_TIMB },
        Vector { _handler: HRTIM1_TIMC },
        Vector { _handler: HRTIM1_TIMD },
        Vector { _handler: HRTIM1_TIME },
        Vector { _handler: HRTIM1_TIMF },
        Vector { _handler: HRTIM1_FLT },
        Vector { _handler: HRTIM2_M },
        Vector { _handler: HRTIM2_TIMA },
        Vector { _handler: HRTIM2_TIMB },
        Vector { _handler: HRTIM2_TIMC },
        Vector { _handler: HRTIM2_TIMD },
        Vector { _handler: HRTIM2_TIME },
        Vector { _handler: HRTIM2_TIMF },
        Vector { _handler: HRTIM2_FLT },
        Vector { _handler: COMP5 },
        Vector { _handler: COMP6 },
        Vector { _handler: COMP7 },
        Vector { _handler: COMP8 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: *mut () = 0x4000_0400usize as _;
pub const TIM4: *mut () = 0x4000_0800usize as _;
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_2400usize as _) };
pub const WWDG: *mut () = 0x4000_2c00usize as _;
pub const TIM10: *mut () = 0x4000_3000usize as _;
pub const TIM11: *mut () = 0x4000_3400usize as _;
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: *mut () = 0x4000_4400usize as _;
pub const USART3: *mut () = 0x4000_4800usize as _;
pub const UART4: *mut () = 0x4000_4c00usize as _;
pub const UART5: *mut () = 0x4000_5000usize as _;
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const I3C1: *mut () = 0x4000_6000usize as _;
pub const UART7: *mut () = 0x4000_7800usize as _;
pub const UART8: *mut () = 0x4000_7c00usize as _;
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_8400usize as _) };
pub const FDCAN1: *mut () = 0x4000_a000usize as _;
pub const FDCAN2: *mut () = 0x4000_a400usize as _;
pub const FDCAN3: *mut () = 0x4000_a800usize as _;
pub const FDCANRAM: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_af50usize as _) };
pub const FDCANRAM3: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_b2a0usize as _) };
pub const UCPD1: *mut () = 0x4000_ec00usize as _;
pub const HSP1: *mut () = 0x4002_0c00usize as _;
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000usize as _) };
pub const ADF1: *mut () = 0x4002_f000usize as _;
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4200_0000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4200_0400usize as _) };
pub const TIM20: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4200_0800usize as _) };
pub const USART1: *mut () = 0x4200_1000usize as _;
pub const USART6: *mut () = 0x4200_1400usize as _;
pub const UART9: *mut () = 0x4200_1800usize as _;
pub const USART10: *mut () = 0x4200_1c00usize as _;
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_3000usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_3400usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4200_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4200_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4200_4800usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4200_4c00usize as _) };
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4200_5000usize as _) };
pub const HRTIM1: hrtim::Hrtim = unsafe { hrtim::Hrtim::from_ptr(0x4200_6000usize as _) };
pub const HRTIM2: hrtim::Hrtim = unsafe { hrtim::Hrtim::from_ptr(0x4200_7000usize as _) };
pub const CORDIC: cordic::Cordic = unsafe { cordic::Cordic::from_ptr(0x4800_4400usize as _) };
pub const FMAC1: *mut () = 0x4800_4800usize as _;
pub const RNG: *mut () = 0x4802_0000usize as _;
pub const HASH: *mut () = 0x4802_0400usize as _;
pub const ADC1: *mut () = 0x4802_6000usize as _;
pub const ADC2: *mut () = 0x4802_6100usize as _;
pub const ADC12_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4802_6300usize as _) };
pub const ADC3: *mut () = 0x4802_6400usize as _;
pub const ADC4: *mut () = 0x4802_6500usize as _;
pub const ADC34_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4802_6700usize as _) };
pub const DAC1: *mut () = 0x4802_7000usize as _;
pub const DAC2: *mut () = 0x4802_7400usize as _;
pub const DAC3: *mut () = 0x4802_7800usize as _;
pub const DAC4: *mut () = 0x4802_7c00usize as _;
pub const HPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x5200_0000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5200_2000usize as _) };
pub const FMC: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x5200_4000usize as _) };
pub const XSPI1: *mut () = 0x5200_5000usize as _;
pub const SDMMC1: *mut () = 0x5200_7000usize as _;
pub const RAMECC1: *mut () = 0x5200_9000usize as _;
pub const NPU: *mut () = 0x520e_0000usize as _;
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x5800_0000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x5800_0400usize as _) };
pub const LPUART1: *mut () = 0x5800_0c00usize as _;
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x5800_1400usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2400usize as _) };
pub const LPTIM3: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2800usize as _) };
pub const LPTIM4: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_2c00usize as _) };
pub const LPTIM5: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x5800_3000usize as _) };
pub const VREFBUF: vrefbuf::Vrefbuf = unsafe { vrefbuf::Vrefbuf::from_ptr(0x5800_3c00usize as _) };
pub const RTC: *mut () = 0x5800_4000usize as _;
pub const TAMP: *mut () = 0x5800_4400usize as _;
pub const IWDG: *mut () = 0x5800_4800usize as _;
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x5800_6800usize as _) };
pub const COMP1: *mut () = 0x5800_7000usize as _;
pub const COMP2: *mut () = 0x5800_7200usize as _;
pub const COMP3: *mut () = 0x5800_7400usize as _;
pub const COMP4: *mut () = 0x5800_7600usize as _;
pub const COMP5: *mut () = 0x5800_7800usize as _;
pub const COMP6: *mut () = 0x5800_7a00usize as _;
pub const COMP7: *mut () = 0x5800_7c00usize as _;
pub const COMP8: *mut () = 0x5800_7e00usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1c00usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_2000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5802_4400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5802_4800usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x5802_4c00usize as _) };
pub const RAMECC2: *mut () = 0x5802_7000usize as _;
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x5c00_1000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/adccommon_v4.rs"]
pub mod adccommon;
#[path = "../../peripherals/cordic_v1.rs"]
pub mod cordic;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dbgmcu_h7.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_h7.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v2.rs"]
pub mod fdcanram;
#[path = "../../peripherals/flash_v5_h7.rs"]
pub mod flash;
#[path = "../../peripherals/fmc_v3x1.rs"]
pub mod fmc;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hrtim_v1.rs"]
pub mod hrtim;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/lptim_v1b_h7.rs"]
pub mod lptim;
#[path = "../../peripherals/pwr_h7p.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_h7p.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_v4_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_h7.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/vrefbuf_v2a1.rs"]
pub mod vrefbuf;
