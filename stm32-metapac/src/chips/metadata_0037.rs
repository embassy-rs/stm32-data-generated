
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x42028000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADCDACSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "ADCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "ADCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
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
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN2",
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
        address: 0x42028300,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "AES",
        address: 0x420c0000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "AESEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "AESRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AES",
        }],
        afio: None,
    },
    Peripheral {
        name: "COMP1",
        address: 0x4000400c,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "COMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "INM3",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "INP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "INP3",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "INM1",
                af: None,
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
        name: "COMP12",
        address: 0x40004000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "COMPRST",
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
        name: "COMP2",
        address: 0x40004010,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1HENR",
                field: "COMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1HRSTR",
                field: "COMPRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "INP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "INP2",
                af: None,
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
        name: "CORDIC",
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
                field: "CORDICEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CORDICRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CORDIC",
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
        pins: &[PeripheralPin {
            pin: "PB3",
            signal: "SYNC",
            af: None,
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x42028400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
                field: "ADCDACSEL",
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
            interrupt: "DAC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x44024000,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "c5",
            block: "DBGMCU",
            ir: &dbgmcu::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXTI",
        address: 0x44022000,
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
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FDCAN1",
        address: 0x4000a400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: None,
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
        address: 0x4000a800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: None,
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
        name: "FLASH",
        address: 0x40022000,
        registers: None,
        rcc: None,
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
        name: "GPIOA",
        address: 0x42020000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x42020400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x42020800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x42020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        address: 0x42021000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        name: "GPIOH",
        address: 0x42021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
        name: "HASH",
        address: 0x420c0400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "HASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "HASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
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
                pin: "PA11",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
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
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PH0",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PH1",
                signal: "SCL",
                af: None,
            },
        ],
        dma_channels: &[],
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
        name: "I3C1",
        address: 0x40005c00,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
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
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
        ],
        dma_channels: &[],
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
        address: 0x40003000,
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
        name: "LPDMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "lpdma",
            version: "v1",
            block: "LPDMA",
            ir: &lpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "LPDMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "LPDMA1RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "LPDMA1_CH0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "LPDMA1_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "LPDMA1_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "LPDMA1_CH3",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "LPDMA2",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "lpdma",
            version: "v1",
            block: "LPDMA",
            ir: &lpdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "LPDMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "LPDMA2RST",
            }),
            stop_mode: StopMode::Stop2,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "LPDMA2_CH0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "LPDMA2_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "LPDMA2_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "LPDMA2_CH3",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x44004400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK3",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR2",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: None,
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
        name: "LPUART1",
        address: 0x44002400,
        registers: None,
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
                pin: "PA11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x40003400,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1LENR",
                field: "OPAMPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1LRSTR",
                field: "OPAMPRST",
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
                signal: "VINM0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VOUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VINP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "VINM1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "VINP2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x44020800,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "PVD_IN",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "PVD",
            interrupt: "PWR_PVD",
        }],
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
        name: "RCC",
        address: 0x44020c00,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "c5",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "LSCO",
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
        ],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x420c0800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
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
                pin: "PB15",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
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
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RDY",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "I2S_MCK",
                af: None,
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
        address: 0x40003800,
        registers: None,
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR1",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "I2S_SDI",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "I2S_SDO",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                af: None,
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
        name: "SYSCFG",
        address: 0x44000400,
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
        address: 0x44007c00,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN3",
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
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
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
                pin: "PA11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "BKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
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
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
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
        pins: &[PeripheralPin {
            pin: "PB15",
            signal: "CH2",
            af: None,
        }],
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
        name: "TIM15",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_2CH_CMP",
            ir: &timer::REGISTERS,
        }),
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
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
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
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
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
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH4",
                af: None,
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
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
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
                pin: "PA1",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH4N",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4N",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "BKIN",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "CC",
            interrupt: "TIM8_CC",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: None,
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
                pin: "PA1",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: None,
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
        address: 0x40005000,
        registers: None,
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
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
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
        registers: None,
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
                pin: "PA11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: None,
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
        address: 0x40004400,
        registers: None,
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
                pin: "PA1",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                af: None,
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
        name: "WWDG",
        address: 0x40002c00,
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
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "PWR_PVD",
        number: 1,
    },
    Interrupt { name: "RTC", number: 2 },
    Interrupt {
        name: "TAMP",
        number: 3,
    },
    Interrupt {
        name: "RAMCFG",
        number: 4,
    },
    Interrupt {
        name: "FLASH",
        number: 5,
    },
    Interrupt { name: "RCC", number: 6 },
    Interrupt {
        name: "EXTI0",
        number: 7,
    },
    Interrupt {
        name: "EXTI1",
        number: 8,
    },
    Interrupt {
        name: "EXTI2",
        number: 9,
    },
    Interrupt {
        name: "EXTI3",
        number: 10,
    },
    Interrupt {
        name: "EXTI4",
        number: 11,
    },
    Interrupt {
        name: "EXTI5",
        number: 12,
    },
    Interrupt {
        name: "EXTI6",
        number: 13,
    },
    Interrupt {
        name: "EXTI7",
        number: 14,
    },
    Interrupt {
        name: "EXTI8",
        number: 15,
    },
    Interrupt {
        name: "EXTI9",
        number: 16,
    },
    Interrupt {
        name: "EXTI10",
        number: 17,
    },
    Interrupt {
        name: "EXTI11",
        number: 18,
    },
    Interrupt {
        name: "EXTI12",
        number: 19,
    },
    Interrupt {
        name: "EXTI13",
        number: 20,
    },
    Interrupt {
        name: "EXTI14",
        number: 21,
    },
    Interrupt {
        name: "EXTI15",
        number: 22,
    },
    Interrupt {
        name: "LPDMA1_CH0",
        number: 23,
    },
    Interrupt {
        name: "LPDMA1_CH1",
        number: 24,
    },
    Interrupt {
        name: "LPDMA1_CH2",
        number: 25,
    },
    Interrupt {
        name: "LPDMA1_CH3",
        number: 26,
    },
    Interrupt {
        name: "IWDG",
        number: 31,
    },
    Interrupt {
        name: "ADC1",
        number: 32,
    },
    Interrupt {
        name: "FDCAN1_IT0",
        number: 34,
    },
    Interrupt {
        name: "FDCAN1_IT1",
        number: 35,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 36,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 37,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 38,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 39,
    },
    Interrupt {
        name: "TIM2",
        number: 40,
    },
    Interrupt {
        name: "TIM6",
        number: 42,
    },
    Interrupt {
        name: "TIM7",
        number: 43,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 44,
    },
    Interrupt {
        name: "I2C1_ERR",
        number: 45,
    },
    Interrupt {
        name: "I3C1_EV",
        number: 46,
    },
    Interrupt {
        name: "I3C1_ERR",
        number: 47,
    },
    Interrupt {
        name: "SPI1",
        number: 48,
    },
    Interrupt {
        name: "SPI2",
        number: 49,
    },
    Interrupt {
        name: "USART1",
        number: 51,
    },
    Interrupt {
        name: "USART2",
        number: 52,
    },
    Interrupt {
        name: "UART4",
        number: 54,
    },
    Interrupt {
        name: "UART5",
        number: 55,
    },
    Interrupt {
        name: "LPUART1",
        number: 56,
    },
    Interrupt {
        name: "LPTIM1",
        number: 57,
    },
    Interrupt {
        name: "TIM12",
        number: 58,
    },
    Interrupt {
        name: "TIM15",
        number: 59,
    },
    Interrupt {
        name: "USB_DRD_FS",
        number: 62,
    },
    Interrupt {
        name: "CRS",
        number: 63,
    },
    Interrupt {
        name: "RNG",
        number: 64,
    },
    Interrupt {
        name: "FPU",
        number: 65,
    },
    Interrupt {
        name: "ICACHE",
        number: 66,
    },
    Interrupt {
        name: "CORDIC",
        number: 67,
    },
    Interrupt {
        name: "AES",
        number: 68,
    },
    Interrupt {
        name: "HASH",
        number: 69,
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
        name: "COMP1",
        number: 76,
    },
    Interrupt {
        name: "DAC1",
        number: 77,
    },
    Interrupt {
        name: "LPDMA2_CH0",
        number: 78,
    },
    Interrupt {
        name: "LPDMA2_CH1",
        number: 79,
    },
    Interrupt {
        name: "LPDMA2_CH2",
        number: 80,
    },
    Interrupt {
        name: "LPDMA2_CH3",
        number: 81,
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
        name: "COMP2",
        number: 88,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA1" },
    Pin { name: "PA2" },
    Pin { name: "PA3" },
    Pin { name: "PA4" },
    Pin { name: "PA5" },
    Pin { name: "PA6" },
    Pin { name: "PA7" },
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
    Pin { name: "PB8" },
    Pin { name: "PB15" },
    Pin { name: "PC4" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
];
#[path = "../registers/cordic_v1.rs"]
pub mod cordic;
#[path = "../registers/dbgmcu_c5.rs"]
pub mod dbgmcu;
#[path = "../registers/exti_v1.rs"]
pub mod exti;
#[path = "../registers/fdcanram_v1.rs"]
pub mod fdcanram;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/lpdma_v1.rs"]
pub mod lpdma;
#[path = "../registers/rcc_c5.rs"]
pub mod rcc;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/uid_v1.rs"]
pub mod uid;
