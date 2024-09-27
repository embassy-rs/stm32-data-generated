
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x42028000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "u5",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "ADCDACSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR1",
                        field: "ADC12EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "ADC12RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN14",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
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
                interrupt: "ADC1_2",
            },
        ],
    },
    Peripheral {
        name: "ADC12_COMMON",
        address: 0x42028300,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC2",
        address: 0x42028100,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "u5",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "ADCDACSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR1",
                        field: "ADC12EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "ADC12RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN14",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC1_2",
            },
        ],
    },
    Peripheral {
        name: "ADC4",
        address: 0x46021000,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "u5",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "ADCDACSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB3ENR",
                        field: "ADC4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB3RSTR",
                        field: "ADC4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN20",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN18",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN19",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN22",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN23",
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
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    7,
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
        address: 0x46021300,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADF1",
        address: 0x46024000,
        registers: Some(
            PeripheralRegisters {
                kind: "adf",
                version: "v1",
                block: "ADF",
                ir: &adf::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "ADF1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB3ENR",
                        field: "ADF1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB3RSTR",
                        field: "ADF1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "CCK0",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDI0",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CCK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SDI0",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    98,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT0",
                interrupt: "ADF1",
            },
        ],
    },
    Peripheral {
        name: "COMP1",
        address: 0x46005400,
        registers: Some(
            PeripheralRegisters {
                kind: "comp",
                version: "u5",
                block: "COMP",
                ir: &comp::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "COMPEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "COMPRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OUT",
                af: Some(
                    12,
                ),
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
                signal: "INP",
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
        name: "COMP2",
        address: 0x46005404,
        registers: Some(
            PeripheralRegisters {
                kind: "comp",
                version: "u5",
                block: "COMP",
                ir: &comp::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "COMPEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "COMPRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
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
                signal: "OUT",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
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
        name: "CORDIC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "cordic",
                version: "v1",
                block: "CORDIC",
                ir: &cordic::REGISTERS,
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
                        field: "CORDICEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "CORDICRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    101,
                ),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    102,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "CORDIC",
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
        name: "CRS",
        address: 0x40006000,
        registers: Some(
            PeripheralRegisters {
                kind: "crs",
                version: "v1",
                block: "CRS",
                ir: &crs::REGISTERS,
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
                        field: "CRSEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "CRSRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SYNC",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SYNC",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC1",
        address: 0x46021800,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v6",
                block: "DAC",
                ir: &dac::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "DAC1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB3ENR",
                        field: "DAC1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB3RSTR",
                        field: "DAC1RST",
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
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
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
                signal: "CH1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    9,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "DAC1",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0044000,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "u5",
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
        name: "DCACHE1",
        address: 0x40031400,
        registers: Some(
            PeripheralRegisters {
                kind: "dcache",
                version: "v1",
                block: "DCACHE",
                ir: &dcache::REGISTERS,
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
                        field: "DCACHE1EN",
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
                interrupt: "DCACHE1",
            },
        ],
    },
    Peripheral {
        name: "DCMI",
        address: 0x4202c000,
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
                        register: "AHB2ENR1",
                        field: "DCMIEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D12",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCMI",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    86,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "DCMI_PSSI",
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
        name: "EXTI",
        address: 0x46022000,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "u5",
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
        name: "FDCAN1",
        address: 0x4000a400,
        registers: Some(
            PeripheralRegisters {
                kind: "can",
                version: "fdcan_v1",
                block: "FDCAN",
                ir: &can::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "FDCAN1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR2",
                        field: "FDCAN1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR2",
                        field: "FDCAN1RST",
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
        name: "FDCANRAM1",
        address: 0x4000ac00,
        registers: Some(
            PeripheralRegisters {
                kind: "fdcanram",
                version: "v1",
                block: "FDCANRAM",
                ir: &fdcanram::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "u5",
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
        name: "FMAC",
        address: 0x40021400,
        registers: Some(
            PeripheralRegisters {
                kind: "fmac",
                version: "v1",
                block: "FMAC",
                ir: &fmac::REGISTERS,
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
                        field: "FMACEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "FMACRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "READ",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    99,
                ),
            },
            PeripheralDmaChannel {
                signal: "WRITE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    100,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "FMAC",
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
                signal: "CH14",
                interrupt: "GPDMA1_CHANNEL14",
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
                        register: "AHB2ENR1",
                        field: "GPIOAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                        register: "AHB2ENR1",
                        field: "GPIOBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                        register: "AHB2ENR1",
                        field: "GPIOCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42020c00,
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
                        register: "AHB2ENR1",
                        field: "GPIODEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42021000,
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
                        register: "AHB2ENR1",
                        field: "GPIOEEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42021400,
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
                        register: "AHB2ENR1",
                        field: "GPIOFEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42021800,
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
                        register: "AHB2ENR1",
                        field: "GPIOGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                        register: "AHB2ENR1",
                        field: "GPIOHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42022000,
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
                        register: "AHB2ENR1",
                        field: "GPIOIEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
        address: 0x42022400,
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
                        register: "AHB2ENR1",
                        field: "GPIOJEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                        register: "AHB2ENR1",
                        field: "HASHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                    89,
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
                pin: "PA1",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
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
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    12,
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
                    13,
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
                    14,
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
                        field: "I2C2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "I2C2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
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
                pin: "PB12",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB14",
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
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    17,
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
                bus_clock: "PCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "I2C3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "I2C3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "I2C3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SMBA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC1",
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
                    "LPDMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "EVC",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    18,
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
                    19,
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
                    20,
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
        name: "I2C4",
        address: 0x40008400,
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
                        field: "I2C4SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR2",
                        field: "I2C4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR2",
                        field: "I2C4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "SMBA",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
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
                    21,
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
                    22,
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
                    23,
                ),
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
        name: "ICACHE",
        address: 0x40030400,
        registers: Some(
            PeripheralRegisters {
                kind: "icache",
                version: "v1_3crr",
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
        name: "LPDMA1",
        address: 0x46025000,
        registers: Some(
            PeripheralRegisters {
                kind: "lpdma",
                version: "v1",
                block: "LPDMA",
                ir: &lpdma::REGISTERS,
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
                        field: "LPDMA1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB3RSTR",
                        field: "LPDMA1RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "LPDMA1_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "LPDMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "LPDMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "LPDMA1_CHANNEL3",
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
                bus_clock: "PCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "LPTIM1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "LPTIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "LPTIM1RST",
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
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    105,
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
                    106,
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
                    107,
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
                pin: "PA10",
                signal: "IN2",
                af: Some(
                    2,
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
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    14,
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
                    108,
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
                    109,
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
                    110,
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
        name: "LPTIM3",
        address: 0x46004800,
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
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "LPTIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "LPTIM3RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IN1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    111,
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
                    112,
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
                    113,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM3",
            },
        ],
    },
    Peripheral {
        name: "LPTIM4",
        address: 0x46004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2a",
                block: "LPTIM_BASIC",
                ir: &lptim::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "LPTIM4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "LPTIM4RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM4",
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
                bus_clock: "PCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "LPUART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "LPUART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "LPUART1RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
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
                    "LPDMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    1,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
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
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    35,
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
        name: "MDF1",
        address: 0x40025000,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "MDF1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB1ENR",
                        field: "MDF1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB1RSTR",
                        field: "MDF1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "SDI0",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDI1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CKI1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDI2",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CKI2",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CKI0",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDI5",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CKI5",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CCK0",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDI4",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CKI4",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CCK1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CKI3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SDI3",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "FLT0",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    92,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    93,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    94,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT3",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    95,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT4",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    96,
                ),
            },
            PeripheralDmaChannel {
                signal: "FLT5",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    97,
                ),
            },
        ],
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
        name: "OCTOSPI1",
        address: 0x420d1400,
        registers: Some(
            PeripheralRegisters {
                kind: "octospi",
                version: "v1",
                block: "OCTOSPI",
                ir: &octospi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "OCTOSPISEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR2",
                        field: "OCTOSPI1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR2",
                        field: "OCTOSPI1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OCTOSPI1",
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
                signal: "GLOBAL",
                interrupt: "OCTOSPI1",
            },
        ],
    },
    Peripheral {
        name: "OCTOSPI2",
        address: 0x420d2400,
        registers: Some(
            PeripheralRegisters {
                kind: "octospi",
                version: "v1",
                block: "OCTOSPI",
                ir: &octospi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "OCTOSPISEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR2",
                        field: "OCTOSPI2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR2",
                        field: "OCTOSPI2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "OCTOSPI2",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    41,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OCTOSPI2",
            },
        ],
    },
    Peripheral {
        name: "OCTOSPIM",
        address: 0x420c4000,
        registers: Some(
            PeripheralRegisters {
                kind: "octospim",
                version: "v1",
                block: "OCTOSPIM",
                ir: &octospim::REGISTERS,
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
                        register: "AHB2ENR1",
                        field: "OCTOSPIMEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "OCTOSPIMRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "P2_NCS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "P1_DQS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "P2_NCS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "P1_NCS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "P1_CLK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "P1_NCS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "P1_IO3",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "P1_IO2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "P1_IO1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "P1_IO0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P1_CLK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P1_NCLK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "P1_DQS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "P1_NCLK",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "P1_IO7",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "P1_IO4",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "P1_NCS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "P1_IO5",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "P1_IO6",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "P1_IO7",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x46005000,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "OPAMPEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "OPAMPRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP2",
        address: 0x46005010,
        registers: None,
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "OPAMPEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "OPAMPRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "VINP",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VINM",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "VOUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PSSI",
        address: 0x4202c400,
        registers: Some(
            PeripheralRegisters {
                kind: "pssi",
                version: "v1",
                block: "PSSI",
                ir: &pssi::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "DE",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D14",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PDCK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D12",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RDY",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D15",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "DCMI_PSSI",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 0x46020800,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "u5",
                block: "PWR",
                ir: &pwr::REGISTERS,
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
                signal: "CDSTOP",
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
                signal: "SRDSTOP",
                af: Some(
                    0,
                ),
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
                pin: "PB10",
                signal: "WKUP8",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "WKUP1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "WKUP6",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "WKUP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PVD_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "WKUP4",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "WKUP2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "WKUP5",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CSLEEP",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CDSTOP",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SRDSTOP",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "S3WU",
                interrupt: "PWR_S3WU",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x46020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "u5",
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
        address: 0x420c0800,
        registers: Some(
            PeripheralRegisters {
                kind: "rng",
                version: "v3",
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
                        register: "AHB2ENR1",
                        field: "RNGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
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
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
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
        name: "SAI1",
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "sai",
                version: "v4_2pdm",
                block: "SAI",
                ir: &sai::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "SAI1SEL",
                    },
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
                pin: "PA10",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK_B",
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
                signal: "SD_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "FS_B",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "MCLK_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "FS_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "D1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SD_A",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "D3",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "A",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "B",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    37,
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
        name: "SDMMC1",
        address: 0x420c8000,
        registers: Some(
            PeripheralRegisters {
                kind: "sdmmc",
                version: "v2",
                block: "SDMMC",
                ir: &sdmmc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "SDMMCSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR1",
                        field: "SDMMC1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "SDMMC1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CKIN",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CDIR",
                af: Some(
                    8,
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
                pin: "PC0",
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
                signal: "D0DIR",
                af: Some(
                    8,
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
                signal: "D123DIR",
                af: Some(
                    8,
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
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SDMMC1",
            },
        ],
    },
    Peripheral {
        name: "SDMMC2",
        address: 0x420c8c00,
        registers: Some(
            PeripheralRegisters {
                kind: "sdmmc",
                version: "v2",
                block: "SDMMC",
                ir: &sdmmc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK2",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR2",
                        field: "SDMMCSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR1",
                        field: "SDMMC2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "SDMMC2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CMD",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "D0",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "D2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CK",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SDMMC2",
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
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RDY",
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
                pin: "PA8",
                signal: "RDY",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RDY",
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
                signal: "TX",
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
                version: "v5",
                block: "SPI",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR1",
                        field: "SPI2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "SPI2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "SPI2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(
                    3,
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
                signal: "NSS",
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
                signal: "MISO",
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
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RDY",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: Some(
                    3,
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
                signal: "MOSI",
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
                bus_clock: "PCLK3",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR3",
                        field: "SPI3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "SPI3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
                        field: "SPI3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "RDY",
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
                signal: "NSS",
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
                signal: "MISO",
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
                pin: "PB8",
                signal: "RDY",
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
                signal: "MISO",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    2,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "LPDMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    10,
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
                    11,
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
                version: "u5",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK3",
                kernel_clock: Clock(
                    "PCLK3",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB3ENR",
                        field: "SYSCFGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB3RSTR",
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
        registers: Some(
            PeripheralRegisters {
                kind: "tamp",
                version: "u5",
                block: "TAMP",
                ir: &tamp::REGISTERS,
            },
        ),
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
                pin: "PC5",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "OUT5",
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
                pin: "PA10",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(
                    2,
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
                pin: "PC5",
                signal: "CH4N",
                af: Some(
                    1,
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
                    42,
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
                    43,
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
                    44,
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
                    45,
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
                    46,
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
                    47,
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
                    48,
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
        name: "TIM15",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
                block: "TIM_2CH_CMP",
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
                        field: "TIM15EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB2RSTR",
                        field: "TIM15RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
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
                    78,
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
                    79,
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
                    80,
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
                    81,
                ),
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
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB8",
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
                    82,
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
                    83,
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
                pin: "PA10",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB7",
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
                    84,
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
                    85,
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
                    14,
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
                    2,
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
                    2,
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
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    1,
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
                    56,
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
                    57,
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
                    58,
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
                    59,
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
                    60,
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
                signal: "CC1",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    61,
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
                    62,
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
                    63,
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
                    64,
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
                    65,
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
                    66,
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
                        field: "TIM4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
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
                    67,
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
                    68,
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
                    69,
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
                    70,
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
                    71,
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
                        field: "TIM5EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
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
                    72,
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
                    73,
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
                    74,
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
                    75,
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
                    76,
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
                    77,
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
                version: "v2",
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
                        register: "APB1ENR1",
                        field: "TIM6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "TIM6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
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
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v2",
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
                        register: "APB1ENR1",
                        field: "TIM7EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "TIM7RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UPD",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    5,
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
        address: 0x40013400,
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
                pin: "PB2",
                signal: "CH4N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN2",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
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
                signal: "BKIN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: Some(
                    3,
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
                    49,
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
                    50,
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
                    51,
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
                    52,
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
                    53,
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
                    54,
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
                    55,
                ),
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
        name: "TSC",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tsc",
                version: "v3",
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
                pin: "PB10",
                signal: "SYNC",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "G1_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "G1_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "G1_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "G2_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "G2_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "G2_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "G2_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "G3_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "G3_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "G3_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "G4_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G4_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "G4_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "G4_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SYNC",
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
        name: "UART4",
        address: 0x40004c00,
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
                        field: "UART4SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "UART4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
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
                pin: "PA15",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
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
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    30,
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
                    31,
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
                        field: "UART5SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "UART5EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "UART5RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "DE",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
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
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    33,
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
        name: "UCPD1",
        address: 0x4000dc00,
        registers: Some(
            PeripheralRegisters {
                kind: "ucpd",
                version: "v1",
                block: "UCPD",
                ir: &ucpd::REGISTERS,
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
                        register: "APB1ENR2",
                        field: "UCPD1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR2",
                        field: "UCPD1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CC1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "FRSTX1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DBCC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CC2",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "FRSTX1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "DBCC1",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "FRSTX2",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    90,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: None,
                dma: Some(
                    "GPDMA1",
                ),
                request: Some(
                    91,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UCPD1",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 0xbfa0700,
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
                signal: "DE",
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
                pin: "PB3",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
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
                signal: "TX",
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
                pin: "PA0",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "DE",
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
                pin: "PA15",
                signal: "RX",
                af: Some(
                    3,
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
                    26,
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
                    27,
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
                        field: "USART3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "USART3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "USART3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
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
                signal: "DE",
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
                pin: "PC4",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "DE",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RTS",
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
                    28,
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
                    29,
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
        address: 0x40006400,
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
                        register: "CCIPR2",
                        field: "USART6SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APB1ENR1",
                        field: "USART6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APB1RSTR1",
                        field: "USART6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PC0",
                signal: "CTS",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CK",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART6",
            },
        ],
    },
    Peripheral {
        name: "USB_OTG_HS",
        address: 0x42040000,
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
                        register: "CCIPR2",
                        field: "OTGHSSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHB2ENR1",
                        field: "USB_OTG_HSEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHB2RSTR1",
                        field: "USB_OTG_HSRST",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SOF",
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
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "EP1_OUT",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OTG_HS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "OTG_HS",
            },
        ],
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x46007400,
        registers: Some(
            PeripheralRegisters {
                kind: "vrefbuf",
                version: "v2a1",
                block: "VREFBUF",
                ir: &vrefbuf::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        name: "PVD_PVM",
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
        name: "ADC1_2",
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
        name: "TIM8_BRK",
        number: 51,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 52,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 53,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 54,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 55,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 56,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 57,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 58,
    },
    Interrupt {
        name: "SPI1",
        number: 59,
    },
    Interrupt {
        name: "SPI2",
        number: 60,
    },
    Interrupt {
        name: "USART1",
        number: 61,
    },
    Interrupt {
        name: "USART2",
        number: 62,
    },
    Interrupt {
        name: "USART3",
        number: 63,
    },
    Interrupt {
        name: "UART4",
        number: 64,
    },
    Interrupt {
        name: "UART5",
        number: 65,
    },
    Interrupt {
        name: "LPUART1",
        number: 66,
    },
    Interrupt {
        name: "LPTIM1",
        number: 67,
    },
    Interrupt {
        name: "LPTIM2",
        number: 68,
    },
    Interrupt {
        name: "TIM15",
        number: 69,
    },
    Interrupt {
        name: "TIM16",
        number: 70,
    },
    Interrupt {
        name: "TIM17",
        number: 71,
    },
    Interrupt {
        name: "COMP",
        number: 72,
    },
    Interrupt {
        name: "OTG_HS",
        number: 73,
    },
    Interrupt {
        name: "CRS",
        number: 74,
    },
    Interrupt {
        name: "FMC",
        number: 75,
    },
    Interrupt {
        name: "OCTOSPI1",
        number: 76,
    },
    Interrupt {
        name: "PWR_S3WU",
        number: 77,
    },
    Interrupt {
        name: "SDMMC1",
        number: 78,
    },
    Interrupt {
        name: "SDMMC2",
        number: 79,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL8",
        number: 80,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL9",
        number: 81,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL10",
        number: 82,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL11",
        number: 83,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL12",
        number: 84,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL13",
        number: 85,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL14",
        number: 86,
    },
    Interrupt {
        name: "GPDMA1_CHANNEL15",
        number: 87,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 88,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 89,
    },
    Interrupt {
        name: "SAI1",
        number: 90,
    },
    Interrupt {
        name: "SAI2",
        number: 91,
    },
    Interrupt {
        name: "TSC",
        number: 92,
    },
    Interrupt {
        name: "RNG",
        number: 94,
    },
    Interrupt {
        name: "FPU",
        number: 95,
    },
    Interrupt {
        name: "HASH",
        number: 96,
    },
    Interrupt {
        name: "LPTIM3",
        number: 98,
    },
    Interrupt {
        name: "SPI3",
        number: 99,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 100,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 101,
    },
    Interrupt {
        name: "MDF1_FLT0",
        number: 102,
    },
    Interrupt {
        name: "MDF1_FLT1",
        number: 103,
    },
    Interrupt {
        name: "MDF1_FLT2",
        number: 104,
    },
    Interrupt {
        name: "MDF1_FLT3",
        number: 105,
    },
    Interrupt {
        name: "UCPD1",
        number: 106,
    },
    Interrupt {
        name: "ICACHE",
        number: 107,
    },
    Interrupt {
        name: "LPTIM4",
        number: 110,
    },
    Interrupt {
        name: "DCACHE1",
        number: 111,
    },
    Interrupt {
        name: "ADF1",
        number: 112,
    },
    Interrupt {
        name: "ADC4",
        number: 113,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL0",
        number: 114,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL1",
        number: 115,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL2",
        number: 116,
    },
    Interrupt {
        name: "LPDMA1_CHANNEL3",
        number: 117,
    },
    Interrupt {
        name: "DMA2D",
        number: 118,
    },
    Interrupt {
        name: "DCMI_PSSI",
        number: 119,
    },
    Interrupt {
        name: "OCTOSPI2",
        number: 120,
    },
    Interrupt {
        name: "MDF1_FLT4",
        number: 121,
    },
    Interrupt {
        name: "MDF1_FLT5",
        number: 122,
    },
    Interrupt {
        name: "CORDIC",
        number: 123,
    },
    Interrupt {
        name: "FMAC",
        number: 124,
    },
    Interrupt {
        name: "LSECSSD",
        number: 125,
    },
    Interrupt {
        name: "USART6",
        number: 126,
    },
    Interrupt {
        name: "I2C5_ER",
        number: 127,
    },
    Interrupt {
        name: "I2C5_EV",
        number: 128,
    },
    Interrupt {
        name: "I2C6_ER",
        number: 129,
    },
    Interrupt {
        name: "I2C6_EV",
        number: 130,
    },
    Interrupt {
        name: "HSPI1",
        number: 131,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "LPDMA1_CH0",
        dma: "LPDMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "LPDMA1_CH1",
        dma: "LPDMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "LPDMA1_CH2",
        dma: "LPDMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "LPDMA1_CH3",
        dma: "LPDMA1",
        channel: 3,
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
    DmaChannel {
        name: "GPDMA1_CH8",
        dma: "GPDMA1",
        channel: 8,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH9",
        dma: "GPDMA1",
        channel: 9,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH10",
        dma: "GPDMA1",
        channel: 10,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH11",
        dma: "GPDMA1",
        channel: 11,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH12",
        dma: "GPDMA1",
        channel: 12,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH13",
        dma: "GPDMA1",
        channel: 13,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH14",
        dma: "GPDMA1",
        channel: 14,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "GPDMA1_CH15",
        dma: "GPDMA1",
        channel: 15,
        dmamux: None,
        dmamux_channel: None,
    },
];
            #[path="../registers/adc_u5.rs"] pub mod adc;
#[path="../registers/adf_v1.rs"] pub mod adf;
#[path="../registers/can_fdcan_v1.rs"] pub mod can;
#[path="../registers/comp_u5.rs"] pub mod comp;
#[path="../registers/cordic_v1.rs"] pub mod cordic;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/crs_v1.rs"] pub mod crs;
#[path="../registers/dac_v6.rs"] pub mod dac;
#[path="../registers/dbgmcu_u5.rs"] pub mod dbgmcu;
#[path="../registers/dcache_v1.rs"] pub mod dcache;
#[path="../registers/dcmi_v1.rs"] pub mod dcmi;
#[path="../registers/dma2d_v1.rs"] pub mod dma2d;
#[path="../registers/exti_u5.rs"] pub mod exti;
#[path="../registers/fdcanram_v1.rs"] pub mod fdcanram;
#[path="../registers/flash_u5.rs"] pub mod flash;
#[path="../registers/fmac_v1.rs"] pub mod fmac;
#[path="../registers/gpdma_v1.rs"] pub mod gpdma;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/hash_v4.rs"] pub mod hash;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/icache_v1_3crr.rs"] pub mod icache;
#[path="../registers/iwdg_v3.rs"] pub mod iwdg;
#[path="../registers/lpdma_v1.rs"] pub mod lpdma;
#[path="../registers/lptim_v2a.rs"] pub mod lptim;
#[path="../registers/octospi_v1.rs"] pub mod octospi;
#[path="../registers/octospim_v1.rs"] pub mod octospim;
#[path="../registers/otg_v1.rs"] pub mod otg;
#[path="../registers/pssi_v1.rs"] pub mod pssi;
#[path="../registers/pwr_u5.rs"] pub mod pwr;
#[path="../registers/rcc_u5.rs"] pub mod rcc;
#[path="../registers/rng_v3.rs"] pub mod rng;
#[path="../registers/rtc_v3u5.rs"] pub mod rtc;
#[path="../registers/sai_v4_2pdm.rs"] pub mod sai;
#[path="../registers/sdmmc_v2.rs"] pub mod sdmmc;
#[path="../registers/spi_v5.rs"] pub mod spi;
#[path="../registers/syscfg_u5.rs"] pub mod syscfg;
#[path="../registers/tamp_u5.rs"] pub mod tamp;
#[path="../registers/timer_v2.rs"] pub mod timer;
#[path="../registers/tsc_v3.rs"] pub mod tsc;
#[path="../registers/ucpd_v1.rs"] pub mod ucpd;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v4.rs"] pub mod usart;
#[path="../registers/vrefbuf_v2a1.rs"] pub mod vrefbuf;
#[path="../registers/wwdg_v2.rs"] pub mod wwdg;
