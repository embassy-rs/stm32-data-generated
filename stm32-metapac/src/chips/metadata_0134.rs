
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1342177280,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f3",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "ADC12RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "IN5",
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
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1342177536,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f3",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "ADC12RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 1342178304,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f3",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "ADC34EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "ADC34RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "IN3",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC3",
            channel: Some("DMA2_CH5"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "ADC3_COMMON",
        address: 1342179072,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "f3",
            block: "ADC_COMMON",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC4",
        address: 1342178560,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f3",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "ADC34EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "ADC34RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "IN2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC4",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "ADC4",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC4",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1342178048,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "f3",
            block: "ADC_COMMON",
        }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "CANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "CANRST",
            }),
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
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "USB_LP_CAN_RX0",
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
                interrupt: "USB_HP_CAN_TX",
            },
        ],
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
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "OUT",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP2",
        address: 1073807392,
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
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP3",
        address: 1073807396,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP4",
        address: 1073807400,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM",
                af: None,
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
        interrupts: &[],
    },
    Peripheral {
        name: "COMP5",
        address: 1073807404,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP6",
        address: 1073807408,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "OUT",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP7",
        address: 1073807412,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "OUT",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP7",
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
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 1073771520,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v1",
            block: "DAC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "DACRST",
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
            interrupt: "TIM6_DAC",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f3",
            block: "DBGMCU",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "DBGMCUEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "DBGMCURST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA1EN",
            }),
            reset: None,
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA2EN",
            }),
            reset: None,
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: None,
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "GPIOARST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "GPIOBRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "GPIOCRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "GPIODRST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "GPIOERST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
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
                register: "APB1ENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C1RST",
            }),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C2RST",
            }),
        }),
        pins: &[
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
                pin: "PA10",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
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
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP1",
        address: 1073807416,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP2",
        address: 1073807420,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "VINP_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP3",
        address: 1073807424,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP4",
        address: 1073807428,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "VINP_SEC",
                af: None,
            },
        ],
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
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
            version: "f3",
            block: "RCC",
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
            PeripheralPin {
                pin: "PE6",
                signal: "TAMP3",
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
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_ext_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(6),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_ext_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "I2S_ext_SD",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SD",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BKIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "BKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "BKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CH3N",
                af: Some(6),
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
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM15EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM15RST",
            }),
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
                pin: "PF9",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CH2",
                af: Some(3),
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
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM15",
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
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM16EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM16RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(1),
            },
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
            PeripheralPin {
                pin: "PE0",
                signal: "CH1",
                af: Some(4),
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
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM16",
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
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM17EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM17RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(1),
            },
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
            PeripheralPin {
                pin: "PE1",
                signal: "CH1",
                af: Some(4),
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM17",
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
                register: "APB1ENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM2RST",
            }),
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
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH3",
                af: Some(2),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM3RST",
            }),
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
                pin: "PA7",
                signal: "CH2",
                af: Some(2),
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
                pin: "PB7",
                signal: "CH4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH4",
                af: Some(2),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(10),
            },
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
                register: "APB1ENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM6RST",
            }),
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
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC",
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
                register: "APB1ENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM7RST",
            }),
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
        name: "TIM8",
        address: 1073820672,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM8RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH3N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "BKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "BKIN",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH2"),
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
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP",
            },
        ],
    },
    Peripheral {
        name: "TSC",
        address: 1073889280,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "TSCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "TSCRST",
            }),
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
                pin: "PA7",
                signal: "G2_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "G4_IO1",
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
                pin: "PB0",
                signal: "G3_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G3_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "G3_IO4",
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
                pin: "PB10",
                signal: "SYNC",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G6_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G6_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G6_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G6_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "G3_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "G8_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "G8_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "G8_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "G8_IO4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "G7_IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "G7_IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "G7_IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "G7_IO4",
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
        name: "UART4",
        address: 1073761280,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 1073762304,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v3",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UID",
        address: 536868780,
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
            version: "v3",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART1RST",
            }),
        }),
        pins: &[
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
                pin: "PB6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE1",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART2RST",
            }),
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
                pin: "PA14",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
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
                pin: "PD3",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
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
                pin: "PC10",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "RX",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USBRST",
            }),
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
                interrupt: "USB_HP_CAN_TX",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_LP_CAN_RX0",
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
            version: "16x1_512",
            block: "USBRAM",
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "WWDGRST",
            }),
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
        name: "ADC1_2",
        number: 18,
    },
    Interrupt {
        name: "USB_HP_CAN_TX",
        number: 19,
    },
    Interrupt {
        name: "USB_LP_CAN_RX0",
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
        name: "TIM1_BRK_TIM15",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM16",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM17",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
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
        name: "USBWAKEUP",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "ADC3",
        number: 47,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6_DAC",
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
        name: "ADC4",
        number: 61,
    },
    Interrupt {
        name: "COMP7",
        number: 66,
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
        name: "USBWAKEUP_RMP",
        number: 76,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
];
const DMA_CHANNELS: &'static [DmaChannel] = &[
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
