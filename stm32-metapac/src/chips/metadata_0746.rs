
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "u0",
                block: "ADC",
                ir: &adc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "ADCSEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR2",
                        field: "ADCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "ADCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN17",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN18",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN15",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN16",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    5,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC_COMP1_2",
            },
        ],
    },
    Peripheral {
        name: "ADC1_COMMON",
        address: 0x40012708,
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
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "aes",
                version: "v2",
                block: "AES",
                ir: &aes::REGISTERS,
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
                        field: "AESEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "OUT",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    7,
                ),
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "COMP1",
        address: 0x40010200,
        registers: Some(
            PeripheralRegisters {
                kind: "comp",
                version: "u0",
                block: "COMP",
                ir: &comp::REGISTERS,
            },
        ),
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
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "INP",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "OUT",
                af: Some(
                    12,
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
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA9",
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
            PeripheralPin {
                pin: "PC6",
                signal: "INP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "ADC_COMP1_2",
            },
        ],
    },
    Peripheral {
        name: "COMP2",
        address: 0x40010204,
        registers: Some(
            PeripheralRegisters {
                kind: "comp",
                version: "u0",
                block: "COMP",
                ir: &comp::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "INM",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OUT",
                af: Some(
                    12,
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
                pin: "PA5",
                signal: "INM",
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
                pin: "PB11",
                signal: "OUT",
                af: Some(
                    12,
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
                interrupt: "ADC_COMP1_2",
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
        name: "CRS",
        address: 0x40006c00,
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
                        register: "APBENR1",
                        field: "CRSEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                    10,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC1",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "dac",
                version: "v4",
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
                        register: "APBENR1",
                        field: "DAC1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    8,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM6_DAC_LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: None,
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
                version: "v1",
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
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "bdma",
                version: "v1",
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
                        field: "DMA2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
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
                signal: "CH1",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
            },
        ],
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "dmamux",
                version: "v1",
                block: "DMAMUX",
                ir: &dmamux::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "u0",
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
                version: "u0",
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
                        register: "AHBENR",
                        field: "FLASHEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "FLASHRST",
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
        name: "GPIOD",
        address: 0x50000c00,
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
                        field: "GPIODEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
        address: 0x50001000,
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
                        field: "GPIOEEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
        address: 0x50001400,
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
                        field: "GPIOFEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "GPIORSTR",
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
                        register: "APBENR1",
                        field: "I2C1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    10,
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
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "I2C2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    5,
                ),
            },
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
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
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
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    12,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_3_4",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_3_4",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 0x40008800,
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
                        field: "I2C3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "I2C3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SCL",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    14,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_3_4",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_3_4",
            },
        ],
    },
    Peripheral {
        name: "I2C4",
        address: 0x4000a000,
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
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "I2C4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C4RST",
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
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SCL",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    16,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_3_4",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_3_4",
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
                interrupt: "WWDG_IWDG",
            },
        ],
    },
    Peripheral {
        name: "LCD",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "lcd",
                version: "v2",
                block: "LCD",
                ir: &lcd::REGISTERS,
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
                        register: "APBENR1",
                        field: "LCDEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "LCDRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SEG42",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SEG0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COM2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SEG40",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SEG41",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SEG17",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SEG1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SEG2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SEG43",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SEG44",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SEG3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SEG4",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "COM0",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "COM1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SEG5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SEG6",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SEG10",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SEG11",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SEG12",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SEG13",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SEG14",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SEG15",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SEG7",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SEG8",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SEG9",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SEG21",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SEG16",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "COM3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "BIAS1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SEG18",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "BIAS2",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SEG19",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "COM4",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SEG28",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SEG48",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "COM5",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SEG29",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SEG49",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "COM6",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SEG30",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SEG50",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "BIAS3",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SEG20",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SEG22",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SEG23",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SEG24",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SEG25",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SEG26",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SEG27",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "COM7",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SEG31",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SEG51",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LCD",
            },
        ],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2b",
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
                        register: "APBENR1",
                        field: "LPTIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                    2,
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
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH4",
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
                pin: "PB8",
                signal: "IN2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: Some(
                    14,
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
                pin: "PC13",
                signal: "CH3",
                af: Some(
                    2,
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    19,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    21,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM6_DAC_LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2b",
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
                        field: "LPTIM2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "LPTIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "LPTIM2RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
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
                    14,
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    23,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    24,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM7_LPTIM2",
            },
        ],
    },
    Peripheral {
        name: "LPTIM3",
        address: 0x40009000,
        registers: Some(
            PeripheralRegisters {
                kind: "lptim",
                version: "v2b",
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
                        field: "LPTIM3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "LPTIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "LPTIM3RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CH3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN2",
                af: Some(
                    4,
                ),
            },
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
                pin: "PB8",
                signal: "IN2",
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
                pin: "PC12",
                signal: "CH3",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH3",
                af: Some(
                    4,
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
                pin: "PC5",
                signal: "CH3",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH4",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "IC1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    25,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "IC4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "UE",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    29,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM15_LPTIM3",
            },
        ],
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40008000,
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "LPUART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "LPUART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                pin: "PB11",
                signal: "TX",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    31,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3_LPUART1",
            },
        ],
    },
    Peripheral {
        name: "LPUART2",
        address: 0x40008400,
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "LPUART2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "LPUART2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "LPUART2RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DE",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                af: Some(
                    10,
                ),
            },
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    33,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2_LPUART2",
            },
        ],
    },
    Peripheral {
        name: "LPUART3",
        address: 0x40008c00,
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "LPUART3SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "LPUART3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "LPUART3RST",
                    },
                ),
                stop_mode: StopMode::Stop2,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
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
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC5",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    34,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    35,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART4_LPUART3",
            },
        ],
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "opamp",
                version: "u0",
                block: "OPAMP",
                ir: &opamp::REGISTERS,
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
                        register: "APBENR1",
                        field: "OPAMPEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "u0",
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
                        register: "APBENR1",
                        field: "PWREN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "PWRRST",
                    },
                ),
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
                pin: "PB15",
                signal: "WKUP7",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PVD_IN",
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
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "u0",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MCO2",
                af: Some(
                    3,
                ),
            },
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
                pin: "PA8",
                signal: "MCO2",
                af: Some(
                    3,
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
                signal: "OSC32_EN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC_EN",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MCO2",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "OSC_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_EN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "OSC_OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MCO",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CRS",
                interrupt: "RCC_CRS",
            },
            PeripheralInterrupt {
                signal: "RCC",
                interrupt: "RCC_CRS",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 0x40025000,
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
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHBENR",
                        field: "RNGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
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
                interrupt: "RNG_CRYP",
            },
        ],
    },
    Peripheral {
        name: "RTC",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v3",
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
                        register: "APBENR1",
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
                af: Some(
                    0,
                ),
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
                signal: "TAMP",
                interrupt: "RTC_TAMP",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
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
                        register: "APBENR2",
                        field: "SPI1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
                pin: "PB0",
                signal: "NSS",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    37,
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
                version: "v2",
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
                        register: "APBENR1",
                        field: "SPI2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "SPI2RST",
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
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "NSS",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(
                    6,
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    38,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    39,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2_3",
            },
        ],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
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
                        register: "APBENR1",
                        field: "SPI3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "SPI3RST",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    41,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2_3",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "u0",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
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
                        register: "APBENR2",
                        field: "SYSCFGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
        address: 0x4000b000,
        registers: None,
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN4",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "TIM1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR2",
                        field: "TIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    45,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    46,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    47,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_UP_TRG_COM",
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "TIM15SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR2",
                        field: "TIM15EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    62,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    63,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    64,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    65,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM15_LPTIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM15_LPTIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM15_LPTIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM15_LPTIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM15_LPTIM3",
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
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR2",
                        field: "TIM16EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    66,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    67,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    68,
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
                        register: "APBENR1",
                        field: "TIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    48,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    50,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    51,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    52,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    53,
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
                        register: "APBENR1",
                        field: "TIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                signal: "CH1",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    54,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    55,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    56,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    57,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    58,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    59,
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
                        register: "APBENR1",
                        field: "TIM6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    60,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_DAC_LPTIM1",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_DAC_LPTIM1",
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
                        register: "APBENR1",
                        field: "TIM7EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    61,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7_LPTIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7_LPTIM2",
            },
        ],
    },
    Peripheral {
        name: "TSC",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "tsc",
                version: "v2",
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
                        register: "AHBENR",
                        field: "TSCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "TSCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "G7_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "G7_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "G3_IO4",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "G3_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G5_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "G7_IO1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "G7_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "G5_IO2",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SYNC",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G5_IO3",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G5_IO4",
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
                pin: "PB15",
                signal: "G1_IO4",
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
        name: "UID",
        address: 0x1fff6e50,
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
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "USART1SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR2",
                        field: "USART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    69,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    70,
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
                        register: "CCIPR",
                        field: "USART2SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "USART2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    71,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    72,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2_LPUART2",
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
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "USART3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
                signal: "TX",
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
                signal: "RX",
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
                pin: "PB8",
                signal: "TX",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
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
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    73,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    74,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3_LPUART1",
            },
        ],
    },
    Peripheral {
        name: "USART4",
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
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "USART4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "USART4RST",
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
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CK",
                af: Some(
                    8,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    75,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some(
                    "DMAMUX1",
                ),
                dma: None,
                request: Some(
                    76,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART4_LPUART3",
            },
        ],
    },
    Peripheral {
        name: "USB",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usb",
                version: "v4",
                block: "USB",
                ir: &usb::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Mux(
                    PeripheralRccRegister {
                        register: "CCIPR",
                        field: "CLK48SEL",
                    },
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "USBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "USBRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
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
            PeripheralPin {
                pin: "PA13",
                signal: "NOE",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "NOE",
                af: Some(
                    10,
                ),
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
        address: 0x40009800,
        registers: Some(
            PeripheralRegisters {
                kind: "usbram",
                version: "32_1024",
                block: "USBRAM",
                ir: &usbram::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VREFBUF",
        address: 0x40010030,
        registers: None,
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
                        register: "APBENR1",
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
                interrupt: "WWDG_IWDG",
            },
        ],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG_IWDG",
        number: 0,
    },
    Interrupt {
        name: "PVD_PVM",
        number: 1,
    },
    Interrupt {
        name: "RTC_TAMP",
        number: 2,
    },
    Interrupt {
        name: "FLASH_ECC",
        number: 3,
    },
    Interrupt {
        name: "RCC_CRS",
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
        name: "USB_DRD_FS",
        number: 8,
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
        name: "DMA1_CH4_7_DMA2_CH1_5_DMAMUX_OVR",
        number: 11,
    },
    Interrupt {
        name: "ADC_COMP1_2",
        number: 12,
    },
    Interrupt {
        name: "TIM1_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 14,
    },
    Interrupt {
        name: "TIM2",
        number: 15,
    },
    Interrupt {
        name: "TIM3",
        number: 16,
    },
    Interrupt {
        name: "TIM6_DAC_LPTIM1",
        number: 17,
    },
    Interrupt {
        name: "TIM7_LPTIM2",
        number: 18,
    },
    Interrupt {
        name: "TIM15_LPTIM3",
        number: 19,
    },
    Interrupt {
        name: "TIM16",
        number: 20,
    },
    Interrupt {
        name: "TSC",
        number: 21,
    },
    Interrupt {
        name: "LCD",
        number: 22,
    },
    Interrupt {
        name: "I2C1",
        number: 23,
    },
    Interrupt {
        name: "I2C2_3_4",
        number: 24,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "SPI2_3",
        number: 26,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
    Interrupt {
        name: "USART2_LPUART2",
        number: 28,
    },
    Interrupt {
        name: "USART3_LPUART1",
        number: 29,
    },
    Interrupt {
        name: "USART4_LPUART3",
        number: 30,
    },
    Interrupt {
        name: "RNG_CRYP",
        number: 31,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            0,
        ),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            1,
        ),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            2,
        ),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            3,
        ),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            4,
        ),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            5,
        ),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            6,
        ),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            7,
        ),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            8,
        ),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            9,
        ),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            10,
        ),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: Some(
            "DMAMUX1",
        ),
        dmamux_channel: Some(
            11,
        ),
    },
];
            #[path="../registers/adc_u0.rs"] pub mod adc;
