#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "2 - IWDG"]
    IWDG = 2,
    #[doc = "4 - PWR_PVD_AVD"]
    PWR_PVD_AVD = 4,
    #[doc = "5 - TAMP"]
    TAMP = 5,
    #[doc = "6 - GTZC1"]
    GTZC1 = 6,
    #[doc = "7 - FLASH_S"]
    FLASH_S = 7,
    #[doc = "8 - FLASH"]
    FLASH = 8,
    #[doc = "11 - RAMCFG"]
    RAMCFG = 11,
    #[doc = "12 - RCC_S"]
    RCC_S = 12,
    #[doc = "13 - RCC"]
    RCC = 13,
    #[doc = "14 - ICACHE"]
    ICACHE = 14,
    #[doc = "15 - DCACHE"]
    DCACHE = 15,
    #[doc = "16 - FPU"]
    FPU = 16,
    #[doc = "17 - RTC_S"]
    RTC_S = 17,
    #[doc = "18 - RTC"]
    RTC = 18,
    #[doc = "19 - EXTI0"]
    EXTI0 = 19,
    #[doc = "20 - EXTI1"]
    EXTI1 = 20,
    #[doc = "21 - EXTI2"]
    EXTI2 = 21,
    #[doc = "22 - EXTI3"]
    EXTI3 = 22,
    #[doc = "23 - EXTI4"]
    EXTI4 = 23,
    #[doc = "24 - EXTI5"]
    EXTI5 = 24,
    #[doc = "25 - EXTI6"]
    EXTI6 = 25,
    #[doc = "26 - EXTI7"]
    EXTI7 = 26,
    #[doc = "27 - EXTI8"]
    EXTI8 = 27,
    #[doc = "28 - EXTI9"]
    EXTI9 = 28,
    #[doc = "29 - EXTI10"]
    EXTI10 = 29,
    #[doc = "30 - EXTI11"]
    EXTI11 = 30,
    #[doc = "31 - EXTI12"]
    EXTI12 = 31,
    #[doc = "32 - EXTI13"]
    EXTI13 = 32,
    #[doc = "33 - EXTI14"]
    EXTI14 = 33,
    #[doc = "34 - EXTI15"]
    EXTI15 = 34,
    #[doc = "35 - GPDMA1_CH0"]
    GPDMA1_CH0 = 35,
    #[doc = "36 - GPDMA1_CH1"]
    GPDMA1_CH1 = 36,
    #[doc = "37 - GPDMA1_CH2"]
    GPDMA1_CH2 = 37,
    #[doc = "38 - GPDMA1_CH3"]
    GPDMA1_CH3 = 38,
    #[doc = "39 - GPDMA1_CH4"]
    GPDMA1_CH4 = 39,
    #[doc = "40 - GPDMA1_CH5"]
    GPDMA1_CH5 = 40,
    #[doc = "41 - GPDMA1_CH6"]
    GPDMA1_CH6 = 41,
    #[doc = "42 - GPDMA1_CH7"]
    GPDMA1_CH7 = 42,
    #[doc = "43 - SAES"]
    SAES = 43,
    #[doc = "44 - ADC1"]
    ADC1 = 44,
    #[doc = "45 - DAC1"]
    DAC1 = 45,
    #[doc = "46 - FDCAN1_IT0"]
    FDCAN1_IT0 = 46,
    #[doc = "47 - FDCAN1_IT1"]
    FDCAN1_IT1 = 47,
    #[doc = "48 - TIM1_BRK"]
    TIM1_BRK = 48,
    #[doc = "49 - TIM1_UP"]
    TIM1_UP = 49,
    #[doc = "50 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 50,
    #[doc = "51 - TIM1_CC"]
    TIM1_CC = 51,
    #[doc = "52 - TIM2"]
    TIM2 = 52,
    #[doc = "53 - TIM3"]
    TIM3 = 53,
    #[doc = "54 - TIM4"]
    TIM4 = 54,
    #[doc = "55 - TIM5"]
    TIM5 = 55,
    #[doc = "56 - TIM6"]
    TIM6 = 56,
    #[doc = "57 - TIM7"]
    TIM7 = 57,
    #[doc = "58 - I2C1_EV"]
    I2C1_EV = 58,
    #[doc = "59 - I2C1_ERR"]
    I2C1_ERR = 59,
    #[doc = "60 - I2C2_EV"]
    I2C2_EV = 60,
    #[doc = "61 - I2C2_ERR"]
    I2C2_ERR = 61,
    #[doc = "62 - SPI1"]
    SPI1 = 62,
    #[doc = "63 - SPI2"]
    SPI2 = 63,
    #[doc = "64 - SPI3"]
    SPI3 = 64,
    #[doc = "65 - USART1"]
    USART1 = 65,
    #[doc = "66 - USART2"]
    USART2 = 66,
    #[doc = "67 - USART3"]
    USART3 = 67,
    #[doc = "68 - UART4"]
    UART4 = 68,
    #[doc = "69 - UART5"]
    UART5 = 69,
    #[doc = "70 - LPUART1"]
    LPUART1 = 70,
    #[doc = "71 - LPTIM1"]
    LPTIM1 = 71,
    #[doc = "72 - TIM8_BRK"]
    TIM8_BRK = 72,
    #[doc = "73 - TIM8_UP"]
    TIM8_UP = 73,
    #[doc = "74 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 74,
    #[doc = "75 - TIM8_CC"]
    TIM8_CC = 75,
    #[doc = "76 - ADC2"]
    ADC2 = 76,
    #[doc = "77 - ADC3"]
    ADC3 = 77,
    #[doc = "78 - TIM15"]
    TIM15 = 78,
    #[doc = "79 - TIM16"]
    TIM16 = 79,
    #[doc = "80 - TIM17"]
    TIM17 = 80,
    #[doc = "81 - USB_DRD_FS"]
    USB_DRD_FS = 81,
    #[doc = "82 - CRS"]
    CRS = 82,
    #[doc = "83 - UCPD1"]
    UCPD1 = 83,
    #[doc = "85 - XSPI1"]
    XSPI1 = 85,
    #[doc = "86 - TIM9"]
    TIM9 = 86,
    #[doc = "87 - I2C3_EV"]
    I2C3_EV = 87,
    #[doc = "88 - I2C3_ERR"]
    I2C3_ERR = 88,
    #[doc = "92 - UART6"]
    UART6 = 92,
    #[doc = "93 - TIM20_BRK_TERR_IERR"]
    TIM20_BRK_TERR_IERR = 93,
    #[doc = "94 - TIM20_UPD"]
    TIM20_UPD = 94,
    #[doc = "95 - TIM20_TRGI_COM_DIR_IDX"]
    TIM20_TRGI_COM_DIR_IDX = 95,
    #[doc = "96 - TIM20_CC"]
    TIM20_CC = 96,
    #[doc = "97 - GPDMA2_CH0"]
    GPDMA2_CH0 = 97,
    #[doc = "98 - GPDMA2_CH1"]
    GPDMA2_CH1 = 98,
    #[doc = "99 - GPDMA2_CH2"]
    GPDMA2_CH2 = 99,
    #[doc = "100 - GPDMA2_CH3"]
    GPDMA2_CH3 = 100,
    #[doc = "101 - GPDMA2_CH4"]
    GPDMA2_CH4 = 101,
    #[doc = "102 - GPDMA2_CH5"]
    GPDMA2_CH5 = 102,
    #[doc = "103 - GPDMA2_CH6"]
    GPDMA2_CH6 = 103,
    #[doc = "104 - GPDMA2_CH7"]
    GPDMA2_CH7 = 104,
    #[doc = "107 - FDCAN3_IT0"]
    FDCAN3_IT0 = 107,
    #[doc = "108 - FDCAN3_IT1"]
    FDCAN3_IT1 = 108,
    #[doc = "111 - FMAC2"]
    FMAC2 = 111,
    #[doc = "112 - FDCAN2_IT0"]
    FDCAN2_IT0 = 112,
    #[doc = "113 - FDCAN2_IT1"]
    FDCAN2_IT1 = 113,
    #[doc = "114 - CORDIC1"]
    CORDIC1 = 114,
    #[doc = "115 - FMAC1"]
    FMAC1 = 115,
    #[doc = "116 - DTS"]
    DTS = 116,
    #[doc = "117 - RNG"]
    RNG = 117,
    #[doc = "118 - OTFDEC1"]
    OTFDEC1 = 118,
    #[doc = "119 - AES"]
    AES = 119,
    #[doc = "120 - HASH"]
    HASH = 120,
    #[doc = "121 - PKA"]
    PKA = 121,
    #[doc = "122 - TIM10"]
    TIM10 = 122,
    #[doc = "123 - TIM12"]
    TIM12 = 123,
    #[doc = "124 - TIM13"]
    TIM13 = 124,
    #[doc = "125 - TIM14"]
    TIM14 = 125,
    #[doc = "126 - I3C1_EV"]
    I3C1_EV = 126,
    #[doc = "127 - I3C1_ERR"]
    I3C1_ERR = 127,
    #[doc = "128 - I2C4_EV"]
    I2C4_EV = 128,
    #[doc = "129 - I2C4_ERR"]
    I2C4_ERR = 129,
    #[doc = "136 - POSENC1"]
    POSENC1 = 136,
    #[doc = "137 - POSENC2"]
    POSENC2 = 137,
    #[doc = "138 - HRTIM1_M"]
    HRTIM1_M = 138,
    #[doc = "139 - HRTIM1_TIMA"]
    HRTIM1_TIMA = 139,
    #[doc = "140 - HRTIM1_TIMB"]
    HRTIM1_TIMB = 140,
    #[doc = "141 - HRTIM1_TIMC"]
    HRTIM1_TIMC = 141,
    #[doc = "142 - HRTIM1_TIMD"]
    HRTIM1_TIMD = 142,
    #[doc = "143 - HRTIM1_TIME"]
    HRTIM1_TIME = 143,
    #[doc = "144 - HRTIM1_TIMF"]
    HRTIM1_TIMF = 144,
    #[doc = "145 - HRTIM1_FLT"]
    HRTIM1_FLT = 145,
    #[doc = "146 - PLAY1_S"]
    PLAY1_S = 146,
    #[doc = "147 - PLAY1"]
    PLAY1 = 147,
    #[doc = "148 - COMP1"]
    COMP1 = 148,
    #[doc = "149 - COMP2"]
    COMP2 = 149,
    #[doc = "150 - COMP3"]
    COMP3 = 150,
    #[doc = "151 - COMP4"]
    COMP4 = 151,
    #[doc = "152 - COMP5"]
    COMP5 = 152,
    #[doc = "153 - COMP6"]
    COMP6 = 153,
    #[doc = "154 - COMP7"]
    COMP7 = 154,
    #[doc = "155 - COMP8"]
    COMP8 = 155,
    #[doc = "164 - DAC2"]
    DAC2 = 164,
    #[doc = "167 - DAC5"]
    DAC5 = 167,
    #[doc = "168 - DAC6"]
    DAC6 = 168,
    #[doc = "169 - DAC7"]
    DAC7 = 169,
    #[doc = "170 - DAC8"]
    DAC8 = 170,
    #[doc = "171 - DAC9"]
    DAC9 = 171,
    #[doc = "172 - DAC10"]
    DAC10 = 172,
    #[doc = "187 - TIM11"]
    TIM11 = 187,
    #[doc = "188 - TIM26"]
    TIM26 = 188,
    #[doc = "189 - TIM27"]
    TIM27 = 189,
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
        fn WWDG();
        fn IWDG();
        fn PWR_PVD_AVD();
        fn TAMP();
        fn GTZC1();
        fn FLASH_S();
        fn FLASH();
        fn RAMCFG();
        fn RCC_S();
        fn RCC();
        fn ICACHE();
        fn DCACHE();
        fn FPU();
        fn RTC_S();
        fn RTC();
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
        fn GPDMA1_CH0();
        fn GPDMA1_CH1();
        fn GPDMA1_CH2();
        fn GPDMA1_CH3();
        fn GPDMA1_CH4();
        fn GPDMA1_CH5();
        fn GPDMA1_CH6();
        fn GPDMA1_CH7();
        fn SAES();
        fn ADC1();
        fn DAC1();
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
        fn I2C1_EV();
        fn I2C1_ERR();
        fn I2C2_EV();
        fn I2C2_ERR();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn USART1();
        fn USART2();
        fn USART3();
        fn UART4();
        fn UART5();
        fn LPUART1();
        fn LPTIM1();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn ADC2();
        fn ADC3();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn USB_DRD_FS();
        fn CRS();
        fn UCPD1();
        fn XSPI1();
        fn TIM9();
        fn I2C3_EV();
        fn I2C3_ERR();
        fn UART6();
        fn TIM20_BRK_TERR_IERR();
        fn TIM20_UPD();
        fn TIM20_TRGI_COM_DIR_IDX();
        fn TIM20_CC();
        fn GPDMA2_CH0();
        fn GPDMA2_CH1();
        fn GPDMA2_CH2();
        fn GPDMA2_CH3();
        fn GPDMA2_CH4();
        fn GPDMA2_CH5();
        fn GPDMA2_CH6();
        fn GPDMA2_CH7();
        fn FDCAN3_IT0();
        fn FDCAN3_IT1();
        fn FMAC2();
        fn FDCAN2_IT0();
        fn FDCAN2_IT1();
        fn CORDIC1();
        fn FMAC1();
        fn DTS();
        fn RNG();
        fn OTFDEC1();
        fn AES();
        fn HASH();
        fn PKA();
        fn TIM10();
        fn TIM12();
        fn TIM13();
        fn TIM14();
        fn I3C1_EV();
        fn I3C1_ERR();
        fn I2C4_EV();
        fn I2C4_ERR();
        fn POSENC1();
        fn POSENC2();
        fn HRTIM1_M();
        fn HRTIM1_TIMA();
        fn HRTIM1_TIMB();
        fn HRTIM1_TIMC();
        fn HRTIM1_TIMD();
        fn HRTIM1_TIME();
        fn HRTIM1_TIMF();
        fn HRTIM1_FLT();
        fn PLAY1_S();
        fn PLAY1();
        fn COMP1();
        fn COMP2();
        fn COMP3();
        fn COMP4();
        fn COMP5();
        fn COMP6();
        fn COMP7();
        fn COMP8();
        fn DAC2();
        fn DAC5();
        fn DAC6();
        fn DAC7();
        fn DAC8();
        fn DAC9();
        fn DAC10();
        fn TIM11();
        fn TIM26();
        fn TIM27();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 190] = [
        Vector { _handler: WWDG },
        Vector { _reserved: 0 },
        Vector { _handler: IWDG },
        Vector { _reserved: 0 },
        Vector { _handler: PWR_PVD_AVD },
        Vector { _handler: TAMP },
        Vector { _handler: GTZC1 },
        Vector { _handler: FLASH_S },
        Vector { _handler: FLASH },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: RAMCFG },
        Vector { _handler: RCC_S },
        Vector { _handler: RCC },
        Vector { _handler: ICACHE },
        Vector { _handler: DCACHE },
        Vector { _handler: FPU },
        Vector { _handler: RTC_S },
        Vector { _handler: RTC },
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
        Vector { _handler: GPDMA1_CH0 },
        Vector { _handler: GPDMA1_CH1 },
        Vector { _handler: GPDMA1_CH2 },
        Vector { _handler: GPDMA1_CH3 },
        Vector { _handler: GPDMA1_CH4 },
        Vector { _handler: GPDMA1_CH5 },
        Vector { _handler: GPDMA1_CH6 },
        Vector { _handler: GPDMA1_CH7 },
        Vector { _handler: SAES },
        Vector { _handler: ADC1 },
        Vector { _handler: DAC1 },
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
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ERR },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ERR },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: USB_DRD_FS },
        Vector { _handler: CRS },
        Vector { _handler: UCPD1 },
        Vector { _reserved: 0 },
        Vector { _handler: XSPI1 },
        Vector { _handler: TIM9 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ERR },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART6 },
        Vector {
            _handler: TIM20_BRK_TERR_IERR,
        },
        Vector { _handler: TIM20_UPD },
        Vector {
            _handler: TIM20_TRGI_COM_DIR_IDX,
        },
        Vector { _handler: TIM20_CC },
        Vector { _handler: GPDMA2_CH0 },
        Vector { _handler: GPDMA2_CH1 },
        Vector { _handler: GPDMA2_CH2 },
        Vector { _handler: GPDMA2_CH3 },
        Vector { _handler: GPDMA2_CH4 },
        Vector { _handler: GPDMA2_CH5 },
        Vector { _handler: GPDMA2_CH6 },
        Vector { _handler: GPDMA2_CH7 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FDCAN3_IT0 },
        Vector { _handler: FDCAN3_IT1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FMAC2 },
        Vector { _handler: FDCAN2_IT0 },
        Vector { _handler: FDCAN2_IT1 },
        Vector { _handler: CORDIC1 },
        Vector { _handler: FMAC1 },
        Vector { _handler: DTS },
        Vector { _handler: RNG },
        Vector { _handler: OTFDEC1 },
        Vector { _handler: AES },
        Vector { _handler: HASH },
        Vector { _handler: PKA },
        Vector { _handler: TIM10 },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM13 },
        Vector { _handler: TIM14 },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ERR },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ERR },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: POSENC1 },
        Vector { _handler: POSENC2 },
        Vector { _handler: HRTIM1_M },
        Vector { _handler: HRTIM1_TIMA },
        Vector { _handler: HRTIM1_TIMB },
        Vector { _handler: HRTIM1_TIMC },
        Vector { _handler: HRTIM1_TIMD },
        Vector { _handler: HRTIM1_TIME },
        Vector { _handler: HRTIM1_TIMF },
        Vector { _handler: HRTIM1_FLT },
        Vector { _handler: PLAY1_S },
        Vector { _handler: PLAY1 },
        Vector { _handler: COMP1 },
        Vector { _handler: COMP2 },
        Vector { _handler: COMP3 },
        Vector { _handler: COMP4 },
        Vector { _handler: COMP5 },
        Vector { _handler: COMP6 },
        Vector { _handler: COMP7 },
        Vector { _handler: COMP8 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DAC2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DAC5 },
        Vector { _handler: DAC6 },
        Vector { _handler: DAC7 },
        Vector { _handler: DAC8 },
        Vector { _handler: DAC9 },
        Vector { _handler: DAC10 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM11 },
        Vector { _handler: TIM26 },
        Vector { _handler: TIM27 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x08ff_f810usize as _) };
