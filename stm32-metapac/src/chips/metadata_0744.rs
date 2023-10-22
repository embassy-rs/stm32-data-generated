
pub(crate) static PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC4",
        address: 1174540288,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk4",
            enable: Some(PeripheralRccRegister {
                register: "ahb4enr",
                field: "adc4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb4rstr",
                field: "adc4rst",
            }),
            mux: None,
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
            PeripheralPin {
                pin: "PB9",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC4",
            channel: None,
            dmamux: None,
            dma: Some("GPDMA1"),
            request: Some(0),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC4",
        }],
    },
    Peripheral {
        name: "AES",
        address: 1108082688,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "aesen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "aesrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(42),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
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
                register: "ahb1enr",
                field: "crcen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "crcrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758374912,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "wba",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 1174544384,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "l5",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
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
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "wba",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "flashen",
            }),
            reset: None,
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "GPDMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "gpdma1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "gpdma1rst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 1107427328,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpioaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpioarst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1107428352,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpioben",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiobrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1107429376,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpiocen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiocrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 1107434496,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "gpiohen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "gpiohrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HASH",
        address: 1108083712,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "hashen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "hashrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "IN",
            channel: None,
            dmamux: None,
            dma: Some("GPDMA1"),
            request: Some(43),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
    },
    Peripheral {
        name: "HSEM",
        address: 1108089856,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "hsemen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "hsemrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HSEM",
        }],
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
                register: "apb1enr1",
                field: "i2c1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "i2c1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr1",
                field: "i2c1sel",
            }),
        }),
        pins: &[
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
                pin: "PB3",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(7),
            },
        ],
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
    },
    Peripheral {
        name: "I2C3",
        address: 1174415360,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "i2c3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb7rstr",
                field: "i2c3rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr3",
                field: "i2c3sel",
            }),
        }),
        pins: &[
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
                pin: "PB15",
                signal: "SMBA",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(10),
            },
        ],
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
    },
    Peripheral {
        name: "ICACHE",
        address: 1073939456,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ICACHE",
        }],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "IWDG",
        }],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1174422528,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "lptim1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb7rstr",
                field: "lptim1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr3",
                field: "lptim1sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: Some(0),
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(48),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
    },
    Peripheral {
        name: "LPTIM2",
        address: 1073779712,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "v1",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr2",
                field: "lptim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr2",
                field: "lptim2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr1",
                field: "lptim2sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(51),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 1174414336,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "lpuart1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb7rstr",
                field: "lpuart1rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr3",
                field: "lpuart1sel",
            }),
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
                pin: "PA2",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(16),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "PKA",
        address: 1108090880,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "pkaen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "pkarst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PKA",
        }],
    },
    Peripheral {
        name: "PWR",
        address: 1174538240,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "wba",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk4",
            enable: Some(PeripheralRccRegister {
                register: "ahb4enr",
                field: "pwren",
            }),
            reset: None,
            mux: None,
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
                pin: "PB8",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "WKUP8",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "WKUP",
        }],
    },
    Peripheral {
        name: "RCC",
        address: 1174539264,
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "RNG",
        address: 1108084736,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "rngen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "rngrst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr2",
                field: "rngsel",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 1174435840,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3u5",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "rtcapben",
            }),
            reset: None,
            mux: None,
        }),
        pins: &[],
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
        name: "SAES",
        address: 1108085760,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk2",
            enable: Some(PeripheralRccRegister {
                register: "ahb2enr",
                field: "saesen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb2rstr",
                field: "saesrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(45),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAES",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5",
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
            mux: Some(PeripheralRccRegister {
                register: "ccipr1",
                field: "spi1sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(2),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 1174413312,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "spi3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb7rstr",
                field: "spi3rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr3",
                field: "spi3sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(6),
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1174406144,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "wba",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk7",
            enable: Some(PeripheralRccRegister {
                register: "apb7enr",
                field: "syscfgen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb7rstr",
                field: "syscfgrst",
            }),
            mux: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TAMP",
        address: 1174436864,
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
                signal: "OUT6",
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TAMP",
        }],
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
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3N",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(25),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
        ],
    },
    Peripheral {
        name: "TIM16",
        address: 1073824768,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(38),
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
            block: "TIM_GP16",
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(40),
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
        name: "TIM2",
        address: 1073741824,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "tim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "tim2rst",
            }),
            mux: None,
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(30),
            },
        ],
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
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
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
                register: "apb1enr1",
                field: "tim3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "tim3rst",
            }),
            mux: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(36),
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
        name: "TSC",
        address: 1073889280,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahb1enr",
                field: "tscen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahb1rstr",
                field: "tscrst",
            }),
            mux: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "G2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "G2_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "G3_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "G3_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "G4_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "G1_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "G1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "G1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "G3_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G3_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "G2_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "G2_IO3",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSC",
        }],
    },
    Peripheral {
        name: "UID",
        address: 200869632,
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
            version: "v4",
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
                register: "ccipr1",
                field: "usart1sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CK",
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
                pin: "PA7",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(12),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART2",
        address: 1073759232,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "usart2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr1",
                field: "usart2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "ccipr1",
                field: "usart2sel",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(14),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v2",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr1",
                field: "wwdgen",
            }),
            reset: None,
            mux: None,
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
        name: "SAES",
        number: 28,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL0",
        number: 29,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 30,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 31,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 32,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 33,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 34,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 35,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 36,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 37,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 38,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 39,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 40,
    },
    Interrupt {
        name: "TIM2",
        number: 41,
    },
    Interrupt {
        name: "TIM3",
        number: 42,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 43,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 44,
    },
    Interrupt {
        name: "SPI1",
        number: 45,
    },
    Interrupt {
        name: "USART1",
        number: 46,
    },
    Interrupt {
        name: "USART2",
        number: 47,
    },
    Interrupt {
        name: "LPUART1",
        number: 48,
    },
    Interrupt {
        name: "LPTIM1",
        number: 49,
    },
    Interrupt {
        name: "LPTIM2",
        number: 50,
    },
    Interrupt {
        name: "TIM16",
        number: 51,
    },
    Interrupt {
        name: "TIM17",
        number: 52,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 54,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 55,
    },
    Interrupt {
        name: "TSC",
        number: 57,
    },
    Interrupt {
        name: "AES",
        number: 58,
    },
    Interrupt {
        name: "RNG",
        number: 59,
    },
    Interrupt {
        name: "FPU",
        number: 60,
    },
    Interrupt {
        name: "HASH",
        number: 61,
    },
    Interrupt {
        name: "PKA",
        number: 62,
    },
    Interrupt {
        name: "SPI3",
        number: 63,
    },
    Interrupt {
        name: "ICACHE",
        number: 64,
    },
    Interrupt {
        name: "ADC4",
        number: 65,
    },
    Interrupt {
        name: "RADIO",
        number: 66,
    },
    Interrupt {
        name: "WKUP",
        number: 67,
    },
    Interrupt {
        name: "HSEM",
        number: 68,
    },
    Interrupt {
        name: "HSEM_S",
        number: 69,
    },
];
pub(crate) static DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "GPDMA1_CH0",
        dma: "GPDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH1",
        dma: "GPDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH2",
        dma: "GPDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH3",
        dma: "GPDMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH4",
        dma: "GPDMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH5",
        dma: "GPDMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH6",
        dma: "GPDMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH7",
        dma: "GPDMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/crc_v2.rs"]
pub mod crc;
#[path = "../registers/dbgmcu_wba.rs"]
pub mod dbgmcu;
#[path = "../registers/exti_l5.rs"]
pub mod exti;
#[path = "../registers/flash_wba.rs"]
pub mod flash;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/lptim_v1.rs"]
pub mod lptim;
#[path = "../registers/pwr_wba.rs"]
pub mod pwr;
#[path = "../registers/rcc_wba.rs"]
pub mod rcc;
#[path = "../registers/rng_v1.rs"]
pub mod rng;
#[path = "../registers/rtc_v3u5.rs"]
pub mod rtc;
#[path = "../registers/spi_v5.rs"]
pub mod spi;
#[path = "../registers/syscfg_wba.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
