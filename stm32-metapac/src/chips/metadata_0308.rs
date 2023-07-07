
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073816576,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "g0",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "ADCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "ADCRST",
            }),
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
            PeripheralPin {
                pin: "PB10",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN18",
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
            interrupt: "ADC1_COMP",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073817352,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "v3",
            block: "ADC_COMMON",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CEC",
        address: 1073772544,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "CECEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "CECRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP1",
        address: 1073807872,
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
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "ADC1_COMP",
        }],
    },
    Peripheral {
        name: "COMP2",
        address: 1073807876,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "OUT",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "ADC1_COMP",
        }],
    },
    Peripheral {
        name: "COMP3",
        address: 1073807880,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "ADC1_COMP",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v2",
            block: "CRC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "CRCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRS",
        address: 1073769472,
        registers: Some(PeripheralRegisters {
            kind: "crs",
            version: "v1",
            block: "CRS",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "CRSRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "DAC1RST",
            }),
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(9),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC_LPTIM1",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 1073829888,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "g0",
            block: "DBGMCU",
        }),
        rcc: None,
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMA1RST",
            }),
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
        address: 1073873920,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMA2RST",
            }),
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
        address: 1073874944,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 1073879040,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "g0",
            block: "EXTI",
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
        name: "FDCAN1",
        address: 1073767424,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan",
            block: "FDCAN",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "FDCANRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "TX",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "TIM17_FDCAN_IT1",
            },
        ],
    },
    Peripheral {
        name: "FDCAN2",
        address: 1073768448,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan",
            block: "FDCAN",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TX",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "TIM17_FDCAN_IT1",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "g0",
            block: "FLASH",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "FLASHRST",
            }),
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
        address: 1342177280,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "GPIO",
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOARST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1342178304,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "GPIO",
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOBRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1342179328,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "GPIO",
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 1342180352,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "GPIO",
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIODRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 1342181376,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 1342182400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "GPIO",
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOFRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
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
        address: 1073764352,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C2RST",
            }),
        }),
        pins: &[
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
                pin: "PB3",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
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
        address: 1073776640,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "I2C3RST",
            }),
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
            PeripheralPin {
                pin: "PC0",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
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
        address: 1073754112,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073773568,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "g0",
            block: "LPTIM",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "LPTIM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "IN1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "OUT",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC_LPTIM1",
        }],
    },
    Peripheral {
        name: "LPTIM2",
        address: 1073779712,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "g0",
            block: "LPTIM",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "LPTIM2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "OUT",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "OUT",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "OUT",
                af: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM7_LPTIM2",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 1073774592,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "LPUART1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(15),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3_4_5_6_LPUART1",
        }],
    },
    Peripheral {
        name: "LPUART2",
        address: 1073775616,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "LPUART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "LPUART2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "RTS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "TX",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(65),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2_LPUART2",
        }],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "g0",
            block: "PWR",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "PWRRST",
            }),
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
            version: "g0",
            block: "RCC",
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
                pin: "PA9",
                signal: "MCO",
                af: Some(0),
            },
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
            PeripheralPin {
                pin: "PD7",
                signal: "MCO_2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "MCO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_EN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MCO",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v3",
            block: "RTC",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP_IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP_IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
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
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "SPI1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MOSI",
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
                pin: "PA11",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
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
            PeripheralPin {
                pin: "PD5",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "I2S_MCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "I2S_CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "I2S_WS",
                af: Some(1),
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
        address: 1073756160,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "SPI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MISO",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
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
                pin: "PC2",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "NSS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "MISO",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "MOSI",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_MCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_MCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "I2S_MCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_MCK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "I2S_MCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SD",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "I2S_WS",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "I2S_CK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "I2S_MCK",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "I2S_SD",
                af: Some(1),
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
        address: 1073757184,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "SPI3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA15",
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
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(4),
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
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "g0",
            block: "SYSCFG",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "SYSCFGRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM1RST",
            }),
        }),
        pins: &[
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
                pin: "PB12",
                signal: "BK",
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
            PeripheralPin {
                pin: "PC8",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH3N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "BK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "BK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: Some(1),
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
        address: 1073750016,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM14RST",
            }),
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
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CH1",
                af: Some(2),
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
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM15EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM15RST",
            }),
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
                pin: "PB12",
                signal: "BK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "BK",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CH1N",
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
        address: 1073824768,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM16EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM16RST",
            }),
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
            PeripheralPin {
                pin: "PD0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CH1",
                af: Some(0),
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
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16_FDCAN_IT0",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16_FDCAN_IT0",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM17EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM17RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BK",
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
            PeripheralPin {
                pin: "PD1",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH1",
                af: Some(0),
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
                interrupt: "TIM17_FDCAN_IT1",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17_FDCAN_IT1",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17_FDCAN_IT1",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17_FDCAN_IT1",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17_FDCAN_IT1",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM2RST",
            }),
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
                pin: "PA5",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(31),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM3RST",
            }),
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
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE3",
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
        address: 1073743872,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM4RST",
            }),
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
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(2),
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
        address: 1073745920,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM6RST",
            }),
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
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC_LPTIM1",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM7RST",
            }),
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
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7_LPTIM2",
            },
        ],
    },
    Peripheral {
        name: "UCPD1",
        address: 1073782784,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "UCPD1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "UCPD1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DBCC1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DBCC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "FRSTX1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "FRSTX2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "FRSTX1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "FRSTX2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "FRSTX1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "FRSTX2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(59),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB_UCPD1_2",
        }],
    },
    Peripheral {
        name: "UCPD2",
        address: 1073783808,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "UCPD2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "UCPD2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "FRSTX1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "FRSTX2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FRSTX1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FRSTX2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "FRSTX1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "FRSTX2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "FRSTX1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "FRSTX2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DBCC1",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "DBCC2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(61),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB_UCPD1_2",
        }],
    },
    Peripheral {
        name: "UID",
        address: 536835472,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "USART1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
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
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC5",
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
        address: 1073759232,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART2RST",
            }),
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
                pin: "PD3",
                signal: "CTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD6",
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
            interrupt: "USART2_LPUART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 1073760256,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART3RST",
            }),
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "NSS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "DE",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "NSS",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CK",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DE",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: Some(0),
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
            interrupt: "USART3_4_5_6_LPUART1",
        }],
    },
    Peripheral {
        name: "USART4",
        address: 1073761280,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART4RST",
            }),
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
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE9",
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
            interrupt: "USART3_4_5_6_LPUART1",
        }],
    },
    Peripheral {
        name: "USART5",
        address: 1073762304,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART5RST",
            }),
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
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NSS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "CK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "TX",
                af: Some(3),
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
            interrupt: "USART3_4_5_6_LPUART1",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 1073822720,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USART6RST",
            }),
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
            PeripheralPin {
                pin: "PB14",
                signal: "CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
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
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
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
            interrupt: "USART3_4_5_6_LPUART1",
        }],
    },
    Peripheral {
        name: "USB",
        address: 1073765376,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v4",
            block: "USB",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "USBRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NOE",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "NOE",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NOE",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "NOE",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_UCPD1_2",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_UCPD1_2",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB_UCPD1_2",
            },
        ],
    },
    Peripheral {
        name: "USBRAM",
        address: 1073780736,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "32_2048",
            block: "USBRAM",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "WWDGEN",
            }),
            reset: None,
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
const INTERRUPTS: &'static [Interrupt] = &[
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
    Interrupt {
        name: "RCC_CRS",
        number: 4,
    },
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
        name: "USB_UCPD1_2",
        number: 8,
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
        name: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX1_OVR",
        number: 11,
    },
    Interrupt {
        name: "ADC1_COMP",
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
        name: "TIM2",
        number: 15,
    },
    Interrupt {
        name: "TIM3_TIM4",
        number: 16,
    },
    Interrupt {
        name: "TIM6_DAC_LPTIM1",
        number: 17,
    },
    Interrupt {
        name: "TIM7_LPTIM2",
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
        name: "TIM16_FDCAN_IT0",
        number: 21,
    },
    Interrupt {
        name: "TIM17_FDCAN_IT1",
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
        name: "USART2_LPUART2",
        number: 28,
    },
    Interrupt {
        name: "USART3_4_5_6_LPUART1",
        number: 29,
    },
    Interrupt {
        name: "CEC",
        number: 30,
    },
];
const DMA_CHANNELS: &'static [DmaChannel] = &[
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
