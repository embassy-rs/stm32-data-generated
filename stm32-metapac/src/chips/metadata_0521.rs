
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "l0",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "ADCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "ADCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC1_COMP",
            },
        ],
    },
    Peripheral {
        name: "ADC1_COMMON",
        address: 0x40012708,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP1",
        address: 0x40010018,
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
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "OUT",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "OUT",
                af: Some(
                    7,
                ),
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
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "OUT",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ADC1_COMP",
            },
        ],
    },
    Peripheral {
        name: "COMP2",
        address: 0x4001001c,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(
                    7,
                ),
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
                pin: "PA4",
                signal: "OUT",
                af: Some(
                    7,
                ),
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
                af: Some(
                    7,
                ),
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
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ADC1_COMP",
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
                        register: "AHBENR",
                        field: "CRCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
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
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "l0",
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
        name: "DMA1",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "bdma",
                version: "v2",
                block: "DMA",
                ir: &bdma::REGISTERS,
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
                        register: "AHBENR",
                        field: "DMA1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "DMA1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
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
                interrupt: "DMA1_CHANNEL4_5",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL4_5",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "v1",
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
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "l0",
                block: "FLASH",
                ir: &flash::REGISTERS,
            },
        ),
        rcc: None,
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
        name: "GPIOA",
        address: 0x50000000,
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
                bus_clock: "GPIO",
                kernel_clock: Clock(
                    "GPIO",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "GPIOENR",
                        field: "GPIOAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
        address: 0x50000400,
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
                bus_clock: "GPIO",
                kernel_clock: Clock(
                    "GPIO",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "GPIOENR",
                        field: "GPIOBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
        address: 0x50000800,
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
                bus_clock: "GPIO",
                kernel_clock: Clock(
                    "GPIO",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "GPIOENR",
                        field: "GPIOCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
                        register: "CCIPR",
                        field: "I2C1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "I2C1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "I2C1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SDA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
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
        name: "IWDG",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v2",
                block: "IWDG",
                ir: &iwdg::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v1",
                block: "LPTIM",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "LPTIM1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "LPTIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OUT",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "OUT",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "LPUART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "LPUART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "LPUART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "LPUART1RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "RX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "l0",
                block: "PWR",
                ir: &pwr::REGISTERS,
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
                        register: "APB1ENR",
                        field: "PWREN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "PWRRST",
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
        name: "RCC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "l0",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CK_IN",
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
                pin: "PA9",
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
        name: "RTC",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v2l0",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "REFIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT_ALARM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT_CALIB",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TAMP3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
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
        name: "SPI1",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2",
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
                pin: "PA13",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MISO",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MOSI",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
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
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "l0",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Clock(
                    "PCLK2",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "SYSCFGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
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
        name: "TIM2",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "l0",
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
                        register: "APB1ENR",
                        field: "TIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    5,
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
                signal: "CH3",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    5,
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
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    8,
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
        name: "TIM21",
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "l0",
                block: "TIM_2CH",
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
                        field: "TIM21EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM21RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "ETR",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "ETR",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM21",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM21",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM21",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM21",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM21",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 0x1ff80050,
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
        name: "USART2",
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v3",
                block: "USART",
                ir: &usart::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "USART2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "USART2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "USART2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "DE",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
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
                version: "v1",
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
                        register: "APB1ENR",
                        field: "WWDGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "WWDGRST",
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
        name: "FLASH",
        number: 3,
    },
    Interrupt {
        name: "RCC",
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
        name: "DMA1_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA1_CHANNEL2_3",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL4_5",
        number: 11,
    },
    Interrupt {
        name: "ADC1_COMP",
        number: 12,
    },
    Interrupt {
        name: "LPTIM1",
        number: 13,
    },
    Interrupt {
        name: "TIM2",
        number: 15,
    },
    Interrupt {
        name: "TIM21",
        number: 20,
    },
    Interrupt {
        name: "I2C1",
        number: 23,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "USART2",
        number: 28,
    },
    Interrupt {
        name: "LPUART1",
        number: 29,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
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
];
            #[path="../registers/adc_l0.rs"] pub mod adc;
#[path="../registers/bdma_v2.rs"] pub mod bdma;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/dbgmcu_l0.rs"] pub mod dbgmcu;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_l0.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/iwdg_v2.rs"] pub mod iwdg;
#[path="../registers/lptim_v1.rs"] pub mod lptim;
#[path="../registers/pwr_l0.rs"] pub mod pwr;
#[path="../registers/rcc_l0.rs"] pub mod rcc;
#[path="../registers/rtc_v2l0.rs"] pub mod rtc;
#[path="../registers/spi_v1.rs"] pub mod spi;
#[path="../registers/syscfg_l0.rs"] pub mod syscfg;
#[path="../registers/timer_l0.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v3.rs"] pub mod usart;
#[path="../registers/wwdg_v1.rs"] pub mod wwdg;
