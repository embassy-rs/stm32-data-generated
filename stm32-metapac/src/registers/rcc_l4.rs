
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Clock control register",
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
                    name: "icscr",
                    description: Some(
                        "Internal clock sources calibration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Clock configuration register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "pllcfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "pllsai1cfgr",
                    description: Some(
                        "PLLSAI1 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllsaicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllsai2cfgr",
                    description: Some(
                        "PLLSAI2 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllsaicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "Clock interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                        "Clock interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "Clock interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                        "AHB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                        "AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    name: "ahb3rstr",
                    description: Some(
                        "AHB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr1",
                    description: Some(
                        "APB1 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr2",
                    description: Some(
                        "APB1 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                    name: "ahb1enr",
                    description: Some(
                        "AHB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                        "AHB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                    name: "ahb3enr",
                    description: Some(
                        "AHB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr1",
                    description: Some(
                        "APB1ENR1",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr2",
                    description: Some(
                        "APB1 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2ENR",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                    name: "ahb1smenr",
                    description: Some(
                        "AHB1 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2smenr",
                    description: Some(
                        "AHB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3smenr",
                    description: Some(
                        "AHB3 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr1",
                    description: Some(
                        "APB1SMENR1",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr2",
                    description: Some(
                        "APB1 peripheral clocks enable in Sleep and Stop modes register 2",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2smenr",
                    description: Some(
                        "APB2SMENR",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr",
                    description: Some(
                        "CCIPR",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "BDCR",
                    ),
                    array: None,
                    byte_offset: 0x90,
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
                    name: "csr",
                    description: Some(
                        "CSR",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crrcr",
                    description: Some(
                        "Clock recovery RC register",
                    ),
                    array: None,
                    byte_offset: 0x98,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA1 clock enable",
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
                    name: "dma2en",
                    description: Some(
                        "DMA2 clock enable",
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
                    name: "flashen",
                    description: Some(
                        "Flash memory interface clock enable",
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
                        "CRC clock enable",
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
                    name: "tscen",
                    description: Some(
                        "Touch Sensing Controller clock enable",
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
                    name: "dma2den",
                    description: Some(
                        "DMA2D clock enable",
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
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA1 reset",
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
                    name: "dma2rst",
                    description: Some(
                        "DMA2 reset",
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
                    name: "flashrst",
                    description: Some(
                        "Flash memory interface reset",
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
                    name: "crcrst",
                    description: Some(
                        "CRC reset",
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
                    name: "tscrst",
                    description: Some(
                        "Touch Sensing Controller reset",
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
                    name: "dma2drst",
                    description: Some(
                        "DMA2D reset",
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
            name: "Ahb1smenr",
            extends: None,
            description: Some(
                "AHB1 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1smen",
                    description: Some(
                        "DMA1 clocks enable during Sleep and Stop modes",
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
                    name: "dma2smen",
                    description: Some(
                        "DMA2 clocks enable during Sleep and Stop modes",
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
                    name: "flashsmen",
                    description: Some(
                        "Flash memory interface clocks enable during Sleep and Stop modes",
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
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 interface clocks enable during Sleep and Stop modes",
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
                    name: "crcsmen",
                    description: Some(
                        "CRCSMEN",
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
                    name: "tscsmen",
                    description: Some(
                        "Touch Sensing Controller clocks enable during Sleep and Stop modes",
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
                    name: "dma2dsmen",
                    description: Some(
                        "DMA2D clock enable during Sleep and Stop modes",
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
                "AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "IO port A clock enable",
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
                        "IO port B clock enable",
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
                        "IO port C clock enable",
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
                        "IO port D clock enable",
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
                    name: "gpioeen",
                    description: Some(
                        "IO port E clock enable",
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
                    name: "gpiofen",
                    description: Some(
                        "IO port F clock enable",
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
                    name: "gpiogen",
                    description: Some(
                        "IO port G clock enable",
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
                    name: "gpiohen",
                    description: Some(
                        "IO port H clock enable",
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
                    name: "gpioien",
                    description: Some(
                        "IO port I clock enable",
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
                    name: "usb_otg_fsen",
                    description: Some(
                        "OTG full speed clock enable",
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
                    name: "adcen",
                    description: Some(
                        "ADC clock enable",
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
                    name: "dcmien",
                    description: Some(
                        "DCMI clock enable",
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
                    name: "aesen",
                    description: Some(
                        "AES accelerator clock enable",
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
                    name: "hashen",
                    description: Some(
                        "HASH clock enable",
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
                        "Random Number Generator clock enable",
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
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset",
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
                        "IO port B reset",
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
                        "IO port C reset",
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
                        "IO port D reset",
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
                    name: "gpioerst",
                    description: Some(
                        "IO port E reset",
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
                    name: "gpiofrst",
                    description: Some(
                        "IO port F reset",
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
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset",
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
                    name: "gpiohrst",
                    description: Some(
                        "IO port H reset",
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
                    name: "gpioirst",
                    description: Some(
                        "IO port I reset",
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
                    name: "usb_otg_fsrst",
                    description: Some(
                        "USB OTG FS reset",
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
                    name: "adcrst",
                    description: Some(
                        "ADC reset",
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
                    name: "dcmirst",
                    description: Some(
                        "Digital Camera Interface reset",
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
                    name: "aesrst",
                    description: Some(
                        "AES hardware accelerator reset",
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
                    name: "hashrst",
                    description: Some(
                        "Hash reset",
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
                        "Random number generator reset",
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
            name: "Ahb2smenr",
            extends: None,
            description: Some(
                "AHB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A clocks enable during Sleep and Stop modes",
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
                    name: "gpiobsmen",
                    description: Some(
                        "IO port B clocks enable during Sleep and Stop modes",
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
                    name: "gpiocsmen",
                    description: Some(
                        "IO port C clocks enable during Sleep and Stop modes",
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
                    name: "gpiodsmen",
                    description: Some(
                        "IO port D clocks enable during Sleep and Stop modes",
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
                    name: "gpioesmen",
                    description: Some(
                        "IO port E clocks enable during Sleep and Stop modes",
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
                    name: "gpiofsmen",
                    description: Some(
                        "IO port F clocks enable during Sleep and Stop modes",
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
                    name: "gpiogsmen",
                    description: Some(
                        "IO port G clocks enable during Sleep and Stop modes",
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
                    name: "gpiohsmen",
                    description: Some(
                        "IO port H clocks enable during Sleep and Stop modes",
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
                    name: "gpioismen",
                    description: Some(
                        "IO port I clocks enable during Sleep and Stop modes",
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
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 interface clocks enable during Sleep and Stop modes",
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
                    name: "usb_otg_fssmen",
                    description: Some(
                        "OTG full speed clocks enable during Sleep and Stop modes",
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
                    name: "adcfssmen",
                    description: Some(
                        "ADC clocks enable during Sleep and Stop modes",
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
                    name: "dcmismen",
                    description: Some(
                        "DCMI clock enable during Sleep and Stop modes",
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
                    name: "aessmen",
                    description: Some(
                        "AES accelerator clocks enable during Sleep and Stop modes",
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
                    name: "hashsmen",
                    description: Some(
                        "HASH clock enable during Sleep and Stop modes",
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
                    name: "rngsmen",
                    description: Some(
                        "Random Number Generator clocks enable during Sleep and Stop modes",
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
            name: "Ahb3enr",
            extends: None,
            description: Some(
                "AHB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcen",
                    description: Some(
                        "Flexible memory controller clock enable",
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
                    name: "quadspien",
                    description: Some(
                        "QUADSPIEN",
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
            ],
        },
        FieldSet {
            name: "Ahb3rstr",
            extends: None,
            description: Some(
                "AHB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcrst",
                    description: Some(
                        "Flexible memory controller reset",
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
                    name: "quadspirst",
                    description: Some(
                        "Quad SPI memory interface reset",
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
            ],
        },
        FieldSet {
            name: "Ahb3smenr",
            extends: None,
            description: Some(
                "AHB3 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcsmen",
                    description: Some(
                        "Flexible memory controller clocks enable during Sleep and Stop modes",
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
                    name: "quadspismen",
                    description: Some(
                        "QUADSPISMEN",
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
            ],
        },
        FieldSet {
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "APB1ENR1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 timer clock enable",
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
                        "TIM3 timer clock enable",
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
                    name: "tim4en",
                    description: Some(
                        "TIM4 timer clock enable",
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
                    name: "tim5en",
                    description: Some(
                        "TIM5 timer clock enable",
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
                    name: "tim6en",
                    description: Some(
                        "TIM6 timer clock enable",
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
                        "TIM7 timer clock enable",
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
                    name: "lcden",
                    description: Some(
                        "LCD clock enable",
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
                    name: "rtcapben",
                    description: Some(
                        "RTC APB clock enable",
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
                    name: "wwdgen",
                    description: Some(
                        "Window watchdog clock enable",
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
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable",
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
                        "SPI3 clock enable",
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
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable",
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
                        "USART3 clock enable",
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
                    name: "uart4en",
                    description: Some(
                        "UART4 clock enable",
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
                    name: "uart5en",
                    description: Some(
                        "UART5 clock enable",
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
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable",
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
                        "I2C2 clock enable",
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
                    name: "i2c3en",
                    description: Some(
                        "I2C3 clock enable",
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
                        "Clock Recovery System clock enable",
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
                    name: "can1en",
                    description: Some(
                        "CAN1 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2en",
                    description: Some(
                        "CAN2 clock enable",
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
                    name: "usben",
                    description: Some(
                        "USB FS clock enable",
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
                    name: "pwren",
                    description: Some(
                        "Power interface clock enable",
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
                    name: "dac1en",
                    description: Some(
                        "DAC1 interface clock enable",
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
                    name: "opampen",
                    description: Some(
                        "OPAMP interface clock enable",
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
                    name: "lptim1en",
                    description: Some(
                        "Low power timer 1 clock enable",
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
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "Low power UART 1 clock enable",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable",
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
                    name: "swpmi1en",
                    description: Some(
                        "Single wire protocol clock enable",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2EN",
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
            ],
        },
        FieldSet {
            name: "Apb1rstr1",
            extends: None,
            description: Some(
                "APB1 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 timer reset",
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
                        "TIM3 timer reset",
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
                    name: "tim4rst",
                    description: Some(
                        "TIM3 timer reset",
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
                    name: "tim5rst",
                    description: Some(
                        "TIM5 timer reset",
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
                    name: "tim6rst",
                    description: Some(
                        "TIM6 timer reset",
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
                        "TIM7 timer reset",
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
                    name: "lcdrst",
                    description: Some(
                        "LCD interface reset",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset",
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
                        "SPI3 reset",
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
                    name: "usart2rst",
                    description: Some(
                        "USART2 reset",
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
                        "USART3 reset",
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
                    name: "uart4rst",
                    description: Some(
                        "UART4 reset",
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
                    name: "usart4rst",
                    description: Some(
                        "USART4 reset.",
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
                    name: "uart5rst",
                    description: Some(
                        "UART5 reset",
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
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset",
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
                        "I2C2 reset",
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
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset",
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
                        "CRS reset",
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
                    name: "can1rst",
                    description: Some(
                        "CAN1 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2rst",
                    description: Some(
                        "CAN2 reset",
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
                    name: "usbrst",
                    description: Some(
                        "USB FS reset",
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
                    name: "pwrrst",
                    description: Some(
                        "Power interface reset",
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
                    name: "dac1rst",
                    description: Some(
                        "DAC1 interface reset",
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
                    name: "opamprst",
                    description: Some(
                        "OPAMP interface reset",
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
                    name: "lptim1rst",
                    description: Some(
                        "Low Power Timer 1 reset",
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
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "APB1 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1rst",
                    description: Some(
                        "Low-power UART 1 reset",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset",
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
                    name: "swpmi1rst",
                    description: Some(
                        "Single wire protocol reset",
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
                    name: "lptim2rst",
                    description: Some(
                        "Low-power timer 2 reset",
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
            ],
        },
        FieldSet {
            name: "Apb1smenr1",
            extends: None,
            description: Some(
                "APB1SMENR1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim3smen",
                    description: Some(
                        "TIM3 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim4smen",
                    description: Some(
                        "TIM4 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim5smen",
                    description: Some(
                        "TIM5 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim6smen",
                    description: Some(
                        "TIM6 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim7smen",
                    description: Some(
                        "TIM7 timer clocks enable during Sleep and Stop modes",
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
                    name: "lcdsmen",
                    description: Some(
                        "LCD clocks enable during Sleep and Stop modes",
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
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC APB clock enable during Sleep and Stop modes",
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
                    name: "wwdgsmen",
                    description: Some(
                        "Window watchdog clocks enable during Sleep and Stop modes",
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
                    name: "spi2smen",
                    description: Some(
                        "SPI2 clocks enable during Sleep and Stop modes",
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
                    name: "sp3smen",
                    description: Some(
                        "SPI3 clocks enable during Sleep and Stop modes",
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
                    name: "usart2smen",
                    description: Some(
                        "USART2 clocks enable during Sleep and Stop modes",
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
                    name: "usart3smen",
                    description: Some(
                        "USART3 clocks enable during Sleep and Stop modes",
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
                    name: "uart4smen",
                    description: Some(
                        "UART4 clocks enable during Sleep and Stop modes",
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
                    name: "uart5smen",
                    description: Some(
                        "UART5 clocks enable during Sleep and Stop modes",
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
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 clocks enable during Sleep and Stop modes",
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
                    name: "i2c2smen",
                    description: Some(
                        "I2C2 clocks enable during Sleep and Stop modes",
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
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 clocks enable during Sleep and Stop modes",
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
                    name: "crssmen",
                    description: Some(
                        "CRS clock enable during Sleep and Stop modes",
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
                    name: "can1smen",
                    description: Some(
                        "CAN1 clocks enable during Sleep and Stop modes",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2smen",
                    description: Some(
                        "CAN2 clocks enable during Sleep and Stop modes",
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
                    name: "usbsmen",
                    description: Some(
                        "USB FS clock enable during Sleep and Stop modes",
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
                    name: "pwrsmen",
                    description: Some(
                        "Power interface clocks enable during Sleep and Stop modes",
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
                    name: "dac1smen",
                    description: Some(
                        "DAC1 interface clocks enable during Sleep and Stop modes",
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
                    name: "opampsmen",
                    description: Some(
                        "OPAMP interface clocks enable during Sleep and Stop modes",
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
                    name: "lptim1smen",
                    description: Some(
                        "Low power timer 1 clocks enable during Sleep and Stop modes",
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
            name: "Apb1smenr2",
            extends: None,
            description: Some(
                "APB1 peripheral clocks enable in Sleep and Stop modes register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1smen",
                    description: Some(
                        "Low power UART 1 clocks enable during Sleep and Stop modes",
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
                    name: "i2c4smen",
                    description: Some(
                        "I2C4 clocks enable during Sleep and Stop modes",
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
                    name: "swpmi1smen",
                    description: Some(
                        "Single wire protocol clocks enable during Sleep and Stop modes",
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
                    name: "lptim2smen",
                    description: Some(
                        "LPTIM2SMEN",
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
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2ENR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG clock enable",
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
                    name: "fwen",
                    description: Some(
                        "Firewall clock enable",
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
                    name: "sdmmcen",
                    description: Some(
                        "SDMMC clock enable",
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
                    name: "tim1en",
                    description: Some(
                        "TIM1 timer clock enable",
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
                        "SPI1 clock enable",
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
                    name: "tim8en",
                    description: Some(
                        "TIM8 timer clock enable",
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
                    name: "usart1en",
                    description: Some(
                        "USART1clock enable",
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
                    name: "tim15en",
                    description: Some(
                        "TIM15 timer clock enable",
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
                    name: "tim16en",
                    description: Some(
                        "TIM16 timer clock enable",
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
                    name: "tim17en",
                    description: Some(
                        "TIM17 timer clock enable",
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
                    name: "sai1en",
                    description: Some(
                        "SAI1 clock enable",
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
                    name: "sai2en",
                    description: Some(
                        "SAI2 clock enable",
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
                    name: "dfsdmen",
                    description: Some(
                        "DFSDMEN enable",
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
                "APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "System configuration (SYSCFG) reset",
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
                    name: "sdmmcrst",
                    description: Some(
                        "SDMMC reset",
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
                    name: "tim1rst",
                    description: Some(
                        "TIM1 timer reset",
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
                        "SPI1 reset",
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
                    name: "tim8rst",
                    description: Some(
                        "TIM8 timer reset",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset",
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
                    name: "tim15rst",
                    description: Some(
                        "TIM15 timer reset",
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
                    name: "tim16rst",
                    description: Some(
                        "TIM16 timer reset",
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
                    name: "tim17rst",
                    description: Some(
                        "TIM17 timer reset",
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
                    name: "sai1rst",
                    description: Some(
                        "Serial audio interface 1 (SAI1) reset",
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
                    name: "sai2rst",
                    description: Some(
                        "Serial audio interface 2 (SAI2) reset",
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
                    name: "dfsdmrst",
                    description: Some(
                        "DFSDM filter reset",
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
            name: "Apb2smenr",
            extends: None,
            description: Some(
                "APB2SMENR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsmen",
                    description: Some(
                        "SYSCFG clocks enable during Sleep and Stop modes",
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
                    name: "sdmmcsmen",
                    description: Some(
                        "SDMMC clocks enable during Sleep and Stop modes",
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
                    name: "tim1smen",
                    description: Some(
                        "TIM1 timer clocks enable during Sleep and Stop modes",
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
                    name: "spi1smen",
                    description: Some(
                        "SPI1 clocks enable during Sleep and Stop modes",
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
                    name: "tim8smen",
                    description: Some(
                        "TIM8 timer clocks enable during Sleep and Stop modes",
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
                    name: "usart1smen",
                    description: Some(
                        "USART1clocks enable during Sleep and Stop modes",
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
                    name: "tim15smen",
                    description: Some(
                        "TIM15 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim16smen",
                    description: Some(
                        "TIM16 timer clocks enable during Sleep and Stop modes",
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
                    name: "tim17smen",
                    description: Some(
                        "TIM17 timer clocks enable during Sleep and Stop modes",
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
                    name: "sai1smen",
                    description: Some(
                        "SAI1 clocks enable during Sleep and Stop modes",
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
                    name: "sai2smen",
                    description: Some(
                        "SAI2 clocks enable during Sleep and Stop modes",
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
                    name: "dfsdmsmen",
                    description: Some(
                        "DFSDM timer clocks enable during Sleep and Stop modes",
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
            name: "Bdcr",
            extends: None,
            description: Some(
                "BDCR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable",
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
                        "LSE oscillator ready",
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
                        "LSE oscillator bypass",
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
                        "SE oscillator drive capability",
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
                        "LSECSSON",
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
                        "LSECSSD",
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
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection",
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
                        "RTC clock enable",
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
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset",
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
                        "Low speed clock output enable",
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
                        "Low speed clock output selection",
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
            ],
        },
        FieldSet {
            name: "Ccipr",
            extends: None,
            description: Some(
                "CCIPR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart4sel",
                    description: Some(
                        "UART4 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart5sel",
                    description: Some(
                        "UART5 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuart1sel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1sel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c2sel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c3sel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "Low power timer 1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "Low power timer 2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sai1sel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sai2sel",
                    ),
                },
                Field {
                    name: "clk48sel",
                    description: Some(
                        "48 MHz clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Clk48sel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "ADCs clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
                Field {
                    name: "swpmi1sel",
                    description: Some(
                        "SWPMI1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Swpmi1sel",
                    ),
                },
                Field {
                    name: "dfsdmsel",
                    description: Some(
                        "DFSDM clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dfsdmsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "Clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock switch",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "System clock switch status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
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
                        "APB low-speed prescaler (APB1)",
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
                    name: "ppre2",
                    description: Some(
                        "APB high-speed prescaler (APB2)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "Wakeup from Stop and CSS backup clock selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "Microcontroller clock output selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "mcopre",
                    description: Some(
                        "Microcontroller clock output prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "Clock interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear",
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
                        "LSE ready interrupt clear",
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
                    name: "msirdyc",
                    description: Some(
                        "MSI ready interrupt clear",
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
                        "HSI ready interrupt clear",
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
                        "HSE ready interrupt clear",
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
                    name: "pllrdyc",
                    description: Some(
                        "PLL ready interrupt clear",
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
                    name: "pllsai1rdyc",
                    description: Some(
                        "PLLSAI1 ready interrupt clear",
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
                    name: "pllsai2rdyc",
                    description: Some(
                        "PLLSAI2 ready interrupt clear",
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
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear",
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
                    name: "lsecssc",
                    description: Some(
                        "LSE Clock security system interrupt clear",
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
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 oscillator ready interrupt clear",
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
                "Clock interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable",
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
                        "LSE ready interrupt enable",
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
                    name: "msirdyie",
                    description: Some(
                        "MSI ready interrupt enable",
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
                        "HSI ready interrupt enable",
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
                        "HSE ready interrupt enable",
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
                    name: "pllrdyie",
                    description: Some(
                        "PLL ready interrupt enable",
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
                    name: "pllsai1rdyie",
                    description: Some(
                        "PLLSAI1 ready interrupt enable",
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
                    name: "pllsai2rdyie",
                    description: Some(
                        "PLLSAI2 ready interrupt enable",
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
                    name: "lsecssie",
                    description: Some(
                        "LSE clock security system interrupt enable",
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
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable",
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
            name: "Cifr",
            extends: None,
            description: Some(
                "Clock interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag",
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
                        "LSE ready interrupt flag",
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
                    name: "msirdyf",
                    description: Some(
                        "MSI ready interrupt flag",
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
                        "HSI ready interrupt flag",
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
                        "HSE ready interrupt flag",
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
                    name: "pllrdyf",
                    description: Some(
                        "PLL ready interrupt flag",
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
                    name: "pllsai1rdyf",
                    description: Some(
                        "PLLSAI1 ready interrupt flag",
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
                    name: "pllsai2rdyf",
                    description: Some(
                        "PLLSAI2 ready interrupt flag",
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
                    name: "cssf",
                    description: Some(
                        "Clock security system interrupt flag",
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
                    name: "lsecssf",
                    description: Some(
                        "LSE Clock security system interrupt flag",
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
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag",
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
                "Clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msion",
                    description: Some(
                        "MSI clock enable",
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
                    name: "msirdy",
                    description: Some(
                        "MSI clock ready flag",
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
                    name: "msipllen",
                    description: Some(
                        "MSI clock PLL enable",
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
                    name: "msirgsel",
                    description: Some(
                        "MSI clock range selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msirgsel",
                    ),
                },
                Field {
                    name: "msirange",
                    description: Some(
                        "MSI clock ranges",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msirange",
                    ),
                },
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable",
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
                    name: "hsikeron",
                    description: Some(
                        "HSI always enable for peripheral kernels",
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
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag",
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
                    name: "hsiasfs",
                    description: Some(
                        "HSI automatic start from Stop",
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
                    name: "hseon",
                    description: Some(
                        "HSE clock enable",
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
                        "HSE clock ready flag",
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
                        "HSE crystal oscillator bypass",
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
                    name: "csson",
                    description: Some(
                        "Clock security system enable",
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
                    name: "pllon",
                    description: Some(
                        "Main PLL enable",
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
                    name: "pllrdy",
                    description: Some(
                        "Main PLL clock ready flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsai1on",
                    description: Some(
                        "SAI1 PLL enable",
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
                    name: "pllsai1rdy",
                    description: Some(
                        "SAI1 PLL clock ready flag",
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
                    name: "pllsai2on",
                    description: Some(
                        "SAI2 PLL enable",
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
                    name: "pllsai2rdy",
                    description: Some(
                        "SAI2 PLL clock ready flag",
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
            ],
        },
        FieldSet {
            name: "Crrcr",
            extends: None,
            description: Some(
                "Clock recovery RC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable",
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
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag",
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
                    name: "hsi48cal",
                    description: Some(
                        "HSI48 clock calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "CSR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable",
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
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready",
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
                    name: "msisrange",
                    description: Some(
                        "SI range after Standby mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
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
                    name: "fwrstf",
                    description: Some(
                        "Firewall reset flag",
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
                    name: "oblrstf",
                    description: Some(
                        "Option byte loader reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "Pin reset flag",
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
                        "BOR flag",
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
                        "Software reset flag",
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
                        "Independent window watchdog reset flag",
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
                        "Window watchdog reset flag",
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
                        "Low-power reset flag",
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
            name: "Icscr",
            extends: None,
            description: Some(
                "Internal clock sources calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msical",
                    description: Some(
                        "MSI clock calibration",
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
                    name: "msitrim",
                    description: Some(
                        "MSI clock trimming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
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
                "PLL configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "Main PLL, PLLSAI1 and PLLSAI2 entry clock source",
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
                    name: "pllm",
                    description: Some(
                        "Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "plln",
                    description: Some(
                        "Main PLL multiplication factor for VCO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plln",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "Main PLL PLLSAI3CLK output enable",
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
                    name: "pllpbit",
                    description: Some(
                        "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllpbit",
                    ),
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "Main PLL PLLUSB1CLK output enable",
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
                    name: "pllq",
                    description: Some(
                        "Main PLL division factor for PLLUSB1CLK(48 MHz clock)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "Main PLL PLLCLK output enable",
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
                    name: "pllr",
                    description: Some(
                        "Main PLL division factor for PLLCLK (system clock)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "Main PLL division factor for PLLSAI2CLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pllsaicfgr",
            extends: None,
            description: Some(
                "PLLSAI configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "SAI1PLL multiplication factor for VCO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: Some(
                        "Plln",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "SAI1PLL PLLSAICLK output enable",
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
                    name: "pllpbit",
                    description: Some(
                        "SAI1PLL division factor for PLLSAICLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllpbit",
                    ),
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "SAI1PLL PLLUSB2CLK output enable",
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
                    name: "pllq",
                    description: Some(
                        "SAI1PLL division factor for PLLUSB2CLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLLSAI PLLADC1CLK output enable",
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
                    name: "pllr",
                    description: Some(
                        "PLLSAI division factor for PLLADC1CLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLLSAI division factor for PLLSAICLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLLADC1CLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Clk48sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLSAI1_Q",
                    description: Some(
                        "PLLSAI1_Q aka PLL48M1CLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLL_Q aka PLL48M2CLK clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dfsdmsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "APB2 clock (PCLK2) selected as DFSDM kernel clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected as DFSDM kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "system clock not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "system clock divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "system clock divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "system clock divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "system clock divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "system clock divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "system clock divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "system clock divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "system clock divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "I2c1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "I2c2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "I2c3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptim2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuart1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lscosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMLOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUMHIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Division by 8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "Division by 16",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI oscillator clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 oscillator clock selected",
                    ),
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Msirange",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RANGE100K",
                    description: Some(
                        "range 0 around 100 kHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE200K",
                    description: Some(
                        "range 1 around 200 kHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE400K",
                    description: Some(
                        "range 2 around 400 kHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE800K",
                    description: Some(
                        "range 3 around 800 kHz",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RANGE1M",
                    description: Some(
                        "range 4 around 1 MHz",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RANGE2M",
                    description: Some(
                        "range 5 around 2 MHz",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "RANGE4M",
                    description: Some(
                        "range 6 around 4 MHz",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RANGE8M",
                    description: Some(
                        "range 7 around 8 MHz",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "RANGE16M",
                    description: Some(
                        "range 8 around 16 MHz",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RANGE24M",
                    description: Some(
                        "range 9 around 24 MHz",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "RANGE32M",
                    description: Some(
                        "range 10 around 32 MHz",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "RANGE48M",
                    description: Some(
                        "range 11 around 48 MHz",
                    ),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "Msirgsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CSR",
                    description: Some(
                        "MSI Range is provided by MSISRANGE[3:0] in RCC_CSR register",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CR",
                    description: Some(
                        "MSI Range is provided by MSIRANGE[3:0] in the RCC_CR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 7,
            variants: &[
                EnumVariant {
                    name: "MUL8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "MUL9",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "MUL10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "MUL11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "MUL12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "MUL13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "MUL14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "MUL15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "MUL16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "MUL17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "MUL18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "MUL19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "MUL20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "MUL21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "MUL22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "MUL23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "MUL24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "MUL25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "MUL26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "MUL27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "MUL28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "MUL29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "MUL30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "MUL31",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "MUL32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "MUL33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "MUL34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "MUL35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "MUL36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "MUL37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "MUL38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "MUL39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "MUL40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "MUL41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "MUL42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "MUL43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "MUL44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "MUL45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "MUL46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "MUL47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "MUL48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "MUL49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "MUL50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "MUL51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "MUL52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "MUL53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "MUL54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "MUL55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "MUL56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "MUL57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "MUL58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "MUL59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "MUL60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "MUL61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "MUL62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "MUL63",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "MUL64",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "MUL65",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "MUL66",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "MUL67",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "MUL68",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "MUL69",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "MUL70",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "MUL71",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "MUL72",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "MUL73",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "MUL74",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "MUL75",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "MUL76",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "MUL77",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "MUL78",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "MUL79",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "MUL80",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "MUL81",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "MUL82",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "MUL83",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "MUL84",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "MUL85",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "MUL86",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "MUL87",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "MUL88",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "MUL89",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "MUL90",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "MUL91",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "MUL92",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "MUL93",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "MUL94",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "MUL95",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "MUL96",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "MUL97",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "MUL98",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "MUL99",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "MUL100",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "MUL101",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "MUL102",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "MUL103",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "MUL104",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "MUL105",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "MUL106",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "MUL107",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "MUL108",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "MUL109",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "MUL110",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "MUL111",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "MUL112",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "MUL113",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "MUL114",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "MUL115",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "MUL116",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "MUL117",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "MUL118",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "MUL119",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "MUL120",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "MUL121",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "MUL122",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "MUL123",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "MUL124",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "MUL125",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "MUL126",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "MUL127",
                    description: None,
                    value: 127,
                },
            ],
        },
        Enum {
            name: "Pllp",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "DIV18",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "DIV19",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "DIV20",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "DIV21",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "DIV22",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "DIV23",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "DIV24",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "DIV25",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "DIV26",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "DIV27",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "DIV28",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "DIV29",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "DIV30",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "DIV31",
                    description: None,
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Pllpbit",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV7",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllq",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
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
                    name: "DISABLE",
                    description: Some(
                        "No clock sent to PLL",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI selected as PLL input clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL input clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL input clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock used as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock used as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by 32 used as the RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sai1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLLSAI1_P",
                    description: Some(
                        "PLLSAI1CLK clock is selected as SAIx clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLSAI2_P",
                    description: Some(
                        "PLLSAI2CLK clock is selected as SAIx clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLLSAI3CLK clock is selected as SAIx clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SAI1_EXTCLK",
                    description: Some(
                        "External clock SAIx_EXTCLK clock selected as SAIx clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sai2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLLSAI1_P",
                    description: Some(
                        "PLLSAI1CLK clock is selected as SAIx clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLSAI2_P",
                    description: Some(
                        "PLLSAI2CLK clock is selected as SAIx clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLLSAI3CLK clock is selected as SAIx clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SAI2_EXTCLK",
                    description: Some(
                        "External clock SAIx_EXTCLK clock selected as SAIx clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI oscillator selected as wake-up from Stop clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI oscillator selected as wake-up from Stop clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MSI",
                    description: Some(
                        "MSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "PLL selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Swpmi1sel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "SYSCLK clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
