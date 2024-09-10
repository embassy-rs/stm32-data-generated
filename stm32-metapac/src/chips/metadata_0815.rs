
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC4",
        address: 0x46021000,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK4",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "ADCSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB4ENR",
                        field: "ADC4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB4RSTR",
                        field: "ADC4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "IN10",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC4",
            },
        ],
    },
    Peripheral {
        name: "ADC4_COMMON",
        address: 0x46021308,
        registers: Some(
            PeripheralRegisters {
                kind: "adccommon",
                version: "v3",
                block: "ADC_COMMON",
                ir: &adccommon::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AES",
        address: 0x420c0000,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v3b",
                block: "AES",
                ir: &aes::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "AESEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "AESRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    42,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "AES",
            },
        ],
    },
    Peripheral {
        name: "COMP1",
        address: 0x46005400,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "COMP",
            },
        ],
    },
    Peripheral {
        name: "COMP2",
        address: 0x46005404,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "INM",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "COMP",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v3",
                block: "CRC",
                ir: &crc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "CRCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "CRCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0044000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "wba",
                block: "DBGMCU",
                ir: &dbgmcu::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x46022000,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "l5",
                block: "EXTI",
                ir: &exti::REGISTERS,
            },
        ),
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
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "wba",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "FLASHEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "FLASH",
            },
        ],
    },
    Peripheral {
        name: "GPDMA1",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpdma",
                version: "v1",
                block: "GPDMA",
                ir: &gpdma::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPDMA1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPDMA1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
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
        name: "GPIOA",
        address: 0x42020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "GPIOAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "GPIOARST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x42020400,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "GPIOBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "GPIOBRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x42020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "GPIOCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "GPIOCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOH",
        address: 0x42021c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v2",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "GPIOHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "GPIOHRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HASH",
        address: 0x420c0400,
        registers: Some(
            PeripheralRegisters {
                kind: "hash",
                version: "v4",
                block: "HASH",
                ir: &hash::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "HASHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "HASHRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    43,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "HASH",
            },
        ],
    },
    Peripheral {
        name: "HSEM",
        address: 0x420c1c00,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "HSEMEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "HSEMRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "HSEM",
            },
        ],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v2",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "I2C1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "I2C1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "I2C1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    7,
                ),
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
        name: "I2C3",
        address: 0x46002800,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v2",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "I2C3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "I2C3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB7RSTR",
                        field: "I2C3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SMBA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    10,
                ),
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
        name: "ICACHE",
        address: 0x40030400,
        registers: Some(
            PeripheralRegisters {
                kind: "icache",
                version: "v1_4crr",
                block: "ICACHE",
                ir: &icache::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ICACHE",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v3",
                block: "IWDG",
                ir: &iwdg::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "IWDG",
            },
        ],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x46004400,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2a",
                block: "LPTIM",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "LPTIM1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "LPTIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB7RSTR",
                        field: "LPTIM1RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "IN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "ETR",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    47,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    48,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2a",
                block: "LPTIM",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "LPTIM2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR2",
                        field: "LPTIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR2",
                        field: "LPTIM2RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "IN1",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    51,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM2",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x46002400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "LPUART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "LPUART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "LPUART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB7RSTR",
                        field: "LPUART1RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    16,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPUART1",
            },
        ],
    },
    Peripheral {
        name: "PKA",
        address: 0x420c2000,
        registers: Some(
            PeripheralRegisters {
                kind: "pka",
                version: "v1a",
                block: "PKA",
                ir: &pka::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "PKAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "PKARST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "PKA",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 0x46020800,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "wba",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK4",
                kernel_clock: Clock(
                    "HCLK4",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB4ENR",
                        field: "PWREN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CSLEEP",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CSTOP",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP2",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "WKUP",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x46020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "wba",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
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
                af: Some(
                    0,
                ),
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
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 0x420c0800,
        registers: Some(
            PeripheralRegisters {
                kind: "rng",
                version: "v1",
                block: "RNG",
                ir: &rng::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "RNGSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "RNGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "RNGRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RNG",
            },
        ],
    },
    Peripheral {
        name: "RTC",
        address: 0x46007800,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v3u5",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Clock(
                    "PCLK7",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "RTCAPBEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Standby,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "REFIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "OUT2",
                af: None,
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
        address: 0x420c0c00,
        registers: Some(
            PeripheralRegisters {
                kind: "saes",
                version: "v1a",
                block: "SAES",
                ir: &saes::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Clock(
                    "HCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "SAESEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "SAESRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IN",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    45,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SAES",
            },
        ],
    },
    Peripheral {
        name: "SAI1",
        address: 0x40015400,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CK2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MCLK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCK_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[],
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
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v5",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "SPI1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "SPI1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SPI1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "RDY",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RDY",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    2,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI1",
            },
        ],
    },
    Peripheral {
        name: "SPI3",
        address: 0x46002000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v5",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "SPI3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "SPI3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB7RSTR",
                        field: "SPI3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RDY",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RDY",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    4,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI3",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x46000400,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "wba",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK7",
                kernel_clock: Clock(
                    "PCLK7",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB7ENR",
                        field: "SYSCFGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB7RSTR",
                        field: "SYSCFGRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TAMP",
        address: 0x46007c00,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "OUT6",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT5",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "OUT1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TAMP",
            },
        ],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_ADV",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "TIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "BKIN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN2",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    19,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    23,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    24,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    25,
                ),
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
        name: "TIM16",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_1CH_CMP",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "TIM16EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM16RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    37,
                ),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    38,
                ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_1CH_CMP",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "TIM17EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM17RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    39,
                ),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    40,
                ),
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
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_GP32",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "TIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "TIM2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    30,
                ),
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
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_GP16",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "TIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "TIM3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC3",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "CC4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    36,
                ),
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
        name: "TSC",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tsc",
                version: "v1",
                block: "TSC",
                ir: &tsc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "TSCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "TSCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "G2_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "G2_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "G3_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "G3_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "G4_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "G4_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "G4_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "G1_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G1_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "G1_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "G1_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SYNC",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G6_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G6_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "G3_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G3_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "G5_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "G5_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "G5_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "G2_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "G2_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "G5_IO1",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSC",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 0xbf90700,
        registers: Some(
            PeripheralRegisters {
                kind: "uid",
                version: "v1",
                block: "UID",
                ir: &uid::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "USART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "USART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "USART1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    12,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART1",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v4",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "USART2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "USART2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "USART2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    14,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2",
            },
        ],
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdg",
                version: "v2",
                block: "WWDG",
                ir: &wwdg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "WWDGEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
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
        name: "PVD",
        number: 1,
    },
    Interrupt {
        name: "RTC",
        number: 2,
    },
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
    Interrupt {
        name: "RCC",
        number: 9,
    },
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
        name: "IWDG",
        number: 27,
    },
    Interrupt {
        name: "SAES",
        number: 28,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL0",
        number: 29,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL1",
        number: 30,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL2",
        number: 31,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL3",
        number: 32,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL4",
        number: 33,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL5",
        number: 34,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL6",
        number: 35,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL7",
        number: 36,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 37,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 38,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 39,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 40,
    },
    Interrupt {
        name: "TIM2",
        number: 41,
    },
    Interrupt {
        name: "TIM3",
        number: 42,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 43,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 44,
    },
    Interrupt {
        name: "SPI1",
        number: 45,
    },
    Interrupt {
        name: "USART1",
        number: 46,
    },
    Interrupt {
        name: "USART2",
        number: 47,
    },
    Interrupt {
        name: "LPUART1",
        number: 48,
    },
    Interrupt {
        name: "LPTIM1",
        number: 49,
    },
    Interrupt {
        name: "LPTIM2",
        number: 50,
    },
    Interrupt {
        name: "TIM16",
        number: 51,
    },
    Interrupt {
        name: "TIM17",
        number: 52,
    },
    Interrupt {
        name: "COMP",
        number: 53,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 54,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 55,
    },
    Interrupt {
        name: "SAI1",
        number: 56,
    },
    Interrupt {
        name: "TSC",
        number: 57,
    },
    Interrupt {
        name: "AES",
        number: 58,
    },
    Interrupt {
        name: "RNG",
        number: 59,
    },
    Interrupt {
        name: "FPU",
        number: 60,
    },
    Interrupt {
        name: "HASH",
        number: 61,
    },
    Interrupt {
        name: "PKA",
        number: 62,
    },
    Interrupt {
        name: "SPI3",
        number: 63,
    },
    Interrupt {
        name: "ICACHE",
        number: 64,
    },
    Interrupt {
        name: "ADC4",
        number: 65,
    },
    Interrupt {
        name: "RADIO",
        number: 66,
    },
    Interrupt {
        name: "WKUP",
        number: 67,
    },
    Interrupt {
        name: "HSEM",
        number: 68,
    },
    Interrupt {
        name: "HSEM_S",
        number: 69,
    },
    Interrupt {
        name: "WKUP_S",
        number: 70,
    },
    Interrupt {
        name: "RCC_AUDIOSYNC",
        number: 71,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
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
            #[path="../registers/adccommon_v3.rs"] pub mod adccommon;
#[path="../registers/aes_v3b.rs"] pub mod aes;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/dbgmcu_wba.rs"] pub mod dbgmcu;
#[path="../registers/exti_l5.rs"] pub mod exti;
#[path="../registers/flash_wba.rs"] pub mod flash;
#[path="../registers/gpdma_v1.rs"] pub mod gpdma;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/hash_v4.rs"] pub mod hash;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/icache_v1_4crr.rs"] pub mod icache;
#[path="../registers/iwdg_v3.rs"] pub mod iwdg;
#[path="../registers/lptim_v2a.rs"] pub mod lptim;
#[path="../registers/pka_v1a.rs"] pub mod pka;
#[path="../registers/pwr_wba.rs"] pub mod pwr;
#[path="../registers/rcc_wba.rs"] pub mod rcc;
#[path="../registers/rng_v1.rs"] pub mod rng;
#[path="../registers/rtc_v3u5.rs"] pub mod rtc;
#[path="../registers/saes_v1a.rs"] pub mod saes;
#[path="../registers/spi_v5.rs"] pub mod spi;
#[path="../registers/syscfg_wba.rs"] pub mod syscfg;
#[path="../registers/timer_v2.rs"] pub mod timer;
#[path="../registers/tsc_v1.rs"] pub mod tsc;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v4.rs"] pub mod usart;
#[path="../registers/wwdg_v2.rs"] pub mod wwdg;
