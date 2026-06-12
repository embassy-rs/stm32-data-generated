
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x48003000,
        registers: None,
        rcc: None,
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
                pin: "PC2",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "INP13",
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
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x48003300,
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
        address: 0x48003100,
        registers: None,
        rcc: None,
        pins: &[
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
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "BSEC",
        address: 0x5c005000,
        registers: Some(PeripheralRegisters {
            kind: "bsec",
            version: "v2",
            block: "BSEC",
            ir: &bsec::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CEC",
        address: 0x40016000,
        registers: Some(PeripheralRegisters {
            kind: "cec",
            version: "v2",
            block: "CEC",
            ir: &cec::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CEC",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CEC",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CEC",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRC1",
        address: 0x58009000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRC2",
        address: 0x4c004000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRYP1",
        address: 0x54001000,
        registers: Some(PeripheralRegisters {
            kind: "cryp",
            version: "v2",
            block: "CRYP",
            ir: &cryp::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CRYP1",
        }],
        afio: None,
    },
    Peripheral {
        name: "CRYP2",
        address: 0x4c005000,
        registers: Some(PeripheralRegisters {
            kind: "cryp",
            version: "v2",
            block: "CRYP",
            ir: &cryp::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CRYP2",
        }],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x40017000,
        registers: None,
        rcc: None,
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
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x50081000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DCMI",
        address: 0x4c006000,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
            ir: &dcmi::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(13),
            },
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
                pin: "PC2",
                signal: "PIXCLK",
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
                pin: "PD9",
                signal: "HSYNC",
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
                pin: "PE11",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D6",
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
                pin: "PH10",
                signal: "D1",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "D2",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "D3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "D4",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "HSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "D13",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "D8",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "D9",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "D10",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "D5",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "VSYNC",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "D6",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "D7",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI",
        }],
        afio: None,
    },
    Peripheral {
        name: "DFSDM1",
        address: 0x4400d000,
        registers: None,
        rcc: None,
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
                pin: "PB13",
                signal: "CKOUT",
                af: Some(3),
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
                pin: "PB2",
                signal: "CKIN1",
                af: Some(3),
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
                pin: "PC2",
                signal: "CKIN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CKOUT",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "DATIN1",
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
                pin: "PD0",
                signal: "CKIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DATIN7",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CKIN7",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DATIN6",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CKOUT",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "DATIN0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CKIN0",
                af: Some(11),
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
                pin: "PF13",
                signal: "DATIN3",
                af: Some(6),
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
            PeripheralPin {
                pin: "PG0",
                signal: "DATIN0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "CKIN1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "CKIN4",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT0",
                interrupt: "DFSDM1_FLT0",
            },
            PeripheralInterrupt {
                signal: "FLT1",
                interrupt: "DFSDM1_FLT1",
            },
            PeripheralInterrupt {
                signal: "FLT2",
                interrupt: "DFSDM1_FLT2",
            },
            PeripheralInterrupt {
                signal: "FLT3",
                interrupt: "DFSDM1_FLT3",
            },
            PeripheralInterrupt {
                signal: "FLT4",
                interrupt: "DFSDM1_FLT4",
            },
            PeripheralInterrupt {
                signal: "FLT5",
                interrupt: "DFSDM1_FLT5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "DMA1",
        address: 0x48000000,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "DMA2",
        address: 0x48001000,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x48002000,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "v1",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "OVR",
            interrupt: "DMAMUX1_OVR",
        }],
        afio: None,
    },
    Peripheral {
        name: "DSIHOST",
        address: 0x5a000000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB11",
                signal: "TE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "TE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "TE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PJ2",
                signal: "TE",
                af: Some(13),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DSI",
        }],
        afio: None,
    },
    Peripheral {
        name: "DTS",
        address: 0x50028000,
        registers: Some(PeripheralRegisters {
            kind: "dts",
            version: "v1",
            block: "DTS",
            ir: &dts::REGISTERS,
        }),
        rcc: None,
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
        address: 0x5000d000,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "v1",
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
        address: 0x4400e000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v2",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: None,
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
            PeripheralPin {
                pin: "PH13",
                signal: "TX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "RX",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "RX",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "FDCAN2",
        address: 0x4400f000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "fdcan_v2",
            block: "FDCAN",
            ir: &can::REGISTERS,
        }),
        rcc: None,
        pins: &[
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
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "FDCANRAM1",
        address: 0x44011000,
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
        address: 0x44011350,
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
        name: "FMC",
        address: 0x58002000,
        registers: None,
        rcc: None,
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
                pin: "PE15",
                signal: "NCE2",
                af: Some(10),
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
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMC",
        }],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x50002000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOB",
        address: 0x50003000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOC",
        address: 0x50004000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOD",
        address: 0x50005000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOE",
        address: 0x50006000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOF",
        address: 0x50007000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOG",
        address: 0x50008000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOH",
        address: 0x50009000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOI",
        address: 0x5000a000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOJ",
        address: 0x5000b000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOK",
        address: 0x5000c000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOZ",
        address: 0x54004000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPU",
        address: 0x59000000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPU",
        }],
        afio: None,
    },
    Peripheral {
        name: "HASH1",
        address: 0x54002000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH1",
        }],
        afio: None,
    },
    Peripheral {
        name: "HASH2",
        address: 0x4c002000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH2",
        }],
        afio: None,
    },
    Peripheral {
        name: "HDP",
        address: 0x5002a000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "HDP0",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "HDP6",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "HDP7",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HDP1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "HDP4",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "HDP5",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "HDP2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "HDP3",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "HDP2",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "HDP0",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PI12",
                signal: "HDP0",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PI13",
                signal: "HDP1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "HDP1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PJ5",
                signal: "HDP2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PJ6",
                signal: "HDP3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PK1",
                signal: "HDP4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PK2",
                signal: "HDP5",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PK5",
                signal: "HDP6",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PK6",
                signal: "HDP7",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HSEM",
        address: 0x4c000000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HSEM_IT2",
        }],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40012000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SDA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "SMBA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "SDA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH10",
                signal: "SMBA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "SDA",
                af: Some(5),
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
        name: "I2C2",
        address: 0x40013000,
        registers: None,
        rcc: None,
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
                pin: "PD7",
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
                pin: "PF2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH4",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ4",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PZ7",
                signal: "SDA",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "I2C3",
        address: 0x40014000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PH7",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "SMBA",
                af: Some(4),
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
        name: "I2C4",
        address: 0x5c002000,
        registers: None,
        rcc: None,
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
                pin: "PE2",
                signal: "SCL",
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
                pin: "PH10",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "SMBA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PZ4",
                signal: "SCL",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "SDA",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "SMBA",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "I2C5",
        address: 0x40015000,
        registers: None,
        rcc: None,
        pins: &[
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
                pin: "PD0",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ4",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C5_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C5_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C6",
        address: 0x5c009000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ4",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PZ7",
                signal: "SDA",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C6_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C6_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "IPCC",
        address: 0x4c001000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "IPCC_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "IPCC_RX1",
            },
            PeripheralInterrupt {
                signal: "TX0",
                interrupt: "IPCC_TX0",
            },
            PeripheralInterrupt {
                signal: "TX1",
                interrupt: "IPCC_TX1",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "IWDG1",
        address: 0x5c003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IWDG2",
        address: 0x5a002000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40009000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PH2",
                signal: "IN2",
                af: Some(1),
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
        address: 0x50021000,
        registers: None,
        rcc: None,
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
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN2",
                af: Some(4),
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
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM3",
        address: 0x50022000,
        registers: None,
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PA1",
            signal: "OUT",
            af: Some(3),
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM4",
        address: 0x50023000,
        registers: None,
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PA2",
            signal: "OUT",
            af: Some(3),
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM4",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM5",
        address: 0x50024000,
        registers: None,
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PA3",
            signal: "OUT",
            af: Some(3),
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM5",
        }],
        afio: None,
    },
    Peripheral {
        name: "LTDC",
        address: 0x5a001000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B1",
                af: Some(14),
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
                signal: "R1",
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
                pin: "PB5",
                signal: "G7",
                af: Some(14),
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
                pin: "PC0",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
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
                pin: "PD10",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "R1",
                af: Some(14),
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
                pin: "PD8",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "B0",
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
                pin: "PE14",
                signal: "G0",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "R7",
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
                pin: "PF10",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "G5",
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
                pin: "PG9",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH10",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "G2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "G3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH4",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH4",
                signal: "G5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "R3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "G6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI11",
                signal: "G6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PI12",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI13",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI14",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI15",
                signal: "G2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PI15",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ0",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ0",
                signal: "R7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PJ1",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ10",
                signal: "G3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ11",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ12",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ12",
                signal: "G3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PJ13",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ13",
                signal: "G4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PJ14",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ15",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ2",
                signal: "R3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ3",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ4",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ5",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ6",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ7",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ8",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PJ9",
                signal: "G2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK0",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK1",
                signal: "G6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK2",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK3",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK4",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK5",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK6",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PK7",
                signal: "DE",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC_ER",
            },
            PeripheralInterrupt {
                signal: "LO",
                interrupt: "LTDC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "MDMA",
        address: 0x58000000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MDMA",
        }],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x50001000,
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
                pin: "PA3",
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
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PI11",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "WKUP4",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "AVD",
                interrupt: "PVD_AVD",
            },
            PeripheralInterrupt {
                signal: "PIN",
                interrupt: "WAKEUP_PIN",
            },
            PeripheralInterrupt {
                signal: "PVD",
                interrupt: "PVD_AVD",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "WAKEUP_PIN",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "QUADSPI",
        address: 0x58003000,
        registers: Some(PeripheralRegisters {
            kind: "quadspi",
            version: "v1",
            block: "QUADSPI",
            ir: &quadspi::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "BK1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BK1_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BK1_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "BK2_NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "BK1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "BK2_NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BK1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "BK1_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "BK1_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "BK1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "BK2_IO3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "BK1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "BK2_IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "BK2_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "BK2_IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "BK1_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "BK1_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "BK1_IO0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "BK1_IO1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "BK2_IO2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "BK2_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "BK2_IO3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "BK2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "BK2_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "BK2_IO1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QUADSPI",
        }],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "mp1",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "MCO_1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MCO_2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MCO_2",
                af: Some(1),
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
                pin: "PG2",
                signal: "MCO_2",
                af: Some(1),
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
            PeripheralPin {
                pin: "PI11",
                signal: "MCO_1",
                af: Some(0),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "RCC_WAKEUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "RNG1",
        address: 0x54003000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG1",
        }],
        afio: None,
    },
    Peripheral {
        name: "RNG2",
        address: 0x4c003000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG2",
        }],
        afio: None,
    },
    Peripheral {
        name: "RTC",
        address: 0x5c004000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "LSCO",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TS",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "REFIN",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PI8",
                signal: "LSCO",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_WKUP_ALARM",
            },
            PeripheralInterrupt {
                signal: "TIMESTAMP",
                interrupt: "RTC_TIMESTAMP",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP_ALARM",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x4400a000,
        registers: None,
        rcc: None,
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
                pin: "PC5",
                signal: "D4",
                af: Some(6),
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
                pin: "PF10",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D4",
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
                pin: "PG13",
                signal: "CK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "MCLK_A",
                af: Some(6),
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
        name: "SAI2",
        address: 0x4400b000,
        registers: None,
        rcc: None,
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
                pin: "PA12",
                signal: "FS_B",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK_B",
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
                pin: "PE6",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "MCLK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "SCK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "FS_A",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI2",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI2",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI3",
        address: 0x4400c000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD0",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "FS_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "MCLK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "MCLK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SCK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "SD_B",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI3",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI3",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SAI4",
        address: 0x50027000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "FS_B",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "FS_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "D2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FS_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CK1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MCLK_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CK2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SCK_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "D1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SD_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SD_B",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MCLK_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CK2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SCK_A",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "MCLK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SCK_B",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "CK2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "SCK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CK1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "MCLK_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "D2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "FS_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SD_B",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI4",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI4",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SDMMC1",
        address: 0x58005000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CDIR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CDIR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
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
                pin: "PD2",
                signal: "CMD",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D123DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D7",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D0DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D123DIR",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CKIN",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D0DIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D2",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "D0DIR",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SDMMC2",
        address: 0x58007000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CMD",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CDIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "D0DIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CKIN",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CDIR",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA9",
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
                pin: "PB7",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CDIR",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0DIR",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D123DIR",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D123DIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D0DIR",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D0",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "D0DIR",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "CMD",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SDMMC3",
        address: 0x48004000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD0",
                signal: "CMD",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CKIN",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CDIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CMD",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "D0DIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "D123DIR",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CK",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPDIFRX1",
        address: 0x4000d000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "IN0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "IN0",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "IN1",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "IN2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "IN3",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPDIF_RX",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x44004000,
        registers: None,
        rcc: None,
        pins: &[
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
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
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
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "I2S_MCK",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI2",
        address: 0x4000b000,
        registers: None,
        rcc: None,
        pins: &[
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
                pin: "PA9",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: Some(7),
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
                pin: "PC1",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
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
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x4000c000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "NSS",
                af: Some(5),
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
                pin: "PA8",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SDO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: Some(7),
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
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "I2S_SDI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SDO",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "I2S_MCK",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "SCK",
                af: Some(5),
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
        name: "SPI4",
        address: 0x44005000,
        registers: None,
        rcc: None,
        pins: &[
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
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0x44009000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: Some(5),
            },
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
                pin: "PH5",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PJ10",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PJ11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PK0",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PK1",
                signal: "NSS",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI6",
        address: 0x5c001000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(7),
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
                pin: "PG8",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "NSS",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x50020000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TAMP",
        address: 0x5c00a000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "OUT3",
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
        address: 0x44000000,
        registers: None,
        rcc: None,
        pins: &[
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
                pin: "PA6",
                signal: "BKIN",
                af: Some(1),
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
                pin: "PE6",
                signal: "BKIN2",
                af: Some(1),
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
                pin: "PG4",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PJ10",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PJ11",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PJ8",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PJ9",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PK0",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PK1",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PK2",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
        name: "TIM12",
        address: 0x40006000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PH6",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CH2",
                af: Some(2),
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
        address: 0x40007000,
        registers: None,
        rcc: None,
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
        address: 0x40008000,
        registers: None,
        rcc: None,
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
        address: 0x44006000,
        registers: None,
        rcc: None,
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
                pin: "PE15",
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
        dma_channels: &[],
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
        address: 0x44007000,
        registers: None,
        rcc: None,
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
                pin: "PD10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN",
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
        address: 0x44008000,
        registers: None,
        rcc: None,
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
        registers: None,
        rcc: None,
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
                pin: "PB3",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "ETR",
                af: Some(1),
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
        name: "TIM3",
        address: 0x40001000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[],
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
        address: 0x40002000,
        registers: None,
        rcc: None,
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
        dma_channels: &[],
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
        address: 0x40003000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PH10",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[],
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
        address: 0x40004000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
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
        address: 0x40005000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
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
        address: 0x44001000,
        registers: None,
        rcc: None,
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
                pin: "PG3",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "BKIN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "CH4",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "BKIN",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "CH3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ10",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ11",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ6",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ7",
                signal: "CH2N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ8",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PJ9",
                signal: "CH1N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PK0",
                signal: "CH3",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PK1",
                signal: "CH3N",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PK2",
                signal: "BKIN",
                af: Some(3),
            },
        ],
        dma_channels: &[],
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
        name: "UART4",
        address: 0x40010000,
        registers: None,
        rcc: None,
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
                pin: "PA13",
                signal: "TX",
                af: Some(8),
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
                pin: "PB2",
                signal: "RX",
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
                pin: "PC8",
                signal: "TX",
                af: Some(6),
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
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART5",
        address: 0x40011000,
        registers: None,
        rcc: None,
        pins: &[
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
                pin: "PB5",
                signal: "RX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(8),
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
                pin: "PD2",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART7",
        address: 0x40018000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: Some(7),
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
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART8",
        address: 0x40019000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CTS",
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
            PeripheralPin {
                pin: "PE14",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PJ8",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PJ9",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
        afio: None,
    },
    Peripheral {
        name: "UID",
        address: 0x5c005234,
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
        address: 0x5c000000,
        registers: None,
        rcc: None,
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
                pin: "PB2",
                signal: "RX",
                af: Some(4),
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
                pin: "PG11",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ0",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ1",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ2",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ3",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "DE",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ5",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "CK",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PZ6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PZ7",
                signal: "TX",
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
        name: "USART2",
        address: 0x4000e000,
        registers: None,
        rcc: None,
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
            PeripheralPin {
                pin: "PE15",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART3",
        address: 0x4000f000,
        registers: None,
        rcc: None,
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
                pin: "PB12",
                signal: "RX",
                af: Some(8),
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
                pin: "PG8",
                signal: "DE",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "NSS",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART6",
        address: 0x44003000,
        registers: None,
        rcc: None,
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
                pin: "PE11",
                signal: "CK",
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
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
        afio: None,
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 0x49000000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "OTG",
            interrupt: "OTG",
        }],
        afio: None,
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x50025000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "WWDG1",
        address: 0x4000a000,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v2",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG1",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG1",
            },
        ],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG1",
        number: 0,
    },
    Interrupt {
        name: "PVD_AVD",
        number: 1,
    },
    Interrupt {
        name: "TAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP_ALARM",
        number: 3,
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
        name: "ADC1",
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
        name: "EXTI5",
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
        name: "EXTI10",
        number: 40,
    },
    Interrupt {
        name: "RTC_TIMESTAMP",
        number: 41,
    },
    Interrupt {
        name: "EXTI11",
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
        name: "TIM6",
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
        name: "ETH1",
        number: 61,
    },
    Interrupt {
        name: "ETH1_WKUP",
        number: 62,
    },
    Interrupt {
        name: "FDCAN_CAL",
        number: 63,
    },
    Interrupt {
        name: "EXTI6",
        number: 64,
    },
    Interrupt {
        name: "EXTI7",
        number: 65,
    },
    Interrupt {
        name: "EXTI8",
        number: 66,
    },
    Interrupt {
        name: "EXTI9",
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
        name: "USBH_OHCI",
        number: 74,
    },
    Interrupt {
        name: "USBH_EHCI",
        number: 75,
    },
    Interrupt {
        name: "EXTI12",
        number: 76,
    },
    Interrupt {
        name: "EXTI13",
        number: 77,
    },
    Interrupt {
        name: "DCMI",
        number: 78,
    },
    Interrupt {
        name: "CRYP1",
        number: 79,
    },
    Interrupt {
        name: "HASH1",
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
        name: "ADC2",
        number: 90,
    },
    Interrupt {
        name: "SAI2",
        number: 91,
    },
    Interrupt {
        name: "QUADSPI",
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
        name: "OTG",
        number: 98,
    },
    Interrupt {
        name: "IPCC_RX0",
        number: 100,
    },
    Interrupt {
        name: "IPCC_TX0",
        number: 101,
    },
    Interrupt {
        name: "DMAMUX1_OVR",
        number: 102,
    },
    Interrupt {
        name: "IPCC_RX1",
        number: 103,
    },
    Interrupt {
        name: "IPCC_TX1",
        number: 104,
    },
    Interrupt {
        name: "CRYP2",
        number: 105,
    },
    Interrupt {
        name: "HASH2",
        number: 106,
    },
    Interrupt {
        name: "I2C5_EV",
        number: 107,
    },
    Interrupt {
        name: "I2C5_ER",
        number: 108,
    },
    Interrupt {
        name: "GPU",
        number: 109,
    },
    Interrupt {
        name: "DFSDM1_FLT0",
        number: 110,
    },
    Interrupt {
        name: "DFSDM1_FLT1",
        number: 111,
    },
    Interrupt {
        name: "DFSDM1_FLT2",
        number: 112,
    },
    Interrupt {
        name: "DFSDM1_FLT3",
        number: 113,
    },
    Interrupt {
        name: "SAI3",
        number: 114,
    },
    Interrupt {
        name: "DFSDM1_FLT4",
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
        name: "TIM12",
        number: 119,
    },
    Interrupt {
        name: "MDIOS",
        number: 120,
    },
    Interrupt {
        name: "EXTI14",
        number: 121,
    },
    Interrupt {
        name: "MDMA",
        number: 122,
    },
    Interrupt {
        name: "DSI",
        number: 123,
    },
    Interrupt {
        name: "SDMMC2",
        number: 124,
    },
    Interrupt {
        name: "HSEM_IT2",
        number: 125,
    },
    Interrupt {
        name: "DFSDM1_FLT5",
        number: 126,
    },
    Interrupt {
        name: "EXTI15",
        number: 127,
    },
    Interrupt {
        name: "NCTIIRQ1",
        number: 128,
    },
    Interrupt {
        name: "NCTIIRQ2",
        number: 129,
    },
    Interrupt {
        name: "TIM13",
        number: 130,
    },
    Interrupt {
        name: "TIM14",
        number: 131,
    },
    Interrupt {
        name: "DAC",
        number: 132,
    },
    Interrupt {
        name: "RNG1",
        number: 133,
    },
    Interrupt {
        name: "RNG2",
        number: 134,
    },
    Interrupt {
        name: "I2C6_EV",
        number: 135,
    },
    Interrupt {
        name: "I2C6_ER",
        number: 136,
    },
    Interrupt {
        name: "SDMMC3",
        number: 137,
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
        name: "LPTIM4",
        number: 140,
    },
    Interrupt {
        name: "LPTIM5",
        number: 141,
    },
    Interrupt {
        name: "ETH1_LPI",
        number: 142,
    },
    Interrupt {
        name: "MPU_SEV",
        number: 144,
    },
    Interrupt {
        name: "RCC_WAKEUP",
        number: 145,
    },
    Interrupt {
        name: "SAI4",
        number: 146,
    },
    Interrupt {
        name: "DTS",
        number: 147,
    },
    Interrupt {
        name: "WAKEUP_PIN",
        number: 149,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
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
    Pin { name: "PB8" },
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
    Pin { name: "PC10" },
    Pin { name: "PC11" },
    Pin { name: "PC12" },
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
    Pin { name: "PD0" },
    Pin { name: "PD1" },
    Pin { name: "PD2" },
    Pin { name: "PD3" },
    Pin { name: "PD4" },
    Pin { name: "PD5" },
    Pin { name: "PD6" },
    Pin { name: "PD7" },
    Pin { name: "PD8" },
    Pin { name: "PD9" },
    Pin { name: "PD10" },
    Pin { name: "PD11" },
    Pin { name: "PD12" },
    Pin { name: "PD13" },
    Pin { name: "PD14" },
    Pin { name: "PD15" },
    Pin { name: "PE0" },
    Pin { name: "PE1" },
    Pin { name: "PE2" },
    Pin { name: "PE3" },
    Pin { name: "PE4" },
    Pin { name: "PE5" },
    Pin { name: "PE6" },
    Pin { name: "PE7" },
    Pin { name: "PE8" },
    Pin { name: "PE9" },
    Pin { name: "PE10" },
    Pin { name: "PE11" },
    Pin { name: "PE12" },
    Pin { name: "PE13" },
    Pin { name: "PE14" },
    Pin { name: "PE15" },
    Pin { name: "PF0" },
    Pin { name: "PF1" },
    Pin { name: "PF2" },
    Pin { name: "PF3" },
    Pin { name: "PF4" },
    Pin { name: "PF5" },
    Pin { name: "PF6" },
    Pin { name: "PF7" },
    Pin { name: "PF8" },
    Pin { name: "PF9" },
    Pin { name: "PF10" },
    Pin { name: "PF11" },
    Pin { name: "PF12" },
    Pin { name: "PF13" },
    Pin { name: "PF14" },
    Pin { name: "PF15" },
    Pin { name: "PG0" },
    Pin { name: "PG1" },
    Pin { name: "PG2" },
    Pin { name: "PG3" },
    Pin { name: "PG4" },
    Pin { name: "PG5" },
    Pin { name: "PG6" },
    Pin { name: "PG7" },
    Pin { name: "PG8" },
    Pin { name: "PG9" },
    Pin { name: "PG10" },
    Pin { name: "PG11" },
    Pin { name: "PG12" },
    Pin { name: "PG13" },
    Pin { name: "PG14" },
    Pin { name: "PG15" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
    Pin { name: "PH2" },
    Pin { name: "PH3" },
    Pin { name: "PH4" },
    Pin { name: "PH5" },
    Pin { name: "PH6" },
    Pin { name: "PH7" },
    Pin { name: "PH8" },
    Pin { name: "PH9" },
    Pin { name: "PH10" },
    Pin { name: "PH11" },
    Pin { name: "PH12" },
    Pin { name: "PH13" },
    Pin { name: "PH14" },
    Pin { name: "PH15" },
    Pin { name: "PI0" },
    Pin { name: "PI1" },
    Pin { name: "PI2" },
    Pin { name: "PI3" },
    Pin { name: "PI4" },
    Pin { name: "PI5" },
    Pin { name: "PI6" },
    Pin { name: "PI7" },
    Pin { name: "PI8" },
    Pin { name: "PI9" },
    Pin { name: "PI10" },
    Pin { name: "PI11" },
    Pin { name: "PI12" },
    Pin { name: "PI13" },
    Pin { name: "PI14" },
    Pin { name: "PI15" },
    Pin { name: "PJ0" },
    Pin { name: "PJ1" },
    Pin { name: "PJ2" },
    Pin { name: "PJ3" },
    Pin { name: "PJ4" },
    Pin { name: "PJ5" },
    Pin { name: "PJ6" },
    Pin { name: "PJ7" },
    Pin { name: "PJ8" },
    Pin { name: "PJ9" },
    Pin { name: "PJ10" },
    Pin { name: "PJ11" },
    Pin { name: "PJ12" },
    Pin { name: "PJ13" },
    Pin { name: "PJ14" },
    Pin { name: "PJ15" },
    Pin { name: "PK0" },
    Pin { name: "PK1" },
    Pin { name: "PK2" },
    Pin { name: "PK3" },
    Pin { name: "PK4" },
    Pin { name: "PK5" },
    Pin { name: "PK6" },
    Pin { name: "PK7" },
    Pin { name: "PZ0" },
    Pin { name: "PZ1" },
    Pin { name: "PZ2" },
    Pin { name: "PZ3" },
    Pin { name: "PZ4" },
    Pin { name: "PZ5" },
    Pin { name: "PZ6" },
    Pin { name: "PZ7" },
];
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/bsec_v2.rs"]
pub mod bsec;
#[path = "../registers/can_fdcan_v2.rs"]
pub mod can;
#[path = "../registers/cec_v2.rs"]
pub mod cec;
#[path = "../registers/cryp_v2.rs"]
pub mod cryp;
#[path = "../registers/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../registers/dmamux_v1.rs"]
pub mod dmamux;
#[path = "../registers/dts_v1.rs"]
pub mod dts;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/quadspi_v1.rs"]
pub mod quadspi;
#[path = "../registers/rcc_mp1.rs"]
pub mod rcc;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
