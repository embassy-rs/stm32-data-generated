#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    #[doc = "8 - EXTI2_TSC"]
    EXTI2_TSC = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC1_2"]
    ADC1_2 = 18,
    #[doc = "19 - USB_HP_CAN_TX"]
    USB_HP_CAN_TX = 19,
    #[doc = "20 - USB_LP_CAN_RX0"]
    USB_LP_CAN_RX0 = 20,
    #[doc = "21 - CAN_RX1"]
    CAN_RX1 = 21,
    #[doc = "22 - CAN_SCE"]
    CAN_SCE = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK_TIM15"]
    TIM1_BRK_TIM15 = 24,
    #[doc = "25 - TIM1_UP_TIM16"]
    TIM1_UP_TIM16 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM17"]
    TIM1_TRG_COM_TIM17 = 26,
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
    #[doc = "42 - USBWAKEUP"]
    USBWAKEUP = 42,
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - ADC3"]
    ADC3 = 47,
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
    #[doc = "56 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 56,
    #[doc = "57 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 57,
    #[doc = "58 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 58,
    #[doc = "59 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 59,
    #[doc = "60 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 60,
    #[doc = "61 - ADC4"]
    ADC4 = 61,
    #[doc = "64 - COMP1_2_3"]
    COMP1_2_3 = 64,
    #[doc = "65 - COMP4_5_6"]
    COMP4_5_6 = 65,
    #[doc = "66 - COMP7"]
    COMP7 = 66,
    #[doc = "74 - USB_HP"]
    USB_HP = 74,
    #[doc = "75 - USB_LP"]
    USB_LP = 75,
    #[doc = "76 - USBWAKEUP_RMP"]
    USBWAKEUP_RMP = 76,
    #[doc = "81 - FPU"]
    FPU = 81,
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
        fn EXTI2_TSC();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1_2();
        fn USB_HP_CAN_TX();
        fn USB_LP_CAN_RX0();
        fn CAN_RX1();
        fn CAN_SCE();
        fn EXTI9_5();
        fn TIM1_BRK_TIM15();
        fn TIM1_UP_TIM16();
        fn TIM1_TRG_COM_TIM17();
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
        fn USBWAKEUP();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn ADC3();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC();
        fn TIM7();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn DMA2_CHANNEL5();
        fn ADC4();
        fn COMP1_2_3();
        fn COMP4_5_6();
        fn COMP7();
        fn USB_HP();
        fn USB_LP();
        fn USBWAKEUP_RMP();
        fn FPU();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 82] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
        Vector { _handler: TAMP_STAMP },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2_TSC },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
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
        Vector { _handler: ADC1_2 },
        Vector {
            _handler: USB_HP_CAN_TX,
        },
        Vector {
            _handler: USB_LP_CAN_RX0,
        },
        Vector { _handler: CAN_RX1 },
        Vector { _handler: CAN_SCE },
        Vector { _handler: EXTI9_5 },
        Vector {
            _handler: TIM1_BRK_TIM15,
        },
        Vector {
            _handler: TIM1_UP_TIM16,
        },
        Vector {
            _handler: TIM1_TRG_COM_TIM17,
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
        Vector { _handler: USBWAKEUP },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector { _handler: ADC3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC },
        Vector { _handler: TIM7 },
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
        Vector { _handler: ADC4 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: COMP1_2_3 },
        Vector { _handler: COMP4_5_6 },
        Vector { _handler: COMP7 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USB_HP },
        Vector { _handler: USB_LP },
        Vector {
            _handler: USBWAKEUP_RMP,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FPU },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_f7acusize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x1fff_f7bausize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const UART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const UART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4000_5c00usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4000_6000usize as _) };
pub const CAN: can::Can = unsafe { can::Can::from_ptr(0x4000_6400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const COMP1: *mut () = 0x4001_001cusize as _;
pub const COMP2: *mut () = 0x4001_0020usize as _;
pub const COMP3: *mut () = 0x4001_0024usize as _;
pub const COMP4: *mut () = 0x4001_0028usize as _;
pub const COMP5: *mut () = 0x4001_002cusize as _;
pub const COMP6: *mut () = 0x4001_0030usize as _;
pub const COMP7: *mut () = 0x4001_0034usize as _;
pub const OPAMP1: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4001_0038usize as _) };
pub const OPAMP2: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4001_003cusize as _) };
pub const OPAMP3: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4001_0040usize as _) };
pub const OPAMP4: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4001_0044usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const TSC: tsc::Tsc = unsafe { tsc::Tsc::from_ptr(0x4002_4000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x5000_0000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x5000_0100usize as _) };
pub const ADC12_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x5000_0300usize as _) };
pub const ADC3: adc::Adc = unsafe { adc::Adc::from_ptr(0x5000_0400usize as _) };
pub const ADC4: adc::Adc = unsafe { adc::Adc::from_ptr(0x5000_0500usize as _) };
pub const ADC34_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x5000_0700usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1207959552 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_f3.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_f3.rs"]
pub mod adccommon;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/can_bxcan.rs"]
pub mod can;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v2.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_f3.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/exti_v1.rs"]
pub mod exti;
#[path = "../../peripherals/flash_f3.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/opamp_f3.rs"]
pub mod opamp;
#[path = "../../peripherals/pwr_f3.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f3v1.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v2f3.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v2.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_f3.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/tsc_v1.rs"]
pub mod tsc;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v3.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v1.rs"]
pub mod usb;
#[path = "../../peripherals/usbram_16x1_512.rs"]
pub mod usbram;
#[path = "../../peripherals/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 131072;
pub const WRITE_SIZE: usize = 8;
