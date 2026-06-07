#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PWR_PVD"]
    PWR_PVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - TAMP"]
    TAMP = 3,
    #[doc = "4 - RAMCFG"]
    RAMCFG = 4,
    #[doc = "5 - FLASH"]
    FLASH = 5,
    #[doc = "6 - RCC"]
    RCC = 6,
    #[doc = "7 - EXTI0"]
    EXTI0 = 7,
    #[doc = "8 - EXTI1"]
    EXTI1 = 8,
    #[doc = "9 - EXTI2"]
    EXTI2 = 9,
    #[doc = "10 - EXTI3"]
    EXTI3 = 10,
    #[doc = "11 - EXTI4"]
    EXTI4 = 11,
    #[doc = "12 - EXTI5"]
    EXTI5 = 12,
    #[doc = "13 - EXTI6"]
    EXTI6 = 13,
    #[doc = "14 - EXTI7"]
    EXTI7 = 14,
    #[doc = "15 - EXTI8"]
    EXTI8 = 15,
    #[doc = "16 - EXTI9"]
    EXTI9 = 16,
    #[doc = "17 - EXTI10"]
    EXTI10 = 17,
    #[doc = "18 - EXTI11"]
    EXTI11 = 18,
    #[doc = "19 - EXTI12"]
    EXTI12 = 19,
    #[doc = "20 - EXTI13"]
    EXTI13 = 20,
    #[doc = "21 - EXTI14"]
    EXTI14 = 21,
    #[doc = "22 - EXTI15"]
    EXTI15 = 22,
    #[doc = "23 - LPDMA1_CH0"]
    LPDMA1_CH0 = 23,
    #[doc = "24 - LPDMA1_CH1"]
    LPDMA1_CH1 = 24,
    #[doc = "25 - LPDMA1_CH2"]
    LPDMA1_CH2 = 25,
    #[doc = "26 - LPDMA1_CH3"]
    LPDMA1_CH3 = 26,
    #[doc = "27 - LPDMA1_CH4"]
    LPDMA1_CH4 = 27,
    #[doc = "28 - LPDMA1_CH5"]
    LPDMA1_CH5 = 28,
    #[doc = "29 - LPDMA1_CH6"]
    LPDMA1_CH6 = 29,
    #[doc = "30 - LPDMA1_CH7"]
    LPDMA1_CH7 = 30,
    #[doc = "31 - IWDG"]
    IWDG = 31,
    #[doc = "32 - ADC1"]
    ADC1 = 32,
    #[doc = "33 - ADC2"]
    ADC2 = 33,
    #[doc = "36 - TIM1_BRK"]
    TIM1_BRK = 36,
    #[doc = "37 - TIM1_UP"]
    TIM1_UP = 37,
    #[doc = "38 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 38,
    #[doc = "39 - TIM1_CC"]
    TIM1_CC = 39,
    #[doc = "40 - TIM2"]
    TIM2 = 40,
    #[doc = "41 - TIM5"]
    TIM5 = 41,
    #[doc = "42 - TIM6"]
    TIM6 = 42,
    #[doc = "43 - TIM7"]
    TIM7 = 43,
    #[doc = "44 - I2C1_EV"]
    I2C1_EV = 44,
    #[doc = "45 - I2C1_ERR"]
    I2C1_ERR = 45,
    #[doc = "46 - I3C1_EV"]
    I3C1_EV = 46,
    #[doc = "47 - I3C1_ERR"]
    I3C1_ERR = 47,
    #[doc = "48 - SPI1"]
    SPI1 = 48,
    #[doc = "49 - SPI2"]
    SPI2 = 49,
    #[doc = "50 - SPI3"]
    SPI3 = 50,
    #[doc = "51 - USART1"]
    USART1 = 51,
    #[doc = "52 - USART2"]
    USART2 = 52,
    #[doc = "53 - USART3"]
    USART3 = 53,
    #[doc = "54 - UART4"]
    UART4 = 54,
    #[doc = "55 - UART5"]
    UART5 = 55,
    #[doc = "56 - LPUART1"]
    LPUART1 = 56,
    #[doc = "57 - LPTIM1"]
    LPTIM1 = 57,
    #[doc = "58 - TIM12"]
    TIM12 = 58,
    #[doc = "59 - TIM15"]
    TIM15 = 59,
    #[doc = "60 - TIM16"]
    TIM16 = 60,
    #[doc = "61 - TIM17"]
    TIM17 = 61,
    #[doc = "62 - USB_DRD_FS"]
    USB_DRD_FS = 62,
    #[doc = "63 - CRS"]
    CRS = 63,
    #[doc = "64 - RNG"]
    RNG = 64,
    #[doc = "65 - FPU"]
    FPU = 65,
    #[doc = "66 - ICACHE"]
    ICACHE = 66,
    #[doc = "67 - CORDIC"]
    CORDIC = 67,
    #[doc = "69 - HASH"]
    HASH = 69,
    #[doc = "70 - I2C2_EV"]
    I2C2_EV = 70,
    #[doc = "71 - I2C2_ERR"]
    I2C2_ERR = 71,
    #[doc = "72 - TIM8_BRK"]
    TIM8_BRK = 72,
    #[doc = "73 - TIM8_UP"]
    TIM8_UP = 73,
    #[doc = "74 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 74,
    #[doc = "75 - TIM8_CC"]
    TIM8_CC = 75,
    #[doc = "76 - COMP1"]
    COMP1 = 76,
    #[doc = "77 - DAC1"]
    DAC1 = 77,
    #[doc = "78 - LPDMA2_CH0"]
    LPDMA2_CH0 = 78,
    #[doc = "79 - LPDMA2_CH1"]
    LPDMA2_CH1 = 79,
    #[doc = "80 - LPDMA2_CH2"]
    LPDMA2_CH2 = 80,
    #[doc = "81 - LPDMA2_CH3"]
    LPDMA2_CH3 = 81,
    #[doc = "82 - LPDMA2_CH4"]
    LPDMA2_CH4 = 82,
    #[doc = "83 - LPDMA2_CH5"]
    LPDMA2_CH5 = 83,
    #[doc = "84 - LPDMA2_CH6"]
    LPDMA2_CH6 = 84,
    #[doc = "85 - LPDMA2_CH7"]
    LPDMA2_CH7 = 85,
    #[doc = "89 - TIM3"]
    TIM3 = 89,
    #[doc = "90 - TIM4"]
    TIM4 = 90,
    #[doc = "91 - XSPI1"]
    XSPI1 = 91,
    #[doc = "93 - PKA"]
    PKA = 93,
    #[doc = "96 - USART6"]
    USART6 = 96,
    #[doc = "97 - UART7"]
    UART7 = 97,
    #[doc = "98 - ADC3"]
    ADC3 = 98,
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
        fn PWR_PVD();
        fn RTC();
        fn TAMP();
        fn RAMCFG();
        fn FLASH();
        fn RCC();
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
        fn LPDMA1_CH0();
        fn LPDMA1_CH1();
        fn LPDMA1_CH2();
        fn LPDMA1_CH3();
        fn LPDMA1_CH4();
        fn LPDMA1_CH5();
        fn LPDMA1_CH6();
        fn LPDMA1_CH7();
        fn IWDG();
        fn ADC1();
        fn ADC2();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM5();
        fn TIM6();
        fn TIM7();
        fn I2C1_EV();
        fn I2C1_ERR();
        fn I3C1_EV();
        fn I3C1_ERR();
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
        fn TIM12();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn USB_DRD_FS();
        fn CRS();
        fn RNG();
        fn FPU();
        fn ICACHE();
        fn CORDIC();
        fn HASH();
        fn I2C2_EV();
        fn I2C2_ERR();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn COMP1();
        fn DAC1();
        fn LPDMA2_CH0();
        fn LPDMA2_CH1();
        fn LPDMA2_CH2();
        fn LPDMA2_CH3();
        fn LPDMA2_CH4();
        fn LPDMA2_CH5();
        fn LPDMA2_CH6();
        fn LPDMA2_CH7();
        fn TIM3();
        fn TIM4();
        fn XSPI1();
        fn PKA();
        fn USART6();
        fn UART7();
        fn ADC3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 99] = [
        Vector { _handler: WWDG },
        Vector { _handler: PWR_PVD },
        Vector { _handler: RTC },
        Vector { _handler: TAMP },
        Vector { _handler: RAMCFG },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
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
        Vector { _handler: LPDMA1_CH0 },
        Vector { _handler: LPDMA1_CH1 },
        Vector { _handler: LPDMA1_CH2 },
        Vector { _handler: LPDMA1_CH3 },
        Vector { _handler: LPDMA1_CH4 },
        Vector { _handler: LPDMA1_CH5 },
        Vector { _handler: LPDMA1_CH6 },
        Vector { _handler: LPDMA1_CH7 },
        Vector { _handler: IWDG },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ERR },
        Vector { _handler: I3C1_EV },
        Vector { _handler: I3C1_ERR },
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
        Vector { _handler: TIM12 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: USB_DRD_FS },
        Vector { _handler: CRS },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: ICACHE },
        Vector { _handler: CORDIC },
        Vector { _reserved: 0 },
        Vector { _handler: HASH },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ERR },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: COMP1 },
        Vector { _handler: DAC1 },
        Vector { _handler: LPDMA2_CH0 },
        Vector { _handler: LPDMA2_CH1 },
        Vector { _handler: LPDMA2_CH2 },
        Vector { _handler: LPDMA2_CH3 },
        Vector { _handler: LPDMA2_CH4 },
        Vector { _handler: LPDMA2_CH5 },
        Vector { _handler: LPDMA2_CH6 },
        Vector { _handler: LPDMA2_CH7 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: XSPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: PKA },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USART6 },
        Vector { _handler: UART7 },
        Vector { _handler: ADC3 },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x08ff_f800usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const WWDG: *mut () = 0x4000_2c00usize as _;
