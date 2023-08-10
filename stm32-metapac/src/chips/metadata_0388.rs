
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1107460096,
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(0),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1107460352,
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(1),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1107460864,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AES",
        address: 1108082688,
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
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(110),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(110),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
    },
    Peripheral {
        name: "CEC",
        address: 1073770496,
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
        name: "CORDIC",
        address: 1073887232,
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(115),
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
            version: "v2",
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
        address: 1073766400,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "CRSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
        address: 1107461120,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(3),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC1",
        }],
    },
    Peripheral {
        name: "DBGMCU",
        address: 1140998144,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DCACHE1",
        address: 1073943552,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DCACHEEN",
            }),
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCACHE1",
        }],
    },
    Peripheral {
        name: "DCMI",
        address: 1107476480,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DCMI_PSSIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DCMI_PSSIRST",
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
                pin: "PA8",
                signal: "D3",
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
                pin: "PA15",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D7",
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
                pin: "PB15",
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
                pin: "PD12",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(13),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(108),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI_PSSI",
        }],
    },
    Peripheral {
        name: "DTS",
        address: 1073777664,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "DTSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
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
        name: "ETH",
        address: 1073905664,
        registers: Some(PeripheralRegisters {
            kind: "eth",
            version: "v2",
            block: "ETH",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "ETHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "ETHRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CRS",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REF_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "COL",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CRS_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX_DV",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX_ER",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PPS_OUT",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TXD3",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX_ER",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX_EN",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TXD2",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX_CLK",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RXD0",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RXD1",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TXD0",
                af: Some(11),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ETH",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ETH_WKUP",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1140989952,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "h5",
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
        address: 1073783808,
        registers: None,
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
                pin: "PB7",
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
                pin: "PE0",
                signal: "RX",
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
        address: 1073784832,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
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
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "h5",
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
        name: "FMAC",
        address: 1073888256,
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(117),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(117),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMAC",
        }],
    },
    Peripheral {
        name: "GPDMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA1RST",
            }),
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
        name: "GPDMA2",
        address: 1073876992,
        registers: Some(PeripheralRegisters {
            kind: "gpdma",
            version: "v1",
            block: "GPDMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPDMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPDMA2RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "GPDMA2_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "GPDMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "GPDMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "GPDMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "GPDMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "GPDMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "GPDMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "GPDMA2_CHANNEL7",
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
        address: 1107428352,
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
        address: 1107429376,
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
        address: 1107430400,
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
        address: 1107431424,
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
        address: 1107432448,
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
        address: 1107433472,
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
        name: "GPIOH",
        address: 1107434496,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOHRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOI",
        address: 1107435520,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "GPIOIRST",
            }),
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
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "HASHRST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(111),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(13),
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
                pin: "PB3",
                signal: "SDA",
                af: Some(4),
            },
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
                signal: "SDA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SMBA",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(15),
            },
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
                dma: Some("GPDMA2"),
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(16),
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
        address: 1140860928,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(19),
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
        address: 1140861952,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v2",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
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
                pin: "PD11",
                signal: "SMBA",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(124),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(124),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(125),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(125),
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
        name: "I3C1",
        address: 1073765376,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "I3C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "I3C1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(120),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(120),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(121),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(121),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "TC",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(122),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(123),
            },
            PeripheralDmaChannel {
                signal: "RS",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(123),
            },
        ],
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
        address: 1140868096,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM1RST",
            }),
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
                pin: "PB2",
                signal: "CH1",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IN1",
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(102),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(104),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(104),
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
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "LPTIM2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "IN2",
                af: Some(4),
            },
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
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "ETR",
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
                signal: "CH2",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(105),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(106),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(107),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(107),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
    },
    Peripheral {
        name: "LPTIM3",
        address: 1140869120,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "IN2",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "ETR",
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "ETR",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IN1",
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(127),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(127),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(129),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(129),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM3",
        }],
    },
    Peripheral {
        name: "LPTIM4",
        address: 1140870144,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM4RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PD2",
            signal: "ETR",
            af: Some(14),
        }],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM4",
        }],
    },
    Peripheral {
        name: "LPTIM5",
        address: 1140871168,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM5RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PB2",
            signal: "ETR",
            af: Some(13),
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(130),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(130),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(131),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(131),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(132),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(132),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM5",
        }],
    },
    Peripheral {
        name: "LPTIM6",
        address: 1140872192,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPTIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "LPTIM6RST",
            }),
        }),
        pins: &[PeripheralPin {
            pin: "PB3",
            signal: "ETR",
            af: Some(14),
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(133),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(133),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(134),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(134),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(135),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(135),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM6",
        }],
    },
    Peripheral {
        name: "LPUART1",
        address: 1140859904,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "LPUART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "LPUART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(46),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
    },
    Peripheral {
        name: "OCTOSPI1",
        address: 2415919104,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "OCTOSPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "OCTOSPI1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "DQS",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IO3",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IO3",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IO2",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IO1",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IO0",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CLK",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "NCLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "NCS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NCLK",
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CLK",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IO7",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IO4",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IO2",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IO5",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IO6",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DQS",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "IO5",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "IO6",
                af: Some(11),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "IO1",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "NCS",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "IO0",
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "IO1",
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OCTOSPI1",
        }],
    },
    Peripheral {
        name: "OTFDEC1",
        address: 1174425600,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "OTFDEC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "OTFDEC1RST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OTFDEC1",
        }],
    },
    Peripheral {
        name: "PKA",
        address: 1108090880,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "PKAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "PKARST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PKA",
        }],
    },
    Peripheral {
        name: "PSSI",
        address: 1107477504,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DCMI_PSSIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DCMI_PSSIRST",
            }),
        }),
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
                pin: "PA8",
                signal: "D3",
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
                pin: "PA15",
                signal: "D11",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D7",
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
                pin: "PB15",
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
                pin: "PD12",
                signal: "D12",
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
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
        address: 1140983808,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "h5",
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
                pin: "PB7",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CSLEEP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CSTOP",
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "WKUP7",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1140984832,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "h5",
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
                pin: "PB2",
                signal: "LSCO",
                af: None,
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
                signal: "GLOBAL",
                interrupt: "RCC",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 1108084736,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v3",
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
        address: 1140881408,
        registers: None,
        rcc: None,
        pins: &[
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
                signal: "TS",
                af: None,
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
        name: "SAES",
        address: 1108085760,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB2",
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "SAESEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "SAESRST",
            }),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(119),
            },
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(119),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAES",
        }],
    },
    Peripheral {
        name: "SAI1",
        address: 1073828864,
        registers: None,
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
                signal: "SD_B",
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
                pin: "PC0",
                signal: "MCLK_A",
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
                pin: "PC3",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CK1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "FS_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SCK_A",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CK1",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "D1",
                af: Some(6),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(54),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI1",
        }],
    },
    Peripheral {
        name: "SAI2",
        address: 1073829888,
        registers: None,
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
                pin: "PC1",
                signal: "SD_A",
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
                pin: "PE0",
                signal: "MCLK_A",
                af: Some(10),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(56),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SAI2",
        }],
    },
    Peripheral {
        name: "SBS",
        address: 1140851712,
        registers: Some(PeripheralRegisters {
            kind: "sbs",
            version: "h5",
            block: "SBS",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB3",
            enable: Some(PeripheralRccRegister {
                register: "APB3ENR",
                field: "SBSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB3RSTR",
                field: "SBSRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SDMMC1",
        address: 1174437888,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "AHB4",
            enable: Some(PeripheralRccRegister {
                register: "AHB4ENR",
                field: "SDMMC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB4RSTR",
                field: "SDMMC1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D0",
                af: Some(12),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CMD",
                af: Some(12),
            },
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
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
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
                pin: "PA8",
                signal: "RDY",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RDY",
                af: Some(4),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(7),
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
            version: "v4",
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
                pin: "PA3",
                signal: "NSS",
                af: Some(5),
            },
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
                pin: "PB11",
                signal: "RDY",
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
                pin: "PC0",
                signal: "RDY",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_WS",
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
                pin: "PC2",
                signal: "I2S_SDI",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SDO",
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(8),
            },
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
                dma: Some("GPDMA2"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(9),
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
            version: "v4",
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
                pin: "PA0",
                signal: "RDY",
                af: Some(6),
            },
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
                pin: "PE0",
                signal: "RDY",
                af: Some(6),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(11),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "SPI6",
        address: 1073827840,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v4",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(52),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
    },
    Peripheral {
        name: "TAMP",
        address: 1140882432,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "OUT5",
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
                pin: "PC5",
                signal: "CH4N",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(64),
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
        pins: &[PeripheralPin {
            pin: "PA6",
            signal: "CH1",
            af: Some(9),
        }],
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
        pins: &[PeripheralPin {
            pin: "PA7",
            signal: "CH1",
            af: Some(9),
        }],
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(97),
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
                pin: "PC0",
                signal: "BKIN",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(99),
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
                pin: "PC2",
                signal: "CH1",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(101),
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
                af: Some(14),
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
                pin: "PC4",
                signal: "CH4",
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(76),
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
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(82),
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
                pin: "PC2",
                signal: "CH4",
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
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
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(87),
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
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(91),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(93),
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(4),
            },
        ],
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(5),
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
                pin: "PB2",
                signal: "CH4N",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "TIG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "TIG",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(71),
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
                pin: "PD11",
                signal: "RX",
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "TX",
                af: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(28),
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
                pin: "PB15",
                signal: "RX",
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(30),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(34),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "UCPD1",
        address: 1073798144,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "UCPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "UCPDRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "DB1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FRSTX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FRSTX",
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "DB2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(113),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(113),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UCPD1",
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
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(22),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART11",
        address: 1073769472,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART11EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "USART11RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "NSS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RTS",
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART11",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(24),
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
                signal: "RTS",
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(26),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART6",
        address: 1073767424,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v4",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA2"),
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some("GPDMA1"),
                request: Some(32),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
    },
    Peripheral {
        name: "USB",
        address: 1073831936,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v4",
            block: "USB",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USBRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: Some(10),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: Some(10),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_DRD_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USB_DRD_FS",
            },
        ],
    },
    Peripheral {
        name: "USBRAM",
        address: 1073832960,
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
        name: "VREFBUF",
        address: 1140880384,
        registers: None,
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
                register: "APB1LENR",
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
        name: "PVD_AVD",
        number: 1,
    },
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
        name: "GPDMA1_CHANNEL0",
        number: 27,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 28,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 29,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 30,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 31,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 32,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 33,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 34,
    },
    Interrupt {
        name: "IWDG",
        number: 35,
    },
    Interrupt {
        name: "SAES",
        number: 36,
    },
    Interrupt {
        name: "ADC1",
        number: 37,
    },
    Interrupt {
        name: "DAC1",
        number: 38,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 39,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 40,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 41,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 42,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 43,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 44,
    },
    Interrupt {
        name: "TIM2",
        number: 45,
    },
    Interrupt {
        name: "TIM3",
        number: 46,
    },
    Interrupt {
        name: "TIM4",
        number: 47,
    },
    Interrupt {
        name: "TIM5",
        number: 48,
    },
    Interrupt {
        name: "TIM6",
        number: 49,
    },
    Interrupt {
        name: "TIM7",
        number: 50,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 51,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 52,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 53,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 54,
    },
    Interrupt {
        name: "SPI1",
        number: 55,
    },
    Interrupt {
        name: "SPI2",
        number: 56,
    },
    Interrupt {
        name: "SPI3",
        number: 57,
    },
    Interrupt {
        name: "USART1",
        number: 58,
    },
    Interrupt {
        name: "USART2",
        number: 59,
    },
    Interrupt {
        name: "USART3",
        number: 60,
    },
    Interrupt {
        name: "UART4",
        number: 61,
    },
    Interrupt {
        name: "UART5",
        number: 62,
    },
    Interrupt {
        name: "LPUART1",
        number: 63,
    },
    Interrupt {
        name: "LPTIM1",
        number: 64,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 65,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 66,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 67,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 68,
    },
    Interrupt {
        name: "ADC2",
        number: 69,
    },
    Interrupt {
        name: "LPTIM2",
        number: 70,
    },
    Interrupt {
        name: "TIM15",
        number: 71,
    },
    Interrupt {
        name: "TIM16",
        number: 72,
    },
    Interrupt {
        name: "TIM17",
        number: 73,
    },
    Interrupt {
        name: "USB_DRD_FS",
        number: 74,
    },
    Interrupt {
        name: "CRS",
        number: 75,
    },
    Interrupt {
        name: "UCPD1",
        number: 76,
    },
    Interrupt {
        name: "FMC",
        number: 77,
    },
    Interrupt {
        name: "OCTOSPI1",
        number: 78,
    },
    Interrupt {
        name: "SDMMC1",
        number: 79,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 80,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 81,
    },
    Interrupt {
        name: "SPI4",
        number: 82,
    },
    Interrupt {
        name: "SPI5",
        number: 83,
    },
    Interrupt {
        name: "SPI6",
        number: 84,
    },
    Interrupt {
        name: "USART6",
        number: 85,
    },
    Interrupt {
        name: "USART10",
        number: 86,
    },
    Interrupt {
        name: "USART11",
        number: 87,
    },
    Interrupt {
        name: "SAI1",
        number: 88,
    },
    Interrupt {
        name: "SAI2",
        number: 89,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL0",
        number: 90,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL1",
        number: 91,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL2",
        number: 92,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL3",
        number: 93,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL4",
        number: 94,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL5",
        number: 95,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL6",
        number: 96,
    },
    Interrupt {
        name: "GPDMA2_CHANNEL7",
        number: 97,
    },
    Interrupt {
        name: "UART7",
        number: 98,
    },
    Interrupt {
        name: "UART8",
        number: 99,
    },
    Interrupt {
        name: "UART9",
        number: 100,
    },
    Interrupt {
        name: "UART12",
        number: 101,
    },
    Interrupt {
        name: "SDMMC2",
        number: 102,
    },
    Interrupt {
        name: "FPU",
        number: 103,
    },
    Interrupt {
        name: "ICACHE",
        number: 104,
    },
    Interrupt {
        name: "DCACHE1",
        number: 105,
    },
    Interrupt {
        name: "ETH",
        number: 106,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 107,
    },
    Interrupt {
        name: "DCMI_PSSI",
        number: 108,
    },
    Interrupt {
        name: "FDCAN2_IT0",
        number: 109,
    },
    Interrupt {
        name: "FDCAN2_IT1",
        number: 110,
    },
    Interrupt {
        name: "CORDIC",
        number: 111,
    },
    Interrupt {
        name: "FMAC",
        number: 112,
    },
    Interrupt {
        name: "DTS",
        number: 113,
    },
    Interrupt {
        name: "RNG",
        number: 114,
    },
    Interrupt {
        name: "OTFDEC1",
        number: 115,
    },
    Interrupt {
        name: "AES",
        number: 116,
    },
    Interrupt {
        name: "HASH",
        number: 117,
    },
    Interrupt {
        name: "PKA",
        number: 118,
    },
    Interrupt {
        name: "CEC",
        number: 119,
    },
    Interrupt {
        name: "TIM12",
        number: 120,
    },
    Interrupt {
        name: "TIM13",
        number: 121,
    },
    Interrupt {
        name: "TIM14",
        number: 122,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 123,
    },
    Interrupt {
        name: "I3C1_ER",
        number: 124,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 125,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 126,
    },
    Interrupt {
        name: "LPTIM3",
        number: 127,
    },
    Interrupt {
        name: "LPTIM4",
        number: 128,
    },
    Interrupt {
        name: "LPTIM5",
        number: 129,
    },
    Interrupt {
        name: "LPTIM6",
        number: 130,
    },
];
const DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "GPDMA2_CH0",
        dma: "GPDMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH1",
        dma: "GPDMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH2",
        dma: "GPDMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH3",
        dma: "GPDMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH4",
        dma: "GPDMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH5",
        dma: "GPDMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH6",
        dma: "GPDMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA2_CH7",
        dma: "GPDMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
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
