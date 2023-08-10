
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1342177280,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN5",
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
                pin: "PF0",
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
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1342177536,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                pin: "PA4",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN13",
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
                pin: "PB11",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN15",
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
                pin: "PF1",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(36),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 1342178304,
        registers: None,
        rcc: None,
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
                signal: "IN4",
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
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(37),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "ADC4",
        address: 1342178560,
        registers: None,
        rcc: None,
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC4",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(38),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC4",
        }],
    },
    Peripheral {
        name: "ADC5",
        address: 1342178816,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "IN2",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC5",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(39),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC5",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1342178048,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AES",
        address: 1342570496,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "AESEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "AESRST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(92),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
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
                pin: "PB1",
                signal: "INP",
                af: None,
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
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
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
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP",
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
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
        }],
    },
    Peripheral {
        name: "COMP3",
        address: 1073807880,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "OUT",
                af: Some(3),
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
            PeripheralPin {
                pin: "PF1",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1_2_3",
        }],
    },
    Peripheral {
        name: "COMP4",
        address: 1073807884,
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
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "OUT",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT",
                af: Some(8),
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
            interrupt: "COMP4_5_6",
        }],
    },
    Peripheral {
        name: "COMP5",
        address: 1073807888,
        registers: None,
        rcc: None,
        pins: &[
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
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP4_5_6",
        }],
    },
    Peripheral {
        name: "COMP6",
        address: 1073807892,
        registers: None,
        rcc: None,
        pins: &[
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
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP4_5_6",
        }],
    },
    Peripheral {
        name: "COMP7",
        address: 1073807896,
        registers: None,
        rcc: None,
        pins: &[
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
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP7",
        }],
    },
    Peripheral {
        name: "CORDIC",
        address: 1073875968,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CORDICEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CORDICRST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(113),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CORDIC",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v3",
            block: "CRC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CRCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRS",
        address: 1073750016,
        registers: Some(PeripheralRegisters {
            kind: "crs",
            version: "v1",
            block: "CRS",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "CRSRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SYNC",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SYNC",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC1",
        address: 1342179328,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                signal: "CHANNEL1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CHANNEL2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(7),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
    },
    Peripheral {
        name: "DAC2",
        address: 1342180352,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC2RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PA6",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CHANNEL1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(41),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM7_DAC",
        }],
    },
    Peripheral {
        name: "DAC3",
        address: 1342181376,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC3RST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CHANNEL1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "CHANNEL2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(103),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
    },
    Peripheral {
        name: "DAC4",
        address: 1342182400,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC4RST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CHANNEL1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(104),
            },
            PeripheralDmaChannel {
                signal: "CHANNEL2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(105),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM7_DAC",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "g4",
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
                register: "AHB1ENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA1_CHANNEL8",
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
                register: "AHB1ENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMA2RST",
            }),
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
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA2_CHANNEL8",
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
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DMAMUX1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMAMUX1RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "OVR",
            interrupt: "DMAMUX_OVR",
        }],
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
                register: "APB1ENR1",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "FDCANRST",
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
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "FDCAN1_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "FDCAN1_IT1",
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
                pin: "PB5",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "FDCAN2_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "FDCAN2_IT1",
            },
        ],
    },
    Peripheral {
        name: "FDCAN3",
        address: 1073769472,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan",
            block: "FDCAN",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "IT0",
                interrupt: "FDCAN3_IT0",
            },
            PeripheralInterrupt {
                signal: "IT1",
                interrupt: "FDCAN3_IT1",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "g4",
            block: "FLASH",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FLASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
        name: "FMAC",
        address: 1073878016,
        registers: Some(PeripheralRegisters {
            kind: "fmac",
            version: "v1",
            block: "FMAC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FMACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "FMACRST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(110),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(111),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMAC",
        }],
    },
    Peripheral {
        name: "FMC",
        address: 1610612736,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB3",
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "FMCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "FMCRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DA3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NCE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DA14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CLE",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "ALE",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "DA5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DA6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DA7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "DA8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "A2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "A3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "A4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "A5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "A1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "A24",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "A25",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "A0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "NE4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "A6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "A7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "A8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "A9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "A10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "A11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "A12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "A13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "A14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "A15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "INT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NCE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NE2",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMC",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOFRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOG",
        address: 1207965696,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOGRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HRTIM1",
        address: 1073833984,
        registers: Some(PeripheralRegisters {
            kind: "hrtim",
            version: "v2",
            block: "HRTIM",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "HRTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "HRTIM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CHA1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CHA2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CHB1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CHB2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FLT1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "FLT2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FLT5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SCOUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCIN",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "EEV9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCOUT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "EEV7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "EEV6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "EEV4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "EEV3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "EEV8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "EEV5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "FLT3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "FLT4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CHC1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CHC2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CHD1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CHD2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "EEV10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CHF1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "EEV10",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CHF2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "FLT5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CHE1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CHE2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "FLT6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "EEV2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "EEV1",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "M",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "C",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "D",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "E",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "F",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(101),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT",
                interrupt: "HRTIM1_FLT",
            },
            PeripheralInterrupt {
                signal: "MASTER",
                interrupt: "HRTIM1_MASTER",
            },
            PeripheralInterrupt {
                signal: "TIMA",
                interrupt: "HRTIM1_TIMA",
            },
            PeripheralInterrupt {
                signal: "TIMB",
                interrupt: "HRTIM1_TIMB",
            },
            PeripheralInterrupt {
                signal: "TIMC",
                interrupt: "HRTIM1_TIMC",
            },
            PeripheralInterrupt {
                signal: "TIMD",
                interrupt: "HRTIM1_TIMD",
            },
            PeripheralInterrupt {
                signal: "TIME",
                interrupt: "HRTIM1_TIME",
            },
            PeripheralInterrupt {
                signal: "TIMF",
                interrupt: "HRTIM1_TIMF",
            },
        ],
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
                register: "APB1ENR1",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                af: Some(4),
            },
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
                register: "APB1ENR1",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "SMBA",
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
        name: "I2C3",
        address: 1073772544,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "I2C3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(21),
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
        name: "I2C4",
        address: 1073775616,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "I2C4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SCL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SDA",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(23),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C4_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C4_EV",
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
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "LPTIM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
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
                register: "APB1ENR2",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "LPUART1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG8",
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
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(35),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "OPAMP1",
        address: 1073808128,
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
                signal: "VINM0",
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
                signal: "VINM1",
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
        address: 1073808132,
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
                signal: "VINM0",
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
                signal: "VINM1",
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
        address: 1073808136,
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
                signal: "VINM0",
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
                signal: "VINM1",
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
        address: 1073808140,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM0",
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
                signal: "VINM1",
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
        name: "OPAMP5",
        address: 1073808144,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "VOUT",
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
                pin: "PB15",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "VINP_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "VINP_SEC",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP6",
        address: 1073808148,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM_SEC",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VINP_SEC",
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
                pin: "PD9",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
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
            version: "g4",
            block: "PWR",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
            version: "g4",
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
            PeripheralPin {
                pin: "PG10",
                signal: "MCO",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "CRS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "LSECSS",
                interrupt: "RTC_TAMP_LSECSS",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1342572544,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v1",
            block: "RNG",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "RNGRST",
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
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
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
                signal: "TAMP",
                interrupt: "RTC_TAMP_LSECSS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SAI1",
        address: 1073828864,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4",
            block: "SAI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MCLK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FS_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SD_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MCLK_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SD_B",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FS_B",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MCLK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FS_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK_A",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CK2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD_A",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "SD_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "SCK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "MCLK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD_B",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS_B",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "MCLK_A",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(109),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI1",
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
            PeripheralPin {
                pin: "PG2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG5",
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
                register: "APB1ENR1",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "SPI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MOSI",
                af: Some(5),
            },
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
                pin: "PF0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCK",
                af: Some(5),
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
                pin: "PA11",
                signal: "I2S_SD",
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
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "I2S_CK",
                af: Some(5),
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
                register: "APB1ENR1",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                pin: "PG9",
                signal: "SCK",
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
                pin: "PC12",
                signal: "I2S_SD",
                af: Some(6),
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
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SPI4",
        address: 1073822720,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(107),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "g4",
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
                pin: "PB9",
                signal: "CH3N",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
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
                pin: "PC0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "BKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH4N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(2),
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
                pin: "PE15",
                signal: "CH4N",
                af: Some(6),
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "CH4",
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
            PeripheralDmaChannel {
                signal: "TRIG",
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
                pin: "PC5",
                signal: "BKIN",
                af: Some(2),
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
            PeripheralPin {
                pin: "PG9",
                signal: "CH1N",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(81),
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(83),
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(85),
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
                register: "APB1ENR1",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                af: Some(14),
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
                af: Some(2),
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
                af: Some(14),
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
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(60),
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
        name: "TIM20",
        address: 1073827840,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM20EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM20RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CH4N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH4",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH3N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "BKIN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "CH1N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "CH2N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CH3N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "CH4N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BKIN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "BKIN",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(94),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM20_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM20_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM20_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM20_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM20_UP",
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
                register: "APB1ENR1",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(66),
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
                register: "APB1ENR1",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(71),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
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
                pin: "PB2",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH4",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(77),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM6RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(8),
        }],
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
                register: "APB1ENR1",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "TIM7RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(9),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7_DAC",
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
                pin: "PC13",
                signal: "CH4N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CH4N",
                af: Some(6),
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
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(55),
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
        name: "UART4",
        address: 1073761280,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: Some(14),
            },
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(31),
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
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "UART5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CTS",
                af: Some(14),
            },
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(33),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UCPD1",
        address: 1073782784,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR2",
                field: "UCPD1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR2",
                field: "UCPD1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "FRSTX2",
                af: Some(14),
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
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FRSTX2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "FRSTX1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "FRSTX2",
                af: Some(14),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(115),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
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
                pin: "PA11",
                signal: "NSS",
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
            PeripheralPin {
                pin: "PG9",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(25),
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
                register: "APB1ENR1",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                pin: "PA0",
                signal: "NSS",
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
                pin: "PD3",
                signal: "NSS",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(27),
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
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
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
                pin: "PA13",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
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
                pin: "PB13",
                signal: "NSS",
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
                pin: "PD11",
                signal: "NSS",
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
                signal: "RTS",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(29),
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
            version: "v3",
            block: "USB",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR1",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR1",
                field: "USBRST",
            }),
        }),
        pins: &[
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
            version: "16x2_1024",
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
                register: "APB1ENR1",
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
        name: "PVD_PVM",
        number: 1,
    },
    Interrupt {
        name: "RTC_TAMP_LSECSS",
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
        name: "EXTI2",
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
        name: "USB_HP",
        number: 19,
    },
    Interrupt {
        name: "USB_LP",
        number: 20,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 21,
    },
    Interrupt {
        name: "FDCAN1_IT1",
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
        name: "FMC",
        number: 48,
    },
    Interrupt {
        name: "LPTIM1",
        number: 49,
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
        name: "TIM7_DAC",
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
        name: "ADC5",
        number: 62,
    },
    Interrupt {
        name: "UCPD1",
        number: 63,
    },
    Interrupt {
        name: "COMP1_2_3",
        number: 64,
    },
    Interrupt {
        name: "COMP4_5_6",
        number: 65,
    },
    Interrupt {
        name: "COMP7",
        number: 66,
    },
    Interrupt {
        name: "HRTIM1_MASTER",
        number: 67,
    },
    Interrupt {
        name: "HRTIM1_TIMA",
        number: 68,
    },
    Interrupt {
        name: "HRTIM1_TIMB",
        number: 69,
    },
    Interrupt {
        name: "HRTIM1_TIMC",
        number: 70,
    },
    Interrupt {
        name: "HRTIM1_TIMD",
        number: 71,
    },
    Interrupt {
        name: "HRTIM1_TIME",
        number: 72,
    },
    Interrupt {
        name: "HRTIM1_FLT",
        number: 73,
    },
    Interrupt {
        name: "HRTIM1_TIMF",
        number: 74,
    },
    Interrupt {
        name: "CRS",
        number: 75,
    },
    Interrupt {
        name: "SAI1",
        number: 76,
    },
    Interrupt {
        name: "TIM20_BRK",
        number: 77,
    },
    Interrupt {
        name: "TIM20_UP",
        number: 78,
    },
    Interrupt {
        name: "TIM20_TRG_COM",
        number: 79,
    },
    Interrupt {
        name: "TIM20_CC",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 82,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 83,
    },
    Interrupt {
        name: "SPI4",
        number: 84,
    },
    Interrupt {
        name: "AES",
        number: 85,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 86,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 87,
    },
    Interrupt {
        name: "FDCAN3_IT0",
        number: 88,
    },
    Interrupt {
        name: "FDCAN3_IT1",
        number: 89,
    },
    Interrupt {
        name: "RNG",
        number: 90,
    },
    Interrupt {
        name: "LPUART1",
        number: 91,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 92,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 93,
    },
    Interrupt {
        name: "DMAMUX_OVR",
        number: 94,
    },
    Interrupt {
        name: "QUADSPI",
        number: 95,
    },
    Interrupt {
        name: "DMA1_CHANNEL8",
        number: 96,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 97,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 98,
    },
    Interrupt {
        name: "DMA2_CHANNEL8",
        number: 99,
    },
    Interrupt {
        name: "CORDIC",
        number: 100,
    },
    Interrupt {
        name: "FMAC",
        number: 101,
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
        name: "DMA1_CH8",
        dma: "DMA1",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(12),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(13),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(14),
    },
    DmaChannel {
        name: "DMA2_CH8",
        dma: "DMA2",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(15),
    },
];
