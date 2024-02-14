
pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 1073816576,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v1",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "adcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "adcrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "ADC",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073817352,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v2",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "crcen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 1073829888,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f0",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "dbgmcuen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "dbgmcurst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "dmaen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4_5",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL4_5",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1073808384,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "v1",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI4_15",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f0",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "flashen",
            }),
            reset: None,
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 1207959552,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "gpioaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpioarst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1207960576,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "gpioben",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpiobrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1207961600,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "gpiocen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpiocrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 1207962624,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "gpioden",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpiodrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 1207964672,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "gpiofen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpiofrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 1073763328,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "i2c1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "i2c1sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SCL",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SDA",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "f0x0",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "pwren",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "pwrrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1073876992,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "f0v1",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v2f0",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT_ALARM",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT_CALIB",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "spi1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "spi1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "f0",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "syscfgen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "syscfgrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 1073818624,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
        ],
    },
    Peripheral {
        name: "TIM14",
        address: 1073750016,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim14en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim14rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM14",
            },
        ],
    },
    Peripheral {
        name: "TIM16",
        address: 1073824768,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim16en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim16rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16",
            },
        ],
    },
    Peripheral {
        name: "TIM17",
        address: 1073825792,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim17en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim17rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 1073742848,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 536868780,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
            ir: &uid::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 1073821696,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "usart1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "usart1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "usart1sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v1",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "wwdgen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "wwdgrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
    },
];
pub(crate) static INTERRUPTS: &'static [Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt { name: "RTC", number: 2 },
    Interrupt {
        name: "FLASH",
        number: 3,
    },
    Interrupt { name: "RCC", number: 4 },
    Interrupt {
        name: "EXTI0_1",
        number: 5,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 6,
    },
    Interrupt {
        name: "EXTI4_15",
        number: 7,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA1_CHANNEL2_3",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL4_5",
        number: 11,
    },
    Interrupt {
        name: "ADC1",
        number: 12,
    },
    Interrupt {
        name: "TIM1_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 14,
    },
    Interrupt {
        name: "TIM3",
        number: 16,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
    },
    Interrupt {
        name: "TIM16",
        number: 21,
    },
    Interrupt {
        name: "TIM17",
        number: 22,
    },
    Interrupt {
        name: "I2C1",
        number: 23,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
];
pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_v1.rs"]
pub mod adc;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/crc_v2.rs"]
pub mod crc;
#[path = "../registers/dbgmcu_f0.rs"]
pub mod dbgmcu;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/flash_f0.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_f0x0.rs"]
pub mod pwr;
#[path = "../registers/rcc_f0v1.rs"]
pub mod rcc;
#[path = "../registers/rtc_v2f0.rs"]
pub mod rtc;
#[path = "../registers/spi_v2.rs"]
pub mod spi;
#[path = "../registers/syscfg_f0.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v3.rs"]
pub mod usart;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;