pub const IWDG: *mut () = 0x4000_3000usize as _;
pub const SPI2: *mut () = 0x4000_3800usize as _;
pub const SPI3: *mut () = 0x4000_3c00usize as _;
pub const COMP1: *mut () = 0x4000_4000usize as _;
pub const USART2: *mut () = 0x4000_4400usize as _;
pub const USART3: *mut () = 0x4000_4800usize as _;
pub const UART4: *mut () = 0x4000_4c00usize as _;
pub const UART5: *mut () = 0x4000_5000usize as _;
pub const I2C1: *mut () = 0x4000_5400usize as _;
pub const I2C2: *mut () = 0x4000_5800usize as _;
pub const I3C1: *mut () = 0x4000_5c00usize as _;
pub const CRS: *mut () = 0x4000_6000usize as _;
pub const USART6: *mut () = 0x4000_6400usize as _;
pub const UART7: *mut () = 0x4000_7800usize as _;
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: *mut () = 0x4001_3000usize as _;
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400usize as _) };
pub const USART1: *mut () = 0x4001_3800usize as _;
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const USBRAM: *mut () = 0x4001_6400usize as _;
pub const LPDMA1: lpdma::Lpdma = unsafe { lpdma::Lpdma::from_ptr(0x4002_0000usize as _) };
pub const LPDMA2: lpdma::Lpdma = unsafe { lpdma::Lpdma::from_ptr(0x4002_1000usize as _) };
pub const FLASH: *mut () = 0x4002_2000usize as _;
pub const CRC: *mut () = 0x4002_3000usize as _;
pub const CORDIC: cordic::Cordic = unsafe { cordic::Cordic::from_ptr(0x4002_3800usize as _) };
pub const RAMCFG: *mut () = 0x4002_6000usize as _;
pub const RAMCFG_SRAM1: *mut () = 0x4002_6000usize as _;
pub const RAMCFG_SRAM2: *mut () = 0x4002_6040usize as _;
pub const ETH1: *mut () = 0x4002_8000usize as _;
pub const ICACHE: *mut () = 0x4003_0400usize as _;
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00usize as _) };
pub const ADC1: *mut () = 0x4202_8000usize as _;
pub const ADC2: *mut () = 0x4202_8100usize as _;
pub const ADC12_COMMON: *mut () = 0x4202_8300usize as _;
pub const DAC1: *mut () = 0x4202_8400usize as _;
pub const ADC3: *mut () = 0x4202_d800usize as _;
pub const ADC3_COMMON: *mut () = 0x4202_db00usize as _;
pub const HASH: *mut () = 0x420c_0400usize as _;
pub const RNG: *mut () = 0x420c_0800usize as _;
pub const PKA: *mut () = 0x420c_2000usize as _;
pub const SYSCFG: *mut () = 0x4400_0400usize as _;
pub const LPUART1: *mut () = 0x4400_2400usize as _;
pub const LPTIM1: *mut () = 0x4400_4400usize as _;
pub const RTC: *mut () = 0x4400_7800usize as _;
pub const TAMP: *mut () = 0x4400_7c00usize as _;
pub const PWR: *mut () = 0x4402_0800usize as _;
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4402_0c00usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4402_2000usize as _) };
pub const DBGMCU: *mut () = 0x4402_4000usize as _;
pub const XSPI1: *mut () = 0x4700_1400usize as _;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/cordic_v1.rs"]
pub mod cordic;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/lpdma_v1.rs"]
pub mod lpdma;
#[path = "../../peripherals/rcc_c5.rs"]
pub mod rcc;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