pub const TIM2: *mut () = 0x4000_0000usize as _;
pub const TIM3: *mut () = 0x4000_0400usize as _;
pub const TIM6: *mut () = 0x4000_1000usize as _;
pub const TIM7: *mut () = 0x4000_1400usize as _;
pub const TIM9: *mut () = 0x4000_1800usize as _;
pub const TIM10: *mut () = 0x4000_1c00usize as _;
pub const TIM11: *mut () = 0x4000_2000usize as _;
pub const TIM12: *mut () = 0x4000_2400usize as _;
pub const TIM13: *mut () = 0x4000_2800usize as _;
pub const TIM14: *mut () = 0x4000_2c00usize as _;
pub const WWDG: *mut () = 0x4000_3000usize as _;
pub const IWDG: *mut () = 0x4000_3400usize as _;
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I3C1: *mut () = 0x4000_5c00usize as _;
pub const CRS: *mut () = 0x4000_6000usize as _;
pub const UART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_6400usize as _) };
pub const DTS: dts::Dts = unsafe { dts::Dts::from_ptr(0x4000_8c00usize as _) };
pub const FDCAN1: *mut () = 0x4000_a000usize as _;
pub const FDCAN2: *mut () = 0x4000_a400usize as _;
pub const FDCAN3: *mut () = 0x4000_a800usize as _;
pub const FDCANRAM: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_ac00usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_af50usize as _) };
pub const FDCANRAM3: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_b2a0usize as _) };
pub const UCPD1: *mut () = 0x4000_dc00usize as _;
pub const HRTIM1: hrtim::Hrtim = unsafe { hrtim::Hrtim::from_ptr(0x4001_0000usize as _) };
pub const PLAY1: play::Play = unsafe { play::Play::from_ptr(0x4001_1400usize as _) };
pub const TIM1: *mut () = 0x4001_2c00usize as _;
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: *mut () = 0x4001_3400usize as _;
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM15: *mut () = 0x4001_4000usize as _;
pub const TIM16: *mut () = 0x4001_4400usize as _;
pub const TIM17: *mut () = 0x4001_4800usize as _;
pub const USB: *mut () = 0x4001_6000usize as _;
pub const USBRAM: *mut () = 0x4001_6400usize as _;
pub const TIM4: *mut () = 0x4001_7400usize as _;
pub const TIM5: *mut () = 0x4001_7800usize as _;
pub const GPDMA1: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_0000usize as _) };
pub const GPDMA2: gpdma::Gpdma = unsafe { gpdma::Gpdma::from_ptr(0x4002_1000usize as _) };
pub const FLASH: *mut () = 0x4002_2000usize as _;
pub const CRC: *mut () = 0x4002_3000usize as _;
pub const CORDIC1: cordic::Cordic = unsafe { cordic::Cordic::from_ptr(0x4002_3800usize as _) };
pub const FMAC1: *mut () = 0x4002_3c00usize as _;
pub const RAMCFG: *mut () = 0x4002_6000usize as _;
pub const RAMCFG_SRAM1: *mut () = 0x4002_6000usize as _;
pub const RAMCFG_SRAM2: *mut () = 0x4002_6040usize as _;
pub const RAMCFG_SRAM3: *mut () = 0x4002_6080usize as _;
pub const ICACHE: *mut () = 0x4003_0400usize as _;
pub const DCACHE: dcache::Dcache = unsafe { dcache::Dcache::from_ptr(0x4003_1400usize as _) };
pub const GTZC1_MPCBB1: *mut () = 0x4003_2c00usize as _;
pub const GTZC1_MPCBB2: *mut () = 0x4003_3000usize as _;
pub const GTZC1_MPCBB3: *mut () = 0x4003_3400usize as _;
pub const ADC1: *mut () = 0x4202_0000usize as _;
pub const ADC2: *mut () = 0x4202_0100usize as _;
pub const ADC12_COMMON: *mut () = 0x4202_0300usize as _;
pub const ADC3: *mut () = 0x4202_0400usize as _;
pub const ADC3_COMMON: *mut () = 0x4202_0700usize as _;
pub const DAC1: *mut () = 0x4202_1400usize as _;
pub const DAC2: *mut () = 0x4202_1800usize as _;
pub const DAC5: *mut () = 0x4202_2400usize as _;
pub const DAC6: *mut () = 0x4202_2800usize as _;
pub const DAC7: *mut () = 0x4202_2c00usize as _;
pub const DAC8: *mut () = 0x4202_3000usize as _;
pub const DAC9: *mut () = 0x4202_3400usize as _;
pub const DAC10: *mut () = 0x4202_3800usize as _;
pub const FMAC2: *mut () = 0x4202_4400usize as _;
pub const SYSCFG: *mut () = 0x4400_0400usize as _;
pub const TIM20: *mut () = 0x4400_0800usize as _;
pub const TIM26: *mut () = 0x4400_0c00usize as _;
pub const TIM27: *mut () = 0x4400_1000usize as _;
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4400_2400usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4400_2800usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4400_2c00usize as _) };
pub const LPTIM1: *mut () = 0x4400_4400usize as _;
pub const OPAMP1: *mut () = 0x4400_5c00usize as _;
pub const OPAMP2: *mut () = 0x4400_6000usize as _;
pub const OPAMP3: *mut () = 0x4400_6400usize as _;
pub const VREFBUF: *mut () = 0x4400_7400usize as _;
pub const RTC: *mut () = 0x4400_7800usize as _;
pub const TAMP: *mut () = 0x4400_7c00usize as _;
pub const COMP1: *mut () = 0x4400_8400usize as _;
pub const COMP2: *mut () = 0x4400_8800usize as _;
pub const COMP3: *mut () = 0x4400_8c00usize as _;
pub const COMP4: *mut () = 0x4400_9000usize as _;
pub const COMP5: *mut () = 0x4400_9400usize as _;
pub const COMP6: *mut () = 0x4400_9800usize as _;
pub const COMP7: *mut () = 0x4400_9c00usize as _;
pub const COMP8: *mut () = 0x4400_a000usize as _;
pub const POSENC1: *mut () = 0x4401_3800usize as _;
pub const POSENC2: *mut () = 0x4401_7800usize as _;
pub const PWR: *mut () = 0x4402_0800usize as _;
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4402_0c00usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4402_2000usize as _) };
pub const DBGMCU: *mut () = 0x4402_6000usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4600_1c00usize as _) };
pub const OTFDEC1: otfdec::Otfdec = unsafe { otfdec::Otfdec::from_ptr(0x4600_5000usize as _) };
pub const AES: *mut () = 0x4600_6000usize as _;
pub const HASH: *mut () = 0x4600_6400usize as _;
pub const RNG: *mut () = 0x4600_6800usize as _;
pub const SAES: *mut () = 0x4600_6c00usize as _;
pub const PKA: *mut () = 0x4600_8000usize as _;
pub const CCB: *mut () = 0x4600_dc00usize as _;
pub const XSPI1: octospi::Octospi = unsafe { octospi::Octospi::from_ptr(0x4700_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/cordic_v1.rs"]
pub mod cordic;
#[path = "../../peripherals/dcache_v1.rs"]
pub mod dcache;
#[path = "../../peripherals/dts_v1.rs"]
pub mod dts;
#[path = "../../peripherals/exti_u5.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/hrtim_g5.rs"]
pub mod hrtim;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/octospi_v2.rs"]
pub mod octospi;
#[path = "../../peripherals/otfdec_v1.rs"]
pub mod otfdec;
#[path = "../../peripherals/play_v1.rs"]
pub mod play;
#[path = "../../peripherals/rcc_g5.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