#[path="../registers/adccommon_v3.rs"] pub mod adccommon;
#[path="../registers/aes_v2.rs"] pub mod aes;
#[path="../registers/bdma_v1.rs"] pub mod bdma;
#[path="../registers/comp_u0.rs"] pub mod comp;
#[path="../registers/crc_v3.rs"] pub mod crc;
#[path="../registers/crs_v1.rs"] pub mod crs;
#[path="../registers/dac_v4.rs"] pub mod dac;
#[path="../registers/dmamux_v1.rs"] pub mod dmamux;
#[path="../registers/exti_u0.rs"] pub mod exti;
#[path="../registers/flash_u0.rs"] pub mod flash;
#[path="../registers/gpio_v2.rs"] pub mod gpio;
#[path="../registers/i2c_v2.rs"] pub mod i2c;
#[path="../registers/iwdg_v3.rs"] pub mod iwdg;
#[path="../registers/lcd_v2.rs"] pub mod lcd;
#[path="../registers/lptim_v2b.rs"] pub mod lptim;
#[path="../registers/opamp_u0.rs"] pub mod opamp;
#[path="../registers/pwr_u0.rs"] pub mod pwr;
#[path="../registers/rcc_u0.rs"] pub mod rcc;
#[path="../registers/rng_v3.rs"] pub mod rng;
#[path="../registers/rtc_v3.rs"] pub mod rtc;
#[path="../registers/spi_v2.rs"] pub mod spi;
#[path="../registers/syscfg_u0.rs"] pub mod syscfg;
#[path="../registers/timer_v2.rs"] pub mod timer;
#[path="../registers/tsc_v2.rs"] pub mod tsc;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v4.rs"] pub mod usart;
#[path="../registers/usb_v4.rs"] pub mod usb;
#[path="../registers/usbram_32_1024.rs"] pub mod usbram;
#[path="../registers/wwdg_v2.rs"] pub mod wwdg;
