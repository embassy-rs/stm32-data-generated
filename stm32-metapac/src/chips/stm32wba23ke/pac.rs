#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "1 - PVD"]
    PVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - RTC_S"]
    RTC_S = 3,
    #[doc = "4 - TAMP"]
    TAMP = 4,
    #[doc = "5 - RAMCFG"]
    RAMCFG = 5,
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
    #[doc = "27 - IWDG"]
    IWDG = 27,
    #[doc = "28 - LPDMA1_CHANNEL0"]
    LPDMA1_CHANNEL0 = 28,
    #[doc = "29 - LPDMA1_CHANNEL1"]
    LPDMA1_CHANNEL1 = 29,
    #[doc = "30 - LPDMA1_CHANNEL2"]
    LPDMA1_CHANNEL2 = 30,
    #[doc = "31 - LPDMA1_CHANNEL3"]
    LPDMA1_CHANNEL3 = 31,
    #[doc = "32 - LPDMA1_CHANNEL4"]
    LPDMA1_CHANNEL4 = 32,
    #[doc = "33 - LPDMA1_CHANNEL5"]
    LPDMA1_CHANNEL5 = 33,
    #[doc = "34 - LPDMA1_CHANNEL6"]
    LPDMA1_CHANNEL6 = 34,
    #[doc = "35 - LPDMA1_CHANNEL7"]
    LPDMA1_CHANNEL7 = 35,
    #[doc = "40 - TIM2"]
    TIM2 = 40,
    #[doc = "41 - I2C1_EV"]
    I2C1_EV = 41,
    #[doc = "42 - I2C1_ER"]
    I2C1_ER = 42,
    #[doc = "43 - USART1"]
    USART1 = 43,
    #[doc = "44 - LPUART1"]
    LPUART1 = 44,
    #[doc = "45 - LPTIM1"]
    LPTIM1 = 45,
    #[doc = "46 - LPTIM2"]
    LPTIM2 = 46,
    #[doc = "47 - TIM16"]
    TIM16 = 47,
    #[doc = "48 - TIM17"]
    TIM17 = 48,
    #[doc = "49 - I2C3_EV"]
    I2C3_EV = 49,
    #[doc = "50 - I2C3_ER"]
    I2C3_ER = 50,
    #[doc = "51 - SAI1"]
    SAI1 = 51,
    #[doc = "52 - AES"]
    AES = 52,
    #[doc = "53 - RNG"]
    RNG = 53,
    #[doc = "54 - FPU"]
    FPU = 54,
    #[doc = "55 - HASH"]
    HASH = 55,
    #[doc = "56 - PKA"]
    PKA = 56,
    #[doc = "57 - SPI3"]
    SPI3 = 57,
    #[doc = "58 - ICACHE"]
    ICACHE = 58,
    #[doc = "59 - ADC4"]
    ADC4 = 59,
    #[doc = "60 - RADIO"]
    RADIO = 60,
    #[doc = "61 - WKUP"]
    WKUP = 61,
    #[doc = "62 - WKUP_S"]
    WKUP_S = 62,
    #[doc = "63 - RCC_AUDIOSYNC"]
    RCC_AUDIOSYNC = 63,
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
        fn PVD();
        fn RTC();
        fn RTC_S();
        fn TAMP();
        fn RAMCFG();
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
        fn IWDG();
        fn LPDMA1_CHANNEL0();
        fn LPDMA1_CHANNEL1();
        fn LPDMA1_CHANNEL2();
        fn LPDMA1_CHANNEL3();
        fn LPDMA1_CHANNEL4();
        fn LPDMA1_CHANNEL5();
        fn LPDMA1_CHANNEL6();
        fn LPDMA1_CHANNEL7();
        fn TIM2();
        fn I2C1_EV();
        fn I2C1_ER();
        fn USART1();
        fn LPUART1();
        fn LPTIM1();
        fn LPTIM2();
        fn TIM16();
        fn TIM17();
        fn I2C3_EV();
        fn I2C3_ER();
        fn SAI1();
        fn AES();
        fn RNG();
        fn FPU();
        fn HASH();
        fn PKA();
        fn SPI3();
        fn ICACHE();
        fn ADC4();
        fn RADIO();
        fn WKUP();
        fn WKUP_S();
        fn RCC_AUDIOSYNC();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 64] = [
        Vector { _reserved: 0 },
        Vector { _handler: PVD },
        Vector { _handler: RTC },
        Vector { _handler: RTC_S },
        Vector { _handler: TAMP },
        Vector { _handler: RAMCFG },
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
        Vector { _handler: IWDG },
        Vector {
            _handler: LPDMA1_CHANNEL0,
        },
        Vector {
            _handler: LPDMA1_CHANNEL1,
        },
        Vector {
            _handler: LPDMA1_CHANNEL2,
        },
        Vector {
            _handler: LPDMA1_CHANNEL3,
        },
        Vector {
            _handler: LPDMA1_CHANNEL4,
        },
        Vector {
            _handler: LPDMA1_CHANNEL5,
        },
        Vector {
            _handler: LPDMA1_CHANNEL6,
        },
        Vector {
            _handler: LPDMA1_CHANNEL7,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM2 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: USART1 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: SAI1 },
        Vector { _handler: AES },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
        Vector { _handler: HASH },
        Vector { _handler: PKA },
        Vector { _handler: SPI3 },
        Vector { _handler: ICACHE },
        Vector { _handler: ADC4 },
        Vector { _handler: RADIO },
        Vector { _handler: WKUP },
        Vector { _handler: WKUP_S },
        Vector {
            _handler: RCC_AUDIOSYNC,
        },
    ];
}
pub const DESIG: desig::Desig = unsafe { desig::Desig::from_ptr(0x0bf8_d500usize as _) };
pub const UID: uid::Uid = unsafe { uid::Uid::from_ptr(0x0bf8_d700usize as _) };
pub const VREFINTCAL: vrefintcal::Vrefintcal = unsafe { vrefintcal::Vrefintcal::from_ptr(0x0bf8_d7a5usize as _) };
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_9400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4001_5400usize as _) };
pub const LPDMA1: lpdma::Lpdma = unsafe { lpdma::Lpdma::from_ptr(0x4002_0000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RAMCFG: ramcfg::Ramcfg = unsafe { ramcfg::Ramcfg::from_ptr(0x4002_6000usize as _) };
pub const ICACHE: icache::Icache = unsafe { icache::Icache::from_ptr(0x4003_0400usize as _) };
pub const GTZC: gtzc::GtzcTzsc = unsafe { gtzc::GtzcTzsc::from_ptr(0x4003_2400usize as _) };
pub const GTZC_TZSC: gtzc::GtzcTzsc = unsafe { gtzc::GtzcTzsc::from_ptr(0x4003_2400usize as _) };
pub const GTZC_MPCBB1: gtzc::Mpcbb = unsafe { gtzc::Mpcbb::from_ptr(0x4003_2c00usize as _) };
pub const GTZC_MPCBB2: gtzc::Mpcbb = unsafe { gtzc::Mpcbb::from_ptr(0x4003_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_0800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4202_1c00usize as _) };
pub const AES: aes::Aes = unsafe { aes::Aes::from_ptr(0x420c_0000usize as _) };
pub const HASH: hash::Hash = unsafe { hash::Hash::from_ptr(0x420c_0400usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x420c_0800usize as _) };
pub const PKA: pka::Pka = unsafe { pka::Pka::from_ptr(0x420c_2000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4600_0400usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4600_2000usize as _) };
pub const LPUART1: usart::Lpuart = unsafe { usart::Lpuart::from_ptr(0x4600_2400usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4600_2800usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4600_4400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4600_7800usize as _) };
pub const TAMP: *mut () = 0x4600_7c00usize as _;
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4602_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4602_0c00usize as _) };
pub const ADC4: adc::Adc = unsafe { adc::Adc::from_ptr(0x4602_1000usize as _) };
pub const ADC4_COMMON: *mut () = 0x4602_1308usize as _;
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4602_2000usize as _) };
pub const GTZC_TZIC: gtzc::Tzic = unsafe { gtzc::Tzic::from_ptr(0x5003_2800usize as _) };
pub const DBGMCU: dbgmcu::Dbgmcu = unsafe { dbgmcu::Dbgmcu::from_ptr(0xe004_4000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/adc_v2_wba.rs"]
pub mod adc;
#[path = "../../peripherals/aes_v3b.rs"]
pub mod aes;
#[path = "../../peripherals/crc_v3.rs"]
pub mod crc;
#[path = "../../peripherals/dbgmcu_wba.rs"]
pub mod dbgmcu;
#[path = "../../peripherals/desig_wba.rs"]
pub mod desig;
#[path = "../../peripherals/exti_l5.rs"]
pub mod exti;
#[path = "../../peripherals/flash_v4_wba.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/gtzc_wba.rs"]
pub mod gtzc;
#[path = "../../peripherals/hash_v4.rs"]
pub mod hash;
#[path = "../../peripherals/i2c_v2.rs"]
pub mod i2c;
#[path = "../../peripherals/icache_v1_4crr.rs"]
pub mod icache;
#[path = "../../peripherals/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../../peripherals/lpdma_v1.rs"]
pub mod lpdma;
#[path = "../../peripherals/lptim_v2a.rs"]
pub mod lptim;
#[path = "../../peripherals/pka_v1a.rs"]
pub mod pka;
#[path = "../../peripherals/pwr_wba.rs"]
pub mod pwr;
#[path = "../../peripherals/ramcfg_wba.rs"]
pub mod ramcfg;
#[path = "../../peripherals/rcc_wba.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v1.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_v3_u5.rs"]
pub mod rtc;
#[path = "../../peripherals/sai_v4_2pdm.rs"]
pub mod sai;
#[path = "../../peripherals/spi_v6.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_wba.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v2.rs"]
pub mod timer;
#[path = "../../peripherals/uid_v1.rs"]
pub mod uid;
#[path = "../../peripherals/usart_v4.rs"]
pub mod usart;
#[path = "../../peripherals/vrefintcal_v2.rs"]
pub mod vrefintcal;
