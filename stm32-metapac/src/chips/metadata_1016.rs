
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC4",
        address: 0x46021000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v2_wba",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "ADC4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "ADC4RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC4",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC4_COMMON",
        address: 0x46021308,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "AES",
        address: 0x420c0000,
        registers: Some(PeripheralRegisters {
            kind: "aes",
            version: "v3b",
            block: "AES",
            ir: &aes::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "AESEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "AESRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v3",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CRCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0044000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "wba",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DESIG",
        address: 0xbf8d500,
        registers: Some(PeripheralRegisters {
            kind: "desig",
            version: "wba",
            block: "DESIG",
            ir: &desig::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXTI",
        address: 0x46022000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "l5",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI11",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI12",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI13",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI14",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI6",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI7",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI8",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "v4_wba",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FLASHEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x42020000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOB",
        address: 0x42020400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOC",
        address: 0x42020800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOH",
        address: 0x42021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC",
        address: 0x40032400,
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "wba",
            block: "GTZC_TZSC",
            ir: &gtzc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC_MPCBB1",
        address: 0x40032c00,
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "wba",
            block: "MPCBB",
            ir: &gtzc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC_MPCBB2",
        address: 0x40033000,
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "wba",
            block: "MPCBB",
            ir: &gtzc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC_TZIC",
        address: 0x50032800,
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "wba",
            block: "TZIC",
            ir: &gtzc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "GTZC",
            },
            PeripheralInterrupt {
                signal: "GTZC",
                interrupt: "GTZC",
            },
            PeripheralInterrupt {
                signal: "ILA",
                interrupt: "GTZC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "GTZC_TZSC",
        address: 0x40032400,
        registers: Some(PeripheralRegisters {
            kind: "gtzc",
            version: "wba",
            block: "GTZC_TZSC",
            ir: &gtzc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HASH",
        address: 0x420c0400,
        registers: Some(PeripheralRegisters {
            kind: "hash",
            version: "v4",
            block: "HASH",
            ir: &hash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "I2C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C3",
        address: 0x46002800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "I2C3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB7RSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SMBA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "ICACHE",
        address: 0x40030400,
        registers: Some(PeripheralRegisters {
            kind: "icache",
            version: "v1_4crr",
            block: "ICACHE",
            ir: &icache::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ICACHE",
        }],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v3",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "IWDG",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPDMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "lpdma",
            version: "v1",
            block: "LPDMA",
            ir: &lpdma::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "LPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "LPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "LPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "LPDMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "LPDMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "LPDMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "LPDMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "LPDMA1_CHANNEL7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x46004400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "LPTIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB7RSTR",
                field: "LPTIM1RST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "IN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "ETR",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v2a",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "LPTIM2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "IN1",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x46002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "LPUART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB7RSTR",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "PKA",
        address: 0x420c2000,
        registers: Some(PeripheralRegisters {
            kind: "pka",
            version: "v1a",
            block: "PKA",
            ir: &pka::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "PKAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "PKARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PKA",
        }],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x46020800,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "wba",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "PWREN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "WKUP",
        }],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "ramcfg",
            version: "wba",
            block: "RAMCFG",
            ir: &ramcfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "RAMCFGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BKP",
                interrupt: "RAMCFG",
            },
            PeripheralInterrupt {
                signal: "ECC",
                interrupt: "RAMCFG",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x46020c00,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "wba",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: None,
            },
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
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "RCC_S",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "RCC",
                interrupt: "RCC_S",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "RCC_S",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x420c0800,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "RNGSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "RNGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
        afio: None,
    },
    Peripheral {
        name: "RTC",
        address: 0x46007800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3_u5",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "BDCR",
                field: "RTCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "RTCAPBEN",
            }),
            reset: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_S",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "RTC_S",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "RTC_S",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC_S",
            },
            PeripheralInterrupt {
                signal: "TIMESTAMP",
                interrupt: "RTC_S",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_S",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4_2pdm",
            block: "SAI",
            ir: &sai::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "SAI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK_A",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CK2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MCLK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SD_A",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "FS_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CK1",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D2",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI1",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x46002000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v6",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "SPI3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB7RSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MOSI",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "MISO",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x46000400,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "wba",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK7",
            kernel_clock: Clock("PCLK7"),
            enable: Some(PeripheralRccRegister {
                register: "APB7ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB7RSTR",
                field: "SYSCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TAMP",
        address: 0x46007c00,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "OUT1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TAMP",
        }],
        afio: None,
    },
    Peripheral {
        name: "TIM16",
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM16EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM16RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM17",
        address: 0x40014800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM17EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM17RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v2",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0xbf8d700,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
            ir: &uid::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "USART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "VREFINTCAL",
        address: 0xbf8d7a4,
        registers: Some(PeripheralRegisters {
            kind: "vrefintcal",
            version: "v2",
            block: "VREFINTCAL",
            ir: &vrefintcal::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt { name: "PVD", number: 1 },
    Interrupt { name: "RTC", number: 2 },
    Interrupt {
        name: "RTC_S",
        number: 3,
    },
    Interrupt {
        name: "TAMP",
        number: 4,
    },
    Interrupt {
        name: "RAMCFG",
        number: 5,
    },
    Interrupt {
        name: "FLASH",
        number: 6,
    },
    Interrupt {
        name: "FLASH_S",
        number: 7,
    },
    Interrupt {
        name: "GTZC",
        number: 8,
    },
    Interrupt { name: "RCC", number: 9 },
    Interrupt {
        name: "RCC_S",
        number: 10,
    },
    Interrupt {
        name: "EXTI0",
        number: 11,
    },
    Interrupt {
        name: "EXTI1",
        number: 12,
    },
    Interrupt {
        name: "EXTI2",
        number: 13,
    },
    Interrupt {
        name: "EXTI3",
        number: 14,
    },
    Interrupt {
        name: "EXTI4",
        number: 15,
    },
    Interrupt {
        name: "EXTI5",
        number: 16,
    },
    Interrupt {
        name: "EXTI6",
        number: 17,
    },
    Interrupt {
        name: "EXTI7",
        number: 18,
    },
    Interrupt {
        name: "EXTI8",
        number: 19,
    },
    Interrupt {
        name: "EXTI9",
        number: 20,
    },
    Interrupt {
        name: "EXTI10",
        number: 21,
    },
    Interrupt {
        name: "EXTI11",
        number: 22,
    },
    Interrupt {
        name: "EXTI12",
        number: 23,
    },
    Interrupt {
        name: "EXTI13",
        number: 24,
    },
    Interrupt {
        name: "EXTI14",
        number: 25,
    },
    Interrupt {
        name: "EXTI15",
        number: 26,
    },
    Interrupt {
        name: "IWDG",
        number: 27,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL0",
        number: 28,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL1",
        number: 29,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL2",
        number: 30,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL3",
        number: 31,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL4",
        number: 32,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL5",
        number: 33,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL6",
        number: 34,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL7",
        number: 35,
    },
    Interrupt {
        name: "TIM2",
        number: 40,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 41,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 42,
    },
    Interrupt {
        name: "USART1",
        number: 43,
    },
    Interrupt {
        name: "LPUART1",
        number: 44,
    },
    Interrupt {
        name: "LPTIM1",
        number: 45,
    },
    Interrupt {
        name: "LPTIM2",
        number: 46,
    },
    Interrupt {
        name: "TIM16",
        number: 47,
    },
    Interrupt {
        name: "TIM17",
        number: 48,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 49,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 50,
    },
    Interrupt {
        name: "SAI1",
        number: 51,
    },
    Interrupt {
        name: "AES",
        number: 52,
    },
    Interrupt {
        name: "RNG",
        number: 53,
    },
    Interrupt {
        name: "FPU",
        number: 54,
    },
    Interrupt {
        name: "HASH",
        number: 55,
    },
    Interrupt {
        name: "PKA",
        number: 56,
    },
    Interrupt {
        name: "SPI3",
        number: 57,
    },
    Interrupt {
        name: "ICACHE",
        number: 58,
    },
    Interrupt {
        name: "ADC4",
        number: 59,
    },
    Interrupt {
        name: "RADIO",
        number: 60,
    },
    Interrupt {
        name: "WKUP",
        number: 61,
    },
    Interrupt {
        name: "WKUP_S",
        number: 62,
    },
    Interrupt {
        name: "RCC_AUDIOSYNC",
        number: 63,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA0" },
    Pin { name: "PA1" },
    Pin { name: "PA2" },
    Pin { name: "PA5" },
    Pin { name: "PA6" },
    Pin { name: "PA7" },
    Pin { name: "PA8" },
    Pin { name: "PA9" },
    Pin { name: "PA10" },
    Pin { name: "PA11" },
    Pin { name: "PA12" },
    Pin { name: "PA13" },
    Pin { name: "PA14" },
    Pin { name: "PA15" },
    Pin { name: "PB2" },
    Pin { name: "PB3" },
    Pin { name: "PB4" },
    Pin { name: "PB5" },
    Pin { name: "PB6" },
    Pin { name: "PB8" },
    Pin { name: "PB9" },
    Pin { name: "PB12" },
    Pin { name: "PB15" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PH3" },
];
#[path = "../registers/adc_v2_wba.rs"]
pub mod adc;
#[path = "../registers/aes_v3b.rs"]
pub mod aes;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/dbgmcu_wba.rs"]
pub mod dbgmcu;
#[path = "../registers/desig_wba.rs"]
pub mod desig;
#[path = "../registers/exti_l5.rs"]
pub mod exti;
#[path = "../registers/flash_v4_wba.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/gtzc_wba.rs"]
pub mod gtzc;
#[path = "../registers/hash_v4.rs"]
pub mod hash;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/icache_v1_4crr.rs"]
pub mod icache;
#[path = "../registers/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../registers/lpdma_v1.rs"]
pub mod lpdma;
#[path = "../registers/lptim_v2a.rs"]
pub mod lptim;
#[path = "../registers/pka_v1a.rs"]
pub mod pka;
#[path = "../registers/pwr_wba.rs"]
pub mod pwr;
#[path = "../registers/ramcfg_wba.rs"]
pub mod ramcfg;
#[path = "../registers/rcc_wba.rs"]
pub mod rcc;
#[path = "../registers/rng_v1.rs"]
pub mod rng;
#[path = "../registers/rtc_v3_u5.rs"]
pub mod rtc;
#[path = "../registers/sai_v4_2pdm.rs"]
pub mod sai;
#[path = "../registers/spi_v6.rs"]
pub mod spi;
#[path = "../registers/syscfg_wba.rs"]
pub mod syscfg;
#[path = "../registers/timer_v2.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/vrefintcal_v2.rs"]
pub mod vrefintcal;
