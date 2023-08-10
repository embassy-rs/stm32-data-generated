
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v4",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP16",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INN16",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP17",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "INP18",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INN18",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INP19",
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
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "INP6",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(9),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1073881344,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v4",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
        }),
        pins: &[
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
                signal: "INP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INP4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "INN2",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "INP6",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(10),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073881856,
        registers: Some(PeripheralRegisters {
            kind: "adccommon",
            version: "v4",
            block: "ADC_COMMON",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BDMA1",
        address: 1208101888,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "BDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "BDMA1RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "BDMA1",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "BDMA1",
            },
        ],
    },
    Peripheral {
        name: "BDMA2",
        address: 1476547584,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "BDMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "BDMA2RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "BDMA2_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "BDMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "BDMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "BDMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "BDMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "BDMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "BDMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "BDMA2_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "CEC",
        address: 1073769472,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "CECEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "CECRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP1",
        address: 1476409356,
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
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "OUT",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP2",
        address: 1476409360,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PE7",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "OUT",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "OUT",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
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
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "CRCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRS",
        address: 1073775616,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "CRSRST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PB3",
            signal: "SYNC",
            af: Some(10),
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC1",
        address: 1073771520,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v3",
            block: "DAC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "DAC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(68),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC",
        }],
    },
    Peripheral {
        name: "DAC2",
        address: 1476408320,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v3",
            block: "DAC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "DAC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "DAC2RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PA6",
            signal: "OUT1",
            af: None,
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CH1",
            channel: None,
            dmamux: Some("DMAMUX2"),
            dma: None,
            request: Some(17),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC2",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 1543507968,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "h7",
            block: "DBGMCU",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCMI",
        address: 1208090624,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DCMIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DCMIRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "VSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: Some(13),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PSSI",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(75),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI_PSSI",
        }],
    },
    Peripheral {
        name: "DFSDM1",
        address: 1073838080,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "DFSDM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "DFSDM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CKOUT",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DATIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CKIN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "DATIN5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CKIN5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN7",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "DATIN7",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DATIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CKIN7",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DATIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CKIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DATIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CKIN2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CKIN0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "DATIN4",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CKIN4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "DATIN0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DATIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CKIN3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "DATIN3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CKIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "DATIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CKIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DATIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CKIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "DATIN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CKIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "DATIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "CKIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DATIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "DATIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CKIN3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DATIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DATIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CKIN4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DATIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CKIN5",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "DATIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CKIN6",
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(104),
            },
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: Some("BDMA1_CH0"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: Some("BDMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: Some("BDMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: Some("BDMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT4",
                channel: Some("BDMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT5",
                channel: Some("BDMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT6",
                channel: Some("BDMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "FLT7",
                channel: Some("BDMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT4",
                interrupt: "DFSDM1_FLT4",
            },
            PeripheralInterrupt {
                signal: "FLT5",
                interrupt: "DFSDM1_FLT5",
            },
            PeripheralInterrupt {
                signal: "FLT6",
                interrupt: "DFSDM1_FLT6",
            },
            PeripheralInterrupt {
                signal: "FLT7",
                interrupt: "DFSDM1_FLT7",
            },
        ],
    },
    Peripheral {
        name: "DFSDM2",
        address: 1476422656,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CKIN1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "DATIN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CKOUT",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DATIN1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CKIN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CKIN0",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "DATIN0",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CKOUT",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CKOUT",
                af: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "FLT0",
            channel: None,
            dmamux: Some("DMAMUX2"),
            dma: None,
            request: Some(18),
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT0",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT1",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT2",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT3",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT4",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT5",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT6",
                interrupt: "DFSDM2",
            },
            PeripheralInterrupt {
                signal: "FLT7",
                interrupt: "DFSDM2",
            },
        ],
    },
    Peripheral {
        name: "DMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "dma",
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
                signal: "CH0",
                interrupt: "DMA1_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073873920,
        registers: Some(PeripheralRegisters {
            kind: "dma",
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
                signal: "CH0",
                interrupt: "DMA2_STREAM0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_STREAM1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_STREAM2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_STREAM3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_STREAM4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_STREAM5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_STREAM6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_STREAM7",
            },
        ],
    },
    Peripheral {
        name: "DMA2D",
        address: 1375735808,
        registers: Some(PeripheralRegisters {
            kind: "dma2d",
            version: "v2",
            block: "DMA2D",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB3",
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "DMA2DEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "DMA2DRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DMA2D",
        }],
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
        name: "DMAMUX2",
        address: 1476548608,
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
        name: "DTS",
        address: 1476421632,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "DTSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "DTSRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DTS",
        }],
    },
    Peripheral {
        name: "EXTI",
        address: 1476395008,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "h7",
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
        address: 1073782784,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan",
            block: "FDCAN",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "FDCANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
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
                signal: "CAL",
                interrupt: "FDCAN_CAL",
            },
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
        address: 1073783808,
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
                signal: "CAL",
                interrupt: "FDCAN_CAL",
            },
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
        name: "FLASH",
        address: 1375739904,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "h7",
            block: "FLASH",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "FMC",
        address: 1375748096,
        registers: Some(PeripheralRegisters {
            kind: "fmc",
            version: "v3x1",
            block: "FMC",
        }),
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
                pin: "PA7",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDCKE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "A25",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SDNE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SDCKE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "NWAIT",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "NE1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "INT",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NCE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NE2",
                af: Some(9),
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
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "ALE",
                af: Some(12),
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
                pin: "PF0",
                signal: "A0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "A1",
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
                pin: "PF11",
                signal: "SDNRAS",
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
                pin: "PG4",
                signal: "BA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "A15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "BA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SDCLK",
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
            PeripheralPin {
                pin: "PG10",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "NE4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "A24",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "A25",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "SDNCAS",
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
        name: "GFXMMU",
        address: 1375780864,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GFXMMU",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 1476526080,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOARST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1476527104,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOBRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1476528128,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 1476529152,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIODRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 1476530176,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOERST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 1476531200,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOFRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOG",
        address: 1476532224,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOGRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 1476533248,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOHRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOI",
        address: 1476534272,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOIRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOJ",
        address: 1476535296,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOJEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOJRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOK",
        address: 1476536320,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "GPIOKEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOKRST",
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
                register: "APB1LENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C1RST",
            }),
        }),
        pins: &[
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
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(34),
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
                register: "APB1LENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
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
                pin: "PF2",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(36),
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
        address: 1073765376,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC9",
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
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(74),
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
        address: 1476402176,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "I2C4RST",
            }),
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PB9",
                signal: "SMBA",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SDA",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(14),
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
        name: "IWDG1",
        address: 1476413440,
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
        name: "JPEG",
        address: 1375744000,
        registers: Some(PeripheralRegisters {
            kind: "jpeg",
            version: "v1",
            block: "JPEG",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "JPEG",
        }],
    },
    Peripheral {
        name: "LPTIM1",
        address: 1073751040,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "LPTIM1RST",
            }),
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "IN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "IN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "OUT",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG14",
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
        name: "LPTIM2",
        address: 1476404224,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "OUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
    },
    Peripheral {
        name: "LPTIM3",
        address: 1476405248,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPTIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPTIM3RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PA1",
            signal: "OUT",
            af: Some(3),
        }],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 1476398080,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "LPUART1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(10),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "LTDC",
        address: 1342181376,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LTDCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LTDCRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "B3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "R3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "R3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "R6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "B5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "G2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "G3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "B1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "B7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "G3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "G3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "B0",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC_ER",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LTDC",
            },
        ],
    },
    Peripheral {
        name: "MDIOS",
        address: 1073779712,
        registers: Some(PeripheralRegisters {
            kind: "mdios",
            version: "v1",
            block: "MDIOS",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "MDIOSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "MDIOSRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "MDIOS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "MDIOS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "MDIOS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "MDMA",
        address: 1375731712,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB3",
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "MDMAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "MDMARST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MDMA",
        }],
    },
    Peripheral {
        name: "OCTOSPI1",
        address: 2415919104,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OCTOSPI1",
        }],
    },
    Peripheral {
        name: "OCTOSPI2",
        address: 1879048192,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB3",
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "OCTOSPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "OCTOSPI2RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OCTOSPI2",
        }],
    },
    Peripheral {
        name: "OCTOSPIM",
        address: 1375777792,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "P1_DQS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "P1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "P1_CLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "P1_IO3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "P1_IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "P1_IO1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "P1_IO0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "P1_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "P1_DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "P1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P1_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P1_NCLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "P1_IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "P1_DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "P1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "P1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "P1_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "P1_IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "P1_IO5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "P1_IO6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "P1_IO7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "P1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "P1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "P1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "P1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "P1_IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "P1_IO5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "P1_IO6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "P1_IO7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "P1_NCS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "P2_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "P2_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "P2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "P2_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "P2_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "P2_NCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "P1_IO3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "P1_IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "P1_IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "P1_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "P1_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "P1_NCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "P2_DQS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "P2_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "P2_IO5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "P1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "P2_DQS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "P1_IO6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "P2_IO6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "P2_IO7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "P2_NCS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "P1_IO7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "P2_DQS",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP1",
        address: 1073778688,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "OPAMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "OPAMPRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "VINM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP2",
        address: 1073778704,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PE7",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PG1",
                signal: "VINM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PSSI",
        address: 1208091648,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "DE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D14",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PDCK",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RDY",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D15",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D15",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "RDY",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI_PSSI",
        }],
    },
    Peripheral {
        name: "PWR",
        address: 1476544512,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "h7smps",
            block: "PWR",
        }),
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
                pin: "PA5",
                signal: "NDSTOP2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PVD_IN",
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
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RAMECC",
        address: 1375768576,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1476543488,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "h7ab",
            block: "RCC",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "MCO_2",
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "CRS",
            },
            PeripheralInterrupt {
                signal: "CSS",
                interrupt: "RTC_TAMP_STAMP_CSS_LSE",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "LSE",
                interrupt: "RTC_TAMP_STAMP_CSS_LSE",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1208096768,
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
        address: 1476411392,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v2h7",
            block: "RTC",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "OUT_ALARM",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT_CALIB",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TAMP3",
                af: None,
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
                interrupt: "RTC_TAMP_STAMP_CSS_LSE",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC_TAMP_STAMP_CSS_LSE",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
    },
    Peripheral {
        name: "SAI1",
        address: 1073829888,
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
                pin: "PB2",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CK1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "MCLK_A",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(88),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI1",
        }],
    },
    Peripheral {
        name: "SAI2",
        address: 1073830912,
        registers: Some(PeripheralRegisters {
            kind: "sai",
            version: "v4",
            block: "SAI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "FS_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "MCLK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SD_B",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(90),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI2",
        }],
    },
    Peripheral {
        name: "SDMMC1",
        address: 1375760384,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v2",
            block: "SDMMC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB3",
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "SDMMC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "SDMMC1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CDIR",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D123DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC1",
        }],
    },
    Peripheral {
        name: "SDMMC2",
        address: 1208099840,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v2",
            block: "SDMMC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "SDMMC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "SDMMC2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CMD",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CMD",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "D0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D7",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC2",
        }],
    },
    Peripheral {
        name: "SPDIFRX",
        address: 1073758208,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPDIFRXEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPDIFRXRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "IN1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPDIF_RX",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
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
                pin: "PD7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG11",
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
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(38),
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
            version: "v3",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "SPI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
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
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_WS",
                af: Some(7),
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
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD3",
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
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(40),
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
            version: "v3",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                pin: "PB2",
                signal: "MOSI",
                af: Some(7),
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
                af: Some(7),
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
                pin: "PD6",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SDO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(7),
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
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SDO",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(62),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SPI4",
        address: 1073820672,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
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
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(84),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
    },
    Peripheral {
        name: "SPI5",
        address: 1073827840,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF6",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF11",
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
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(86),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
    },
    Peripheral {
        name: "SPI6",
        address: 1476400128,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "SPI6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "SPI6RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "I2S_SDO",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX2"),
                dma: None,
                request: Some(12),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
    },
    Peripheral {
        name: "SWPMI1",
        address: 1073776640,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC6",
                signal: "IO",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SUSPEND",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "RX",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SWPMI1",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 1476396032,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "h7",
            block: "SYSCFG",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB4",
            enable: Some(PeripheralRccRegister {
                register: "APB4ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4RSTR",
                field: "SYSCFGRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 1073807360,
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
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN_COMP1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN_COMP2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2_COMP1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2_COMP2",
                af: Some(11),
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
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN_COMP1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN_COMP2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BKIN2_COMP1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BKIN2_COMP2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "ETR",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(17),
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
        name: "TIM12",
        address: 1073747968,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM12RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_BRK_TIM12",
            },
        ],
    },
    Peripheral {
        name: "TIM13",
        address: 1073748992,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM13EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM13RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
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
                register: "APB1LENR",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM14RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_TRG_COM_TIM14",
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
                pin: "PA0",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "BKIN",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(107),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(108),
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
                pin: "PB4",
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
                pin: "PF6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(110),
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
                pin: "PB5",
                signal: "BKIN",
                af: Some(1),
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
                pin: "PF7",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(112),
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
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(22),
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
                register: "APB1LENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM3RST",
            }),
        }),
        pins: &[
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(28),
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
                register: "APB1LENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM4RST",
            }),
        }),
        pins: &[
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(32),
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
                register: "APB1LENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                pin: "PA4",
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
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(60),
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
                register: "APB1LENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM6RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(69),
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
                register: "APB1LENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM7RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            dma: None,
            request: Some(70),
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
        name: "TIM8",
        address: 1073808384,
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
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN_COMP2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BKIN2_COMP1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BKIN2_COMP2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BKIN_COMP1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BKIN_COMP2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BKIN2_COMP1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BKIN2_COMP2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "ETR",
                af: Some(3),
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
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(53),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
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
                register: "APB1LENR",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART4RST",
            }),
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
                pin: "PA11",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(6),
            },
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
                pin: "PB0",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
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
                pin: "PC10",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(64),
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
                register: "APB1LENR",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB6",
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
                pin: "PC8",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD2",
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
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(66),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UART7",
        address: 1073772544,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART7RST",
            }),
        }),
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
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CTS",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(80),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "UART8",
        address: 1073773568,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART8RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(82),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
    },
    Peripheral {
        name: "UART9",
        address: 1073813504,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "UART9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "UART9RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD0",
                signal: "CTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "TX",
                af: Some(11),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(117),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART9",
        }],
    },
    Peripheral {
        name: "UID",
        address: 150992896,
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
        address: 1073811456,
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
                pin: "PB14",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
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
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(42),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART10",
        address: 1073814528,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART10RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE2",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "NSS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "DE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "RTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CK",
                af: Some(11),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(119),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART10",
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
                register: "APB1LENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(44),
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
                register: "APB1LENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "USART3RST",
            }),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(46),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 1073812480,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART6RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "NSS",
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                dma: None,
                request: Some(72),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 1074003968,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "USB_OTG_HSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "USB_OTG_HSRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "ULPI_D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ULPI_CK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "VBUS",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: Some(10),
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
                pin: "PB0",
                signal: "ULPI_D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "ULPI_D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ULPI_D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "ULPI_D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ULPI_D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ULPI_D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ULPI_D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ULPI_STP",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS_EP1_IN",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS_EP1_OUT",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "VREFBUF",
        address: 1476410368,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG1",
        address: 1342189568,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "WWDG1EN",
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
        name: "RTC_TAMP_STAMP_CSS_LSE",
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
        name: "DMA1_STREAM0",
        number: 11,
    },
    Interrupt {
        name: "DMA1_STREAM1",
        number: 12,
    },
    Interrupt {
        name: "DMA1_STREAM2",
        number: 13,
    },
    Interrupt {
        name: "DMA1_STREAM3",
        number: 14,
    },
    Interrupt {
        name: "DMA1_STREAM4",
        number: 15,
    },
    Interrupt {
        name: "DMA1_STREAM5",
        number: 16,
    },
    Interrupt {
        name: "DMA1_STREAM6",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 19,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 20,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 21,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
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
        name: "DFSDM2",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK_TIM12",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP_TIM13",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM_TIM14",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "DMA1_STREAM7",
        number: 47,
    },
    Interrupt {
        name: "FMC",
        number: 48,
    },
    Interrupt {
        name: "SDMMC1",
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
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_STREAM0",
        number: 56,
    },
    Interrupt {
        name: "DMA2_STREAM1",
        number: 57,
    },
    Interrupt {
        name: "DMA2_STREAM2",
        number: 58,
    },
    Interrupt {
        name: "DMA2_STREAM3",
        number: 59,
    },
    Interrupt {
        name: "DMA2_STREAM4",
        number: 60,
    },
    Interrupt {
        name: "FDCAN_CAL",
        number: 63,
    },
    Interrupt {
        name: "DFSDM1_FLT4",
        number: 64,
    },
    Interrupt {
        name: "DFSDM1_FLT5",
        number: 65,
    },
    Interrupt {
        name: "DFSDM1_FLT6",
        number: 66,
    },
    Interrupt {
        name: "DFSDM1_FLT7",
        number: 67,
    },
    Interrupt {
        name: "DMA2_STREAM5",
        number: 68,
    },
    Interrupt {
        name: "DMA2_STREAM6",
        number: 69,
    },
    Interrupt {
        name: "DMA2_STREAM7",
        number: 70,
    },
    Interrupt {
        name: "USART6",
        number: 71,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 73,
    },
    Interrupt {
        name: "OTG_HS_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "OTG_HS_EP1_IN",
        number: 75,
    },
    Interrupt {
        name: "OTG_HS_WKUP",
        number: 76,
    },
    Interrupt {
        name: "OTG_HS",
        number: 77,
    },
    Interrupt {
        name: "DCMI_PSSI",
        number: 78,
    },
    Interrupt {
        name: "RNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "UART7",
        number: 82,
    },
    Interrupt {
        name: "UART8",
        number: 83,
    },
    Interrupt {
        name: "SPI4",
        number: 84,
    },
    Interrupt {
        name: "SPI5",
        number: 85,
    },
    Interrupt {
        name: "SPI6",
        number: 86,
    },
    Interrupt {
        name: "SAI1",
        number: 87,
    },
    Interrupt {
        name: "LTDC",
        number: 88,
    },
    Interrupt {
        name: "LTDC_ER",
        number: 89,
    },
    Interrupt {
        name: "DMA2D",
        number: 90,
    },
    Interrupt {
        name: "SAI2",
        number: 91,
    },
    Interrupt {
        name: "OCTOSPI1",
        number: 92,
    },
    Interrupt {
        name: "LPTIM1",
        number: 93,
    },
    Interrupt {
        name: "CEC",
        number: 94,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 95,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 96,
    },
    Interrupt {
        name: "SPDIF_RX",
        number: 97,
    },
    Interrupt {
        name: "SWPMI1",
        number: 115,
    },
    Interrupt {
        name: "TIM15",
        number: 116,
    },
    Interrupt {
        name: "TIM16",
        number: 117,
    },
    Interrupt {
        name: "TIM17",
        number: 118,
    },
    Interrupt {
        name: "MDIOS_WKUP",
        number: 119,
    },
    Interrupt {
        name: "MDIOS",
        number: 120,
    },
    Interrupt {
        name: "JPEG",
        number: 121,
    },
    Interrupt {
        name: "MDMA",
        number: 122,
    },
    Interrupt {
        name: "SDMMC2",
        number: 124,
    },
    Interrupt {
        name: "HSEM1",
        number: 125,
    },
    Interrupt {
        name: "DAC2",
        number: 127,
    },
    Interrupt {
        name: "BDMA2_CHANNEL0",
        number: 129,
    },
    Interrupt {
        name: "BDMA2_CHANNEL1",
        number: 130,
    },
    Interrupt {
        name: "BDMA2_CHANNEL2",
        number: 131,
    },
    Interrupt {
        name: "BDMA2_CHANNEL3",
        number: 132,
    },
    Interrupt {
        name: "BDMA2_CHANNEL4",
        number: 133,
    },
    Interrupt {
        name: "BDMA2_CHANNEL5",
        number: 134,
    },
    Interrupt {
        name: "BDMA2_CHANNEL6",
        number: 135,
    },
    Interrupt {
        name: "BDMA2_CHANNEL7",
        number: 136,
    },
    Interrupt {
        name: "LPTIM2",
        number: 138,
    },
    Interrupt {
        name: "LPTIM3",
        number: 139,
    },
    Interrupt {
        name: "UART9",
        number: 140,
    },
    Interrupt {
        name: "USART10",
        number: 141,
    },
    Interrupt {
        name: "LPUART1",
        number: 142,
    },
    Interrupt {
        name: "CRS",
        number: 144,
    },
    Interrupt {
        name: "ECC",
        number: 145,
    },
    Interrupt {
        name: "DTS",
        number: 147,
    },
    Interrupt {
        name: "WAKEUP_PIN",
        number: 149,
    },
    Interrupt {
        name: "OCTOSPI2",
        number: 150,
    },
    Interrupt {
        name: "GFXMMU",
        number: 153,
    },
    Interrupt {
        name: "BDMA1",
        number: 154,
    },
];
const DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(12),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(13),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(14),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(15),
    },
    DmaChannel {
        name: "BDMA1_CH0",
        dma: "BDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH1",
        dma: "BDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH2",
        dma: "BDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH3",
        dma: "BDMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH4",
        dma: "BDMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH5",
        dma: "BDMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH6",
        dma: "BDMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA1_CH7",
        dma: "BDMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "BDMA2_CH0",
        dma: "BDMA2",
        channel: 0,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "BDMA2_CH1",
        dma: "BDMA2",
        channel: 1,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "BDMA2_CH2",
        dma: "BDMA2",
        channel: 2,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "BDMA2_CH3",
        dma: "BDMA2",
        channel: 3,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "BDMA2_CH4",
        dma: "BDMA2",
        channel: 4,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "BDMA2_CH5",
        dma: "BDMA2",
        channel: 5,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "BDMA2_CH6",
        dma: "BDMA2",
        channel: 6,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(6),
    },
    DmaChannel {
        name: "BDMA2_CH7",
        dma: "BDMA2",
        channel: 7,
        dmamux: Some("DMAMUX2"),
        dmamux_channel: Some(7),
    },
];
