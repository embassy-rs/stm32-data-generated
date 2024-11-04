#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD_VDDIO2"]
    PVD_VDDIO2 = 1,
    #[doc = "2 - RTC_TAMP"]
    RTC_TAMP = 2,
    #[doc = "3 - FLASH"]
    FLASH = 3,
    #[doc = "4 - RCC_CRS"]
    RCC_CRS = 4,
    #[doc = "5 - EXTI0_1"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15"]
    EXTI4_15 = 7,
    #[doc = "8 - USB_UCPD1_2"]
    USB_UCPD1_2 = 8,
    #[doc = "9 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 9,
    #[doc = "10 - DMA1_CHANNEL2_3"]
    DMA1_CHANNEL2_3 = 10,
    #[doc = "11 - DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR"]
    DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR = 11,
    #[doc = "12 - ADC1_COMP"]
    ADC1_COMP = 12,
    #[doc = "13 - TIM1_BRK_UP_TRG_COM"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1_CC"]
    TIM1_CC = 14,
    #[doc = "15 - TIM2"]
    TIM2 = 15,
    #[doc = "16 - TIM3_TIM4"]
    TIM3_TIM4 = 16,
    #[doc = "17 - TIM6_DAC_LPTIM1"]
    TIM6_DAC_LPTIM1 = 17,
    #[doc = "18 - TIM7_LPTIM2"]
    TIM7_LPTIM2 = 18,
    #[doc = "19 - TIM14"]
    TIM14 = 19,
    #[doc = "20 - TIM15"]
    TIM15 = 20,
    #[doc = "21 - TIM16_FDCAN_IT0"]
    TIM16_FDCAN_IT0 = 21,
    #[doc = "22 - TIM17_FDCAN_IT1"]
    TIM17_FDCAN_IT1 = 22,
    #[doc = "23 - I2C1"]
    I2C1 = 23,
    #[doc = "24 - I2C2_3"]
    I2C2_3 = 24,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "26 - SPI2_3"]
    SPI2_3 = 26,
    #[doc = "27 - USART1"]
    USART1 = 27,
    #[doc = "28 - USART2_LPUART2"]
    USART2_LPUART2 = 28,
    #[doc = "29 - USART3_4_5_6_LPUART1"]
    USART3_4_5_6_LPUART1 = 29,
    #[doc = "30 - CEC"]
    CEC = 30,
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
        fn PVD_VDDIO2();
        fn RTC_TAMP();
        fn FLASH();
        fn RCC_CRS();
        fn EXTI0_1();
        fn EXTI2_3();
        fn EXTI4_15();
        fn USB_UCPD1_2();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2_3();
        fn DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR();
        fn ADC1_COMP();
        fn TIM1_BRK_UP_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3_TIM4();
        fn TIM6_DAC_LPTIM1();
        fn TIM7_LPTIM2();
        fn TIM14();
        fn TIM15();
        fn TIM16_FDCAN_IT0();
        fn TIM17_FDCAN_IT1();
        fn I2C1();
        fn I2C2_3();
        fn SPI1();
        fn SPI2_3();
        fn USART1();
        fn USART2_LPUART2();
        fn USART3_4_5_6_LPUART1();
        fn CEC();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 31] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD_VDDIO2 },
        Vector { _handler: RTC_TAMP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC_CRS },
        Vector { _handler: EXTI0_1 },
        Vector { _handler: EXTI2_3 },
        Vector { _handler: EXTI4_15 },
        Vector { _handler: USB_UCPD1_2 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2_3,
        },
        Vector {
            _handler: DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR,
        },
        Vector { _handler: ADC1_COMP },
        Vector {
            _handler: TIM1_BRK_UP_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3_TIM4 },
        Vector {
            _handler: TIM6_DAC_LPTIM1,
        },
        Vector { _handler: TIM7_LPTIM2 },
        Vector { _handler: TIM14 },
        Vector { _handler: TIM15 },
        Vector {
            _handler: TIM16_FDCAN_IT0,
        },
        Vector {
            _handler: TIM17_FDCAN_IT1,
        },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2_3 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2_3 },
        Vector { _handler: USART1 },
        Vector {
            _handler: USART2_LPUART2,
        },
        Vector {
            _handler: USART3_4_5_6_LPUART1,
        },
        Vector { _handler: CEC },
    ];
}
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x1fff_7590usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x4000_5c00usize as _) };
pub const FDCAN1: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_6400usize as _) };
pub const FDCAN2: can::Fdcan = unsafe { can::Fdcan::from_ptr(0x4000_6800usize as _) };
pub const CRS: crs::Crs = unsafe { crs::Crs::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const CEC: cec::Cec = unsafe { cec::Cec::from_ptr(0x4000_7800usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_7c00usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_8000usize as _) };
pub const LPUART2: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4000_8400usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_8800usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4000_9800usize as _) };
pub const UCPD1: ucpd::Ucpd = unsafe { ucpd::Ucpd::from_ptr(0x4000_a000usize as _) };
pub const UCPD2: ucpd::Ucpd = unsafe { ucpd::Ucpd::from_ptr(0x4000_a400usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0x4000_b000usize as _) };
pub const FDCANRAM1: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_b400usize as _) };
pub const FDCANRAM2: fdcanram::Fdcanram = unsafe { fdcanram::Fdcanram::from_ptr(0x4000_b750usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const COMP1: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_0200usize as _) };
pub const COMP2: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_0204usize as _) };
pub const COMP3: comp::Comp = unsafe { comp::Comp::from_ptr(0x4001_0208usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC1_COMMON: adccommon::AdcCommon = unsafe { adccommon::AdcCommon::from_ptr(0x4001_2708usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3c00usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0x4001_5800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4002_1800usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5000_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1342177280 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_g0.rs"]
pub mod adc;
#[path = "../../peripherals/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/can_fdcan_v1.rs"]
pub mod can;
#[path = "../../peripherals/cec_v2.rs"]
pub mod cec;
#[path = "../../peripherals/comp_v1.rs"]
pub mod comp;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/crs_v1.rs"]
pub mod crs;
#[path = "../../peripherals/dac_v4.rs"]
pub mod dac;
#[path = "../../peripherals/dbgmcu_g0.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../../peripherals/exti_g0.rs"]
pub mod exti;
#[path = "../../peripherals/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../../peripherals/flash_g0.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_v1b.rs"]
pub mod lptim;
#[path = "../../peripherals/pwr_g0.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_g0.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v3.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v2.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_g0.rs"]
pub mod syscfg;
#[path = "../../peripherals/tamp_g0.rs"]
pub mod tamp;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v4.rs"]
pub mod usb;
#[path = "../../peripherals/usbram_32_2048.rs"]
pub mod usbram;
#[path = "../../peripherals/wwdg_v2.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 131072;
pub const WRITE_SIZE: usize = 8;
