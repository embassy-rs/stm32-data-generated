
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40022000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "ADC12SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "INP13",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP4",
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
            PeripheralPin {
                pin: "PF3",
                signal: "INP16",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "INP18",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "INP7",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x40022300,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40022100,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "ADC12SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ADC12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ADC12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "INP0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "INN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "INP11",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "INP12",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "INN12",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "INP13",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP14",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INP18",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "INP5",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "INP10",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INN4",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "INP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "INP4",
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
            PeripheralPin {
                pin: "PF6",
                signal: "INP15",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "INN5",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "INP9",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "INN3",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "INP7",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADF1",
        address: 0x42026000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "ADF1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADF1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADF1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CCK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SDI0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CCK0",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CCK1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CCK0",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "FLT0",
            interrupt: "ADF1_FLT0",
        }],
    },
    Peripheral {
        name: "BSEC",
        address: 0x46009000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4HENR",
                field: "BSECEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x46024c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "CRCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CSI",
        address: 0x48006000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "CSIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "CSIRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x44001000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCMI",
        address: 0x48028400,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
            ir: &dcmi::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "VSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "VSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "D5",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI_PSSI",
        }],
    },
    Peripheral {
        name: "DCMIPP",
        address: 0x48002000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
                field: "DCMIPPSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "DCMIPPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "DCMIPPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "VSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "VSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D15",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "HSYNC",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "PIXCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "D5",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CSI",
                interrupt: "CSI",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "DCMIPP",
            },
        ],
    },
    Peripheral {
        name: "DMA2D",
        address: 0x48021000,
        registers: Some(PeripheralRegisters {
            kind: "dma2d",
            version: "v1",
            block: "DMA2D",
            ir: &dma2d::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "DMA2DEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "DMA2DRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DMA2D",
        }],
    },
    Peripheral {
        name: "DTS",
        address: 0x4600a000,
        registers: Some(PeripheralRegisters {
            kind: "dts",
            version: "v1",
            block: "DTS",
            ir: &dts::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4HENR",
                field: "DTSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4HRSTR",
                field: "DTSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETH1",
        address: 0x48036000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "ETH1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "ETH1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD1",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "PHY_INTN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CRS_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "RGMII_RX_CTL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "RX_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "RGMII_TX_CTL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "RGMII_TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "RGMII_TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "RGMII_RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "RGMII_RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "RGMII_CLK125",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "PPS_OUT",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "COL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "RGMII_RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "RGMII_RXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "RXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "PHY_INTN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "RX_CLK",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ETH1",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH1",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 0x46025000,
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
        name: "FDCAN1",
        address: 0x4000a000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
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
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "RX",
                af: Some(6),
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
        address: 0x4000a400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
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
                pin: "PB12",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "TX",
                af: Some(6),
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
        address: 0x4000e800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
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
                pin: "PE11",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "RX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "TX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "RX",
                af: Some(6),
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
        name: "FDCANRAM1",
        address: 0x4000c000,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FDCANRAM2",
        address: 0x4000c350,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FDCANRAM3",
        address: 0x4000c6a0,
        registers: Some(PeripheralRegisters {
            kind: "fdcanram",
            version: "v1",
            block: "FDCANRAM",
            ir: &fdcanram::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FMC",
        address: 0x48024000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR3",
                field: "FMCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "FMCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "FMCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "D7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "DA7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DA6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "D1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "D5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "DA5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "A17",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "ALE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NOE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DA3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "D2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "DA2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "A23",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "NBL1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "DA13",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "DA14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "DA15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NE4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "A22",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "A6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "A23",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "A7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "A19",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "A3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "DA8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A21",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A5",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DA4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "A9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "A8",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "A0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "A16",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CLE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "A10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "A11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "D6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "DA6",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "A1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "A17",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "ALE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "A18",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "A2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "NBL0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "SDCLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "DA9",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "DA10",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "A15",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "BA1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SDNWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SDNRAS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SDNCAS",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "NWE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "SDNE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SDCKE0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "D11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "DA11",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "D12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "DA12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SDNE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SDCKE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "A20",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "A4",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "A12",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "A14",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "BA0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "NWAIT",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "NL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "NWE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "A19",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "A16",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "CLE",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "A18",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "NE1",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "NCE",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "NE2",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CLK",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "A21",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "A20",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "NE3",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "DA9",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PN0",
                signal: "A25",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN1",
                signal: "A24",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN2",
                signal: "A23",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PN3",
                signal: "A22",
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
        address: 0x48030000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "GFXMMUEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "GFXMMURST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GFXMMU",
        }],
    },
    Peripheral {
        name: "GFXTIM",
        address: 0x48004000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "GFXTIMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "GFXTIMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "FCKCAL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "LCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "TE",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "FCKCAL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "LCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "FCKCAL",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "LCKCAL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "FCKCAL",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "LCKCAL",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TE",
                af: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GFXTIM",
        }],
    },
    Peripheral {
        name: "GPDMA1",
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
                signal: "CH10",
                interrupt: "GPDMA1_CHANNEL10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "GPDMA1_CHANNEL11",
            },
            PeripheralInterrupt {
                signal: "CH12",
                interrupt: "GPDMA1_CHANNEL12",
            },
            PeripheralInterrupt {
                signal: "CH13",
                interrupt: "GPDMA1_CHANNEL13",
            },
            PeripheralInterrupt {
                signal: "CH15",
                interrupt: "GPDMA1_CHANNEL15",
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
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "GPDMA1_CHANNEL8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "GPDMA1_CHANNEL9",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x46020000,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x46020400,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x46020800,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x46020c00,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 0x46021000,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 0x46021400,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOG",
        address: 0x46021800,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 0x46021c00,
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPION",
        address: 0x46023400,
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
                field: "GPIONEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIONRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOO",
        address: 0x46023800,
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
                field: "GPIOOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOORST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOP",
        address: 0x46023c00,
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
                field: "GPIOPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOQ",
        address: 0x46024000,
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
                field: "GPIOQEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "GPIOQRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPU2D",
        address: 0x48034000,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "ER",
            interrupt: "GPU2D_ER",
        }],
    },
    Peripheral {
        name: "HASH",
        address: 0x44020400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
    },
    Peripheral {
        name: "HPDMA1",
        address: 0x48020000,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
            ir: &gpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "HPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "HPDMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "HPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "HPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH10",
                interrupt: "HPDMA1_CHANNEL10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "HPDMA1_CHANNEL11",
            },
            PeripheralInterrupt {
                signal: "CH12",
                interrupt: "HPDMA1_CHANNEL12",
            },
            PeripheralInterrupt {
                signal: "CH13",
                interrupt: "HPDMA1_CHANNEL13",
            },
            PeripheralInterrupt {
                signal: "CH15",
                interrupt: "HPDMA1_CHANNEL15",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "HPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "HPDMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "HPDMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "HPDMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "HPDMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "HPDMA1_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "HPDMA1_CHANNEL8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "HPDMA1_CHANNEL9",
            },
        ],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: None,
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
                pin: "PB5",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "SCL",
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
        address: 0x40005800,
        registers: None,
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
                pin: "PD14",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
        address: 0x40005c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
        address: 0x46001c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I2C4SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "I2C4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
        name: "I3C1",
        address: 0x40006000,
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
                pin: "PA5",
                signal: "SCL",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCL",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SDA",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "SCL",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I3C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I3C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I3C2",
        address: 0x40006400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "I3C2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I3C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I3C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCL",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SDA",
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I3C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I3C2_EV",
            },
        ],
    },
    Peripheral {
        name: "ICACHE",
        address: 0x48035000,
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
        address: 0x46004800,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v3",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "IWDG",
        }],
    },
    Peripheral {
        name: "JPEG",
        address: 0x48023000,
        registers: Some(PeripheralRegisters {
            kind: "jpeg",
            version: "v1",
            block: "JPEG",
            ir: &jpeg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "JPEGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "JPEGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "JPEG",
        }],
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x46002400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop2,
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
                pin: "PB12",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "IN1",
                af: Some(3),
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
        address: 0x46002800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "LPTIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "LPTIM3RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "ETR",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x46000c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "LPUART1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[
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
                signal: "RTS",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "TX",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "RX",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "LTDC",
        address: 0x48001000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR4",
                field: "LTDCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "LTDCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "LTDCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "G3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "G2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "R5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "B7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CLK",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "B7",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "HSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "R4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "B6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "B5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G5",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "B2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "R3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "B3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "R2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "R7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "R1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "R4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "R3",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "VSYNC",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "R6",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "VSYNC",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "G1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "G4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "G0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "DE",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "B1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "B0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "R0",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "G7",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PN10",
                signal: "B4",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PN11",
                signal: "B6",
                af: Some(14),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ERR",
                interrupt: "LTDC_LO_ERR",
            },
            PeripheralInterrupt {
                signal: "LO",
                interrupt: "LTDC_LO",
            },
        ],
    },
    Peripheral {
        name: "MDF1",
        address: 0x42025000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR5",
                field: "MDF1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "MDF1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "MDF1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "SDI2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SDI1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CKI1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CCK1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDI1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CKI3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SDI3",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SDI1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "SDI4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CKI5",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SDI5",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CCK0",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CKI2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "CKI0",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "SDI0",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CKI4",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "SDI3",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT0",
                interrupt: "MDF1_FLT0",
            },
            PeripheralInterrupt {
                signal: "FLT1",
                interrupt: "MDF1_FLT1",
            },
            PeripheralInterrupt {
                signal: "FLT2",
                interrupt: "MDF1_FLT2",
            },
            PeripheralInterrupt {
                signal: "FLT3",
                interrupt: "MDF1_FLT3",
            },
            PeripheralInterrupt {
                signal: "FLT4",
                interrupt: "MDF1_FLT4",
            },
            PeripheralInterrupt {
                signal: "FLT5",
                interrupt: "MDF1_FLT5",
            },
        ],
    },
    Peripheral {
        name: "MDIOS",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "mdios",
            version: "v1",
            block: "MDIOS",
            ir: &mdios::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "MDIOSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "MDIOSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "MDC",
                af: Some(10),
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
        ],
    },
    Peripheral {
        name: "PSSI",
        address: 0x48026400,
        registers: Some(PeripheralRegisters {
            kind: "pssi",
            version: "v1",
            block: "PSSI",
            ir: &pssi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR7",
                field: "PSSISEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "PSSIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "PSSIRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PDCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D14",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "D9",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "PDCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "D0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D8",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D5",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D15",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D15",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "D13",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "D10",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "DE",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "PDCK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "D12",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "D11",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "D5",
                af: Some(10),
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
        address: 0x46024800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK4",
            kernel_clock: Clock("HCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "PWRRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "PVD_IN",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RAMCFG",
        address: 0x42023000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "RAMCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "RAMCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BKP",
                interrupt: "BKP_ECC",
            },
            PeripheralInterrupt {
                signal: "ECC",
                interrupt: "BKP_ECC",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x46028000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "n6",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
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
                pin: "PC9",
                signal: "MCO_2",
                af: Some(0),
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "RNG",
        address: 0x44020000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "RNGRST",
            }),
            stop_mode: StopMode::Stop1,
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
        address: 0x46004000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR7",
                field: "RTCSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "RTCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "RTCRST",
            }),
            stop_mode: StopMode::Standby,
        }),
        pins: &[
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
            PeripheralPin {
                pin: "PG8",
                signal: "REFIN",
                af: Some(0),
            },
        ],
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
        name: "SAI1",
        address: 0x42005800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR7",
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
                pin: "PA3",
                signal: "SD_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "FS_A",
                af: Some(6),
            },
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
                pin: "PB6",
                signal: "CK2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SD_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
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
                pin: "PF10",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "SCK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MCLK_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "FS_B",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SCK_B",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI1_A",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI1_B",
            },
        ],
    },
    Peripheral {
        name: "SAI2",
        address: 0x42005c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR7",
                field: "SAI2SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SAI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SAI2RST",
            }),
            stop_mode: StopMode::Stop1,
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
                pin: "PA12",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "FS_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MCLK_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "FS_B",
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
                pin: "PE7",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SD_B",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "SD_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "FS_A",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "MCLK_B",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "A",
                interrupt: "SAI2_A",
            },
            PeripheralInterrupt {
                signal: "B",
                interrupt: "SAI2_B",
            },
        ],
    },
    Peripheral {
        name: "SDMMC1",
        address: 0x48027000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR8",
                field: "SDMMC1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "SDMMC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "SDMMC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "CDIR",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "D5",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0DIR",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D123DIR",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D0",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "CMD",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CKIN",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D4",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDMMC1",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x42003000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
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
                pin: "PD10",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RDY",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "RDY",
                af: Some(10),
            },
        ],
        dma_channels: &[],
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
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
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
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
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
                pin: "PC1",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
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
                pin: "PD7",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NSS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "I2S_WS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "NSS",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SPI4",
        address: 0x42003400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
                field: "SPI4SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB7",
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
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RDY",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "RDY",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
    },
    Peripheral {
        name: "SPI5",
        address: 0x42005000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
                field: "SPI5SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RDY",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
    },
    Peripheral {
        name: "SPI6",
        address: 0x46001400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR9",
                field: "SPI6SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "SPI6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "SPI6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "I2S_WS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SDO",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_CK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "I2S_WS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "NSS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "I2S_MCK",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "MOSI",
                af: Some(5),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x46008000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4HENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4HRSTR",
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
        address: 0x46004400,
        registers: None,
        rcc: None,
        pins: &[
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
                pin: "PD4",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "OUT8",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "OUT6",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "OUT5",
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
        address: 0x42000000,
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
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH3",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "BKIN2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH2",
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
                pin: "PE2",
                signal: "CH2N",
                af: Some(13),
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
                pin: "PF2",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "ETR",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH3",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH2N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "CH4N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CH4",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "CH3N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "ETR",
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
        name: "TIM10",
        address: 0x40003000,
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
        pins: &[PeripheralPin {
            pin: "PA5",
            signal: "CH1",
            af: Some(10),
        }],
        dma_channels: &[],
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
                signal: "TRG",
                interrupt: "TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM11",
        address: 0x40003400,
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
        pins: &[PeripheralPin {
            pin: "PA8",
            signal: "CH1",
            af: Some(9),
        }],
        dma_channels: &[],
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
                signal: "TRG",
                interrupt: "TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM11",
            },
        ],
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
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
                pin: "PG0",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "CH2",
                af: Some(2),
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
        address: 0x40001c00,
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
                pin: "PA6",
                signal: "CH1",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "CH1",
                af: Some(10),
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
        address: 0x40002000,
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
                pin: "PA7",
                signal: "CH1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CH1",
                af: Some(11),
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
        address: 0x42004000,
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
                pin: "PB0",
                signal: "CH1N",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CH1N",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH1N",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "BKIN",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "BKIN",
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
        address: 0x42004400,
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
                pin: "PA3",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CH1",
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
        address: 0x42004800,
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
                pin: "PB5",
                signal: "BKIN",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH1N",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CH1N",
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
        address: 0x42003c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM18EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM18RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM18",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM18",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM18",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM18",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM18",
            },
        ],
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
                pin: "PA15",
                signal: "CH1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
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
                pin: "PF14",
                signal: "CH2",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH4",
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
                pin: "PF7",
                signal: "CH3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "ETR",
                af: Some(2),
            },
        ],
        dma_channels: &[],
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
        address: 0x40000800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CH3",
                af: Some(2),
            },
        ],
        dma_channels: &[],
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
        address: 0x40000c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
                pin: "PF4",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH4",
                af: Some(2),
            },
        ],
        dma_channels: &[],
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
        dma_channels: &[],
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
        dma_channels: &[],
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
        name: "TIM9",
        address: 0x42004c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
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
                pin: "PF7",
                signal: "CH1",
                af: Some(3),
            },
        ],
        dma_channels: &[],
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
                signal: "TRG",
                interrupt: "TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM9",
            },
        ],
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PC1",
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
                pin: "PF7",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "RX",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PB12",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "DE",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "TX",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "RX",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UART7",
        address: 0x40007800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "UART7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "UART7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "RTS",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "TX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CTS",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "UART9",
        address: 0x42001800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "UART9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "UART9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD0",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "RX",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART9",
        }],
    },
    Peripheral {
        name: "UCPD1",
        address: 0x4000fc00,
        registers: Some(PeripheralRegisters {
            kind: "ucpd",
            version: "v1",
            block: "UCPD",
            ir: &ucpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PC8",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "FRSTX1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "FRSTX2",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "FRSTX1",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "FRSTX1",
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
        }],
    },
    Peripheral {
        name: "UID",
        address: 0x46009014,
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
        address: 0x42001000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
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
                signal: "RX",
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
                pin: "PB3",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "RX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "TX",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "RTS",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "RX",
                af: Some(4),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PD5",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "RTS",
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PE0",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CK",
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 0x42001400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART6RST",
            }),
            stop_mode: StopMode::Stop1,
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
                pin: "PC9",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TX",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
    },
    Peripheral {
        name: "USB1_OTG_HS",
        address: 0x48040000,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "USB1_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "USB1_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USB1_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB1_OTG_HS",
            },
        ],
    },
    Peripheral {
        name: "USB2_OTG_HS",
        address: 0x48080000,
        registers: Some(PeripheralRegisters {
            kind: "otg",
            version: "v1",
            block: "OTG",
            ir: &otg::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "USB2_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "USB2_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USB2_OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB2_OTG_HS",
            },
        ],
    },
    Peripheral {
        name: "VENC",
        address: 0x48005000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK5",
            kernel_clock: Clock("PCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "APB5ENR",
                field: "VENCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB5RSTR",
                field: "VENCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x46003c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK4",
            kernel_clock: Clock("PCLK4"),
            enable: Some(PeripheralRccRegister {
                register: "APB4LENR",
                field: "VREFBUFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB4LRSTR",
                field: "VREFBUFRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
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
                register: "APB1LENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "WWDGRST",
            }),
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
    Peripheral {
        name: "XSPI3",
        address: 0x80000000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR6",
                field: "XSPI3SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "XSPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "XSPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XSPI3",
        }],
    },
    Peripheral {
        name: "XSPIM",
        address: 0x4802b400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK5",
            kernel_clock: Clock("HCLK5"),
            enable: Some(PeripheralRccRegister {
                register: "AHB5ENR",
                field: "XSPIMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB5RSTR",
                field: "XSPIMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PN0",
                signal: "P2_DQS0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN1",
                signal: "P2_NCS1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN10",
                signal: "P2_IO6",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN11",
                signal: "P2_IO7",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN12",
                signal: "P2_NCS2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN2",
                signal: "P2_IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN3",
                signal: "P2_IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN4",
                signal: "P2_IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN5",
                signal: "P2_IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN6",
                signal: "P2_CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN7",
                signal: "P2_NCLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN8",
                signal: "P2_IO4",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PN9",
                signal: "P2_IO5",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "PVD_PVM",
        number: 0,
    },
    Interrupt { name: "DTS", number: 2 },
    Interrupt { name: "RCC", number: 3 },
    Interrupt {
        name: "LOCKUP",
        number: 4,
    },
    Interrupt {
        name: "CACHE_ECC",
        number: 5,
    },
    Interrupt {
        name: "TCM_ECC",
        number: 6,
    },
    Interrupt {
        name: "BKP_ECC",
        number: 7,
    },
    Interrupt { name: "FPU", number: 8 },
    Interrupt {
        name: "RTC_S",
        number: 10,
    },
    Interrupt {
        name: "TAMP",
        number: 11,
    },
    Interrupt {
        name: "RIFSC_TAMPER",
        number: 12,
    },
    Interrupt {
        name: "IAC",
        number: 13,
    },
    Interrupt {
        name: "RCC_S",
        number: 14,
    },
    Interrupt {
        name: "RTC",
        number: 16,
    },
    Interrupt {
        name: "IWDG",
        number: 18,
    },
    Interrupt {
        name: "WWDG",
        number: 19,
    },
    Interrupt {
        name: "EXTI0",
        number: 20,
    },
    Interrupt {
        name: "EXTI1",
        number: 21,
    },
    Interrupt {
        name: "EXTI2",
        number: 22,
    },
    Interrupt {
        name: "EXTI3",
        number: 23,
    },
    Interrupt {
        name: "EXTI4",
        number: 24,
    },
    Interrupt {
        name: "EXTI5",
        number: 25,
    },
    Interrupt {
        name: "EXTI6",
        number: 26,
    },
    Interrupt {
        name: "EXTI7",
        number: 27,
    },
    Interrupt {
        name: "EXTI8",
        number: 28,
    },
    Interrupt {
        name: "EXTI9",
        number: 29,
    },
    Interrupt {
        name: "EXTI10",
        number: 30,
    },
    Interrupt {
        name: "EXTI11",
        number: 31,
    },
    Interrupt {
        name: "EXTI12",
        number: 32,
    },
    Interrupt {
        name: "EXTI13",
        number: 33,
    },
    Interrupt {
        name: "EXTI14",
        number: 34,
    },
    Interrupt {
        name: "EXTI15",
        number: 35,
    },
    Interrupt {
        name: "PKA",
        number: 38,
    },
    Interrupt {
        name: "HASH",
        number: 39,
    },
    Interrupt {
        name: "RNG",
        number: 40,
    },
    Interrupt {
        name: "ADC1_2",
        number: 46,
    },
    Interrupt {
        name: "CSI",
        number: 47,
    },
    Interrupt {
        name: "DCMIPP",
        number: 48,
    },
    Interrupt {
        name: "PAHB_ERR",
        number: 52,
    },
    Interrupt {
        name: "LTDC_LO",
        number: 58,
    },
    Interrupt {
        name: "LTDC_LO_ERR",
        number: 59,
    },
    Interrupt {
        name: "DMA2D",
        number: 60,
    },
    Interrupt {
        name: "JPEG",
        number: 61,
    },
    Interrupt {
        name: "VENC",
        number: 62,
    },
    Interrupt {
        name: "GFXMMU",
        number: 63,
    },
    Interrupt {
        name: "GFXTIM",
        number: 64,
    },
    Interrupt {
        name: "GPU2D",
        number: 65,
    },
    Interrupt {
        name: "GPU2D_ER",
        number: 66,
    },
    Interrupt {
        name: "ICACHE",
        number: 67,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL0",
        number: 68,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL1",
        number: 69,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL2",
        number: 70,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL3",
        number: 71,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL4",
        number: 72,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL5",
        number: 73,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL6",
        number: 74,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL7",
        number: 75,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL8",
        number: 76,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL9",
        number: 77,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL10",
        number: 78,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL11",
        number: 79,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL12",
        number: 80,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL13",
        number: 81,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL14",
        number: 82,
    },
    Interrupt {
        name: "HPDMA1_CHANNEL15",
        number: 83,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL0",
        number: 84,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 85,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 86,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 87,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 88,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 89,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 90,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 91,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL8",
        number: 92,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL9",
        number: 93,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL10",
        number: 94,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL11",
        number: 95,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL12",
        number: 96,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL13",
        number: 97,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL14",
        number: 98,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL15",
        number: 99,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 100,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 101,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 102,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 103,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 104,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 105,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 106,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 107,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 108,
    },
    Interrupt {
        name: "I3C1_ER",
        number: 109,
    },
    Interrupt {
        name: "I3C2_EV",
        number: 110,
    },
    Interrupt {
        name: "I3C2_ER",
        number: 111,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 112,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 113,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 114,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 115,
    },
    Interrupt {
        name: "TIM2",
        number: 116,
    },
    Interrupt {
        name: "TIM3",
        number: 117,
    },
    Interrupt {
        name: "TIM4",
        number: 118,
    },
    Interrupt {
        name: "TIM5",
        number: 119,
    },
    Interrupt {
        name: "TIM6",
        number: 120,
    },
    Interrupt {
        name: "TIM7",
        number: 121,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 122,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 123,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 124,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 125,
    },
    Interrupt {
        name: "TIM9",
        number: 126,
    },
    Interrupt {
        name: "TIM10",
        number: 127,
    },
    Interrupt {
        name: "TIM11",
        number: 128,
    },
    Interrupt {
        name: "TIM12",
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
        name: "TIM15",
        number: 132,
    },
    Interrupt {
        name: "TIM16",
        number: 133,
    },
    Interrupt {
        name: "TIM17",
        number: 134,
    },
    Interrupt {
        name: "TIM18",
        number: 135,
    },
    Interrupt {
        name: "LPTIM1",
        number: 136,
    },
    Interrupt {
        name: "LPTIM2",
        number: 137,
    },
    Interrupt {
        name: "LPTIM3",
        number: 138,
    },
    Interrupt {
        name: "LPTIM4",
        number: 139,
    },
    Interrupt {
        name: "LPTIM5",
        number: 140,
    },
    Interrupt {
        name: "ADF1_FLT0",
        number: 141,
    },
    Interrupt {
        name: "MDF1_FLT0",
        number: 142,
    },
    Interrupt {
        name: "MDF1_FLT1",
        number: 143,
    },
    Interrupt {
        name: "MDF1_FLT2",
        number: 144,
    },
    Interrupt {
        name: "MDF1_FLT3",
        number: 145,
    },
    Interrupt {
        name: "MDF1_FLT4",
        number: 146,
    },
    Interrupt {
        name: "MDF1_FLT5",
        number: 147,
    },
    Interrupt {
        name: "SAI1_A",
        number: 148,
    },
    Interrupt {
        name: "SAI1_B",
        number: 149,
    },
    Interrupt {
        name: "SAI2_A",
        number: 150,
    },
    Interrupt {
        name: "SAI2_B",
        number: 151,
    },
    Interrupt {
        name: "SPDIFRX1",
        number: 152,
    },
    Interrupt {
        name: "SPI1",
        number: 153,
    },
    Interrupt {
        name: "SPI2",
        number: 154,
    },
    Interrupt {
        name: "SPI3",
        number: 155,
    },
    Interrupt {
        name: "SPI4",
        number: 156,
    },
    Interrupt {
        name: "SPI5",
        number: 157,
    },
    Interrupt {
        name: "SPI6",
        number: 158,
    },
    Interrupt {
        name: "USART1",
        number: 159,
    },
    Interrupt {
        name: "USART2",
        number: 160,
    },
    Interrupt {
        name: "USART3",
        number: 161,
    },
    Interrupt {
        name: "UART4",
        number: 162,
    },
    Interrupt {
        name: "UART5",
        number: 163,
    },
    Interrupt {
        name: "USART6",
        number: 164,
    },
    Interrupt {
        name: "UART7",
        number: 165,
    },
    Interrupt {
        name: "UART8",
        number: 166,
    },
    Interrupt {
        name: "UART9",
        number: 167,
    },
    Interrupt {
        name: "USART10",
        number: 168,
    },
    Interrupt {
        name: "LPUART1",
        number: 169,
    },
    Interrupt {
        name: "XSPI1",
        number: 170,
    },
    Interrupt {
        name: "XSPI2",
        number: 171,
    },
    Interrupt {
        name: "XSPI3",
        number: 172,
    },
    Interrupt {
        name: "FMC",
        number: 173,
    },
    Interrupt {
        name: "SDMMC1",
        number: 174,
    },
    Interrupt {
        name: "SDMMC2",
        number: 175,
    },
    Interrupt {
        name: "UCPD1",
        number: 176,
    },
    Interrupt {
        name: "USB1_OTG_HS",
        number: 177,
    },
    Interrupt {
        name: "USB2_OTG_HS",
        number: 178,
    },
    Interrupt {
        name: "ETH1",
        number: 179,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 180,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 181,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 182,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 183,
    },
    Interrupt {
        name: "FDCAN3_IT0",
        number: 184,
    },
    Interrupt {
        name: "FDCAN3_IT1",
        number: 185,
    },
    Interrupt {
        name: "FDCAN_CU",
        number: 186,
    },
    Interrupt {
        name: "MDIOS",
        number: 187,
    },
    Interrupt {
        name: "DCMI_PSSI",
        number: 188,
    },
    Interrupt {
        name: "WAKEUP_PIN",
        number: 189,
    },
    Interrupt {
        name: "CTI_INT0",
        number: 190,
    },
    Interrupt {
        name: "CTI_INT1",
        number: 191,
    },
    Interrupt {
        name: "LTDC_UP",
        number: 193,
    },
    Interrupt {
        name: "LTDC_UP_ERR",
        number: 194,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA0" },
    Pin { name: "PA1" },
    Pin { name: "PA2" },
    Pin { name: "PA3" },
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
    Pin { name: "PB2" },
    Pin { name: "PB3" },
    Pin { name: "PB4" },
    Pin { name: "PB5" },
    Pin { name: "PB6" },
    Pin { name: "PB7" },
    Pin { name: "PB10" },
    Pin { name: "PB11" },
    Pin { name: "PB12" },
    Pin { name: "PC1" },
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
    Pin { name: "PF2" },
    Pin { name: "PF3" },
    Pin { name: "PF4" },
    Pin { name: "PF5" },
    Pin { name: "PF6" },
    Pin { name: "PF7" },
    Pin { name: "PF8" },
    Pin { name: "PF10" },
    Pin { name: "PF11" },
    Pin { name: "PF12" },
    Pin { name: "PF13" },
    Pin { name: "PF14" },
    Pin { name: "PF15" },
    Pin { name: "PG0" },
    Pin { name: "PG1" },
    Pin { name: "PG2" },
    Pin { name: "PG8" },
    Pin { name: "PG10" },
    Pin { name: "PG12" },
    Pin { name: "PG13" },
    Pin { name: "PG14" },
    Pin { name: "PG15" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
    Pin { name: "PH2" },
    Pin { name: "PH9" },
    Pin { name: "PN0" },
    Pin { name: "PN1" },
    Pin { name: "PN2" },
    Pin { name: "PN3" },
    Pin { name: "PN4" },
    Pin { name: "PN5" },
    Pin { name: "PN6" },
    Pin { name: "PN7" },
    Pin { name: "PN8" },
    Pin { name: "PN9" },
    Pin { name: "PN10" },
    Pin { name: "PN11" },
    Pin { name: "PN12" },
];
#[path = "../registers/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../registers/dma2d_v1.rs"]
pub mod dma2d;
#[path = "../registers/dts_v1.rs"]
pub mod dts;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/gpdma_v1.rs"]
pub mod gpdma;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/iwdg_v3.rs"]
pub mod iwdg;
#[path = "../registers/jpeg_v1.rs"]
pub mod jpeg;
#[path = "../registers/mdios_v1.rs"]
pub mod mdios;
#[path = "../registers/otg_v1.rs"]
pub mod otg;
#[path = "../registers/pssi_v1.rs"]
pub mod pssi;
#[path = "../registers/rcc_n6.rs"]
pub mod rcc;
#[path = "../registers/spi_v4.rs"]
pub mod spi;
#[path = "../registers/ucpd_v1.rs"]
pub mod ucpd;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
#[path = "../registers/wwdg_v2.rs"]
pub mod wwdg;
