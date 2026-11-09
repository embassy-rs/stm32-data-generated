
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x42020000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INN8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PF15",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "INN9",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "INP10",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(0),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x42020300,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x42020100,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INN15",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INP17",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INP13",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INN13",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN6",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INN8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "INN9",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "INP10",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(1),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0x42020400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "ADCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADC3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADC3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "INN6",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "INN7",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "INN8",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "INN9",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "INP6",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "INN14",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INP2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(53),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC3_COMMON",
        address: 0x42020700,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "COMP1",
        address: 0x44008400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "OUT",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP1",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP2",
        address: 0x44008800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP2",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP3",
        address: 0x44008c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "OUT",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP3",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP4",
        address: 0x44009000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "INP2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP4",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP5",
        address: 0x44009400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "OUT",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP5",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP6",
        address: 0x44009800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "INM1",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "INP2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP6",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP7",
        address: 0x44009c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "OUT",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP7",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP8",
        address: 0x4400a000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "COMP8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "COMP8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "OUT",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INP1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "WKUP",
            interrupt: "COMP8",
        }],
        afio: None,
    },
    Peripheral {
        name: "CORDIC1",
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "cordic",
            version: "v1",
            block: "CORDIC",
            ir: &cordic::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CORDIC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CORDIC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(117),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(117),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CORDIC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: None,
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
        name: "CRS",
        address: 0x40006000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "CRSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SYNC",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x42021400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PA4",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(2),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC10",
        address: 0x42023800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC10SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC10RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(140),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(140),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC10",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC2",
        address: 0x42021800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[PeripheralPin {
            pin: "PA5",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(3),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC5",
        address: 0x42022400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC5SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(135),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(135),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC5",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC6",
        address: 0x42022800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC6SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(136),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(136),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC6",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC7",
        address: 0x42022c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC7SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(137),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(137),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC7",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC8",
        address: 0x42023000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC8SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(138),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(138),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC8",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC9",
        address: 0x42023400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "DAC9SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DAC9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DAC9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(139),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(139),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC9",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x44026000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DCACHE",
        address: 0x40031400,
        registers: Some(PeripheralRegisters {
            kind: "dcache",
            version: "v1",
            block: "DCACHE",
            ir: &dcache::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DCACHEEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCACHE",
        }],
        afio: None,
    },
    Peripheral {
        name: "DTS",
        address: 0x40008c00,
        registers: Some(PeripheralRegisters {
            kind: "dts",
            version: "v1",
            block: "DTS",
            ir: &dts::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "DTSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "DTSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DTS",
        }],
        afio: None,
    },
    Peripheral {
        name: "EXTI",
        address: 0x44022000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "u5",
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
        name: "FDCAN1",
        address: 0x4000a000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "FDCANRST",
            }),
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
                pin: "PA6",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "RX",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "FDCAN2",
        address: 0x4000a400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "FDCANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "FDCAN3",
        address: 0x4000a800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "FDCANSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "FDCANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM",
        address: 0x4000ac00,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM1",
        address: 0x4000ac00,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM2",
        address: 0x4000af50,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM3",
        address: 0x4000b2a0,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: None,
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
        name: "FMAC1",
        address: 0x40023c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "FMAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "FMAC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(119),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(119),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMAC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "FMAC2",
        address: 0x42024400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "FMAC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "FMAC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "RD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(129),
            },
            PeripheralDmaChannel {
                signal: "WR",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(129),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMAC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "GPDMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA1_CH0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA1_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA1_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA1_CH3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA1_CH4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA1_CH5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA1_CH6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA1_CH7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "GPDMA2",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPDMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA2_CH0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA2_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA2_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA2_CH3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA2_CH4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA2_CH5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA2_CH6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA2_CH7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x46000000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        address: 0x46000400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        address: 0x46000800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        name: "GPIOD",
        address: 0x46000c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIODRST",
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
        name: "GPIOE",
        address: 0x46001000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOERST",
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
        name: "GPIOF",
        address: 0x46001400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOFRST",
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
        name: "GPIOG",
        address: 0x46001800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOGRST",
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
        address: 0x46001c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        name: "GTZC1_MPCBB1",
        address: 0x40032c00,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC1_MPCBB2",
        address: 0x40033000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTZC1_MPCBB3",
        address: 0x40033400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HASH",
        address: 0x46006400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(113),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(113),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "HRTIM1",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "hrtim",
            version: "g5",
            block: "HRTIM",
            ir: &hrtim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "HRTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "HRTIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CHB2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FLT1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "EEV7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "FLT7",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "EEV6",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "EEV3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "EEV10",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "EEV4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "EEV9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FLT3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FLT5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "EEV6",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SCOUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "FLT3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "FLT4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FLT2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "FLT6",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "EEV1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "EEV2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "FLT6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCIN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "EEV11",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "FLT5",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "EEV15",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CHA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CHA2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CHB1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "EEV5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "EEV8",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "EEV5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CHC1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CHC2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "EEV14",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "FLT7",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "FLT8",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "EEV9",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "FLT1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "EEV10",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CHF1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "EEV10",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CHF2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "FLT5",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "EEV15",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "EEV16",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "EEV9",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SCOUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CHE1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CHE2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "FLT1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "EEV4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "EEV6",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "FLT6",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "EEV3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "FLT2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "EEV12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "EEV4",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "SCOUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "EEV16",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "EEV2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CHD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "CHD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "EEV7",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "EEV11",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "FLT1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "MASTER",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "MASTER",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "TIMA",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(38),
            },
            PeripheralDmaChannel {
                signal: "TIMA",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(38),
            },
            PeripheralDmaChannel {
                signal: "TIMB",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "TIMB",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "TIMC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "TIMC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "TIMD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TIMD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TIME",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TIME",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TIMF",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "TIMF",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(43),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT",
                interrupt: "HRTIM1_FLT",
            },
            PeripheralInterrupt {
                signal: "MASTER",
                interrupt: "HRTIM1_M",
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
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
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
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(13),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ERR",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(16),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ERR",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C3",
        address: 0x44002800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SCL",
                af: None,
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(19),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ERR",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C4",
        address: 0x44002c00,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C4SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "I2C4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SCL",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SDA",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SCL",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(126),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(126),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(127),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(127),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C4_ERR",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C4_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I3C1",
        address: 0x40005c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I3C1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I3C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I3C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(123),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(123),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(124),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(124),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(125),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(125),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I3C1_ERR",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I3C1_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "ICACHE",
        address: 0x40030400,
        registers: None,
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
        address: 0x40003400,
        registers: None,
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
        name: "LPTIM1",
        address: 0x44004400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "LPTIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ETR",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "IN2",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(107),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(107),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x44002400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "LPUART1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(3),
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
                pin: "PC12",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(46),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x44005c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "OPAMP1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "OPAMP1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OPAMP2",
        address: 0x44006000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "OPAMP2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "OPAMP2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "VINP3",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OPAMP3",
        address: 0x44006400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "OPAMP3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "OPAMP3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "VINP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "VINM0",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "PKA",
        address: 0x46008000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "PKAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        name: "PLAY1",
        address: 0x40011400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "PLAY1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "PLAY1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "PLAY1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "IN0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT12",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "IN12",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "OUT4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "OUT0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "OUT1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "OUT13",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "IN11",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "OUT11",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "IN5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "OUT6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "IN6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "IN1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "OUT4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "OUT9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "IN0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "IN9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "IN10",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PLAY1",
        }],
        afio: None,
    },
    Peripheral {
        name: "POSENC1",
        address: 0x44013800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "POSENC1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "POSENC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "POSENC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "MOSI",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SEL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "EN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "EN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "OUT",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "EN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "SEL",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PH1",
                signal: "SCK",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "POSENC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "POSENC2",
        address: 0x44017800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "POSENC2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "POSENC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "POSENC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "IN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "EN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MISO",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SEL",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SEL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "MOSI",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "IN",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MOSI",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "MISO",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "POSENC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x44020800,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CSLEEP",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "AVD",
                interrupt: "PWR_PVD_AVD",
            },
            PeripheralInterrupt {
                signal: "PVD",
                interrupt: "PWR_PVD_AVD",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG",
        address: 0x40026000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "RAMCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "RAMCFGRST",
            }),
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
        name: "RAMCFG_SRAM1",
        address: 0x40026000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG_SRAM2",
        address: 0x40026040,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMCFG_SRAM3",
        address: 0x40026080,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x44020c00,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "g5",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "MCO1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCO2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MCO2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "MCO1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "AUDIOCLK",
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
                pin: "PH0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "OSC_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        address: 0x46006800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
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
        address: 0x44007800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "RTCAPBEN",
            }),
            reset: None,
            stop_mode: StopMode::Standby,
        }),
        pins: &[
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
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "SPI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "MOSI",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "SCK",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(7),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "SPI2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_MCK",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RDY",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "SCK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "SCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(9),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v5_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "SPI3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MISO",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NSS",
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
                pin: "PC3",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(11),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x44000400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "SBSEN",
            }),
            reset: None,
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
        address: 0x44007c00,
        registers: None,
        rcc: None,
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
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
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
        name: "TIM1",
        address: 0x40012c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2N",
                af: Some(6),
            },
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
                signal: "CH3N",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH3N",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH3",
                af: Some(1),
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
                signal: "BKIN2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH4N",
                af: Some(6),
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
                pin: "PE7",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(64),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM10",
        address: 0x40001c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM10RST",
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
                pin: "PC8",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH1",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM10",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM11",
        address: 0x40002000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM11EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM11RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH1",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM11",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40002400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH2",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM12",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM13",
        address: 0x40002800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM13EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM13RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM13",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM14",
        address: 0x40002c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM14RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CH1",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM15",
        address: 0x40014000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM15EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM15RST",
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
                pin: "PA13",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: None,
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
                pin: "PA8",
                signal: "BKIN",
                af: Some(14),
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
                pin: "PC15",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "BKIN",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(100),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM16",
        address: 0x40014400,
        registers: None,
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
                pin: "PA12",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH1N",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(102),
            },
        ],
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
        registers: None,
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
                pin: "PA10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1N",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH1",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(104),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(104),
            },
        ],
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
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM2RST",
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
                pin: "PA1",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH1",
                af: Some(2),
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
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "ETR",
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
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH4",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(77),
            },
        ],
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
        name: "TIM20",
        address: 0x44000800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "TIM20EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "TIM20RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH2N",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CH3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "ETR",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "CH3N",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "BKIN2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CH4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH3",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "CH1N",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "BKIN",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CH4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BKIN",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "CH4N",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(141),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(141),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(142),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(142),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(143),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(143),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(144),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(144),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(145),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(145),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(146),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(146),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(147),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(147),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM20_BRK_TERR_IERR",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM20_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM20_TRGI_COM_DIR_IDX",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM20_TRGI_COM_DIR_IDX",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM20_BRK_TERR_IERR",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM20_TRGI_COM_DIR_IDX",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM20_TRGI_COM_DIR_IDX",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM20_UPD",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM26",
        address: 0x44000c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "TIM26EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "TIM26RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(155),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(155),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM26",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM26",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM27",
        address: 0x44001000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Clock("PCLK3_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "TIM27EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "TIM27RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(156),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(156),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM27",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM27",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH4",
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
                pin: "PB1",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
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
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(83),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM4",
        address: 0x40017400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(89),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM5",
        address: 0x40017800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
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
                pin: "PB12",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(95),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(4),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(5),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM8",
        address: 0x40013400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BKIN",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH1N",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "ETR",
                af: Some(4),
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
                pin: "PB12",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH4N",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2N",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH2N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH3N",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH3",
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
                pin: "PD11",
                signal: "ETR",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CH4N",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BKIN2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "ETR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "TRGI",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(71),
            },
        ],
        triggers: &[],
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
                signal: "DIR",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM8_BRK",
            },
            PeripheralInterrupt {
                signal: "IDX",
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
        afio: None,
    },
    Peripheral {
        name: "TIM9",
        address: 0x40001800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "CH2",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "DIR",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "IDX",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "UART4SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
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
                pin: "PB6",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(28),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "UART5SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "RTS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CTS",
                af: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(30),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART6",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "UART6SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CTS",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(32),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART6",
        }],
        afio: None,
    },
    Peripheral {
        name: "UCPD1",
        address: 0x4000dc00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "UCPD1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "UCPD1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "UCPD1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FRSTX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FRSTX",
                af: Some(12),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(115),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
        }],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0x8fff800,
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
                pin: "PA11",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(22),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: None,
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
                register: "CCIPR1",
                field: "USART2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "USART2RST",
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
                pin: "PA0",
                signal: "NSS",
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
                pin: "PA8",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(24),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
        afio: None,
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
                register: "CCIPR1",
                field: "USART3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: None,
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
                pin: "PC7",
                signal: "RTS",
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
                signal: "RTS",
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
                pin: "PE15",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(26),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
        afio: None,
    },
    Peripheral {
        name: "USB",
        address: 0x40016000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "USBSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USBRST",
            }),
            stop_mode: StopMode::Stop1,
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
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EHCI",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "OHCI",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "USBH",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB_DRD_FS",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "USBRAM",
        address: 0x40016400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x44007400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "VREFINTCAL",
        address: 0x8fff810,
        registers: Some(PeripheralRegisters {
            kind: "vrefintcal",
            version: "v1",
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
    Peripheral {
        name: "WWDG",
        address: 0x40003000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "WWDGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "XSPI1",
        address: 0x47001400,
        registers: Some(PeripheralRegisters {
            kind: "octospi",
            version: "v2",
            block: "OCTOSPI",
            ir: &octospi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "XSPI1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "XSPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "XSPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "NCS2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DQS0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NCS1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "NCS1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "DQS0",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "IO0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IO7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IO6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "IO0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IO5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IO0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "IO3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "IO2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "IO7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "IO1",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DQS0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IO3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "IO2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "NCS2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "IO0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "NCS1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "NCS1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "NCS2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "NCS1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "DQS0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "IO1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "IO2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PH0",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "NCS1",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "DQS0",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "NCS2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "GLOBAL",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA1"),
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "GLOBAL",
                channel: None,
                dmamux: None,
                remap: &[],
                dma: Some("GPDMA2"),
                request: Some(57),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XSPI1",
        }],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "IWDG",
        number: 2,
    },
    Interrupt {
        name: "PWR_PVD_AVD",
        number: 4,
    },
    Interrupt {
        name: "TAMP",
        number: 5,
    },
    Interrupt {
        name: "GTZC1",
        number: 6,
    },
    Interrupt {
        name: "FLASH_S",
        number: 7,
    },
    Interrupt {
        name: "FLASH",
        number: 8,
    },
    Interrupt {
        name: "RAMCFG",
        number: 11,
    },
    Interrupt {
        name: "RCC_S",
        number: 12,
    },
    Interrupt {
        name: "RCC",
        number: 13,
    },
    Interrupt {
        name: "ICACHE",
        number: 14,
    },
    Interrupt {
        name: "DCACHE",
        number: 15,
    },
    Interrupt {
        name: "FPU",
        number: 16,
    },
    Interrupt {
        name: "RTC_S",
        number: 17,
    },
    Interrupt {
        name: "RTC",
        number: 18,
    },
    Interrupt {
        name: "EXTI0",
        number: 19,
    },
    Interrupt {
        name: "EXTI1",
        number: 20,
    },
    Interrupt {
        name: "EXTI2",
        number: 21,
    },
    Interrupt {
        name: "EXTI3",
        number: 22,
    },
    Interrupt {
        name: "EXTI4",
        number: 23,
    },
    Interrupt {
        name: "EXTI5",
        number: 24,
    },
    Interrupt {
        name: "EXTI6",
        number: 25,
    },
    Interrupt {
        name: "EXTI7",
        number: 26,
    },
    Interrupt {
        name: "EXTI8",
        number: 27,
    },
    Interrupt {
        name: "EXTI9",
        number: 28,
    },
    Interrupt {
        name: "EXTI10",
        number: 29,
    },
    Interrupt {
        name: "EXTI11",
        number: 30,
    },
    Interrupt {
        name: "EXTI12",
        number: 31,
    },
    Interrupt {
        name: "EXTI13",
        number: 32,
    },
    Interrupt {
        name: "EXTI14",
        number: 33,
    },
    Interrupt {
        name: "EXTI15",
        number: 34,
    },
    Interrupt {
        name: "GPDMA1_CH0",
        number: 35,
    },
    Interrupt {
        name: "GPDMA1_CH1",
        number: 36,
    },
    Interrupt {
        name: "GPDMA1_CH2",
        number: 37,
    },
    Interrupt {
        name: "GPDMA1_CH3",
        number: 38,
    },
    Interrupt {
        name: "GPDMA1_CH4",
        number: 39,
    },
    Interrupt {
        name: "GPDMA1_CH5",
        number: 40,
    },
    Interrupt {
        name: "GPDMA1_CH6",
        number: 41,
    },
    Interrupt {
        name: "GPDMA1_CH7",
        number: 42,
    },
    Interrupt {
        name: "ADC1",
        number: 44,
    },
    Interrupt {
        name: "DAC1",
        number: 45,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 46,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 47,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 48,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 49,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 50,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 51,
    },
    Interrupt {
        name: "TIM2",
        number: 52,
    },
    Interrupt {
        name: "TIM3",
        number: 53,
    },
    Interrupt {
        name: "TIM4",
        number: 54,
    },
    Interrupt {
        name: "TIM5",
        number: 55,
    },
    Interrupt {
        name: "TIM6",
        number: 56,
    },
    Interrupt {
        name: "TIM7",
        number: 57,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 58,
    },
    Interrupt {
        name: "I2C1_ERR",
        number: 59,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 60,
    },
    Interrupt {
        name: "I2C2_ERR",
        number: 61,
    },
    Interrupt {
        name: "SPI1",
        number: 62,
    },
    Interrupt {
        name: "SPI2",
        number: 63,
    },
    Interrupt {
        name: "SPI3",
        number: 64,
    },
    Interrupt {
        name: "USART1",
        number: 65,
    },
    Interrupt {
        name: "USART2",
        number: 66,
    },
    Interrupt {
        name: "USART3",
        number: 67,
    },
    Interrupt {
        name: "UART4",
        number: 68,
    },
    Interrupt {
        name: "UART5",
        number: 69,
    },
    Interrupt {
        name: "LPUART1",
        number: 70,
    },
    Interrupt {
        name: "LPTIM1",
        number: 71,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 72,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 73,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 74,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 75,
    },
    Interrupt {
        name: "ADC2",
        number: 76,
    },
    Interrupt {
        name: "ADC3",
        number: 77,
    },
    Interrupt {
        name: "TIM15",
        number: 78,
    },
    Interrupt {
        name: "TIM16",
        number: 79,
    },
    Interrupt {
        name: "TIM17",
        number: 80,
    },
    Interrupt {
        name: "USB_DRD_FS",
        number: 81,
    },
    Interrupt {
        name: "CRS",
        number: 82,
    },
    Interrupt {
        name: "UCPD1",
        number: 83,
    },
    Interrupt {
        name: "XSPI1",
        number: 85,
    },
    Interrupt {
        name: "TIM9",
        number: 86,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 87,
    },
    Interrupt {
        name: "I2C3_ERR",
        number: 88,
    },
    Interrupt {
        name: "UART6",
        number: 92,
    },
    Interrupt {
        name: "TIM20_BRK_TERR_IERR",
        number: 93,
    },
    Interrupt {
        name: "TIM20_UPD",
        number: 94,
    },
    Interrupt {
        name: "TIM20_TRGI_COM_DIR_IDX",
        number: 95,
    },
    Interrupt {
        name: "TIM20_CC",
        number: 96,
    },
    Interrupt {
        name: "GPDMA2_CH0",
        number: 97,
    },
    Interrupt {
        name: "GPDMA2_CH1",
        number: 98,
    },
    Interrupt {
        name: "GPDMA2_CH2",
        number: 99,
    },
    Interrupt {
        name: "GPDMA2_CH3",
        number: 100,
    },
    Interrupt {
        name: "GPDMA2_CH4",
        number: 101,
    },
    Interrupt {
        name: "GPDMA2_CH5",
        number: 102,
    },
    Interrupt {
        name: "GPDMA2_CH6",
        number: 103,
    },
    Interrupt {
        name: "GPDMA2_CH7",
        number: 104,
    },
    Interrupt {
        name: "FDCAN3_IT0",
        number: 107,
    },
    Interrupt {
        name: "FDCAN3_IT1",
        number: 108,
    },
    Interrupt {
        name: "FMAC2",
        number: 111,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 112,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 113,
    },
    Interrupt {
        name: "CORDIC1",
        number: 114,
    },
    Interrupt {
        name: "FMAC1",
        number: 115,
    },
    Interrupt {
        name: "DTS",
        number: 116,
    },
    Interrupt {
        name: "RNG",
        number: 117,
    },
    Interrupt {
        name: "HASH",
        number: 120,
    },
    Interrupt {
        name: "PKA",
        number: 121,
    },
    Interrupt {
        name: "TIM10",
        number: 122,
    },
    Interrupt {
        name: "TIM12",
        number: 123,
    },
    Interrupt {
        name: "TIM13",
        number: 124,
    },
    Interrupt {
        name: "TIM14",
        number: 125,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 126,
    },
    Interrupt {
        name: "I3C1_ERR",
        number: 127,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 128,
    },
    Interrupt {
        name: "I2C4_ERR",
        number: 129,
    },
    Interrupt {
        name: "POSENC1",
        number: 136,
    },
    Interrupt {
        name: "POSENC2",
        number: 137,
    },
    Interrupt {
        name: "HRTIM1_M",
        number: 138,
    },
    Interrupt {
        name: "HRTIM1_TIMA",
        number: 139,
    },
    Interrupt {
        name: "HRTIM1_TIMB",
        number: 140,
    },
    Interrupt {
        name: "HRTIM1_TIMC",
        number: 141,
    },
    Interrupt {
        name: "HRTIM1_TIMD",
        number: 142,
    },
    Interrupt {
        name: "HRTIM1_TIME",
        number: 143,
    },
    Interrupt {
        name: "HRTIM1_TIMF",
        number: 144,
    },
    Interrupt {
        name: "HRTIM1_FLT",
        number: 145,
    },
    Interrupt {
        name: "PLAY1_S",
        number: 146,
    },
    Interrupt {
        name: "PLAY1",
        number: 147,
    },
    Interrupt {
        name: "COMP1",
        number: 148,
    },
    Interrupt {
        name: "COMP2",
        number: 149,
    },
    Interrupt {
        name: "COMP3",
        number: 150,
    },
    Interrupt {
        name: "COMP4",
        number: 151,
    },
    Interrupt {
        name: "COMP5",
        number: 152,
    },
    Interrupt {
        name: "COMP6",
        number: 153,
    },
    Interrupt {
        name: "COMP7",
        number: 154,
    },
    Interrupt {
        name: "COMP8",
        number: 155,
    },
    Interrupt {
        name: "DAC2",
        number: 164,
    },
    Interrupt {
        name: "DAC5",
        number: 167,
    },
    Interrupt {
        name: "DAC6",
        number: 168,
    },
    Interrupt {
        name: "DAC7",
        number: 169,
    },
    Interrupt {
        name: "DAC8",
        number: 170,
    },
    Interrupt {
        name: "DAC9",
        number: 171,
    },
    Interrupt {
        name: "DAC10",
        number: 172,
    },
    Interrupt {
        name: "TIM11",
        number: 187,
    },
    Interrupt {
        name: "TIM26",
        number: 188,
    },
    Interrupt {
        name: "TIM27",
        number: 189,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "GPDMA1_CH0",
        dma: "GPDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH1",
        dma: "GPDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH2",
        dma: "GPDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH3",
        dma: "GPDMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH4",
        dma: "GPDMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH5",
        dma: "GPDMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH6",
        dma: "GPDMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA1_CH7",
        dma: "GPDMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH0",
        dma: "GPDMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH1",
        dma: "GPDMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH2",
        dma: "GPDMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH3",
        dma: "GPDMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH4",
        dma: "GPDMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH5",
        dma: "GPDMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH6",
        dma: "GPDMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
    DmaChannel {
        name: "GPDMA2_CH7",
        dma: "GPDMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
        supports_2d: None,
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA0" },
    Pin { name: "PA1" },
    Pin { name: "PA2" },
    Pin { name: "PA3" },
    Pin { name: "PA4" },
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
    Pin { name: "PB0" },
    Pin { name: "PB1" },
    Pin { name: "PB2" },
    Pin { name: "PB3" },
    Pin { name: "PB4" },
    Pin { name: "PB5" },
    Pin { name: "PB6" },
    Pin { name: "PB7" },
    Pin { name: "PB9" },
    Pin { name: "PB10" },
    Pin { name: "PB11" },
    Pin { name: "PB12" },
    Pin { name: "PB13" },
    Pin { name: "PB14" },
    Pin { name: "PB15" },
    Pin { name: "PC0" },
    Pin { name: "PC1" },
    Pin { name: "PC2" },
    Pin { name: "PC3" },
    Pin { name: "PC4" },
    Pin { name: "PC5" },
    Pin { name: "PC6" },
    Pin { name: "PC7" },
    Pin { name: "PC8" },
    Pin { name: "PC9" },
    Pin { name: "PC11" },
    Pin { name: "PC12" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PD8" },
    Pin { name: "PD9" },
    Pin { name: "PD10" },
    Pin { name: "PD11" },
    Pin { name: "PD12" },
    Pin { name: "PD13" },
    Pin { name: "PE2" },
    Pin { name: "PE7" },
    Pin { name: "PE13" },
    Pin { name: "PE14" },
    Pin { name: "PE15" },
    Pin { name: "PF2" },
    Pin { name: "PF4" },
    Pin { name: "PF7" },
    Pin { name: "PF9" },
    Pin { name: "PF10" },
    Pin { name: "PF15" },
    Pin { name: "PG2" },
    Pin { name: "PG3" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
];
#[path = "../registers/cordic_v1.rs"]
pub mod cordic;
#[path = "../registers/dcache_v1.rs"]
pub mod dcache;
#[path = "../registers/dts_v1.rs"]
pub mod dts;
#[path = "../registers/exti_u5.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/hrtim_g5.rs"]
pub mod hrtim;
#[path = "../registers/i2c_v3.rs"]
pub mod i2c;
#[path = "../registers/octospi_v2.rs"]
pub mod octospi;
#[path = "../registers/rcc_g5.rs"]
pub mod rcc;
#[path = "../registers/spi_v5_i2s.rs"]
pub mod spi;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/usart_v4.rs"]
pub mod usart;
#[path = "../registers/vrefintcal_v1.rs"]
pub mod vrefintcal;
