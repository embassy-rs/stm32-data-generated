
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock controller",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RCC clock control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hsicfgr",
                    description: Some(
                        "RCC HSI calibration register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hsicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crrcr",
                    description: Some(
                        "RCC clock recovery RC register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csicfgr",
                    description: Some(
                        "RCC CSI calibration register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "RCC clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "RCC CPU domain clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "pllcfgr",
                    description: Some(
                        "RCC PLL clock source selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plldivr",
                    description: Some(
                        "RCC PLL1 dividers register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plldivr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllfracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllfracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock source interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cifr",
                    description: Some(
                        "RCC clock source interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cicr",
                    description: Some(
                        "RCC clock source interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 reset register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr",
                    description: Some(
                        "RCC AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lrstr",
                    description: Some(
                        "RCC APB1 peripheral low reset register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hrstr",
                    description: Some(
                        "RCC APB1 peripheral high reset register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "RCC APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3rstr",
                    description: Some(
                        "RCC APB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripherals clock register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr",
                    description: Some(
                        "RCC AHB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lenr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1henr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1henr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "RCC APB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3enr",
                    description: Some(
                        "RCC APB3 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1lpenr",
                    description: Some(
                        "RCC AHB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2lpenr",
                    description: Some(
                        "RCC AHB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1llpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1llpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hlpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hlpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "RCC APB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3lpenr",
                    description: Some(
                        "RCC APB3 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr2",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr3",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr4",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr5",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsr",
                    description: Some(
                        "RCC reset status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "RCC AHB1 peripherals clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2en",
                    description: Some(
                        "GPDMA2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flitfen",
                    description: Some(
                        "Flash interface clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramen",
                    description: Some(
                        "BKPRAM clock enable\r Set and reset by software",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1en",
                    description: Some(
                        "SRAM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "RCC AHB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1lpen",
                    description: Some(
                        "GPDMA1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2lpen",
                    description: Some(
                        "GPDMA2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flitflpen",
                    description: Some(
                        "Flash interface (FLITF) clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfglpen",
                    description: Some(
                        "RAMCFG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramlpen",
                    description: Some(
                        "BKPRAM clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icachelpen",
                    description: Some(
                        "ICACHE clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1lpen",
                    description: Some(
                        "SRAM1 clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "RCC AHB1 reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2rst",
                    description: Some(
                        "GPDMA2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC block reset Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgrst",
                    description: Some(
                        "RAMCFG block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "GPIOA clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "GPIOB clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "GPIOC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioden",
                    description: Some(
                        "GPIOD clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "GPIOH clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1en",
                    description: Some(
                        "ADC1 peripherals clock enabled\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1en",
                    description: Some(
                        "DAC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashen",
                    description: Some(
                        "HASH clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "RNG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2en",
                    description: Some(
                        "SRAM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "RCC AHB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "GPIOA clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioblpen",
                    description: Some(
                        "GPIOB clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioclpen",
                    description: Some(
                        "GPIOC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodlpen",
                    description: Some(
                        "GPIOD clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohlpen",
                    description: Some(
                        "GPIOH clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1lpen",
                    description: Some(
                        "ADC1 peripherals clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1lpen",
                    description: Some(
                        "DAC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashlpen",
                    description: Some(
                        "HASH clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rnglpen",
                    description: Some(
                        "RNG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2lpen",
                    description: Some(
                        "SRAM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "GPIOA block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobrst",
                    description: Some(
                        "GPIOB block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocrst",
                    description: Some(
                        "GPIOC block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodrst",
                    description: Some(
                        "GPIOD block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "GPIOH block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1rst",
                    description: Some(
                        "ADC1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1rst",
                    description: Some(
                        "DAC block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashrst",
                    description: Some(
                        "HASH block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngrst",
                    description: Some(
                        "RNG block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1henr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtsen",
                    description: Some(
                        "DTS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan12en",
                    description: Some(
                        "FDCAN1 peripheral clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1hlpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtslpen",
                    description: Some(
                        "DTS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2lpen",
                    description: Some(
                        "LPTIM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan12lpen",
                    description: Some(
                        "FDCAN1 peripheral clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1hrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral high reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtsrst",
                    description: Some(
                        "DTS block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan12rst",
                    description: Some(
                        "FDCAN1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lenr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3en",
                    description: Some(
                        "TIM3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6en",
                    description: Some(
                        "TIM6 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7en",
                    description: Some(
                        "TIM7 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opampen",
                    description: Some(
                        "OPAMP clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compen",
                    description: Some(
                        "COMP clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1en",
                    description: Some(
                        "I3C1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsen",
                    description: Some(
                        "CRS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1llpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3lpen",
                    description: Some(
                        "TIM3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6lpen",
                    description: Some(
                        "TIM6 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7lpen",
                    description: Some(
                        "TIM7 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdglpen",
                    description: Some(
                        "WWDG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opamplpen",
                    description: Some(
                        "OPAMP clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2lpen",
                    description: Some(
                        "SPI2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3lpen",
                    description: Some(
                        "SPI3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "complpen",
                    description: Some(
                        "COMP clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2lpen",
                    description: Some(
                        "USART2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3lpen",
                    description: Some(
                        "USART3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1lpen",
                    description: Some(
                        "I2C1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2lpen",
                    description: Some(
                        "I2C2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1lpen",
                    description: Some(
                        "I3C1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crslpen",
                    description: Some(
                        "CRS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral low reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3rst",
                    description: Some(
                        "TIM3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6rst",
                    description: Some(
                        "TIM6 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7rst",
                    description: Some(
                        "TIM7 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opamprst",
                    description: Some(
                        "OPAMP block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2rst",
                    description: Some(
                        "SPI2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3rst",
                    description: Some(
                        "SPI3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comprst",
                    description: Some(
                        "COMP block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1rst",
                    description: Some(
                        "I3C1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsrst",
                    description: Some(
                        "CRS block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "SPI1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usben",
                    description: Some(
                        "USB clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "RCC APB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1lpen",
                    description: Some(
                        "SPI1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1lpen",
                    description: Some(
                        "USART1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usblpen",
                    description: Some(
                        "USB clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1rst",
                    description: Some(
                        "USART1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbrst",
                    description: Some(
                        "USB block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3enr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SBS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2en",
                    description: Some(
                        "I3C2EN clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREF clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "RTC APB interface clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3lpenr",
            extends: None,
            description: Some(
                "RCC APB3 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfglpen",
                    description: Some(
                        "SBS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1lpen",
                    description: Some(
                        "LPUART1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2lpen",
                    description: Some(
                        "I3C2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1lpen",
                    description: Some(
                        "LPTIM1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vreflpen",
                    description: Some(
                        "VREF clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapblpen",
                    description: Some(
                        "RTC APB interface clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SBS block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2rst",
                    description: Some(
                        "I3C2RST block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefrst",
                    description: Some(
                        "VREF block reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enabled\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdy",
                    description: Some(
                        "LSE oscillator ready\r Set and reset by hardware to indicate when the LSE is stable.\r This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "LSE oscillator bypass\r Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator driving capability\r Set by software to select the driving capability of the LSE oscillator.\r These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "lsecsson",
                    description: Some(
                        "LSE clock security system enable\r Set by software to enable the clock security system on 32 kHz oscillator.\r LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssd",
                    description: Some(
                        "LSE clock security system failure detection\r Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lseext",
                    description: Some(
                        "low-speed external clock type in bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the LSEON bit, to be used by the device.\r The LSEEXT bit can be written only if the LSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lseext",
                    ),
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection\r Set by software to select the clock source for the RTC.\r These bits can be written only one time (except in case of failure detection on LSE).\r These bits must be written before LSECSSON is enabled.\r The VSWRST bit can be used to reset them, then it can be written one time again.\r If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vswrst",
                    description: Some(
                        "VSwitch domain software reset\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscosel",
                    description: Some(
                        "Low-speed clock output selection\r Set and cleared by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lscosel",
                    ),
                },
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable\r Set and cleared by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready\r Set and cleared by hardware to indicate when the LSI oscillator is stable.\r After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles.\r This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "TIM2, TIM3 and LPTIM2 input capture source selection\r Set and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timicsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr2",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "LPTIM2 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr4",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "systicksel",
                    description: Some(
                        "SYSTICK clock source selection\r Note: rcc_hclk frequency must be four times higher than\r lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "usbsel",
                    description: Some(
                        "USB kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbsel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i3c1sel",
                    description: Some(
                        "I3C1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i3c2sel",
                    description: Some(
                        "I3C2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I3c2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr5",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcdacsel",
                    description: Some(
                        "ADC and DAC kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcdacsel",
                    ),
                },
                Field {
                    name: "dacholdsel",
                    description: Some(
                        "DAC hold clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dacholdsel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNG kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "fdcan12sel",
                    description: Some(
                        "FDCAN1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "persel",
                    description: Some(
                        "per_ck clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Persel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "RCC clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock and trace clock switch\r Set and reset by software to select system clock and trace clock sources (sys_ck).\r Set by hardware in order to:\r -\tforce the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode\r -\tforce the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock\r others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "system clock switch status\r Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset).\r others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "system clock selection after a wakeup from system Stop\r Set and reset by software to select the system wakeup clock from system Stop.\r The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset)\r STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "stopkerwuck",
                    description: Some(
                        "kernel clock selection after a wakeup from system Stop\r Set and reset by software to select the kernel wakeup clock from system Stop.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopkerwuck",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock\r Set and cleared by software to divide the HSE to generate a clock for RTC.\r Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source.\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "timers clocks prescaler selection\r This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timpre",
                    ),
                },
                Field {
                    name: "mco1pre",
                    description: Some(
                        "MCO1 prescaler\r Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco1sel",
                    description: Some(
                        "Microcontroller clock output 1\r Set and cleared by software. Clock source selection may generate glitches on MCO1.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco1sel",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "MCO2 prescaler\r Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2sel",
                    description: Some(
                        "microcontroller clock output 2\r Set and cleared by software. Clock source selection may generate glitches on MCO2.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC CPU domain clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler\r Set and reset by software to control the division factor of rcc_hclk. Changing\r this division ratio has an impact on the frequency of all bus matrix clocks\r 0xxx: rcc_hclk = sys_ck (default after reset)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB low-speed prescaler (APB1)\r Set and reset by software to control the division factor of rcc_pclk1.\r The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write.\r 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "APB high-speed prescaler (APB2)\r Set and reset by software to control APB high-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write.\r 0xx: rcc_pclk2 = rcc_hclk1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre3",
                    description: Some(
                        "APB low-speed prescaler (APB3)\r Set and reset by software to control APB low-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write.\r 0xx: rcc_pclk3 = rcc_hclk1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ahb1dis",
                    description: Some(
                        "AHB1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB1\r peripherals from RCC_AHB1ENR are used and when their clocks are disabled in\r RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from\r RCC_AHB1ENR are off.\r enable control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb2dis",
                    description: Some(
                        "AHB2 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB2\r peripherals from RCC_AHB2ENR are used and when their clocks are disabled in\r RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from\r RCC_AHB2ENR are off.\r enable control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb4dis",
                    description: Some(
                        "AHB4 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB4\r peripherals from RCC_AHB4ENR are used and when their clocks are disabled in\r RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from\r RCC_AHB4ENR are off.\r enable control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb1dis",
                    description: Some(
                        "APB1 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB1\r peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR.\r When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.\r control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb2dis",
                    description: Some(
                        "APB2 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB2\r peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is\r set, all the APB2 peripherals clocks are off.\r control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb3dis",
                    description: Some(
                        "APB3 clock disable value.Set and cleared by software\r This bit can be set in order to further reduce power consumption, when none of the APB3\r peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is\r set, all the APB3 peripherals clocks are off.\r control bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock source interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear\r Set by software to clear LSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear\r Set by software to clear LSERDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear CSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear HSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear\r Set by software to clear HSERDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 ready interrupt clear\r Set by software to clear HSI48RDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL1 ready interrupt clear\r Set by software to clear PLL1RDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hsecssc",
                    description: Some(
                        "HSE clock security system interrupt clear\r Set by software to clear HSECSSF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock source interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyie",
                    description: Some(
                        "CSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "PLL1 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by PLL1 lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock source interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag\r Reset by software by writing LSIRDYC bit.\r Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag\r Reset by software by writing LSERDYC bit.\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyf",
                    description: Some(
                        "CSI ready interrupt flag\r Reset by software by writing CSIRDYC bit.\r Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag\r Reset by software by writing HSIRDYC bit.\r Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag\r Reset by software by writing HSERDYC bit.\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag\r Reset by software by writing HSI48RDYC bit.\r Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "PLL1 ready interrupt flag\r Reset by software by writing PLL1RDYC bit.\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag\r Reset by software by writing HSECSSC bit.\r Set by hardware in case of HSE clock failure.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "RCC clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable\r Set and cleared by software.\r Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source.\r This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag\r Set by hardware to indicate that the HSI oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsikeron",
                    description: Some(
                        "HSI clock enable in Stop mode\r Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsidiv",
                    description: Some(
                        "HSI clock divider\r Set and reset by software.\r These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The\r HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hsidiv",
                    ),
                },
                Field {
                    name: "hsidivf",
                    description: Some(
                        "HSI divider flag\r Set and reset by hardware.\r As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the\r current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csion",
                    description: Some(
                        "CSI clock enable\r Set and reset by software to enable/disable CSI clock for system and/or peripheral.\r Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdy",
                    description: Some(
                        "CSI clock ready flag\r Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csikeron",
                    description: Some(
                        "CSI clock enable in Stop mode\r Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable\r Set by software and cleared by software or by the hardware when the system enters to Stop\r or Standby mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag\r Set by hardware to indicate that the HSI48 oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE when entering Stop or Standby mode.\r This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the\r HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "HSE clock bypass\r Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device.\r The HSEBYP bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable\r Set by software to enable clock security system on HSE.\r This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseext",
                    description: Some(
                        "external high speed clock type in Bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hseext",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable PLL1.\r Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents\r writing this bit to 0, if the PLL1 output is used as the system clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crrcr",
            extends: None,
            description: Some(
                "RCC clock recovery RC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "Internal RC 48 MHz clock calibration\r Set by hardware by option-byte loading during system reset NRESET. Read-only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csicfgr",
            extends: None,
            description: Some(
                "RCC CSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csical",
                    description: Some(
                        "CSI clock calibration\r Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM.\r This field represents the sum of engineering option byte calibration value and CSITRIM bits value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csitrim",
                    description: Some(
                        "CSI clock trimming\r Set by software to adjust calibration.\r CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value.\r CSICAL = CSITRIM + FLASH_CSI_OPT.\r Note: The reset value of the field is 0x20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hsicfgr",
            extends: None,
            description: Some(
                "RCC HSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM.\r This field represents the sum of engineering option byte calibration value and HSITRIM bits value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming\r Set by software to adjust calibration.\r HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value.\r HSICAL = HSITRIM + FLASH_HSI_OPT.\r After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated\r Note: The reset value of the field is 0x40.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "RCC PLL clock source selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "DIVMx and PLLs clock source selection\r Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled.\r In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator.\r In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllvcosel",
                    description: Some(
                        "PLL1 VCO selection\r Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllvcosel",
                    ),
                },
                Field {
                    name: "divm",
                    description: Some(
                        "prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1.\r The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1).\r In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0.\r ...\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1_p_ck output of the PLL1.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1_q_ck output of the PLL1.\r In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLL1 DIVR divider output enable\r Set and reset by software to enable the pll1_r_ck output of the PLL1.\r To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Plldivr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r Others: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: Some(
                        "Plln",
                    ),
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1_p_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the pll1_q_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1_r_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plldiv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pllfracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is:\r * 128 to 560 MHz if PLL1VCOSEL = 0\r * \t150 to 420 MHz if PLL1VCOSEL = 1\r VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with\r * \tPLL1N between 8 and 420\r * \tPLL1FRACN can be between 0 and 213- 1\r * \tThe input frequency Fref1_ck must be between 1 and 16 MHz.\r To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r * \tSet the bit PLL1FRACEN to 0\r * \tWrite the new fractional value into PLL1FRACN\r * \tSet the bit PLL1FRACEN to 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsr",
            extends: None,
            description: Some(
                "RCC reset status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "remove reset flag\r Set and reset by software to reset the value of the reset flags.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "pin reset flag (NRST)\r Reset by software by writing the RMVF bit.\r Set by hardware when a reset from pin occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a BOR reset occurs (pwr_bor_rst).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "system reset from CPU reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdgrstf",
                    description: Some(
                        "independent watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when an independent watchdog reset occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "window watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a window watchdog reset occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag\r Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcdacsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Hclk2",
                    description: Some(
                        "rcc_hclk selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "hse_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pll2R",
                    description: Some(
                        "pll2_r_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "sys_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dacholdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DacHold",
                    description: Some(
                        "dac_hold_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DacHold2",
                    description: Some(
                        "dac_hold_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fdcansel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "hse_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pll2Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "sys_ck not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div128",
                    description: Some(
                        "sys_ck divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some(
                        "sys_ck divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "sys_ck divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Div256",
                    description: Some(
                        "sys_ck divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "sys_ck divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "Div512",
                    description: Some(
                        "sys_ck divided by 512",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "Div64",
                    description: Some(
                        "sys_ck divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "sys_ck divided by 8",
                    ),
                    value: 10,
                },
            ],
        },
        Enum {
            name: "Hseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Analog",
                    description: Some(
                        "HSE in analog mode (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Digital",
                    description: Some(
                        "HSE in digital mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsidiv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "Division by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2csel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll3R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I3c2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk3",
                    description: Some(
                        "rcc_pclk3 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll3R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pclk3",
                    description: Some(
                        "rcc_pclk3 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Per",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pll2P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lptim2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Per",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pll2P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pclk3",
                    description: Some(
                        "rcc_pclk3 selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll2Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lscosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "High",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Low",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MediumHigh",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MediumLow",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Analog",
                    description: Some(
                        "LSE in analog mode (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Digital",
                    description: Some(
                        "LSE in digital mode (do not use if RTC is active).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mco1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hsi48",
                    description: Some(
                        "HSI48 selected for micro-controller clock output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1_q selected for micro-controller clock output",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "CSI selected for micro-controller clock output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected for micro-controller clock output",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1_p selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pll2P",
                    description: Some(
                        "pll2_p selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "System clock selected for micro-controller clock output",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "Divide by 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Div10",
                    description: Some(
                        "Divide by 10",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "Div11",
                    description: Some(
                        "Divide by 11",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "Div12",
                    description: Some(
                        "Divide by 12",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "Div13",
                    description: Some(
                        "Divide by 13",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "Div14",
                    description: Some(
                        "Divide by 14",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "Div15",
                    description: Some(
                        "Divide by 15",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "Divide by 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Div3",
                    description: Some(
                        "Divide by 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "Divide by 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div5",
                    description: Some(
                        "Divide by 5",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Div6",
                    description: Some(
                        "Divide by 6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Div7",
                    description: Some(
                        "Divide by 7",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "Divide by 8",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Div9",
                    description: Some(
                        "Divide by 9",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Persel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "hse_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Plldiv",
            description: None,
            bit_size: 7,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Div10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "Div100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "Div101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "Div102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "Div103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "Div104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "Div105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "Div106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "Div107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "Div108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "Div109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "Div11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "Div110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "Div111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "Div112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "Div113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "Div114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "Div115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "Div116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "Div117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "Div118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "Div119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "Div12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "Div120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "Div121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "Div122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "Div123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "Div124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "Div125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "Div126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "Div127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "Div128",
                    description: None,
                    value: 127,
                },
                EnumVariant {
                    name: "Div13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "Div14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "Div15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "Div16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "Div17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "Div18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "Div19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "Div2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Div20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "Div21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "Div22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "Div23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "Div24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "Div25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "Div26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "Div27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "Div28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "Div29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "Div3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Div30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "Div31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "Div32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "Div33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "Div34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "Div35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "Div36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "Div37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "Div38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "Div39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "Div4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Div40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "Div41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "Div42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "Div43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "Div44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "Div45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "Div46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "Div47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "Div48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "Div49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "Div5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Div50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "Div51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "Div52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "Div53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "Div54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "Div55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "Div56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "Div57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "Div58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "Div59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "Div6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Div60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "Div61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "Div62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "Div63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "Div64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "Div65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "Div66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "Div67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "Div68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "Div69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "Div7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Div70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "Div71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "Div72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "Div73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "Div74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "Div75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "Div76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "Div77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "Div78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "Div79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "Div8",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "Div80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "Div81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "Div82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "Div83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "Div84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "Div85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "Div86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "Div87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "Div88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "Div89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "Div9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "Div90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "Div91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "Div92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "Div93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "Div94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "Div95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "Div96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "Div97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "Div98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "Div99",
                    description: None,
                    value: 98,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Div10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "Div11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "Div12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "Div13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "Div14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "Div15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "Div16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "Div17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "Div18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "Div19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "Div2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Div20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "Div21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "Div22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "Div23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "Div24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "Div25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "Div26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "Div27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "Div28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "Div29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "Div3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Div30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "Div31",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "Div32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "Div33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "Div34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "Div35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "Div36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "Div37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "Div38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "Div39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "Div4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Div40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "Div41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "Div42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "Div43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "Div44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "Div45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "Div46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "Div47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "Div48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "Div49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "Div5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Div50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "Div51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "Div52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "Div53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "Div54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "Div55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "Div56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "Div57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "Div58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "Div59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "Div6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Div60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "Div61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "Div62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "Div7",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "Div8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "Div9",
                    description: None,
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 9,
            variants: &[
                EnumVariant {
                    name: "Mul10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "Mul100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "Mul101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "Mul102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "Mul103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "Mul104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "Mul105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "Mul106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "Mul107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "Mul108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "Mul109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "Mul11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "Mul110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "Mul111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "Mul112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "Mul113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "Mul114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "Mul115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "Mul116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "Mul117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "Mul118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "Mul119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "Mul12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "Mul120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "Mul121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "Mul122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "Mul123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "Mul124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "Mul125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "Mul126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "Mul127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "Mul128",
                    description: None,
                    value: 127,
                },
                EnumVariant {
                    name: "Mul129",
                    description: None,
                    value: 128,
                },
                EnumVariant {
                    name: "Mul13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "Mul130",
                    description: None,
                    value: 129,
                },
                EnumVariant {
                    name: "Mul131",
                    description: None,
                    value: 130,
                },
                EnumVariant {
                    name: "Mul132",
                    description: None,
                    value: 131,
                },
                EnumVariant {
                    name: "Mul133",
                    description: None,
                    value: 132,
                },
                EnumVariant {
                    name: "Mul134",
                    description: None,
                    value: 133,
                },
                EnumVariant {
                    name: "Mul135",
                    description: None,
                    value: 134,
                },
                EnumVariant {
                    name: "Mul136",
                    description: None,
                    value: 135,
                },
                EnumVariant {
                    name: "Mul137",
                    description: None,
                    value: 136,
                },
                EnumVariant {
                    name: "Mul138",
                    description: None,
                    value: 137,
                },
                EnumVariant {
                    name: "Mul139",
                    description: None,
                    value: 138,
                },
                EnumVariant {
                    name: "Mul14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "Mul140",
                    description: None,
                    value: 139,
                },
                EnumVariant {
                    name: "Mul141",
                    description: None,
                    value: 140,
                },
                EnumVariant {
                    name: "Mul142",
                    description: None,
                    value: 141,
                },
                EnumVariant {
                    name: "Mul143",
                    description: None,
                    value: 142,
                },
                EnumVariant {
                    name: "Mul144",
                    description: None,
                    value: 143,
                },
                EnumVariant {
                    name: "Mul145",
                    description: None,
                    value: 144,
                },
                EnumVariant {
                    name: "Mul146",
                    description: None,
                    value: 145,
                },
                EnumVariant {
                    name: "Mul147",
                    description: None,
                    value: 146,
                },
                EnumVariant {
                    name: "Mul148",
                    description: None,
                    value: 147,
                },
                EnumVariant {
                    name: "Mul149",
                    description: None,
                    value: 148,
                },
                EnumVariant {
                    name: "Mul15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "Mul150",
                    description: None,
                    value: 149,
                },
                EnumVariant {
                    name: "Mul151",
                    description: None,
                    value: 150,
                },
                EnumVariant {
                    name: "Mul152",
                    description: None,
                    value: 151,
                },
                EnumVariant {
                    name: "Mul153",
                    description: None,
                    value: 152,
                },
                EnumVariant {
                    name: "Mul154",
                    description: None,
                    value: 153,
                },
                EnumVariant {
                    name: "Mul155",
                    description: None,
                    value: 154,
                },
                EnumVariant {
                    name: "Mul156",
                    description: None,
                    value: 155,
                },
                EnumVariant {
                    name: "Mul157",
                    description: None,
                    value: 156,
                },
                EnumVariant {
                    name: "Mul158",
                    description: None,
                    value: 157,
                },
                EnumVariant {
                    name: "Mul159",
                    description: None,
                    value: 158,
                },
                EnumVariant {
                    name: "Mul16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "Mul160",
                    description: None,
                    value: 159,
                },
                EnumVariant {
                    name: "Mul161",
                    description: None,
                    value: 160,
                },
                EnumVariant {
                    name: "Mul162",
                    description: None,
                    value: 161,
                },
                EnumVariant {
                    name: "Mul163",
                    description: None,
                    value: 162,
                },
                EnumVariant {
                    name: "Mul164",
                    description: None,
                    value: 163,
                },
                EnumVariant {
                    name: "Mul165",
                    description: None,
                    value: 164,
                },
                EnumVariant {
                    name: "Mul166",
                    description: None,
                    value: 165,
                },
                EnumVariant {
                    name: "Mul167",
                    description: None,
                    value: 166,
                },
                EnumVariant {
                    name: "Mul168",
                    description: None,
                    value: 167,
                },
                EnumVariant {
                    name: "Mul169",
                    description: None,
                    value: 168,
                },
                EnumVariant {
                    name: "Mul17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "Mul170",
                    description: None,
                    value: 169,
                },
                EnumVariant {
                    name: "Mul171",
                    description: None,
                    value: 170,
                },
                EnumVariant {
                    name: "Mul172",
                    description: None,
                    value: 171,
                },
                EnumVariant {
                    name: "Mul173",
                    description: None,
                    value: 172,
                },
                EnumVariant {
                    name: "Mul174",
                    description: None,
                    value: 173,
                },
                EnumVariant {
                    name: "Mul175",
                    description: None,
                    value: 174,
                },
                EnumVariant {
                    name: "Mul176",
                    description: None,
                    value: 175,
                },
                EnumVariant {
                    name: "Mul177",
                    description: None,
                    value: 176,
                },
                EnumVariant {
                    name: "Mul178",
                    description: None,
                    value: 177,
                },
                EnumVariant {
                    name: "Mul179",
                    description: None,
                    value: 178,
                },
                EnumVariant {
                    name: "Mul18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "Mul180",
                    description: None,
                    value: 179,
                },
                EnumVariant {
                    name: "Mul181",
                    description: None,
                    value: 180,
                },
                EnumVariant {
                    name: "Mul182",
                    description: None,
                    value: 181,
                },
                EnumVariant {
                    name: "Mul183",
                    description: None,
                    value: 182,
                },
                EnumVariant {
                    name: "Mul184",
                    description: None,
                    value: 183,
                },
                EnumVariant {
                    name: "Mul185",
                    description: None,
                    value: 184,
                },
                EnumVariant {
                    name: "Mul186",
                    description: None,
                    value: 185,
                },
                EnumVariant {
                    name: "Mul187",
                    description: None,
                    value: 186,
                },
                EnumVariant {
                    name: "Mul188",
                    description: None,
                    value: 187,
                },
                EnumVariant {
                    name: "Mul189",
                    description: None,
                    value: 188,
                },
                EnumVariant {
                    name: "Mul19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "Mul190",
                    description: None,
                    value: 189,
                },
                EnumVariant {
                    name: "Mul191",
                    description: None,
                    value: 190,
                },
                EnumVariant {
                    name: "Mul192",
                    description: None,
                    value: 191,
                },
                EnumVariant {
                    name: "Mul193",
                    description: None,
                    value: 192,
                },
                EnumVariant {
                    name: "Mul194",
                    description: None,
                    value: 193,
                },
                EnumVariant {
                    name: "Mul195",
                    description: None,
                    value: 194,
                },
                EnumVariant {
                    name: "Mul196",
                    description: None,
                    value: 195,
                },
                EnumVariant {
                    name: "Mul197",
                    description: None,
                    value: 196,
                },
                EnumVariant {
                    name: "Mul198",
                    description: None,
                    value: 197,
                },
                EnumVariant {
                    name: "Mul199",
                    description: None,
                    value: 198,
                },
                EnumVariant {
                    name: "Mul20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "Mul200",
                    description: None,
                    value: 199,
                },
                EnumVariant {
                    name: "Mul201",
                    description: None,
                    value: 200,
                },
                EnumVariant {
                    name: "Mul202",
                    description: None,
                    value: 201,
                },
                EnumVariant {
                    name: "Mul203",
                    description: None,
                    value: 202,
                },
                EnumVariant {
                    name: "Mul204",
                    description: None,
                    value: 203,
                },
                EnumVariant {
                    name: "Mul205",
                    description: None,
                    value: 204,
                },
                EnumVariant {
                    name: "Mul206",
                    description: None,
                    value: 205,
                },
                EnumVariant {
                    name: "Mul207",
                    description: None,
                    value: 206,
                },
                EnumVariant {
                    name: "Mul208",
                    description: None,
                    value: 207,
                },
                EnumVariant {
                    name: "Mul209",
                    description: None,
                    value: 208,
                },
                EnumVariant {
                    name: "Mul21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "Mul210",
                    description: None,
                    value: 209,
                },
                EnumVariant {
                    name: "Mul211",
                    description: None,
                    value: 210,
                },
                EnumVariant {
                    name: "Mul212",
                    description: None,
                    value: 211,
                },
                EnumVariant {
                    name: "Mul213",
                    description: None,
                    value: 212,
                },
                EnumVariant {
                    name: "Mul214",
                    description: None,
                    value: 213,
                },
                EnumVariant {
                    name: "Mul215",
                    description: None,
                    value: 214,
                },
                EnumVariant {
                    name: "Mul216",
                    description: None,
                    value: 215,
                },
                EnumVariant {
                    name: "Mul217",
                    description: None,
                    value: 216,
                },
                EnumVariant {
                    name: "Mul218",
                    description: None,
                    value: 217,
                },
                EnumVariant {
                    name: "Mul219",
                    description: None,
                    value: 218,
                },
                EnumVariant {
                    name: "Mul22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "Mul220",
                    description: None,
                    value: 219,
                },
                EnumVariant {
                    name: "Mul221",
                    description: None,
                    value: 220,
                },
                EnumVariant {
                    name: "Mul222",
                    description: None,
                    value: 221,
                },
                EnumVariant {
                    name: "Mul223",
                    description: None,
                    value: 222,
                },
                EnumVariant {
                    name: "Mul224",
                    description: None,
                    value: 223,
                },
                EnumVariant {
                    name: "Mul225",
                    description: None,
                    value: 224,
                },
                EnumVariant {
                    name: "Mul226",
                    description: None,
                    value: 225,
                },
                EnumVariant {
                    name: "Mul227",
                    description: None,
                    value: 226,
                },
                EnumVariant {
                    name: "Mul228",
                    description: None,
                    value: 227,
                },
                EnumVariant {
                    name: "Mul229",
                    description: None,
                    value: 228,
                },
                EnumVariant {
                    name: "Mul23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "Mul230",
                    description: None,
                    value: 229,
                },
                EnumVariant {
                    name: "Mul231",
                    description: None,
                    value: 230,
                },
                EnumVariant {
                    name: "Mul232",
                    description: None,
                    value: 231,
                },
                EnumVariant {
                    name: "Mul233",
                    description: None,
                    value: 232,
                },
                EnumVariant {
                    name: "Mul234",
                    description: None,
                    value: 233,
                },
                EnumVariant {
                    name: "Mul235",
                    description: None,
                    value: 234,
                },
                EnumVariant {
                    name: "Mul236",
                    description: None,
                    value: 235,
                },
                EnumVariant {
                    name: "Mul237",
                    description: None,
                    value: 236,
                },
                EnumVariant {
                    name: "Mul238",
                    description: None,
                    value: 237,
                },
                EnumVariant {
                    name: "Mul239",
                    description: None,
                    value: 238,
                },
                EnumVariant {
                    name: "Mul24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "Mul240",
                    description: None,
                    value: 239,
                },
                EnumVariant {
                    name: "Mul241",
                    description: None,
                    value: 240,
                },
                EnumVariant {
                    name: "Mul242",
                    description: None,
                    value: 241,
                },
                EnumVariant {
                    name: "Mul243",
                    description: None,
                    value: 242,
                },
                EnumVariant {
                    name: "Mul244",
                    description: None,
                    value: 243,
                },
                EnumVariant {
                    name: "Mul245",
                    description: None,
                    value: 244,
                },
                EnumVariant {
                    name: "Mul246",
                    description: None,
                    value: 245,
                },
                EnumVariant {
                    name: "Mul247",
                    description: None,
                    value: 246,
                },
                EnumVariant {
                    name: "Mul248",
                    description: None,
                    value: 247,
                },
                EnumVariant {
                    name: "Mul249",
                    description: None,
                    value: 248,
                },
                EnumVariant {
                    name: "Mul25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "Mul250",
                    description: None,
                    value: 249,
                },
                EnumVariant {
                    name: "Mul251",
                    description: None,
                    value: 250,
                },
                EnumVariant {
                    name: "Mul252",
                    description: None,
                    value: 251,
                },
                EnumVariant {
                    name: "Mul253",
                    description: None,
                    value: 252,
                },
                EnumVariant {
                    name: "Mul254",
                    description: None,
                    value: 253,
                },
                EnumVariant {
                    name: "Mul255",
                    description: None,
                    value: 254,
                },
                EnumVariant {
                    name: "Mul256",
                    description: None,
                    value: 255,
                },
                EnumVariant {
                    name: "Mul257",
                    description: None,
                    value: 256,
                },
                EnumVariant {
                    name: "Mul258",
                    description: None,
                    value: 257,
                },
                EnumVariant {
                    name: "Mul259",
                    description: None,
                    value: 258,
                },
                EnumVariant {
                    name: "Mul26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "Mul260",
                    description: None,
                    value: 259,
                },
                EnumVariant {
                    name: "Mul261",
                    description: None,
                    value: 260,
                },
                EnumVariant {
                    name: "Mul262",
                    description: None,
                    value: 261,
                },
                EnumVariant {
                    name: "Mul263",
                    description: None,
                    value: 262,
                },
                EnumVariant {
                    name: "Mul264",
                    description: None,
                    value: 263,
                },
                EnumVariant {
                    name: "Mul265",
                    description: None,
                    value: 264,
                },
                EnumVariant {
                    name: "Mul266",
                    description: None,
                    value: 265,
                },
                EnumVariant {
                    name: "Mul267",
                    description: None,
                    value: 266,
                },
                EnumVariant {
                    name: "Mul268",
                    description: None,
                    value: 267,
                },
                EnumVariant {
                    name: "Mul269",
                    description: None,
                    value: 268,
                },
                EnumVariant {
                    name: "Mul27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "Mul270",
                    description: None,
                    value: 269,
                },
                EnumVariant {
                    name: "Mul271",
                    description: None,
                    value: 270,
                },
                EnumVariant {
                    name: "Mul272",
                    description: None,
                    value: 271,
                },
                EnumVariant {
                    name: "Mul273",
                    description: None,
                    value: 272,
                },
                EnumVariant {
                    name: "Mul274",
                    description: None,
                    value: 273,
                },
                EnumVariant {
                    name: "Mul275",
                    description: None,
                    value: 274,
                },
                EnumVariant {
                    name: "Mul276",
                    description: None,
                    value: 275,
                },
                EnumVariant {
                    name: "Mul277",
                    description: None,
                    value: 276,
                },
                EnumVariant {
                    name: "Mul278",
                    description: None,
                    value: 277,
                },
                EnumVariant {
                    name: "Mul279",
                    description: None,
                    value: 278,
                },
                EnumVariant {
                    name: "Mul28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "Mul280",
                    description: None,
                    value: 279,
                },
                EnumVariant {
                    name: "Mul281",
                    description: None,
                    value: 280,
                },
                EnumVariant {
                    name: "Mul282",
                    description: None,
                    value: 281,
                },
                EnumVariant {
                    name: "Mul283",
                    description: None,
                    value: 282,
                },
                EnumVariant {
                    name: "Mul284",
                    description: None,
                    value: 283,
                },
                EnumVariant {
                    name: "Mul285",
                    description: None,
                    value: 284,
                },
                EnumVariant {
                    name: "Mul286",
                    description: None,
                    value: 285,
                },
                EnumVariant {
                    name: "Mul287",
                    description: None,
                    value: 286,
                },
                EnumVariant {
                    name: "Mul288",
                    description: None,
                    value: 287,
                },
                EnumVariant {
                    name: "Mul289",
                    description: None,
                    value: 288,
                },
                EnumVariant {
                    name: "Mul29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "Mul290",
                    description: None,
                    value: 289,
                },
                EnumVariant {
                    name: "Mul291",
                    description: None,
                    value: 290,
                },
                EnumVariant {
                    name: "Mul292",
                    description: None,
                    value: 291,
                },
                EnumVariant {
                    name: "Mul293",
                    description: None,
                    value: 292,
                },
                EnumVariant {
                    name: "Mul294",
                    description: None,
                    value: 293,
                },
                EnumVariant {
                    name: "Mul295",
                    description: None,
                    value: 294,
                },
                EnumVariant {
                    name: "Mul296",
                    description: None,
                    value: 295,
                },
                EnumVariant {
                    name: "Mul297",
                    description: None,
                    value: 296,
                },
                EnumVariant {
                    name: "Mul298",
                    description: None,
                    value: 297,
                },
                EnumVariant {
                    name: "Mul299",
                    description: None,
                    value: 298,
                },
                EnumVariant {
                    name: "Mul30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "Mul300",
                    description: None,
                    value: 299,
                },
                EnumVariant {
                    name: "Mul301",
                    description: None,
                    value: 300,
                },
                EnumVariant {
                    name: "Mul302",
                    description: None,
                    value: 301,
                },
                EnumVariant {
                    name: "Mul303",
                    description: None,
                    value: 302,
                },
                EnumVariant {
                    name: "Mul304",
                    description: None,
                    value: 303,
                },
                EnumVariant {
                    name: "Mul305",
                    description: None,
                    value: 304,
                },
                EnumVariant {
                    name: "Mul306",
                    description: None,
                    value: 305,
                },
                EnumVariant {
                    name: "Mul307",
                    description: None,
                    value: 306,
                },
                EnumVariant {
                    name: "Mul308",
                    description: None,
                    value: 307,
                },
                EnumVariant {
                    name: "Mul309",
                    description: None,
                    value: 308,
                },
                EnumVariant {
                    name: "Mul31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "Mul310",
                    description: None,
                    value: 309,
                },
                EnumVariant {
                    name: "Mul311",
                    description: None,
                    value: 310,
                },
                EnumVariant {
                    name: "Mul312",
                    description: None,
                    value: 311,
                },
                EnumVariant {
                    name: "Mul313",
                    description: None,
                    value: 312,
                },
                EnumVariant {
                    name: "Mul314",
                    description: None,
                    value: 313,
                },
                EnumVariant {
                    name: "Mul315",
                    description: None,
                    value: 314,
                },
                EnumVariant {
                    name: "Mul316",
                    description: None,
                    value: 315,
                },
                EnumVariant {
                    name: "Mul317",
                    description: None,
                    value: 316,
                },
                EnumVariant {
                    name: "Mul318",
                    description: None,
                    value: 317,
                },
                EnumVariant {
                    name: "Mul319",
                    description: None,
                    value: 318,
                },
                EnumVariant {
                    name: "Mul32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "Mul320",
                    description: None,
                    value: 319,
                },
                EnumVariant {
                    name: "Mul321",
                    description: None,
                    value: 320,
                },
                EnumVariant {
                    name: "Mul322",
                    description: None,
                    value: 321,
                },
                EnumVariant {
                    name: "Mul323",
                    description: None,
                    value: 322,
                },
                EnumVariant {
                    name: "Mul324",
                    description: None,
                    value: 323,
                },
                EnumVariant {
                    name: "Mul325",
                    description: None,
                    value: 324,
                },
                EnumVariant {
                    name: "Mul326",
                    description: None,
                    value: 325,
                },
                EnumVariant {
                    name: "Mul327",
                    description: None,
                    value: 326,
                },
                EnumVariant {
                    name: "Mul328",
                    description: None,
                    value: 327,
                },
                EnumVariant {
                    name: "Mul329",
                    description: None,
                    value: 328,
                },
                EnumVariant {
                    name: "Mul33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "Mul330",
                    description: None,
                    value: 329,
                },
                EnumVariant {
                    name: "Mul331",
                    description: None,
                    value: 330,
                },
                EnumVariant {
                    name: "Mul332",
                    description: None,
                    value: 331,
                },
                EnumVariant {
                    name: "Mul333",
                    description: None,
                    value: 332,
                },
                EnumVariant {
                    name: "Mul334",
                    description: None,
                    value: 333,
                },
                EnumVariant {
                    name: "Mul335",
                    description: None,
                    value: 334,
                },
                EnumVariant {
                    name: "Mul336",
                    description: None,
                    value: 335,
                },
                EnumVariant {
                    name: "Mul337",
                    description: None,
                    value: 336,
                },
                EnumVariant {
                    name: "Mul338",
                    description: None,
                    value: 337,
                },
                EnumVariant {
                    name: "Mul339",
                    description: None,
                    value: 338,
                },
                EnumVariant {
                    name: "Mul34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "Mul340",
                    description: None,
                    value: 339,
                },
                EnumVariant {
                    name: "Mul341",
                    description: None,
                    value: 340,
                },
                EnumVariant {
                    name: "Mul342",
                    description: None,
                    value: 341,
                },
                EnumVariant {
                    name: "Mul343",
                    description: None,
                    value: 342,
                },
                EnumVariant {
                    name: "Mul344",
                    description: None,
                    value: 343,
                },
                EnumVariant {
                    name: "Mul345",
                    description: None,
                    value: 344,
                },
                EnumVariant {
                    name: "Mul346",
                    description: None,
                    value: 345,
                },
                EnumVariant {
                    name: "Mul347",
                    description: None,
                    value: 346,
                },
                EnumVariant {
                    name: "Mul348",
                    description: None,
                    value: 347,
                },
                EnumVariant {
                    name: "Mul349",
                    description: None,
                    value: 348,
                },
                EnumVariant {
                    name: "Mul35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "Mul350",
                    description: None,
                    value: 349,
                },
                EnumVariant {
                    name: "Mul351",
                    description: None,
                    value: 350,
                },
                EnumVariant {
                    name: "Mul352",
                    description: None,
                    value: 351,
                },
                EnumVariant {
                    name: "Mul353",
                    description: None,
                    value: 352,
                },
                EnumVariant {
                    name: "Mul354",
                    description: None,
                    value: 353,
                },
                EnumVariant {
                    name: "Mul355",
                    description: None,
                    value: 354,
                },
                EnumVariant {
                    name: "Mul356",
                    description: None,
                    value: 355,
                },
                EnumVariant {
                    name: "Mul357",
                    description: None,
                    value: 356,
                },
                EnumVariant {
                    name: "Mul358",
                    description: None,
                    value: 357,
                },
                EnumVariant {
                    name: "Mul359",
                    description: None,
                    value: 358,
                },
                EnumVariant {
                    name: "Mul36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "Mul360",
                    description: None,
                    value: 359,
                },
                EnumVariant {
                    name: "Mul361",
                    description: None,
                    value: 360,
                },
                EnumVariant {
                    name: "Mul362",
                    description: None,
                    value: 361,
                },
                EnumVariant {
                    name: "Mul363",
                    description: None,
                    value: 362,
                },
                EnumVariant {
                    name: "Mul364",
                    description: None,
                    value: 363,
                },
                EnumVariant {
                    name: "Mul365",
                    description: None,
                    value: 364,
                },
                EnumVariant {
                    name: "Mul366",
                    description: None,
                    value: 365,
                },
                EnumVariant {
                    name: "Mul367",
                    description: None,
                    value: 366,
                },
                EnumVariant {
                    name: "Mul368",
                    description: None,
                    value: 367,
                },
                EnumVariant {
                    name: "Mul369",
                    description: None,
                    value: 368,
                },
                EnumVariant {
                    name: "Mul37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "Mul370",
                    description: None,
                    value: 369,
                },
                EnumVariant {
                    name: "Mul371",
                    description: None,
                    value: 370,
                },
                EnumVariant {
                    name: "Mul372",
                    description: None,
                    value: 371,
                },
                EnumVariant {
                    name: "Mul373",
                    description: None,
                    value: 372,
                },
                EnumVariant {
                    name: "Mul374",
                    description: None,
                    value: 373,
                },
                EnumVariant {
                    name: "Mul375",
                    description: None,
                    value: 374,
                },
                EnumVariant {
                    name: "Mul376",
                    description: None,
                    value: 375,
                },
                EnumVariant {
                    name: "Mul377",
                    description: None,
                    value: 376,
                },
                EnumVariant {
                    name: "Mul378",
                    description: None,
                    value: 377,
                },
                EnumVariant {
                    name: "Mul379",
                    description: None,
                    value: 378,
                },
                EnumVariant {
                    name: "Mul38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "Mul380",
                    description: None,
                    value: 379,
                },
                EnumVariant {
                    name: "Mul381",
                    description: None,
                    value: 380,
                },
                EnumVariant {
                    name: "Mul382",
                    description: None,
                    value: 381,
                },
                EnumVariant {
                    name: "Mul383",
                    description: None,
                    value: 382,
                },
                EnumVariant {
                    name: "Mul384",
                    description: None,
                    value: 383,
                },
                EnumVariant {
                    name: "Mul385",
                    description: None,
                    value: 384,
                },
                EnumVariant {
                    name: "Mul386",
                    description: None,
                    value: 385,
                },
                EnumVariant {
                    name: "Mul387",
                    description: None,
                    value: 386,
                },
                EnumVariant {
                    name: "Mul388",
                    description: None,
                    value: 387,
                },
                EnumVariant {
                    name: "Mul389",
                    description: None,
                    value: 388,
                },
                EnumVariant {
                    name: "Mul39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "Mul390",
                    description: None,
                    value: 389,
                },
                EnumVariant {
                    name: "Mul391",
                    description: None,
                    value: 390,
                },
                EnumVariant {
                    name: "Mul392",
                    description: None,
                    value: 391,
                },
                EnumVariant {
                    name: "Mul393",
                    description: None,
                    value: 392,
                },
                EnumVariant {
                    name: "Mul394",
                    description: None,
                    value: 393,
                },
                EnumVariant {
                    name: "Mul395",
                    description: None,
                    value: 394,
                },
                EnumVariant {
                    name: "Mul396",
                    description: None,
                    value: 395,
                },
                EnumVariant {
                    name: "Mul397",
                    description: None,
                    value: 396,
                },
                EnumVariant {
                    name: "Mul398",
                    description: None,
                    value: 397,
                },
                EnumVariant {
                    name: "Mul399",
                    description: None,
                    value: 398,
                },
                EnumVariant {
                    name: "Mul4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Mul40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "Mul400",
                    description: None,
                    value: 399,
                },
                EnumVariant {
                    name: "Mul401",
                    description: None,
                    value: 400,
                },
                EnumVariant {
                    name: "Mul402",
                    description: None,
                    value: 401,
                },
                EnumVariant {
                    name: "Mul403",
                    description: None,
                    value: 402,
                },
                EnumVariant {
                    name: "Mul404",
                    description: None,
                    value: 403,
                },
                EnumVariant {
                    name: "Mul405",
                    description: None,
                    value: 404,
                },
                EnumVariant {
                    name: "Mul406",
                    description: None,
                    value: 405,
                },
                EnumVariant {
                    name: "Mul407",
                    description: None,
                    value: 406,
                },
                EnumVariant {
                    name: "Mul408",
                    description: None,
                    value: 407,
                },
                EnumVariant {
                    name: "Mul409",
                    description: None,
                    value: 408,
                },
                EnumVariant {
                    name: "Mul41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "Mul410",
                    description: None,
                    value: 409,
                },
                EnumVariant {
                    name: "Mul411",
                    description: None,
                    value: 410,
                },
                EnumVariant {
                    name: "Mul412",
                    description: None,
                    value: 411,
                },
                EnumVariant {
                    name: "Mul413",
                    description: None,
                    value: 412,
                },
                EnumVariant {
                    name: "Mul414",
                    description: None,
                    value: 413,
                },
                EnumVariant {
                    name: "Mul415",
                    description: None,
                    value: 414,
                },
                EnumVariant {
                    name: "Mul416",
                    description: None,
                    value: 415,
                },
                EnumVariant {
                    name: "Mul417",
                    description: None,
                    value: 416,
                },
                EnumVariant {
                    name: "Mul418",
                    description: None,
                    value: 417,
                },
                EnumVariant {
                    name: "Mul419",
                    description: None,
                    value: 418,
                },
                EnumVariant {
                    name: "Mul42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "Mul420",
                    description: None,
                    value: 419,
                },
                EnumVariant {
                    name: "Mul421",
                    description: None,
                    value: 420,
                },
                EnumVariant {
                    name: "Mul422",
                    description: None,
                    value: 421,
                },
                EnumVariant {
                    name: "Mul423",
                    description: None,
                    value: 422,
                },
                EnumVariant {
                    name: "Mul424",
                    description: None,
                    value: 423,
                },
                EnumVariant {
                    name: "Mul425",
                    description: None,
                    value: 424,
                },
                EnumVariant {
                    name: "Mul426",
                    description: None,
                    value: 425,
                },
                EnumVariant {
                    name: "Mul427",
                    description: None,
                    value: 426,
                },
                EnumVariant {
                    name: "Mul428",
                    description: None,
                    value: 427,
                },
                EnumVariant {
                    name: "Mul429",
                    description: None,
                    value: 428,
                },
                EnumVariant {
                    name: "Mul43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "Mul430",
                    description: None,
                    value: 429,
                },
                EnumVariant {
                    name: "Mul431",
                    description: None,
                    value: 430,
                },
                EnumVariant {
                    name: "Mul432",
                    description: None,
                    value: 431,
                },
                EnumVariant {
                    name: "Mul433",
                    description: None,
                    value: 432,
                },
                EnumVariant {
                    name: "Mul434",
                    description: None,
                    value: 433,
                },
                EnumVariant {
                    name: "Mul435",
                    description: None,
                    value: 434,
                },
                EnumVariant {
                    name: "Mul436",
                    description: None,
                    value: 435,
                },
                EnumVariant {
                    name: "Mul437",
                    description: None,
                    value: 436,
                },
                EnumVariant {
                    name: "Mul438",
                    description: None,
                    value: 437,
                },
                EnumVariant {
                    name: "Mul439",
                    description: None,
                    value: 438,
                },
                EnumVariant {
                    name: "Mul44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "Mul440",
                    description: None,
                    value: 439,
                },
                EnumVariant {
                    name: "Mul441",
                    description: None,
                    value: 440,
                },
                EnumVariant {
                    name: "Mul442",
                    description: None,
                    value: 441,
                },
                EnumVariant {
                    name: "Mul443",
                    description: None,
                    value: 442,
                },
                EnumVariant {
                    name: "Mul444",
                    description: None,
                    value: 443,
                },
                EnumVariant {
                    name: "Mul445",
                    description: None,
                    value: 444,
                },
                EnumVariant {
                    name: "Mul446",
                    description: None,
                    value: 445,
                },
                EnumVariant {
                    name: "Mul447",
                    description: None,
                    value: 446,
                },
                EnumVariant {
                    name: "Mul448",
                    description: None,
                    value: 447,
                },
                EnumVariant {
                    name: "Mul449",
                    description: None,
                    value: 448,
                },
                EnumVariant {
                    name: "Mul45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "Mul450",
                    description: None,
                    value: 449,
                },
                EnumVariant {
                    name: "Mul451",
                    description: None,
                    value: 450,
                },
                EnumVariant {
                    name: "Mul452",
                    description: None,
                    value: 451,
                },
                EnumVariant {
                    name: "Mul453",
                    description: None,
                    value: 452,
                },
                EnumVariant {
                    name: "Mul454",
                    description: None,
                    value: 453,
                },
                EnumVariant {
                    name: "Mul455",
                    description: None,
                    value: 454,
                },
                EnumVariant {
                    name: "Mul456",
                    description: None,
                    value: 455,
                },
                EnumVariant {
                    name: "Mul457",
                    description: None,
                    value: 456,
                },
                EnumVariant {
                    name: "Mul458",
                    description: None,
                    value: 457,
                },
                EnumVariant {
                    name: "Mul459",
                    description: None,
                    value: 458,
                },
                EnumVariant {
                    name: "Mul46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "Mul460",
                    description: None,
                    value: 459,
                },
                EnumVariant {
                    name: "Mul461",
                    description: None,
                    value: 460,
                },
                EnumVariant {
                    name: "Mul462",
                    description: None,
                    value: 461,
                },
                EnumVariant {
                    name: "Mul463",
                    description: None,
                    value: 462,
                },
                EnumVariant {
                    name: "Mul464",
                    description: None,
                    value: 463,
                },
                EnumVariant {
                    name: "Mul465",
                    description: None,
                    value: 464,
                },
                EnumVariant {
                    name: "Mul466",
                    description: None,
                    value: 465,
                },
                EnumVariant {
                    name: "Mul467",
                    description: None,
                    value: 466,
                },
                EnumVariant {
                    name: "Mul468",
                    description: None,
                    value: 467,
                },
                EnumVariant {
                    name: "Mul469",
                    description: None,
                    value: 468,
                },
                EnumVariant {
                    name: "Mul47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "Mul470",
                    description: None,
                    value: 469,
                },
                EnumVariant {
                    name: "Mul471",
                    description: None,
                    value: 470,
                },
                EnumVariant {
                    name: "Mul472",
                    description: None,
                    value: 471,
                },
                EnumVariant {
                    name: "Mul473",
                    description: None,
                    value: 472,
                },
                EnumVariant {
                    name: "Mul474",
                    description: None,
                    value: 473,
                },
                EnumVariant {
                    name: "Mul475",
                    description: None,
                    value: 474,
                },
                EnumVariant {
                    name: "Mul476",
                    description: None,
                    value: 475,
                },
                EnumVariant {
                    name: "Mul477",
                    description: None,
                    value: 476,
                },
                EnumVariant {
                    name: "Mul478",
                    description: None,
                    value: 477,
                },
                EnumVariant {
                    name: "Mul479",
                    description: None,
                    value: 478,
                },
                EnumVariant {
                    name: "Mul48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "Mul480",
                    description: None,
                    value: 479,
                },
                EnumVariant {
                    name: "Mul481",
                    description: None,
                    value: 480,
                },
                EnumVariant {
                    name: "Mul482",
                    description: None,
                    value: 481,
                },
                EnumVariant {
                    name: "Mul483",
                    description: None,
                    value: 482,
                },
                EnumVariant {
                    name: "Mul484",
                    description: None,
                    value: 483,
                },
                EnumVariant {
                    name: "Mul485",
                    description: None,
                    value: 484,
                },
                EnumVariant {
                    name: "Mul486",
                    description: None,
                    value: 485,
                },
                EnumVariant {
                    name: "Mul487",
                    description: None,
                    value: 486,
                },
                EnumVariant {
                    name: "Mul488",
                    description: None,
                    value: 487,
                },
                EnumVariant {
                    name: "Mul489",
                    description: None,
                    value: 488,
                },
                EnumVariant {
                    name: "Mul49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "Mul490",
                    description: None,
                    value: 489,
                },
                EnumVariant {
                    name: "Mul491",
                    description: None,
                    value: 490,
                },
                EnumVariant {
                    name: "Mul492",
                    description: None,
                    value: 491,
                },
                EnumVariant {
                    name: "Mul493",
                    description: None,
                    value: 492,
                },
                EnumVariant {
                    name: "Mul494",
                    description: None,
                    value: 493,
                },
                EnumVariant {
                    name: "Mul495",
                    description: None,
                    value: 494,
                },
                EnumVariant {
                    name: "Mul496",
                    description: None,
                    value: 495,
                },
                EnumVariant {
                    name: "Mul497",
                    description: None,
                    value: 496,
                },
                EnumVariant {
                    name: "Mul498",
                    description: None,
                    value: 497,
                },
                EnumVariant {
                    name: "Mul499",
                    description: None,
                    value: 498,
                },
                EnumVariant {
                    name: "Mul5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Mul50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "Mul500",
                    description: None,
                    value: 499,
                },
                EnumVariant {
                    name: "Mul501",
                    description: None,
                    value: 500,
                },
                EnumVariant {
                    name: "Mul502",
                    description: None,
                    value: 501,
                },
                EnumVariant {
                    name: "Mul503",
                    description: None,
                    value: 502,
                },
                EnumVariant {
                    name: "Mul504",
                    description: None,
                    value: 503,
                },
                EnumVariant {
                    name: "Mul505",
                    description: None,
                    value: 504,
                },
                EnumVariant {
                    name: "Mul506",
                    description: None,
                    value: 505,
                },
                EnumVariant {
                    name: "Mul507",
                    description: None,
                    value: 506,
                },
                EnumVariant {
                    name: "Mul508",
                    description: None,
                    value: 507,
                },
                EnumVariant {
                    name: "Mul509",
                    description: None,
                    value: 508,
                },
                EnumVariant {
                    name: "Mul51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "Mul510",
                    description: None,
                    value: 509,
                },
                EnumVariant {
                    name: "Mul511",
                    description: None,
                    value: 510,
                },
                EnumVariant {
                    name: "Mul512",
                    description: None,
                    value: 511,
                },
                EnumVariant {
                    name: "Mul52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "Mul53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "Mul54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "Mul55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "Mul56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "Mul57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "Mul58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "Mul59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "Mul6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Mul60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "Mul61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "Mul62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "Mul63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "Mul64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "Mul65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "Mul66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "Mul67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "Mul68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "Mul69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "Mul7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Mul70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "Mul71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "Mul72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "Mul73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "Mul74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "Mul75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "Mul76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "Mul77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "Mul78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "Mul79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "Mul8",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "Mul80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "Mul81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "Mul82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "Mul83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "Mul84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "Mul85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "Mul86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "Mul87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "Mul88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "Mul89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "Mul9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "Mul90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "Mul91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "Mul92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "Mul93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "Mul94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "Mul95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "Mul96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "Mul97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "Mul98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "Mul99",
                    description: None,
                    value: 98,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Range1",
                    description: Some(
                        "Frequency is between 1 and 2 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Range2",
                    description: Some(
                        "Frequency is between 2 and 4 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Range4",
                    description: Some(
                        "Frequency is between 4 and 8 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Range8",
                    description: Some(
                        "Frequency is between 8 and 16 MHz",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "CSI selected as PLL clock (csi_ck)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Disable",
                    description: Some(
                        "no clock send to DIVMx divider and PLLs (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE selected as PLL clock (hse_ck)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected as PLL clock (hsi_ck)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllvcosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MediumVco",
                    description: Some(
                        "VCO frequency range 150 to 420 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "WideVco",
                    description: Some(
                        "VCO frequency range 192 to 836 MHz",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 16",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 8",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi48",
                    description: Some(
                        "hsi48_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "lsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Disable",
                    description: Some(
                        "no clock (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HseDivRtcpre",
                    description: Some(
                        "HSE divided by RTCPRE value selected as RTC clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected as RTC clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Spisel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Audioclk",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Per",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll2P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stopkerwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected as wakeup clock from system Stop (default after reset)",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "CSI selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "PLL1 selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Systicksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hclk1Div8",
                    description: Some(
                        "rcc_hclk/8 selected as clock source (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "lse_ck[1] selected as clock source",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "lsi_ker_ck[1] selected as clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Timicsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "No internal clock available for timers input capture (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some(
                        "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Timpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DefaultX2",
                    description: Some(
                        "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DefaultX4",
                    description: Some(
                        "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pclk2",
                    description: Some(
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll2Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Csi",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll2Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usbsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Disable",
                    description: Some(
                        "Disable the kernel clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hsi48",
                    description: Some(
                        "HSI48 selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
