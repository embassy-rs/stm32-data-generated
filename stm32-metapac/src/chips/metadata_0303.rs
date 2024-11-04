
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g0",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "ADCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "ADCRST",
            }),
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
            PeripheralPin {
                pin: "PB2",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(5),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC1_COMMON",
        address: 0x40012708,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "v3",
            block: "ADC_COMMON",
            ir: &adccommon::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
                register: "AHBENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "CRCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "g0",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMA1RST",
            }),
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
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 0x40020400,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMA2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
            },
        ],
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40021800,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "g0",
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
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "g0",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "FLASHRST",
            }),
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
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x50000400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x50000800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x50000c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIODRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 0x50001000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 0x50001400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOFRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
                register: "CCIPR",
                field: "I2C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(11),
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
        name: "I2C2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SMBA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SMBA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(13),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_3",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_3",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 0x40008800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(63),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_3",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_3",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
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
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "g0",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "PWRRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "g0",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MCO_2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MCO_2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "LSCO",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MCO_1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MCO_2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_EN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC_EN",
                af: Some(1),
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
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "RTCAPBEN",
            }),
            reset: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP_IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TAMP_IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "TAMP",
            interrupt: "RTC_TAMP",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(17),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_MCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_MCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_MCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCK",
                af: Some(1),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(19),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2_3",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(67),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2_3",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "g0",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "SYSCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TAMP",
        address: 0x4000b000,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "g0",
            block: "TAMP",
            ir: &tamp::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "TIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM1RST",
            }),
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
                signal: "BK2",
                af: Some(5),
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
                signal: "BK",
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
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BK2",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(25),
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
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM14RST",
            }),
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
        name: "TIM15",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_2CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "TIM15SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM15EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM15RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "BK",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(43),
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
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM16EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM16RST",
            }),
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
                signal: "BK",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(46),
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
        address: 0x40014800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH_CMP",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM17EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM17RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "BK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BK",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(49),
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
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM3RST",
            }),
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
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(37),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3_TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(73),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3_TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3_TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(38),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6",
            },
        ],
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(39),
        }],
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
        name: "UID",
        address: 0x1fff7590,
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
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "USART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
                pin: "PA11",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CK",
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
                pin: "PA9",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DE",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(4),
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
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(51),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "USART2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART2RST",
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
                pin: "PA0",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CK",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(53),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "USART3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "NSS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(55),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3_4_5_6",
        }],
    },
    Peripheral {
        name: "USART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NSS",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(57),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3_4_5_6",
        }],
    },
    Peripheral {
        name: "USART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "NSS",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(75),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3_4_5_6",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 0x40013c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "NSS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(77),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3_4_5_6",
        }],
    },
    Peripheral {
        name: "USBRAM",
        address: 0x40009800,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "32_2048",
            block: "USBRAM",
            ir: &usbram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v2",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "WWDGEN",
            }),
            reset: None,
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
    Interrupt {
        name: "RTC_TAMP",
        number: 2,
    },
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
    Interrupt { name: "USB", number: 8 },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA1_CHANNEL2_3",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
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
        name: "TIM3_TIM4",
        number: 16,
    },
    Interrupt {
        name: "TIM6",
        number: 17,
    },
    Interrupt {
        name: "TIM7",
        number: 18,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
    },
    Interrupt {
        name: "TIM15",
        number: 20,
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
        name: "I2C2_3",
        number: 24,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "SPI2_3",
        number: 26,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
    Interrupt {
        name: "USART2",
        number: 28,
    },
    Interrupt {
        name: "USART3_4_5_6",
        number: 29,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
];
#[path = "../registers/adc_g0.rs"]
pub mod adc;
#[path = "../registers/adccommon_v3.rs"]
pub mod adccommon;
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/crc_v3.rs"]
pub mod crc;
#[path = "../registers/dbgmcu_g0.rs"]
pub mod dbgmcu;
#[path = "../registers/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../registers/exti_g0.rs"]
pub mod exti;
#[path = "../registers/flash_g0.rs"]
pub mod flash;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v2.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_g0.rs"]
pub mod pwr;
#[path = "../registers/rcc_g0.rs"]
pub mod rcc;
#[path = "../registers/rtc_v3.rs"]
pub mod rtc;
#[path = "../registers/spi_v2.rs"]
pub mod spi;
#[path = "../registers/syscfg_g0.rs"]
pub mod syscfg;
#[path = "../registers/tamp_g0.rs"]
pub mod tamp;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/usbram_32_2048.rs"]
pub mod usbram;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
