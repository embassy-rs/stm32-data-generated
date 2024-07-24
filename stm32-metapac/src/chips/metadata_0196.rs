
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
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
                        field: "ADC1EN",
                    },
                ),
                reset: None,
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
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA2_CH4",
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
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "ADC123_COMMON",
        address: 0x40012300,
        registers: Some(
            PeripheralRegisters {
                kind: "adccommon",
                version: "v2",
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
        name: "ADC2",
        address: 0x40012100,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
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
                        field: "ADC2EN",
                    },
                ),
                reset: None,
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
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some(
                    "DMA2_CH3",
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
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "ADC3",
        address: 0x40012200,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
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
                        field: "ADC3EN",
                    },
                ),
                reset: None,
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
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "IN7",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC3",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC",
            },
        ],
    },
    Peripheral {
        name: "CAN1",
        address: 0x40006400,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "bxcan",
                block: "CAN",
                ir: &can::REGISTERS,
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
                        field: "CAN1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "CAN1RST",
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
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
            },
        ],
    },
    Peripheral {
        name: "CAN2",
        address: 0x40006800,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "bxcan",
                block: "CAN",
                ir: &can::REGISTERS,
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
                        field: "CAN2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "CAN2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN2_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN2_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN2_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN2_TX",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v1",
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
        name: "DAC1",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v2",
                block: "DAC",
                ir: &dac::REGISTERS,
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
                        field: "DACEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "DACRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
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
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM6_DAC",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "f4",
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
        name: "DCMI",
        address: 0x50050000,
        registers: Some(
            PeripheralRegisters {
                kind: "dcmi",
                version: "v1",
                block: "DCMI",
                ir: &dcmi::REGISTERS,
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
                        field: "DCMIEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "DCMIRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "VSYNC",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH10",
                signal: "D1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "D2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "D3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "D4",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "D11",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "D8",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "D9",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "HSYNC",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D0",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "D13",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "D8",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "D9",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "D10",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "D5",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "VSYNC",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "D6",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "D7",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: Some(
                    "DMA2_CH7",
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
                interrupt: "DCMI",
            },
        ],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma::REGISTERS,
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
                        field: "DMA1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
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
        address: 0x40026400,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "v2",
                block: "DMA",
                ir: &dma::REGISTERS,
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
                        field: "DMA2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "DMA2RST",
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
        address: 0x4002b000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma2d",
                version: "v1",
                block: "DMA2D",
                ir: &dma2d::REGISTERS,
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
                        field: "DMA2DEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "DMA2DRST",
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
                interrupt: "DMA2D",
            },
        ],
    },
    Peripheral {
        name: "ETH",
        address: 0x40028000,
        registers: Some(
            PeripheralRegisters {
                kind: "eth",
                version: "v1b",
                block: "ETH",
                ir: &eth::REGISTERS,
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
                        field: "ETHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "ETHRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CRS",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "REF_CLK",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX_CLK",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MDIO",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "COL",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CRS_DV",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX_DV",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RXD2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RXD3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX_ER",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX_EN",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "PPS_OUT",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TXD3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MDC",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TXD2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX_CLK",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RXD0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RXD1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "TXD3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "TX_EN",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "TXD0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TXD1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "PPS_OUT",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "CRS",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "COL",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "RXD2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "RXD3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "RX_ER",
                af: Some(
                    11,
                ),
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
                interrupt: "ETH_WKUP",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40013c00,
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
        name: "FLASH",
        address: 0x40023c00,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "f4",
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
        name: "FMC",
        address: 0xa0000000,
        registers: Some(
            PeripheralRegisters {
                kind: "fmc",
                version: "v1x3",
                block: "FMC",
                ir: &fmc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK3",
                kernel_clock: Clock(
                    "HCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB3ENR",
                        field: "FMCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB3RSTR",
                        field: "FMCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "SDCKE1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDNWE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SDNE0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SDCKE0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DA2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DA3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DA15",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CLE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "ALE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DA0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DA1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NCE2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "DA13",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DA14",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DA7",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "DA8",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DA9",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DA10",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DA11",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DA12",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DA4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "DA5",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DA6",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "A0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "A1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "INTR",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SDNRAS",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "A6",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "A7",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "A8",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "A9",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "A2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "A3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "A4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "A5",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NIORD",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "NREG",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "NIOWR",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CD",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG0",
                signal: "A10",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG1",
                signal: "A11",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NCE4_1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NE3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "NCE4_2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "NE4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "A24",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "A25",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "SDNCAS",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG2",
                signal: "A12",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG3",
                signal: "A13",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "A14",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BA0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "A15",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG5",
                signal: "BA1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "INT2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SDCLK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NCE3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NE2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH10",
                signal: "D18",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "D19",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "D20",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "D21",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "D22",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "D23",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "SDCKE0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "SDNE0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SDNWE",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SDNE1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "SDCKE1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "D16",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D17",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "D24",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "D25",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "D31",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "D26",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "D27",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "NBL2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "NBL3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "D28",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "D29",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "D30",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "FMC",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40020000,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
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
        address: 0x40020400,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
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
        address: 0x40020800,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
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
        name: "GPIOD",
        address: 0x40020c00,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIODEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIODRST",
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
        name: "GPIOE",
        address: 0x40021000,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOEEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOERST",
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
        name: "GPIOF",
        address: 0x40021400,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOFEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOFRST",
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
        name: "GPIOG",
        address: 0x40021800,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOGRST",
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
        address: 0x40021c00,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
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
        name: "GPIOI",
        address: 0x40022000,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOIEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOIRST",
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
        name: "GPIOJ",
        address: 0x40022400,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOJEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOJRST",
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
        name: "GPIOK",
        address: 0x40022800,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "GPIOKEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "GPIOKRST",
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
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
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
                pin: "PB5",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH0",
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
                    "DMA1_CH5",
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
                    "DMA1_CH6",
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
                    "DMA1_CH7",
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
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
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
                        field: "I2C2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "I2C2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH4",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SMBA",
                af: Some(
                    4,
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
                    7,
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
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
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
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
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
                        field: "I2C3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "I2C3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "SMBA",
                af: Some(
                    4,
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
                    3,
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
                    3,
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
        name: "IWDG",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v1",
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
        name: "LTDC",
        address: 0x40016800,
        registers: Some(
            PeripheralRegisters {
                kind: "ltdc",
                version: "v1",
                block: "LTDC",
                ir: &ltdc::REGISTERS,
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
                        field: "LTDCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "LTDCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "R4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "R5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "VSYNC",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "R6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "R3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "R6",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "B6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "B7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HSYNC",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "G7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "B2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "G3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "B4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DE",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CLK",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "R7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "B0",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "G0",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "G1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DE",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "B2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PG10",
                signal: "G3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PG11",
                signal: "B3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PG6",
                signal: "R7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CLK",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH10",
                signal: "R4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "R5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "R6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "G2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "G3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "G4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH2",
                signal: "R0",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH3",
                signal: "R1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH8",
                signal: "R2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "R3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "G5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "G6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI10",
                signal: "HSYNC",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "G7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "B4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "B5",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "B6",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "B7",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PI9",
                signal: "VSYNC",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "f4",
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
        address: 0x40023800,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "f4",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO_1",
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
            PeripheralPin {
                pin: "PC9",
                signal: "MCO_2",
                af: Some(
                    0,
                ),
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
                signal: "GLOBAL",
                interrupt: "RCC",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 0x50060800,
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
                kernel_clock: Clock(
                    "HCLK2",
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
                interrupt: "HASH_RNG",
            },
        ],
    },
    Peripheral {
        name: "RTC",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v2f4",
                block: "RTC",
                ir: &rtc::REGISTERS,
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
                        field: "RTCAPBEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Standby,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "AF1",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "AF2",
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
        name: "SAI1",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "sai",
                version: "v1",
                block: "SAI",
                ir: &sai::REGISTERS,
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
                        field: "SAI1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SAI1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PD6",
                signal: "SD_A",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK_A",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD_B",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS_A",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK_A",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD_A",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD_B",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK_B",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK_B",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS_B",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "A",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: Some(
                    "DMA2_CH5",
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
        name: "SDIO",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "sdmmc",
                version: "v1",
                block: "SDMMC",
                ir: &sdmmc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "DCKCFGR",
                        field: "SDIOSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB2ENR",
                        field: "SDIOEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SDIORST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SDIO",
                channel: Some(
                    "DMA2_CH3",
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
                    "DMA2_CH3",
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
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "SDIO",
                channel: Some(
                    "DMA2_CH6",
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
                    "DMA2_CH6",
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
                    "DMA2_CH6",
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
                interrupt: "SDIO",
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
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
        name: "SPI2",
        address: 0x40003800,
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
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "SPI2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "SPI2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "I2S_ext_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "I2S_ext_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "I2S_WS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "I2S_CK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI1",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "I2S_ext_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
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
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2",
            },
        ],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
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
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR",
                        field: "SPI3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "SPI3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "I2S_WS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "I2S_ext_SD",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "I2S_CK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "I2S_ext_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "I2S_SD",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "I2S_SD",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
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
                interrupt: "SPI3",
            },
        ],
    },
    Peripheral {
        name: "SPI4",
        address: 0x40013400,
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
                        field: "SPI4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SPI4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH0",
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
                    "DMA2_CH1",
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
                    "DMA2_CH3",
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
                    "DMA2_CH4",
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
                interrupt: "SPI4",
            },
        ],
    },
    Peripheral {
        name: "SPI5",
        address: 0x40015000,
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
                        field: "SPI5EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SPI5RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PH5",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PH7",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI5",
            },
        ],
    },
    Peripheral {
        name: "SPI6",
        address: 0x40015400,
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
                        field: "SPI6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "SPI6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA2_CH5",
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
                    "DMA2_CH6",
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
                interrupt: "SPI6",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "f4",
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
        name: "TIM1",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                pin: "PA10",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA2_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH6",
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
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM10",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH",
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
                        field: "TIM10EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM10RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM11",
        address: 0x40014800,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH",
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
                        field: "TIM11EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM11RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
        ],
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_2CH",
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
                        field: "TIM12EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM12RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PH6",
                signal: "CH1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CH2",
                af: Some(
                    9,
                ),
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
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH",
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
                        field: "TIM13EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM13RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: Some(
                    9,
                ),
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
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH",
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
                        field: "TIM14EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM14RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: Some(
                    9,
                ),
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
        name: "TIM2",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
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
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
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
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    3,
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
                version: "v1",
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
                        field: "TIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
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
                signal: "UP",
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
                signal: "CH1",
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
                signal: "TRIG",
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
                signal: "CH2",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH7",
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
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                        field: "TIM4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
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
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
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
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    2,
                ),
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
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                        register: "APB1ENR",
                        field: "TIM5EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM5RST",
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
                pin: "PA1",
                signal: "CH2",
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
                pin: "PH10",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PH11",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PH12",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PI0",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH0",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
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
                signal: "CH4",
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
                signal: "TRIG",
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
                signal: "CH2",
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
                signal: "UP",
                channel: Some(
                    "DMA1_CH6",
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
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_BASIC",
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
                        field: "TIM6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
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
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_BASIC",
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
                        field: "TIM7EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "TIM7RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
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
                signal: "UP",
                channel: Some(
                    "DMA1_CH4",
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
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                        field: "TIM8EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM8RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PH13",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PH14",
                signal: "CH2N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PH15",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI2",
                signal: "CH4",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI3",
                signal: "ETR",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI4",
                signal: "BKIN",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI5",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI6",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PI7",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA2_CH1",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA2_CH2",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA2_CH3",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA2_CH4",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA2_CH7",
                ),
                dmamux: None,
                dma: None,
                request: Some(
                    7,
                ),
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
        name: "TIM9",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
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
                        field: "TIM9EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM9RST",
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
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM9",
            },
        ],
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "UART4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "UART4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
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
                pin: "PC10",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    8,
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART4",
            },
        ],
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "UART5EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "UART5RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH0",
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
                    "DMA1_CH7",
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
                interrupt: "UART5",
            },
        ],
    },
    Peripheral {
        name: "UART7",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "UART7EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "UART7RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
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
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART7",
            },
        ],
    },
    Peripheral {
        name: "UART8",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "UART8EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "UART8RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PE0",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH0",
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
                    "DMA1_CH6",
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
                interrupt: "UART8",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 0x1fff7a10,
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
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                pin: "PA10",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH2",
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
                    "DMA2_CH5",
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
                    "DMA2_CH7",
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
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
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
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
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
        name: "USART3",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "USART3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR",
                        field: "USART3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
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
                    7,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3",
            },
        ],
    },
    Peripheral {
        name: "USART6",
        address: 0x40011400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "USART6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "USART6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG12",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG8",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PG9",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA2_CH1",
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
                    "DMA2_CH2",
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
                    "DMA2_CH6",
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
                    "DMA2_CH7",
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
                interrupt: "USART6",
            },
        ],
    },
    Peripheral {
        name: "USB_OTG_FS",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "otg",
                version: "v1",
                block: "OTG",
                ir: &otg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "DCKCFGR",
                        field: "CLK48SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR",
                        field: "USB_OTG_FSEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR",
                        field: "USB_OTG_FSRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "ID",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SOF",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "VBUS",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_FS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_FS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "otg",
                version: "v1",
                block: "OTG",
                ir: &otg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "DCKCFGR",
                        field: "CLK48SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "USB_OTG_HSEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "USB_OTG_HSRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "ULPI_D0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SOF",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ULPI_CK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "ULPI_D1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "ULPI_D2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "ULPI_D3",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ULPI_D4",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ID",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "ULPI_D5",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ULPI_D6",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VBUS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DM",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DP",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ULPI_D7",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ULPI_STP",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "ULPI_DIR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ULPI_NXT",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PH4",
                signal: "ULPI_NXT",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PI11",
                signal: "ULPI_DIR",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EP1_IN",
                interrupt: "OTG_HS_EP1_IN",
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
                interrupt: "OTG_HS_WKUP",
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
    Interrupt {
        name: "RCC",
        number: 5,
    },
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
        name: "CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM9",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM10",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM11",
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
        name: "OTG_FS_WKUP",
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
        name: "SDIO",
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
        name: "ETH",
        number: 61,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 62,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 66,
    },
    Interrupt {
        name: "OTG_FS",
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
        name: "DCMI",
        number: 78,
    },
    Interrupt {
        name: "HASH_RNG",
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
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
];
            #[path="../registers/adc_v2.rs"] pub mod adc;
