
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073816576,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f3_v2",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073816576,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN",
        address: 1073767424,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "canen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "canrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN_TX",
            },
        ],
    },
    Peripheral {
        name: "CEC",
        address: 1073772544,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "cecen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "cecrst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "cecsw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP1",
        address: 1073807388,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
    },
    Peripheral {
        name: "COMP2",
        address: 1073807390,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v3",
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
        name: "DAC1",
        address: 1073771520,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v2",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "dacen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "dacrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                af: None,
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
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC1",
        }],
    },
    Peripheral {
        name: "DAC2",
        address: 1073780736,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v2",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "dac2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "dac2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PA6",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM18_DAC2",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f3",
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
                field: "dma1en",
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
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073873920,
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
                field: "dma2en",
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
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
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
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
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
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f3",
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
        name: "GPIOE",
        address: 1207963648,
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
                field: "gpioeen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "gpioerst",
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
                pin: "PA14",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
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
        name: "I2C2",
        address: 1073764352,
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
                field: "i2c2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "i2c2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "i2c2sw",
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
                pin: "PA8",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
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
            version: "f3",
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
            version: "f37",
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
            version: "v2f3",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REFIN",
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
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMP_STAMP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SDADC1",
        address: 1073831936,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sdadc1en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sdadc1rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "AIN6P",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "AIN5P",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "AIN6M",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "AIN4P",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "AIN8P",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "AIN7P",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "AIN8M",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDADC1",
            channel: Some("DMA2_CH3"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDADC1",
        }],
    },
    Peripheral {
        name: "SDADC2",
        address: 1073832960,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sdadc2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sdadc2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "AIN6P",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "AIN8P",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "AIN7P",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "AIN8M",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDADC2",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDADC2",
        }],
    },
    Peripheral {
        name: "SDADC3",
        address: 1073833984,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "pclk2",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "sdadc3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "sdadc3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "AIN8P",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "AIN7P",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "AIN8M",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "AIN6P",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDADC3",
            channel: Some("DMA2_CH5"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDADC3",
        }],
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
                pin: "PA11",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "MOSI",
                af: Some(5),
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
        name: "SPI2",
        address: 1073756160,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "spi2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "spi2rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 1073757184,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "spi3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "spi3rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "f3",
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
        name: "TIM12",
        address: 1073747968,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim12en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim12rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM12",
            },
        ],
    },
    Peripheral {
        name: "TIM13",
        address: 1073748992,
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
                field: "tim13en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim13rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM13",
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
                pin: "PA10",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(9),
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
        name: "TIM15",
        address: 1073823744,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_2CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk2_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb2enr",
                field: "tim15en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim15rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "tim15sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1N",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
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
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM15",
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
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "tim16sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(1),
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
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
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
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "tim17sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(1),
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
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
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
        name: "TIM18",
        address: 1073781760,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim18en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim18rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM18_DAC2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM18_DAC2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM18_DAC2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM18_DAC2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM18_DAC2",
            },
        ],
    },
    Peripheral {
        name: "TIM19",
        address: 1073830912,
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
                field: "tim19en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb2rstr",
                field: "tim19rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ETR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(11),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
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
                signal: "CH2",
                channel: Some("DMA1_CH3"),
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
                interrupt: "TIM19",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM19",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM19",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM19",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM19",
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
                register: "apb1enr",
                field: "tim2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "tim2sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
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
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
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
                pin: "PA4",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4",
                af: Some(10),
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
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH6"),
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
        name: "TIM4",
        address: 1073743872,
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
                field: "tim4en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim4rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH4",
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
                signal: "CH2",
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
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM5",
        address: 1073744896,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim5en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim5rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 1073745920,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim6en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim6rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_DAC1",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC1",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC1",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC1",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC1",
            },
        ],
    },
    Peripheral {
        name: "TIM7",
        address: 1073746944,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1_tim",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "tim7en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "tim7rst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
    },
    Peripheral {
        name: "TSC",
        address: 1073889280,
        registers: Some(PeripheralRegisters {
            kind: "tsc",
            version: "v1",
            block: "TSC",
            ir: &tsc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "hclk1",
            enable: Some(PeripheralRccRegister {
                register: "ahbenr",
                field: "tscen",
            }),
            reset: Some(PeripheralRccRegister {
                register: "ahbrstr",
                field: "tscrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "G1_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "G1_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "G4_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "G4_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "G4_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SYNC",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "G1_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "G1_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "G2_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "G2_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G2_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "G4_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "G3_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G3_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G6_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G6_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "G5_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G5_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "G5_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "G5_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SYNC",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "G6_IO3",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "EXTI2_TSC",
        }],
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
                pin: "PA10",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[
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
        name: "USART2",
        address: 1073759232,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "usart2en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "usart2rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "usart2sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CK",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 1073760256,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "usart3en",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "usart3rst",
            }),
            mux: Some(PeripheralRccRegister {
                register: "cfgr3",
                field: "usart3sw",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RTS",
                af: Some(7),
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USB",
        address: 1073765376,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v1",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            clock: "pclk1",
            enable: Some(PeripheralRccRegister {
                register: "apb1enr",
                field: "usben",
            }),
            reset: Some(PeripheralRccRegister {
                register: "apb1rstr",
                field: "usbrst",
            }),
            mux: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_HP",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_LP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBWAKEUP",
            },
        ],
    },
    Peripheral {
        name: "USBRAM",
        address: 1073766400,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "16x2_512",
            block: "USBRAM",
            ir: &usbram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VREFINTCAL",
        address: 536868794,
        registers: Some(PeripheralRegisters {
            kind: "vrefintcal",
            version: "v1",
            block: "VREFINTCAL",
            ir: &vrefintcal::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt { name: "PVD", number: 1 },
    Interrupt {
        name: "TAMP_STAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 3,
    },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt { name: "RCC", number: 5 },
    Interrupt {
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2_TSC",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC1",
        number: 18,
    },
    Interrupt {
        name: "CAN_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM15",
        number: 24,
    },
    Interrupt {
        name: "TIM16",
        number: 25,
    },
    Interrupt {
        name: "TIM17",
        number: 26,
    },
    Interrupt {
        name: "TIM18_DAC2",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
        number: 30,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "USART3",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "CEC",
        number: 42,
    },
    Interrupt {
        name: "TIM12",
        number: 43,
    },
    Interrupt {
        name: "TIM13",
        number: 44,
    },
    Interrupt {
        name: "TIM14",
        number: 45,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "TIM6_DAC1",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 60,
    },
    Interrupt {
        name: "SDADC1",
        number: 61,
    },
    Interrupt {
        name: "SDADC2",
        number: 62,
    },
    Interrupt {
        name: "SDADC3",
        number: 63,
    },
    Interrupt {
        name: "COMP",
        number: 64,
    },
    Interrupt {
        name: "USB_HP",
        number: 74,
    },
    Interrupt {
        name: "USB_LP",
        number: 75,
    },
    Interrupt {
        name: "USBWAKEUP",
        number: 76,
    },
    Interrupt {
        name: "TIM19",
        number: 78,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
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
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_f3_v2.rs"]
pub mod adc;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/can_bxcan.rs"]
pub mod can;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/dac_v2.rs"]
pub mod dac;
#[path = "../registers/dbgmcu_f3.rs"]
pub mod dbgmcu;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/flash_f3.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_f3.rs"]
pub mod pwr;
#[path = "../registers/rcc_f37.rs"]
pub mod rcc;
#[path = "../registers/rtc_v2f3.rs"]
pub mod rtc;
#[path = "../registers/spi_v2.rs"]
pub mod spi;
#[path = "../registers/syscfg_f3.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/tsc_v1.rs"]
pub mod tsc;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v3.rs"]
pub mod usart;
#[path = "../registers/usb_v1.rs"]
pub mod usb;
#[path = "../registers/usbram_16x2_512.rs"]
pub mod usbram;
#[path = "../registers/vrefintcal_v1.rs"]
pub mod vrefintcal;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;
