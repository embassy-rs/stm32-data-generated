
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "external interrupt configuration register 1",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exticr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some(
                        "Memory mapping selection bits",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "MemMode",
                    ),
                },
                Field {
                    name: "pa11_pa12_rmp",
                    description: Some(
                        "PA11 and PA12 remapping bit for small packages (28 and 20 pins)",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pa11Pa12Rmp",
                    ),
                },
                Field {
                    name: "ir_mod",
                    description: Some(
                        "IR Modulation Envelope signal selection",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "IrMod",
                    ),
                },
                Field {
                    name: "adc_dma_rmp",
                    description: Some(
                        "ADC DMA remapping bit",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AdcDmaRmp",
                    ),
                },
                Field {
                    name: "usart1_tx_dma_rmp",
                    description: Some(
                        "USART1_TX DMA remapping bit",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usart1TxDmaRmp",
                    ),
                },
                Field {
                    name: "usart1_rx_dma_rmp",
                    description: Some(
                        "USART1_RX DMA request remapping bit",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usart1RxDmaRmp",
                    ),
                },
                Field {
                    name: "tim16_dma_rmp",
                    description: Some(
                        "TIM16 DMA request remapping bit",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim16DmaRmp",
                    ),
                },
                Field {
                    name: "tim17_dma_rmp",
                    description: Some(
                        "TIM17 DMA request remapping bit",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim17DmaRmp",
                    ),
                },
                Field {
                    name: "tim16_dma_rmp2",
                    description: Some(
                        "TIM16 alternate DMA request remapping bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim16DmaRmp2",
                    ),
                },
                Field {
                    name: "tim17_dma_rmp2",
                    description: Some(
                        "TIM17 alternate DMA request remapping bit",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim17DmaRmp2",
                    ),
                },
                Field {
                    name: "i2c_pb6_fmp",
                    description: Some(
                        "Fast Mode Plus (FM plus) driving capability activation bits.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPb6Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb7_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPb7Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb8_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPb8Fmp",
                    ),
                },
                Field {
                    name: "i2c_pb9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPb9Fmp",
                    ),
                },
                Field {
                    name: "i2c1_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C1",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2c1Fmp",
                    ),
                },
                Field {
                    name: "i2c2_fmp",
                    description: Some(
                        "FM+ driving capability activation for I2C2",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2c2Fmp",
                    ),
                },
                Field {
                    name: "i2c_pa9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPa9Fmp",
                    ),
                },
                Field {
                    name: "i2c_pa10_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2cPa10Fmp",
                    ),
                },
                Field {
                    name: "spi2_dma_rmp",
                    description: Some(
                        "SPI2 DMA request remapping bit",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Spi2DmaRmp",
                    ),
                },
                Field {
                    name: "usart2_dma_rmp",
                    description: Some(
                        "USART2 DMA request remapping bit",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usart2DmaRmp",
                    ),
                },
                Field {
                    name: "usart3_dma_rmp",
                    description: Some(
                        "USART3 DMA request remapping bit",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usart3DmaRmp",
                    ),
                },
                Field {
                    name: "i2c1_dma_rmp",
                    description: Some(
                        "I2C1 DMA request remapping bit",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2c1DmaRmp",
                    ),
                },
                Field {
                    name: "tim1_dma_rmp",
                    description: Some(
                        "TIM1 DMA request remapping bit",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim1DmaRmp",
                    ),
                },
                Field {
                    name: "tim2_dma_rmp",
                    description: Some(
                        "TIM2 DMA request remapping bit",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim2DmaRmp",
                    ),
                },
                Field {
                    name: "tim3_dma_rmp",
                    description: Some(
                        "TIM3 DMA request remapping bit",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim3DmaRmp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex-M0 LOCKUP bit enable bit",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "LockupLock",
                    ),
                },
                Field {
                    name: "sram_parity_lock",
                    description: Some(
                        "SRAM parity lock bit",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SramParityLock",
                    ),
                },
                Field {
                    name: "pvd_lock",
                    description: Some(
                        "PVD lock enable bit",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PvdLock",
                    ),
                },
                Field {
                    name: "sram_pef",
                    description: Some(
                        "SRAM parity flag",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some(
                "external interrupt configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI configuration bits",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "AdcDmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "ADC DMA request mapped on DMA channel 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "ADC DMA request mapped on DMA channel 2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 3 and 2 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "I2C1_RX and I2C1_TX DMA requests mapped on DMA channel 7 and 6 respectively",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c1Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "FM+ mode is controlled by I2C_Pxx_FMP bits only",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "FM+ mode is enabled on all I2C1 pins selected through selection bits in GPIOx_AFR registers",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c2Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "FM+ mode is controlled by I2C_Pxx_FMP bits only",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "FM+ mode is enabled on all I2C2 pins selected through selection bits in GPIOx_AFR registers",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPa10Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PA10 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PA10 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPa9Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PA9 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PA9 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPb6Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PB6 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PB6 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPb7Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PB7 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PB7 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPb8Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PB8 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PB8 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2cPb9Fmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "PB9 pin operate in standard mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FMP",
                    description: Some(
                        "I2C FM+ mode enabled on PB9 and the Speed control is bypassed",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "IrMod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TIM16",
                    description: Some(
                        "TIM16 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USART1",
                    description: Some(
                        "USART1 selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USART4",
                    description: Some(
                        "USART4 selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "LockupLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISCONNECTED",
                    description: Some(
                        "Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "MemMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAINFLASH",
                    description: Some(
                        "Main Flash memory mapped at 0x0000_0000",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSTEMFLASH",
                    description: Some(
                        "System Flash memory mapped at 0x0000_0000",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAINFLASH2",
                    description: Some(
                        "Main Flash memory mapped at 0x0000_0000",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "Embedded SRAM mapped at 0x0000_0000",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pa11Pa12Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Pin pair PA9/PA10 mapped on the pins",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "Pin pair PA11/PA12 mapped instead of PA9/PA10",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PvdLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISCONNECTED",
                    description: Some(
                        "PVD interrupt disconnected from TIM1/15/16/17 Break input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "PVD interrupt connected to TIM1/15/16/17 Break input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spi2DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 4 and 5 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "SPI2_RX and SPI2_TX DMA requests mapped on DMA channel 6 and 7 respectively",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SramParityLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISCONNECTED",
                    description: Some(
                        "SRAM parity error disconnected from TIM1/15/16/17 Break input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "SRAM parity error connected to TIM1/15/16/17 Break input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim16DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim16DmaRmp2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTALTERNATEREMAPPED",
                    description: Some(
                        "TIM16 DMA request mapped according to TIM16_DMA_RMP bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALTERNATEREMAPPED",
                    description: Some(
                        "TIM16_CH1 and TIM16_UP DMA request mapped on DMA channel 6",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim17DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim17DmaRmp2",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTALTERNATEREMAPPED",
                    description: Some(
                        "TIM17 DMA request mapped according to TIM16_DMA_RMP bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALTERNATEREMAPPED",
                    description: Some(
                        "TIM17_CH1 and TIM17_UP DMA request mapped on DMA channel 7",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 2, 3 and 4 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM1_CH1, TIM1_CH2 and TIM1_CH3 DMA requests mapped on DMA channel 6",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim2DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 3 and 4 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM2_CH2 and TIM2_CH4 DMA requests mapped on DMA channel 7",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim3DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 4",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM3_CH1 and TIM3_TRIG DMA requests mapped on DMA channel 6",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1RxDmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "USART1_RX DMA request mapped on DMA channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "USART1_RX DMA request mapped on DMA channel 5",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1TxDmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "USART1_TX DMA request mapped on DMA channel 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "USART1_TX DMA request mapped on DMA channel 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart2DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "USART2_RX and USART2_TX DMA requests mapped on DMA channel 5 and 4 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "USART2_RX and USART2_TX DMA requests mapped on DMA channel 6 and 7 respectively",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart3DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "USART3_RX and USART3_TX DMA requests mapped on DMA channel 6 and 7 respectively (or simply disabled on STM32F0x0)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "USART3_RX and USART3_TX DMA requests mapped on DMA channel 3 and 2 respectively",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