#[path="../registers/adccommon_v2.rs"] pub mod adccommon;
#[path="../registers/can_bxcan.rs"] pub mod can;
#[path="../registers/crc_v1.rs"] pub mod crc;
#[path="../registers/dac_v2.rs"] pub mod dac;
#[path="../registers/dbgmcu_f4.rs"] pub mod dbgmcu;
#[path="../registers/dcmi_v1.rs"] pub mod dcmi;
#[path="../registers/dma_v2.rs"] pub mod dma;
#[path="../registers/dma2d_v1.rs"] pub mod dma2d;
#[path="../registers/eth_v1b.rs"] pub mod eth;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_f4.rs"] pub mod flash;
#[path="../registers/fmc_v1x3.rs"] pub mod fmc;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v1.rs"] pub mod i2c;
#[path="../registers/iwdg_v1.rs"] pub mod iwdg;
#[path="../registers/ltdc_v1.rs"] pub mod ltdc;
#[path="../registers/otg_v1.rs"] pub mod otg;
#[path="../registers/pwr_f4.rs"] pub mod pwr;
#[path="../registers/rcc_f4.rs"] pub mod rcc;
#[path="../registers/rng_v1.rs"] pub mod rng;
#[path="../registers/rtc_v2f4.rs"] pub mod rtc;
#[path="../registers/sai_v1.rs"] pub mod sai;
#[path="../registers/sdmmc_v1.rs"] pub mod sdmmc;
#[path="../registers/spi_v1.rs"] pub mod spi;
#[path="../registers/syscfg_f4.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v2.rs"] pub mod usart;
#[path="../registers/wwdg_v1.rs"] pub mod wwdg;
