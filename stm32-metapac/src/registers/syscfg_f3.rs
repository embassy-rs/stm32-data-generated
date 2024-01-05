
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
                    name: "rcr",
                    description: Some(
                        "CCM SRAM protection register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exticr",
                    description: Some(
                        "external interrupt configuration register",
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
                BlockItem {
                    name: "cfgr4",
                    description: Some(
                        "configuration register 4",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr3",
                    description: Some(
                        "configuration register 3",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr3",
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
                    name: "usb_it_rmp",
                    description: Some(
                        "USB interrupt remap",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "UsbItRmp",
                    ),
                },
                Field {
                    name: "tim1_itr3_rmp",
                    description: Some(
                        "Timer 1 ITR3 selection",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim1Itr3Rmp",
                    ),
                },
                Field {
                    name: "dac1_trig_rmp",
                    description: Some(
                        "DAC trigger remap (when TSEL = 001)",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dac1TrigRmp",
                    ),
                },
                Field {
                    name: "dac_trig_rmp",
                    description: Some(
                        "DAC trigger remap (when TSEL = 001)",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "DacTrigRmp",
                    ),
                },
                Field {
                    name: "adc2_dma_rmp",
                    description: Some(
                        "ADC24 DMA remapping bit",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc2DmaRmpCfgr1",
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
                    name: "tim6_dac1_ch1_dma_rmp",
                    description: Some(
                        "TIM6 and DAC1 DMA request remapping bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim6Dac1Ch1DmaRmp",
                    ),
                },
                Field {
                    name: "tim6_dac1_dma_rmp",
                    description: Some(
                        "TIM6 and DAC1 DMA request remapping bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim6Dac1DmaRmp",
                    ),
                },
                Field {
                    name: "tim6_dac1_out1_dma_rmp",
                    description: Some(
                        "TIM6 and DAC1 DMA request remapping bit",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim6Dac1Out1DmaRmp",
                    ),
                },
                Field {
                    name: "tim7_dac1_ch2_dma_rmp",
                    description: Some(
                        "TIM7 and DAC2 DMA request remapping bit",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim7Dac1Ch2DmaRmp",
                    ),
                },
                Field {
                    name: "tim7_dac1_out2_dma_rmp",
                    description: Some(
                        "TIM7 and DAC2 DMA request remapping bit",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim7Dac1Out2DmaRmp",
                    ),
                },
                Field {
                    name: "dac2_ch1_dma_rmp",
                    description: Some(
                        "DAC2 channel1 DMA remap",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dac2Ch1DmaRmp",
                    ),
                },
                Field {
                    name: "tim18_dac2_out1_dma_rmp",
                    description: Some(
                        "TIM18 and DAC2_OUT1 DMA request remapping bit",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tim18Dac2Out1DmaRmp",
                    ),
                },
                Field {
                    name: "i2c_pb6_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) driving capability activation bits.",
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
                        "I2C1 Fast Mode Plus",
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
                        "I2C2 Fast Mode Plus",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2c2Fmp",
                    ),
                },
                Field {
                    name: "encoder_mode",
                    description: Some(
                        "Encoder mode",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EncoderMode",
                    ),
                },
                Field {
                    name: "i2c3_fmp",
                    description: Some(
                        "I2C3 Fast Mode Plus",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "I2c3Fmp",
                    ),
                },
                Field {
                    name: "vbat_mon",
                    description: Some(
                        "Enable the power switch to deliver VBAT voltage on ADC channel 18 input",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpu_ie",
                    description: Some(
                        "Idx 0: Invalid operation interrupt enable;\nIdx 1: Devide-by-zero interrupt enable;\nIdx 2: Underflow interrupt enable;\nIdx 3: Overflow interrupt enable;\nIdx 4: Input denormal interrupt enable;\nIdx 5: Inexact interrupt enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
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
                    name: "byp_addr_par",
                    description: Some(
                        "Bypass address bit 29 in parity calculation",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BypAddrPar",
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
            name: "Cfgr3",
            extends: None,
            description: Some(
                "configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1_rx_dma_rmp",
                    description: Some(
                        "SPI1_RX DMA remapping bit",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi1RxDmaRmp",
                    ),
                },
                Field {
                    name: "spi1_tx_dma_rmp",
                    description: Some(
                        "SPI1_TX DMA remapping bit",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi1TxDmaRmp",
                    ),
                },
                Field {
                    name: "i2c1_rx_dma_rmp",
                    description: Some(
                        "I2C1_RX DMA remapping bit",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1RxDmaRmp",
                    ),
                },
                Field {
                    name: "i2c1_tx_dma_rmp",
                    description: Some(
                        "I2C1_TX DMA remapping bit",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1TxDmaRmp",
                    ),
                },
                Field {
                    name: "adc2_dma_rmp",
                    description: Some(
                        "ADC2 DMA remapping bit",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adc2DmaRmpCfgr3",
                    ),
                },
                Field {
                    name: "dac1_trig3_rmp",
                    description: Some(
                        "DAC1_CH1 / DAC1_CH2 Trigger remap",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dac1Trig3Rmp",
                    ),
                },
                Field {
                    name: "dac1_trig5_rmp",
                    description: Some(
                        "DAC1_CH1 / DAC1_CH2 Trigger remap",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dac1Trig5Rmp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr4",
            extends: None,
            description: Some(
                "configuration register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc12_ext2_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 regular channel EXT2",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Ext2Rmp",
                    ),
                },
                Field {
                    name: "adc12_ext3_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 regular channel EXT3",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Ext3Rmp",
                    ),
                },
                Field {
                    name: "adc12_ext5_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 regular channel EXT5",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Ext5Rmp",
                    ),
                },
                Field {
                    name: "adc12_ext13_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 regular channel EXT13",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Ext13Rmp",
                    ),
                },
                Field {
                    name: "adc12_ext15_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 regular channel EXT15",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Ext15Rmp",
                    ),
                },
                Field {
                    name: "adc12_jext3_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 injected channel JEXT3",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Jext3Rmp",
                    ),
                },
                Field {
                    name: "adc12_jext6_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 injected channel JEXT6",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Jext6Rmp",
                    ),
                },
                Field {
                    name: "adc12_jext13_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC12 injected channel JEXT13",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc12Jext13Rmp",
                    ),
                },
                Field {
                    name: "adc34_ext5_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 regular channel EXT5",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Ext5Rmp",
                    ),
                },
                Field {
                    name: "adc34_ext6_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 regular channel EXT6",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Ext6Rmp",
                    ),
                },
                Field {
                    name: "adc34_ext15_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 regular channel EXT15",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Ext15Rmp",
                    ),
                },
                Field {
                    name: "adc34_jext5_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 injected channel JEXT5",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Jext5Rmp",
                    ),
                },
                Field {
                    name: "adc34_jext11_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 injected channel JEXT11",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Jext11Rmp",
                    ),
                },
                Field {
                    name: "adc34_jext14_rmp",
                    description: Some(
                        "Controls the Input trigger of ADC34 injected channel JEXT14",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adc34Jext14Rmp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some(
                "external interrupt configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI x configuration",
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
        FieldSet {
            name: "Rcr",
            extends: None,
            description: Some(
                "CCM SRAM protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "page_wp",
                    description: Some(
                        "CCM SRAM page x write protection enabled",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
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
            name: "Adc12Ext13Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM6",
                    description: Some(
                        "Trigger source is TIM6_TRGO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Ext15Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM3",
                    description: Some(
                        "Trigger source is TIM3_CC4",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC3",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Ext2Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM1",
                    description: Some(
                        "Trigger source is TIM3_CC3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "rigger source is TIM20_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Ext3Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM2",
                    description: Some(
                        "Trigger source is TIM2_CC2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "rigger source is TIM20_TRGO2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Ext5Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM4",
                    description: Some(
                        "Trigger source is TIM4_CC4",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Jext13Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM3",
                    description: Some(
                        "Trigger source is TIM3_CC1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Jext3Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM2",
                    description: Some(
                        "Trigger source is TIM2_CC1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc12Jext6Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EXTI15",
                    description: Some(
                        "Trigger source is EXTI line 15",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc2DmaRmpCfgr1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "ADC24 DMA requests mapped on DMA2 channels 1 and 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "ADC24 DMA requests mapped on DMA2 channels 3 and 4",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Adc2DmaRmpCfgr3",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAPDMA1CH2",
                    description: Some(
                        "ADC2 mapped on DMA1 channel 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MAPDMA1CH4",
                    description: Some(
                        "ADC2 mapped on DMA1 channel 4",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Adc34Ext15Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM2",
                    description: Some(
                        "Trigger source is TIM2_CC1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc34Ext5Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EXTI2",
                    description: Some(
                        "Trigger source is EXTI line 2 when reset at 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc34Ext6Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM4",
                    description: Some(
                        "Trigger source is TIM4_CC1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc34Jext11Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM1",
                    description: Some(
                        "Trigger source is TIM1_CC3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc34Jext14Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM7",
                    description: Some(
                        "Trigger source is TIM7_TRGO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_CC2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adc34Jext5Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM4",
                    description: Some(
                        "Trigger source is TIM4_CC3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM20",
                    description: Some(
                        "Trigger source is TIM20_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BypAddrPar",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOBYPASS",
                    description: Some(
                        "The ramload operation is performed taking into consideration bit 29 of the address when the parity is calculated",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BYPASS",
                    description: Some(
                        "The ramload operation is performed without taking into consideration bit 29 of the address when the parity is calculated",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dac1Trig3Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIM15",
                    description: Some(
                        "DAC trigger is TIM15_TRGO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HRTIM1",
                    description: Some(
                        "DAC trigger is HRTIM1_DAC1_TRIG1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dac1Trig5Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Not remapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "DAC trigger is HRTIM1_DAC1_TRIG2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dac1TrigRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "DAC trigger is TIM3_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dac2Ch1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Not remapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "DAC2_CH1 DMA requests mapped on DMA1 channel 5",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "DacTrigRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Not remapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "DAC trigger is TIM3_TRGO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EncoderMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NOREDIRECTION",
                    description: Some(
                        "No redirection",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MAPTIM2TIM15",
                    description: Some(
                        "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAPTIM3TIM15",
                    description: Some(
                        "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively",
                    ),
                    value: 2,
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
                        "FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c1RxDmaRmp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAPDMA1CH7",
                    description: Some(
                        "I2C1_RX mapped on DMA1 CH7",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MAPDMA1CH3",
                    description: Some(
                        "I2C1_RX mapped on DMA1 CH3",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAPDMA1CH5",
                    description: Some(
                        "I2C1_RX mapped on DMA1 CH5",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "I2c1TxDmaRmp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAPDMA1CH6",
                    description: Some(
                        "I2C1_TX mapped on DMA1 CH6",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MAPDMA1CH2",
                    description: Some(
                        "I2C1_TX mapped on DMA1 CH2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAPDMA1CH4",
                    description: Some(
                        "I2C1_TX mapped on DMA1 CH4",
                    ),
                    value: 2,
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
                        "FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c3Fmp",
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
                        "FM+ mode is enabled on all I2C3 pins selected through selection trhough IOPORT control registers AF selection bits",
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
            name: "LockupLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISCONNECTED",
                    description: Some(
                        "Cortex-M4 LOCKUP output disconnected from TIM1/15/16/17 Break inputs and HRTIM1 SYSFLT.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "Cortex-M4 LOCKUP output connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs",
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
            name: "PvdLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISCONNECTED",
                    description: Some(
                        "PVD interrupt disconnected from TIM15/16/17 Break input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "PVD interrupt connected to TIM15/16/17 Break input",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spi1RxDmaRmp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAPDMA1CH3",
                    description: Some(
                        "SPI1_RX mapped on DMA1 CH2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MAPDMA1CH5",
                    description: Some(
                        "SPI1_RX mapped on DMA1 CH4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAPDMA1CH7",
                    description: Some(
                        "SPI1_RX mapped on DMA1 CH6",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Spi1TxDmaRmp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAPDMA1CH3",
                    description: Some(
                        "SPI1_TX mapped on DMA1 CH3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MAPDMA1CH5",
                    description: Some(
                        "SPI1_TX mapped on DMA1 CH5",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAPDMA1CH7",
                    description: Some(
                        "SPI1_TX mapped on DMA1 CH7",
                    ),
                    value: 2,
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
                        "SRAM parity error signal disconnected from TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONNECTED",
                    description: Some(
                        "SRAM parity error signal connected to TIM1/15/16/17 and HRTIM1 SYSFLT Break inputs",
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
                        "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4",
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
                        "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim18Dac2Out1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM18 and DAC2_OUT1 DMA requests mapped on DMA2 channel 5",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM18 and DAC2_OUT1 DMA requests mapped on DMA1 channel 5",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim1Itr3Rmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Not remapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM1_ITR3 = TIM17_OC",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim6Dac1Ch1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim6Dac1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim6Dac1Out1DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM7 and DAC1_OUT1 DMA requests mapped on DMA2 channel 3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM7 and DAC1_OUT1 DMA requests mapped on DMA1 channel 3",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim7Dac1Ch2DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "Not remapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tim7Dac1Out2DmaRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "TIM7 and DAC1_OUT2 DMA requests mapped on DMA2 channel 4",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "TIM7 and DAC1_OUT2 DMA requests mapped on DMA1 channel 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "UsbItRmp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREMAPPED",
                    description: Some(
                        "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REMAPPED",
                    description: Some(
                        "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
