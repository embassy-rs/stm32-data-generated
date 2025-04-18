
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
                        "clock control register",
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
                    name: "pllcfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cfgr",
                    description: Some(
                        "clock configuration register",
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
                    name: "cir",
                    description: Some(
                        "clock interrupt register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cir",
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
                    byte_offset: 0x10,
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
                    byte_offset: 0x14,
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
                    byte_offset: 0x18,
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
                    name: "apb1rstr",
                    description: Some(
                        "APB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr",
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
                    byte_offset: 0x24,
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
                        "AHB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    byte_offset: 0x34,
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
                    byte_offset: 0x38,
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
                    name: "apb1enr",
                    description: Some(
                        "APB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                    name: "ahb1lpenr",
                    description: Some(
                        "AHB1 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                        "AHB2 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 0x54,
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
                    name: "ahb3lpenr",
                    description: Some(
                        "AHB3 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lpenr",
                    description: Some(
                        "APB1 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "APB2 peripheral clock enabled in low power mode register",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                    name: "bdcr",
                    description: Some(
                        "Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 0x70,
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
                        "clock control & status register",
                    ),
                    array: None,
                    byte_offset: 0x74,
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
                    name: "sscgr",
                    description: Some(
                        "spread spectrum clock generation register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sscgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plli2scfgr",
                    description: Some(
                        "PLLI2S configuration register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plli2scfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllsaicfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 0x88,
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
                    name: "dckcfgr1",
                    description: Some(
                        "dedicated clocks configuration register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dckcfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dckcfgr2",
                    description: Some(
                        "dedicated clocks configuration register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dckcfgr2",
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
                "AHB1 peripheral clock register",
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
                    name: "gpiojen",
                    description: Some(
                        "IO port J clock enable",
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
                    name: "gpioken",
                    description: Some(
                        "IO port K clock enable",
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
                    name: "bkpsramen",
                    description: Some(
                        "Backup SRAM interface clock enable",
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
                    name: "dtcmramen",
                    description: Some(
                        "CCM data RAM clock enable",
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
                    name: "dma1en",
                    description: Some(
                        "DMA1 clock enable",
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
                    name: "dma2en",
                    description: Some(
                        "DMA2 clock enable",
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
                    name: "dma2den",
                    description: Some(
                        "DMA2D clock enable",
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
                    name: "ethen",
                    description: Some(
                        "Ethernet MAC clock enable",
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
                    name: "ethtxen",
                    description: Some(
                        "Ethernet Transmission clock enable",
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
                    name: "ethrxen",
                    description: Some(
                        "Ethernet Reception clock enable",
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
                    name: "ethptpen",
                    description: Some(
                        "Ethernet PTP clock enable",
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
                    name: "usb_otg_hsen",
                    description: Some(
                        "USB OTG HS clock enable",
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
                    name: "usb_otg_hsulpien",
                    description: Some(
                        "USB OTG HSULPI clock enable",
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
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "AHB1 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "IO port A clock enable during sleep mode",
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
                        "IO port B clock enable during Sleep mode",
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
                        "IO port C clock enable during Sleep mode",
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
                        "IO port D clock enable during Sleep mode",
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
                    name: "gpioelpen",
                    description: Some(
                        "IO port E clock enable during Sleep mode",
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
                    name: "gpioflpen",
                    description: Some(
                        "IO port F clock enable during Sleep mode",
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
                    name: "gpioglpen",
                    description: Some(
                        "IO port G clock enable during Sleep mode",
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
                    name: "gpiohlpen",
                    description: Some(
                        "IO port H clock enable during Sleep mode",
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
                    name: "gpioilpen",
                    description: Some(
                        "IO port I clock enable during Sleep mode",
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
                    name: "gpiojlpen",
                    description: Some(
                        "IO port J clock enable during Sleep mode",
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
                    name: "gpioklpen",
                    description: Some(
                        "IO port K clock enable during Sleep mode",
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
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable during Sleep mode",
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
                    name: "axilpen",
                    description: Some(
                        "AXI to AHB bridge clock enable during Sleep mode",
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
                    name: "flashlpen",
                    description: Some(
                        "Flash interface clock enable during Sleep mode",
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
                    name: "sram1lpen",
                    description: Some(
                        "SRAM 1interface clock enable during Sleep mode",
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
                    name: "sram2lpen",
                    description: Some(
                        "SRAM 2 interface clock enable during Sleep mode",
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
                    name: "bkpsramlpen",
                    description: Some(
                        "Backup SRAM interface clock enable during Sleep mode",
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
                    name: "sram3lpen",
                    description: Some(
                        "SRAM 3 interface clock enable during Sleep mode",
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
                    name: "dtcmlpen",
                    description: Some(
                        "DTCM RAM interface clock enable during Sleep mode",
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
                    name: "dma1lpen",
                    description: Some(
                        "DMA1 clock enable during Sleep mode",
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
                    name: "dma2lpen",
                    description: Some(
                        "DMA2 clock enable during Sleep mode",
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
                    name: "dma2dlpen",
                    description: Some(
                        "DMA2D clock enable during Sleep mode",
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
                    name: "ethlpen",
                    description: Some(
                        "Ethernet MAC clock enable during Sleep mode",
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
                    name: "ethtxlpen",
                    description: Some(
                        "Ethernet transmission clock enable during Sleep mode",
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
                    name: "ethrxlpen",
                    description: Some(
                        "Ethernet reception clock enable during Sleep mode",
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
                    name: "ethptplpen",
                    description: Some(
                        "Ethernet PTP clock enable during Sleep mode",
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
                    name: "usb_otg_hslpen",
                    description: Some(
                        "USB OTG HS clock enable during Sleep mode",
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
                    name: "usb_otg_hsulpilpen",
                    description: Some(
                        "USB OTG HS ULPI clock enable during Sleep mode",
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
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "AHB1 peripheral reset register",
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
                    name: "gpiojrst",
                    description: Some(
                        "IO port J reset",
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
                    name: "gpiokrst",
                    description: Some(
                        "IO port K reset",
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
                    name: "dma1rst",
                    description: Some(
                        "DMA2 reset",
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
                    name: "dma2rst",
                    description: Some(
                        "DMA2 reset",
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
                    name: "dma2drst",
                    description: Some(
                        "DMA2D reset",
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
                    name: "ethrst",
                    description: Some(
                        "Ethernet MAC reset",
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
                    name: "usb_otg_hsrst",
                    description: Some(
                        "USB OTG HS module reset",
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
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmien",
                    description: Some(
                        "Camera interface enable",
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
                    name: "jpegen",
                    description: Some(
                        "JPEG enable",
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
                    name: "aesen",
                    description: Some(
                        "AES module clock enable",
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
                    name: "crypen",
                    description: Some(
                        "Cryptographic modules clock enable",
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
                    name: "hashen",
                    description: Some(
                        "Hash modules clock enable",
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
                    name: "rngen",
                    description: Some(
                        "Random number generator clock enable",
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
                    name: "usb_otg_fsen",
                    description: Some(
                        "USB OTG FS clock enable",
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
            ],
        },
        FieldSet {
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "AHB2 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmilpen",
                    description: Some(
                        "Camera interface enable during Sleep mode",
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
                    name: "jpeglpen",
                    description: Some(
                        "JPEG module enabled during Sleep mode",
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
                    name: "aeslpen",
                    description: Some(
                        "AES module clock enable during Sleep mode",
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
                    name: "cryplpen",
                    description: Some(
                        "Cryptography modules clock enable during Sleep mode",
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
                    name: "hashlpen",
                    description: Some(
                        "Hash modules clock enable during Sleep mode",
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
                    name: "rnglpen",
                    description: Some(
                        "Random number generator clock enable during Sleep mode",
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
                    name: "usb_otg_fslpen",
                    description: Some(
                        "USB OTG FS clock enable during Sleep mode",
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
                    name: "dcmirst",
                    description: Some(
                        "Camera interface reset",
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
                    name: "aesrst",
                    description: Some(
                        "AES module reset",
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
                    name: "cryprst",
                    description: Some(
                        "Cryptographic module reset",
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
                    name: "hsahrst",
                    description: Some(
                        "Hash module reset",
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
                    name: "rngrst",
                    description: Some(
                        "Random number generator module reset",
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
                    name: "usb_otg_fsrst",
                    description: Some(
                        "USB OTG FS module reset",
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
                        "Flexible memory controller module clock enable",
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
                        "Quad SPI memory controller clock enable",
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
            ],
        },
        FieldSet {
            name: "Ahb3lpenr",
            extends: None,
            description: Some(
                "AHB3 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmclpen",
                    description: Some(
                        "Flexible memory controller module clock enable during Sleep mode",
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
                    name: "quadspilpen",
                    description: Some(
                        "Quand SPI memory controller clock enable during Sleep mode",
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
                        "Flexible memory controller module reset",
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
                        "Quad SPI memory controller reset",
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
            ],
        },
        FieldSet {
            name: "Apb1enr",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable",
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
                        "TIM3 clock enable",
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
                        "TIM4 clock enable",
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
                        "TIM5 clock enable",
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
                        "TIM6 clock enable",
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
                        "TIM7 clock enable",
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
                    name: "tim12en",
                    description: Some(
                        "TIM12 clock enable",
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
                    name: "tim13en",
                    description: Some(
                        "TIM13 clock enable",
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
                    name: "tim14en",
                    description: Some(
                        "TIM14 clock enable",
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
                    name: "lptim1en",
                    description: Some(
                        "Low power timer 1 clock enable",
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
                    name: "rtcen",
                    description: Some(
                        "RTCAPB clock enable",
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
                    name: "can3en",
                    description: Some(
                        "CAN 3 enable",
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
                    name: "spdifrxen",
                    description: Some(
                        "SPDIF-RX clock enable",
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
                        "USART 2 clock enable",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable",
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
                        "CAN 1 clock enable",
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
                        "CAN 2 clock enable",
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
                    name: "cecen",
                    description: Some(
                        "HDMI-CEN clock enable",
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
                    name: "dacen",
                    description: Some(
                        "DAC interface clock enable",
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
                    name: "uart7en",
                    description: Some(
                        "UART7 clock enable",
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
                    name: "uart8en",
                    description: Some(
                        "UART8 clock enable",
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
            name: "Apb1lpenr",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 clock enable during Sleep mode",
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
                        "TIM3 clock enable during Sleep mode",
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
                    name: "tim4lpen",
                    description: Some(
                        "TIM4 clock enable during Sleep mode",
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
                    name: "tim5lpen",
                    description: Some(
                        "TIM5 clock enable during Sleep mode",
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
                    name: "tim6lpen",
                    description: Some(
                        "TIM6 clock enable during Sleep mode",
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
                        "TIM7 clock enable during Sleep mode",
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
                    name: "tim12lpen",
                    description: Some(
                        "TIM12 clock enable during Sleep mode",
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
                    name: "tim13lpen",
                    description: Some(
                        "TIM13 clock enable during Sleep mode",
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
                    name: "tim14lpen",
                    description: Some(
                        "TIM14 clock enable during Sleep mode",
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
                    name: "lptim1lpen",
                    description: Some(
                        "low power timer 1 clock enable during Sleep mode",
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
                    name: "rtclpen",
                    description: Some(
                        "RTCAPB clock enable during Sleep mode",
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
                    name: "wwdglpen",
                    description: Some(
                        "Window watchdog clock enable during Sleep mode",
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
                    name: "can3lpen",
                    description: Some(
                        "CAN 3 clock enable during Sleep mode",
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
                        "SPI2 clock enable during Sleep mode",
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
                        "SPI3 clock enable during Sleep mode",
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
                    name: "spdifrxlpen",
                    description: Some(
                        "SPDIF-RX clock enable during sleep mode",
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
                        "USART2 clock enable during Sleep mode",
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
                        "USART3 clock enable during Sleep mode",
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
                    name: "uart4lpen",
                    description: Some(
                        "UART4 clock enable during Sleep mode",
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
                    name: "uart5lpen",
                    description: Some(
                        "UART5 clock enable during Sleep mode",
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
                    name: "i2c1lpen",
                    description: Some(
                        "I2C1 clock enable during Sleep mode",
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
                        "I2C2 clock enable during Sleep mode",
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
                    name: "i2c3lpen",
                    description: Some(
                        "I2C3 clock enable during Sleep mode",
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
                    name: "i2c4lpen",
                    description: Some(
                        "I2C4 clock enable during Sleep mode",
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
                    name: "can1lpen",
                    description: Some(
                        "CAN 1 clock enable during Sleep mode",
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
                    name: "can2lpen",
                    description: Some(
                        "CAN 2 clock enable during Sleep mode",
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
                    name: "ceclpen",
                    description: Some(
                        "HDMI-CEN clock enable during Sleep mode",
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
                    name: "pwrlpen",
                    description: Some(
                        "Power interface clock enable during Sleep mode",
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
                    name: "daclpen",
                    description: Some(
                        "DAC interface clock enable during Sleep mode",
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
                    name: "uart7lpen",
                    description: Some(
                        "UART7 clock enable during Sleep mode",
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
                    name: "uart8lpen",
                    description: Some(
                        "UART8 clock enable during Sleep mode",
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
            name: "Apb1rstr",
            extends: None,
            description: Some(
                "APB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 reset",
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
                        "TIM3 reset",
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
                        "TIM4 reset",
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
                        "TIM5 reset",
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
                        "TIM6 reset",
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
                        "TIM7 reset",
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
                    name: "tim12rst",
                    description: Some(
                        "TIM12 reset",
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
                    name: "tim13rst",
                    description: Some(
                        "TIM13 reset",
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
                    name: "tim14rst",
                    description: Some(
                        "TIM14 reset",
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
                    name: "lptim1rst",
                    description: Some(
                        "Low power timer 1 reset",
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
                    name: "wwdgrst",
                    description: Some(
                        "Window watchdog reset",
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
                    name: "can3rst",
                    description: Some(
                        "CAN 3 reset",
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
                        "SPI 2 reset",
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
                        "SPI 3 reset",
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
                    name: "spdifrxrst",
                    description: Some(
                        "SPDIF-RX reset",
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
                        "USART 2 reset",
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
                        "USART 3 reset",
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
                        "USART 4 reset",
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
                        "USART 5 reset",
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
                        "I2C 1 reset",
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
                        "I2C 2 reset",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C 4 reset",
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
                    name: "cecrst",
                    description: Some(
                        "HDMI-CEC reset",
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
                    name: "dacrst",
                    description: Some(
                        "DAC reset",
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
                    name: "uart7rst",
                    description: Some(
                        "UART7 reset",
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
                    name: "uart8rst",
                    description: Some(
                        "UART8 reset",
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
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable",
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
                    name: "tim8en",
                    description: Some(
                        "TIM8 clock enable",
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
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable",
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
                    name: "usart6en",
                    description: Some(
                        "USART6 clock enable",
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
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 clock enable",
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
                        "ADC1 clock enable",
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
                    name: "adc2en",
                    description: Some(
                        "ADC2 clock enable",
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
                    name: "adc3en",
                    description: Some(
                        "ADC3 clock enable",
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
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 clock enable",
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
                    name: "spi4en",
                    description: Some(
                        "SPI4 clock enable",
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
                    name: "syscfgen",
                    description: Some(
                        "System configuration controller clock enable",
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
                    name: "tim9en",
                    description: Some(
                        "TIM9 clock enable",
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
                    name: "tim10en",
                    description: Some(
                        "TIM10 clock enable",
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
                    name: "tim11en",
                    description: Some(
                        "TIM11 clock enable",
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
                    name: "spi5en",
                    description: Some(
                        "SPI5 clock enable",
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
                    name: "spi6en",
                    description: Some(
                        "SPI6 clock enable",
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
                    name: "sai1en",
                    description: Some(
                        "SAI1 clock enable",
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
                    name: "sai2en",
                    description: Some(
                        "SAI2 clock enable",
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
                    name: "ltdcen",
                    description: Some(
                        "LTDC clock enable",
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
                    name: "dsien",
                    description: Some(
                        "DSI clock enable",
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
                    name: "dfsdm1en",
                    description: Some(
                        "DFSDM1 clock enable",
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
                    name: "mdiosen",
                    description: Some(
                        "MDIO clock enable",
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
                    name: "usbphycen",
                    description: Some(
                        "USB OTG HS PHY controller clock enable",
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
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enabled in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 clock enable during Sleep mode",
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
                    name: "tim8lpen",
                    description: Some(
                        "TIM8 clock enable during Sleep mode",
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
                    name: "usart1lpen",
                    description: Some(
                        "USART1 clock enable during Sleep mode",
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
                    name: "usart6lpen",
                    description: Some(
                        "USART6 clock enable during Sleep mode",
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
                    name: "sdmmc2lpen",
                    description: Some(
                        "SDMMC2 clock enable during Sleep mode",
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
                        "ADC1 clock enable during Sleep mode",
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
                    name: "adc2lpen",
                    description: Some(
                        "ADC2 clock enable during Sleep mode",
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
                    name: "adc3lpen",
                    description: Some(
                        "ADC 3 clock enable during Sleep mode",
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
                    name: "sdmmc1lpen",
                    description: Some(
                        "SDMMC1 clock enable during Sleep mode",
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
                        "SPI 1 clock enable during Sleep mode",
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
                    name: "spi4lpen",
                    description: Some(
                        "SPI 4 clock enable during Sleep mode",
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
                    name: "syscfglpen",
                    description: Some(
                        "System configuration controller clock enable during Sleep mode",
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
                    name: "tim9lpen",
                    description: Some(
                        "TIM9 clock enable during sleep mode",
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
                    name: "tim10lpen",
                    description: Some(
                        "TIM10 clock enable during Sleep mode",
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
                    name: "tim11lpen",
                    description: Some(
                        "TIM11 clock enable during Sleep mode",
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
                    name: "spi5lpen",
                    description: Some(
                        "SPI 5 clock enable during Sleep mode",
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
                    name: "spi6lpen",
                    description: Some(
                        "SPI 6 clock enable during Sleep mode",
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
                    name: "sai1lpen",
                    description: Some(
                        "SAI1 clock enable during sleep mode",
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
                    name: "sai2lpen",
                    description: Some(
                        "SAI2 clock enable during sleep mode",
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
                    name: "ltdclpen",
                    description: Some(
                        "LTDC clock enable during sleep mode",
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
                    name: "dsilpen",
                    description: Some(
                        "DSI clock enable during Sleep mode",
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
                    name: "dfsdm1lpen",
                    description: Some(
                        "DFSDM1 clock enable during Sleep mode",
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
                    name: "mdioslpen",
                    description: Some(
                        "MDIO clock enable during Sleep mode",
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
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 reset",
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
                    name: "tim8rst",
                    description: Some(
                        "TIM8 reset",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset",
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
                    name: "usart6rst",
                    description: Some(
                        "USART6 reset",
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
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 reset",
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
                    name: "adcrst",
                    description: Some(
                        "ADC interface reset (common to all ADCs)",
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
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 reset",
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
                        "SPI 1 reset",
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
                    name: "spi4rst",
                    description: Some(
                        "SPI4 reset",
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
                    name: "syscfgrst",
                    description: Some(
                        "System configuration controller reset",
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
                    name: "tim9rst",
                    description: Some(
                        "TIM9 reset",
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
                    name: "tim10rst",
                    description: Some(
                        "TIM10 reset",
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
                    name: "tim11rst",
                    description: Some(
                        "TIM11 reset",
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
                    name: "spi5rst",
                    description: Some(
                        "SPI5 reset",
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
                    name: "spi6rst",
                    description: Some(
                        "SPI6 reset",
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
                    name: "sai1rst",
                    description: Some(
                        "SAI1 reset",
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
                    name: "sai2rst",
                    description: Some(
                        "SAI2 reset",
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
                    name: "ltdcrst",
                    description: Some(
                        "LTDC reset",
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
                    name: "dsirst",
                    description: Some(
                        "DSI reset",
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
                    name: "dfsdm1rst",
                    description: Some(
                        "DFSDM 1 reset",
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
                    name: "mdiosrst",
                    description: Some(
                        "MDIOS reset",
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
                    name: "usbphycrst",
                    description: Some(
                        "USB OTG HS PHY controller reset",
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
            name: "Bdcr",
            extends: None,
            description: Some(
                "Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "External low-speed oscillator enable",
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
                        "External low-speed oscillator ready",
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
                        "External low-speed oscillator bypass",
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
                        "LSE oscillator drive capability",
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
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "clock configuration register",
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
                        "APB Low speed prescaler (APB1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mco1sel",
                    description: Some(
                        "Microcontroller clock output 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mco1sel",
                    ),
                },
                Field {
                    name: "i2ssrc",
                    description: Some(
                        "I2S clock selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Issrc",
                    ),
                },
                Field {
                    name: "mco1pre",
                    description: Some(
                        "MCO1 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "MCO2 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2sel",
                    description: Some(
                        "Microcontroller clock output 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mco2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cir",
            extends: None,
            description: Some(
                "clock interrupt register",
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
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag",
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
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag",
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
                    name: "pllrdyf",
                    description: Some(
                        "PLL ready interrupt flag",
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
                    name: "plli2srdyf",
                    description: Some(
                        "PLLI2S ready interrupt flag",
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
                    name: "pllsairdyf",
                    description: Some(
                        "PLLSAI ready interrupt flag",
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
                    name: "cssf",
                    description: Some(
                        "Clock security system interrupt flag",
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
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable",
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
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable",
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
                    name: "pllrdyie",
                    description: Some(
                        "PLL ready interrupt enable",
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
                    name: "plli2srdyie",
                    description: Some(
                        "PLLI2S ready interrupt enable",
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
                    name: "pllsairdyie",
                    description: Some(
                        "PLLSAI Ready Interrupt Enable",
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
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear",
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
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear",
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
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear",
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
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear",
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
                    name: "pllrdyc",
                    description: Some(
                        "Main PLL(PLL) ready interrupt clear",
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
                    name: "plli2srdyc",
                    description: Some(
                        "PLLI2S ready interrupt clear",
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
                    name: "pllsairdyc",
                    description: Some(
                        "PLLSAI Ready Interrupt Clear",
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
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear",
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
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "Internal high-speed clock enable",
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
                        "Internal high-speed clock ready flag",
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
                    name: "hsitrim",
                    description: Some(
                        "Internal high-speed clock trimming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "Internal high-speed clock calibration",
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
                        "HSE clock bypass",
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
                        "PLL enable",
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
                        "PLL clock ready flag",
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
                    name: "plli2son",
                    description: Some(
                        "PLLI2S enable",
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
                    name: "plli2srdy",
                    description: Some(
                        "PLLI2S clock ready flag",
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
                    name: "pllsaion",
                    description: Some(
                        "PLLSAI enable",
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
                    name: "pllsairdy",
                    description: Some(
                        "PLLSAI clock ready flag",
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
            name: "Csr",
            extends: None,
            description: Some(
                "clock control & status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "Internal low-speed oscillator enable",
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
                        "Internal low-speed oscillator ready",
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
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
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
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag",
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
                    name: "padrstf",
                    description: Some(
                        "PIN reset flag",
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
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag",
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
                    name: "wdgrstf",
                    description: Some(
                        "Independent watchdog reset flag",
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
            name: "Dckcfgr1",
            extends: None,
            description: Some(
                "dedicated clocks configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plli2sdivq",
                    description: Some(
                        "PLLI2S division factor for SAI1 clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Plli2sdivq",
                    ),
                },
                Field {
                    name: "pllsaidivq",
                    description: Some(
                        "PLLSAI division factor for SAI1 clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllsaidivq",
                    ),
                },
                Field {
                    name: "pllsaidivr",
                    description: Some(
                        "division factor for LCD_CLK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsaidivr",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "Timers clocks prescalers selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timpre",
                    ),
                },
                Field {
                    name: "dfsdm1sel",
                    description: Some(
                        "DFSDM1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dfsdmsel",
                    ),
                },
                Field {
                    name: "adfsdm1sel",
                    description: Some(
                        "DFSDM1 AUDIO clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adfsdmsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dckcfgr2",
            extends: None,
            description: Some(
                "dedicated clocks configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART 1 clock source selection",
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
                        "USART 2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART 3 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart4sel",
                    description: Some(
                        "UART 4 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart5sel",
                    description: Some(
                        "UART 5 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "usart6sel",
                    description: Some(
                        "USART 6 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "uart7sel",
                    description: Some(
                        "UART 7 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart8sel",
                    description: Some(
                        "UART 8 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 clock source selection",
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
                        "I2C2 clock source selection",
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
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2csel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "Low power timer 1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "cecsel",
                    description: Some(
                        "HDMI-CEC clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cecsel",
                    ),
                },
                Field {
                    name: "clk48sel",
                    description: Some(
                        "48MHz clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Clk48sel",
                    ),
                },
                Field {
                    name: "sdmmc1sel",
                    description: Some(
                        "SDMMC1 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "sdmmc2sel",
                    description: Some(
                        "SDMMC2 clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "dsisel",
                    description: Some(
                        "DSI clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dsisel",
                    ),
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
                    name: "pllm",
                    description: Some(
                        "Division factor for the PLL and audio PLL (PLLI2S) input clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "plln",
                    description: Some(
                        "PLL multiplication factor for VCO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
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
                        "PLL division factor for main system clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL and audio PLL (PLLI2S, PLLSAI) entry clock source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL division factor for USB OTG FS, SDIO and random number generator clocks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL division factor for DSI clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Plli2scfgr",
            extends: None,
            description: Some(
                "PLLI2S configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "PLL multiplication factor for VCO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
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
                        "PLL division factor for main system clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL division factor for USB OTG FS, SDIO and random number generator clocks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL division factor for DSI clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pllsaicfgr",
            extends: None,
            description: Some(
                "PLL configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "PLL multiplication factor for VCO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
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
                        "PLL division factor for main system clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL division factor for USB OTG FS, SDIO and random number generator clocks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllq",
                    ),
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL division factor for DSI clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sscgr",
            extends: None,
            description: Some(
                "spread spectrum clock generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "modper",
                    description: Some(
                        "Modulation period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "incstep",
                    description: Some(
                        "Incrementation step",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spreadsel",
                    description: Some(
                        "Spread Select",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Spreadsel",
                    ),
                },
                Field {
                    name: "sscgen",
                    description: Some(
                        "Spread spectrum modulation enable",
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
            name: "Adfsdmsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SAI1",
                    description: Some(
                        "SAI1 clock selected as DFSDM1 Audio clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SAI2",
                    description: Some(
                        "SAI2 clock selected as DFSDM1 Audio clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cecsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as HDMI-CEC clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI_DIV_488",
                    description: Some(
                        "HSI divided by 488 clock is selected as HDMI-CEC clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Clk48sel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "48MHz clock from PLL is selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLSAI1_P",
                    description: Some(
                        "48MHz clock from PLLSAI is selected",
                    ),
                    value: 1,
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
                        "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dsisel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DSI_PHY",
                    description: Some(
                        "DSI-PHY used as DSI byte lane clock source (usual case)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)",
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
                        "SYSCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SYSCLK divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "SYSCLK divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "SYSCLK divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "SYSCLK divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "SYSCLK divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "SYSCLK divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "SYSCLK divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "SYSCLK divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "I2csel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "APB clock selected as I2C clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected as I2C clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected as I2C clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Issrc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLLI2S",
                    description: Some(
                        "PLLI2S clock used as I2S clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CKIN",
                    description: Some(
                        "External clock mapped on the I2S_CKIN pin used as I2S clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lptimsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "APB1 clock (PCLK1) selected as LPTILM1 clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock is selected as LPTILM1 clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as LPTILM1 clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as LPTILM1 clock",
                    ),
                    value: 3,
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
                    name: "MEDIUM_HIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM_LOW",
                    description: Some(
                        "Medium low driving capability",
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
            name: "Mco1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock (SYSCLK) selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLI2S",
                    description: Some(
                        "PLLI2S clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected",
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
                    value: 4,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Division by 3",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Division by 5",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Plli2sdivq",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "PLLI2SDIVQ = /1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLI2SDIVQ = /2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "PLLI2SDIVQ = /3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLI2SDIVQ = /4",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "PLLI2SDIVQ = /5",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLI2SDIVQ = /6",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "PLLI2SDIVQ = /7",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLI2SDIVQ = /8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "PLLI2SDIVQ = /9",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "PLLI2SDIVQ = /10",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "PLLI2SDIVQ = /11",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "PLLI2SDIVQ = /12",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "PLLI2SDIVQ = /13",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "PLLI2SDIVQ = /14",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "PLLI2SDIVQ = /15",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLI2SDIVQ = /16",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: Some(
                        "PLLI2SDIVQ = /17",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: Some(
                        "PLLI2SDIVQ = /18",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: Some(
                        "PLLI2SDIVQ = /19",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: Some(
                        "PLLI2SDIVQ = /20",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: Some(
                        "PLLI2SDIVQ = /21",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: Some(
                        "PLLI2SDIVQ = /22",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: Some(
                        "PLLI2SDIVQ = /23",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: Some(
                        "PLLI2SDIVQ = /24",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: Some(
                        "PLLI2SDIVQ = /25",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: Some(
                        "PLLI2SDIVQ = /26",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: Some(
                        "PLLI2SDIVQ = /27",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: Some(
                        "PLLI2SDIVQ = /28",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: Some(
                        "PLLI2SDIVQ = /29",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: Some(
                        "PLLI2SDIVQ = /30",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: Some(
                        "PLLI2SDIVQ = /31",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "PLLI2SDIVQ = /32",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 6,
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
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "DIV33",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "DIV34",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "DIV35",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "DIV36",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "DIV37",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "DIV38",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "DIV39",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "DIV40",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "DIV41",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "DIV42",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "DIV43",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "DIV44",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "DIV45",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "DIV46",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "DIV47",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "DIV48",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "DIV49",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "DIV50",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "DIV51",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "DIV52",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "DIV53",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "DIV54",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "DIV55",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "DIV56",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "DIV57",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "DIV58",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "DIV59",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "DIV60",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "DIV61",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "DIV62",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "DIV63",
                    description: None,
                    value: 63,
                },
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 9,
            variants: &[
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
                EnumVariant {
                    name: "MUL128",
                    description: None,
                    value: 128,
                },
                EnumVariant {
                    name: "MUL129",
                    description: None,
                    value: 129,
                },
                EnumVariant {
                    name: "MUL130",
                    description: None,
                    value: 130,
                },
                EnumVariant {
                    name: "MUL131",
                    description: None,
                    value: 131,
                },
                EnumVariant {
                    name: "MUL132",
                    description: None,
                    value: 132,
                },
                EnumVariant {
                    name: "MUL133",
                    description: None,
                    value: 133,
                },
                EnumVariant {
                    name: "MUL134",
                    description: None,
                    value: 134,
                },
                EnumVariant {
                    name: "MUL135",
                    description: None,
                    value: 135,
                },
                EnumVariant {
                    name: "MUL136",
                    description: None,
                    value: 136,
                },
                EnumVariant {
                    name: "MUL137",
                    description: None,
                    value: 137,
                },
                EnumVariant {
                    name: "MUL138",
                    description: None,
                    value: 138,
                },
                EnumVariant {
                    name: "MUL139",
                    description: None,
                    value: 139,
                },
                EnumVariant {
                    name: "MUL140",
                    description: None,
                    value: 140,
                },
                EnumVariant {
                    name: "MUL141",
                    description: None,
                    value: 141,
                },
                EnumVariant {
                    name: "MUL142",
                    description: None,
                    value: 142,
                },
                EnumVariant {
                    name: "MUL143",
                    description: None,
                    value: 143,
                },
                EnumVariant {
                    name: "MUL144",
                    description: None,
                    value: 144,
                },
                EnumVariant {
                    name: "MUL145",
                    description: None,
                    value: 145,
                },
                EnumVariant {
                    name: "MUL146",
                    description: None,
                    value: 146,
                },
                EnumVariant {
                    name: "MUL147",
                    description: None,
                    value: 147,
                },
                EnumVariant {
                    name: "MUL148",
                    description: None,
                    value: 148,
                },
                EnumVariant {
                    name: "MUL149",
                    description: None,
                    value: 149,
                },
                EnumVariant {
                    name: "MUL150",
                    description: None,
                    value: 150,
                },
                EnumVariant {
                    name: "MUL151",
                    description: None,
                    value: 151,
                },
                EnumVariant {
                    name: "MUL152",
                    description: None,
                    value: 152,
                },
                EnumVariant {
                    name: "MUL153",
                    description: None,
                    value: 153,
                },
                EnumVariant {
                    name: "MUL154",
                    description: None,
                    value: 154,
                },
                EnumVariant {
                    name: "MUL155",
                    description: None,
                    value: 155,
                },
                EnumVariant {
                    name: "MUL156",
                    description: None,
                    value: 156,
                },
                EnumVariant {
                    name: "MUL157",
                    description: None,
                    value: 157,
                },
                EnumVariant {
                    name: "MUL158",
                    description: None,
                    value: 158,
                },
                EnumVariant {
                    name: "MUL159",
                    description: None,
                    value: 159,
                },
                EnumVariant {
                    name: "MUL160",
                    description: None,
                    value: 160,
                },
                EnumVariant {
                    name: "MUL161",
                    description: None,
                    value: 161,
                },
                EnumVariant {
                    name: "MUL162",
                    description: None,
                    value: 162,
                },
                EnumVariant {
                    name: "MUL163",
                    description: None,
                    value: 163,
                },
                EnumVariant {
                    name: "MUL164",
                    description: None,
                    value: 164,
                },
                EnumVariant {
                    name: "MUL165",
                    description: None,
                    value: 165,
                },
                EnumVariant {
                    name: "MUL166",
                    description: None,
                    value: 166,
                },
                EnumVariant {
                    name: "MUL167",
                    description: None,
                    value: 167,
                },
                EnumVariant {
                    name: "MUL168",
                    description: None,
                    value: 168,
                },
                EnumVariant {
                    name: "MUL169",
                    description: None,
                    value: 169,
                },
                EnumVariant {
                    name: "MUL170",
                    description: None,
                    value: 170,
                },
                EnumVariant {
                    name: "MUL171",
                    description: None,
                    value: 171,
                },
                EnumVariant {
                    name: "MUL172",
                    description: None,
                    value: 172,
                },
                EnumVariant {
                    name: "MUL173",
                    description: None,
                    value: 173,
                },
                EnumVariant {
                    name: "MUL174",
                    description: None,
                    value: 174,
                },
                EnumVariant {
                    name: "MUL175",
                    description: None,
                    value: 175,
                },
                EnumVariant {
                    name: "MUL176",
                    description: None,
                    value: 176,
                },
                EnumVariant {
                    name: "MUL177",
                    description: None,
                    value: 177,
                },
                EnumVariant {
                    name: "MUL178",
                    description: None,
                    value: 178,
                },
                EnumVariant {
                    name: "MUL179",
                    description: None,
                    value: 179,
                },
                EnumVariant {
                    name: "MUL180",
                    description: None,
                    value: 180,
                },
                EnumVariant {
                    name: "MUL181",
                    description: None,
                    value: 181,
                },
                EnumVariant {
                    name: "MUL182",
                    description: None,
                    value: 182,
                },
                EnumVariant {
                    name: "MUL183",
                    description: None,
                    value: 183,
                },
                EnumVariant {
                    name: "MUL184",
                    description: None,
                    value: 184,
                },
                EnumVariant {
                    name: "MUL185",
                    description: None,
                    value: 185,
                },
                EnumVariant {
                    name: "MUL186",
                    description: None,
                    value: 186,
                },
                EnumVariant {
                    name: "MUL187",
                    description: None,
                    value: 187,
                },
                EnumVariant {
                    name: "MUL188",
                    description: None,
                    value: 188,
                },
                EnumVariant {
                    name: "MUL189",
                    description: None,
                    value: 189,
                },
                EnumVariant {
                    name: "MUL190",
                    description: None,
                    value: 190,
                },
                EnumVariant {
                    name: "MUL191",
                    description: None,
                    value: 191,
                },
                EnumVariant {
                    name: "MUL192",
                    description: None,
                    value: 192,
                },
                EnumVariant {
                    name: "MUL193",
                    description: None,
                    value: 193,
                },
                EnumVariant {
                    name: "MUL194",
                    description: None,
                    value: 194,
                },
                EnumVariant {
                    name: "MUL195",
                    description: None,
                    value: 195,
                },
                EnumVariant {
                    name: "MUL196",
                    description: None,
                    value: 196,
                },
                EnumVariant {
                    name: "MUL197",
                    description: None,
                    value: 197,
                },
                EnumVariant {
                    name: "MUL198",
                    description: None,
                    value: 198,
                },
                EnumVariant {
                    name: "MUL199",
                    description: None,
                    value: 199,
                },
                EnumVariant {
                    name: "MUL200",
                    description: None,
                    value: 200,
                },
                EnumVariant {
                    name: "MUL201",
                    description: None,
                    value: 201,
                },
                EnumVariant {
                    name: "MUL202",
                    description: None,
                    value: 202,
                },
                EnumVariant {
                    name: "MUL203",
                    description: None,
                    value: 203,
                },
                EnumVariant {
                    name: "MUL204",
                    description: None,
                    value: 204,
                },
                EnumVariant {
                    name: "MUL205",
                    description: None,
                    value: 205,
                },
                EnumVariant {
                    name: "MUL206",
                    description: None,
                    value: 206,
                },
                EnumVariant {
                    name: "MUL207",
                    description: None,
                    value: 207,
                },
                EnumVariant {
                    name: "MUL208",
                    description: None,
                    value: 208,
                },
                EnumVariant {
                    name: "MUL209",
                    description: None,
                    value: 209,
                },
                EnumVariant {
                    name: "MUL210",
                    description: None,
                    value: 210,
                },
                EnumVariant {
                    name: "MUL211",
                    description: None,
                    value: 211,
                },
                EnumVariant {
                    name: "MUL212",
                    description: None,
                    value: 212,
                },
                EnumVariant {
                    name: "MUL213",
                    description: None,
                    value: 213,
                },
                EnumVariant {
                    name: "MUL214",
                    description: None,
                    value: 214,
                },
                EnumVariant {
                    name: "MUL215",
                    description: None,
                    value: 215,
                },
                EnumVariant {
                    name: "MUL216",
                    description: None,
                    value: 216,
                },
                EnumVariant {
                    name: "MUL217",
                    description: None,
                    value: 217,
                },
                EnumVariant {
                    name: "MUL218",
                    description: None,
                    value: 218,
                },
                EnumVariant {
                    name: "MUL219",
                    description: None,
                    value: 219,
                },
                EnumVariant {
                    name: "MUL220",
                    description: None,
                    value: 220,
                },
                EnumVariant {
                    name: "MUL221",
                    description: None,
                    value: 221,
                },
                EnumVariant {
                    name: "MUL222",
                    description: None,
                    value: 222,
                },
                EnumVariant {
                    name: "MUL223",
                    description: None,
                    value: 223,
                },
                EnumVariant {
                    name: "MUL224",
                    description: None,
                    value: 224,
                },
                EnumVariant {
                    name: "MUL225",
                    description: None,
                    value: 225,
                },
                EnumVariant {
                    name: "MUL226",
                    description: None,
                    value: 226,
                },
                EnumVariant {
                    name: "MUL227",
                    description: None,
                    value: 227,
                },
                EnumVariant {
                    name: "MUL228",
                    description: None,
                    value: 228,
                },
                EnumVariant {
                    name: "MUL229",
                    description: None,
                    value: 229,
                },
                EnumVariant {
                    name: "MUL230",
                    description: None,
                    value: 230,
                },
                EnumVariant {
                    name: "MUL231",
                    description: None,
                    value: 231,
                },
                EnumVariant {
                    name: "MUL232",
                    description: None,
                    value: 232,
                },
                EnumVariant {
                    name: "MUL233",
                    description: None,
                    value: 233,
                },
                EnumVariant {
                    name: "MUL234",
                    description: None,
                    value: 234,
                },
                EnumVariant {
                    name: "MUL235",
                    description: None,
                    value: 235,
                },
                EnumVariant {
                    name: "MUL236",
                    description: None,
                    value: 236,
                },
                EnumVariant {
                    name: "MUL237",
                    description: None,
                    value: 237,
                },
                EnumVariant {
                    name: "MUL238",
                    description: None,
                    value: 238,
                },
                EnumVariant {
                    name: "MUL239",
                    description: None,
                    value: 239,
                },
                EnumVariant {
                    name: "MUL240",
                    description: None,
                    value: 240,
                },
                EnumVariant {
                    name: "MUL241",
                    description: None,
                    value: 241,
                },
                EnumVariant {
                    name: "MUL242",
                    description: None,
                    value: 242,
                },
                EnumVariant {
                    name: "MUL243",
                    description: None,
                    value: 243,
                },
                EnumVariant {
                    name: "MUL244",
                    description: None,
                    value: 244,
                },
                EnumVariant {
                    name: "MUL245",
                    description: None,
                    value: 245,
                },
                EnumVariant {
                    name: "MUL246",
                    description: None,
                    value: 246,
                },
                EnumVariant {
                    name: "MUL247",
                    description: None,
                    value: 247,
                },
                EnumVariant {
                    name: "MUL248",
                    description: None,
                    value: 248,
                },
                EnumVariant {
                    name: "MUL249",
                    description: None,
                    value: 249,
                },
                EnumVariant {
                    name: "MUL250",
                    description: None,
                    value: 250,
                },
                EnumVariant {
                    name: "MUL251",
                    description: None,
                    value: 251,
                },
                EnumVariant {
                    name: "MUL252",
                    description: None,
                    value: 252,
                },
                EnumVariant {
                    name: "MUL253",
                    description: None,
                    value: 253,
                },
                EnumVariant {
                    name: "MUL254",
                    description: None,
                    value: 254,
                },
                EnumVariant {
                    name: "MUL255",
                    description: None,
                    value: 255,
                },
                EnumVariant {
                    name: "MUL256",
                    description: None,
                    value: 256,
                },
                EnumVariant {
                    name: "MUL257",
                    description: None,
                    value: 257,
                },
                EnumVariant {
                    name: "MUL258",
                    description: None,
                    value: 258,
                },
                EnumVariant {
                    name: "MUL259",
                    description: None,
                    value: 259,
                },
                EnumVariant {
                    name: "MUL260",
                    description: None,
                    value: 260,
                },
                EnumVariant {
                    name: "MUL261",
                    description: None,
                    value: 261,
                },
                EnumVariant {
                    name: "MUL262",
                    description: None,
                    value: 262,
                },
                EnumVariant {
                    name: "MUL263",
                    description: None,
                    value: 263,
                },
                EnumVariant {
                    name: "MUL264",
                    description: None,
                    value: 264,
                },
                EnumVariant {
                    name: "MUL265",
                    description: None,
                    value: 265,
                },
                EnumVariant {
                    name: "MUL266",
                    description: None,
                    value: 266,
                },
                EnumVariant {
                    name: "MUL267",
                    description: None,
                    value: 267,
                },
                EnumVariant {
                    name: "MUL268",
                    description: None,
                    value: 268,
                },
                EnumVariant {
                    name: "MUL269",
                    description: None,
                    value: 269,
                },
                EnumVariant {
                    name: "MUL270",
                    description: None,
                    value: 270,
                },
                EnumVariant {
                    name: "MUL271",
                    description: None,
                    value: 271,
                },
                EnumVariant {
                    name: "MUL272",
                    description: None,
                    value: 272,
                },
                EnumVariant {
                    name: "MUL273",
                    description: None,
                    value: 273,
                },
                EnumVariant {
                    name: "MUL274",
                    description: None,
                    value: 274,
                },
                EnumVariant {
                    name: "MUL275",
                    description: None,
                    value: 275,
                },
                EnumVariant {
                    name: "MUL276",
                    description: None,
                    value: 276,
                },
                EnumVariant {
                    name: "MUL277",
                    description: None,
                    value: 277,
                },
                EnumVariant {
                    name: "MUL278",
                    description: None,
                    value: 278,
                },
                EnumVariant {
                    name: "MUL279",
                    description: None,
                    value: 279,
                },
                EnumVariant {
                    name: "MUL280",
                    description: None,
                    value: 280,
                },
                EnumVariant {
                    name: "MUL281",
                    description: None,
                    value: 281,
                },
                EnumVariant {
                    name: "MUL282",
                    description: None,
                    value: 282,
                },
                EnumVariant {
                    name: "MUL283",
                    description: None,
                    value: 283,
                },
                EnumVariant {
                    name: "MUL284",
                    description: None,
                    value: 284,
                },
                EnumVariant {
                    name: "MUL285",
                    description: None,
                    value: 285,
                },
                EnumVariant {
                    name: "MUL286",
                    description: None,
                    value: 286,
                },
                EnumVariant {
                    name: "MUL287",
                    description: None,
                    value: 287,
                },
                EnumVariant {
                    name: "MUL288",
                    description: None,
                    value: 288,
                },
                EnumVariant {
                    name: "MUL289",
                    description: None,
                    value: 289,
                },
                EnumVariant {
                    name: "MUL290",
                    description: None,
                    value: 290,
                },
                EnumVariant {
                    name: "MUL291",
                    description: None,
                    value: 291,
                },
                EnumVariant {
                    name: "MUL292",
                    description: None,
                    value: 292,
                },
                EnumVariant {
                    name: "MUL293",
                    description: None,
                    value: 293,
                },
                EnumVariant {
                    name: "MUL294",
                    description: None,
                    value: 294,
                },
                EnumVariant {
                    name: "MUL295",
                    description: None,
                    value: 295,
                },
                EnumVariant {
                    name: "MUL296",
                    description: None,
                    value: 296,
                },
                EnumVariant {
                    name: "MUL297",
                    description: None,
                    value: 297,
                },
                EnumVariant {
                    name: "MUL298",
                    description: None,
                    value: 298,
                },
                EnumVariant {
                    name: "MUL299",
                    description: None,
                    value: 299,
                },
                EnumVariant {
                    name: "MUL300",
                    description: None,
                    value: 300,
                },
                EnumVariant {
                    name: "MUL301",
                    description: None,
                    value: 301,
                },
                EnumVariant {
                    name: "MUL302",
                    description: None,
                    value: 302,
                },
                EnumVariant {
                    name: "MUL303",
                    description: None,
                    value: 303,
                },
                EnumVariant {
                    name: "MUL304",
                    description: None,
                    value: 304,
                },
                EnumVariant {
                    name: "MUL305",
                    description: None,
                    value: 305,
                },
                EnumVariant {
                    name: "MUL306",
                    description: None,
                    value: 306,
                },
                EnumVariant {
                    name: "MUL307",
                    description: None,
                    value: 307,
                },
                EnumVariant {
                    name: "MUL308",
                    description: None,
                    value: 308,
                },
                EnumVariant {
                    name: "MUL309",
                    description: None,
                    value: 309,
                },
                EnumVariant {
                    name: "MUL310",
                    description: None,
                    value: 310,
                },
                EnumVariant {
                    name: "MUL311",
                    description: None,
                    value: 311,
                },
                EnumVariant {
                    name: "MUL312",
                    description: None,
                    value: 312,
                },
                EnumVariant {
                    name: "MUL313",
                    description: None,
                    value: 313,
                },
                EnumVariant {
                    name: "MUL314",
                    description: None,
                    value: 314,
                },
                EnumVariant {
                    name: "MUL315",
                    description: None,
                    value: 315,
                },
                EnumVariant {
                    name: "MUL316",
                    description: None,
                    value: 316,
                },
                EnumVariant {
                    name: "MUL317",
                    description: None,
                    value: 317,
                },
                EnumVariant {
                    name: "MUL318",
                    description: None,
                    value: 318,
                },
                EnumVariant {
                    name: "MUL319",
                    description: None,
                    value: 319,
                },
                EnumVariant {
                    name: "MUL320",
                    description: None,
                    value: 320,
                },
                EnumVariant {
                    name: "MUL321",
                    description: None,
                    value: 321,
                },
                EnumVariant {
                    name: "MUL322",
                    description: None,
                    value: 322,
                },
                EnumVariant {
                    name: "MUL323",
                    description: None,
                    value: 323,
                },
                EnumVariant {
                    name: "MUL324",
                    description: None,
                    value: 324,
                },
                EnumVariant {
                    name: "MUL325",
                    description: None,
                    value: 325,
                },
                EnumVariant {
                    name: "MUL326",
                    description: None,
                    value: 326,
                },
                EnumVariant {
                    name: "MUL327",
                    description: None,
                    value: 327,
                },
                EnumVariant {
                    name: "MUL328",
                    description: None,
                    value: 328,
                },
                EnumVariant {
                    name: "MUL329",
                    description: None,
                    value: 329,
                },
                EnumVariant {
                    name: "MUL330",
                    description: None,
                    value: 330,
                },
                EnumVariant {
                    name: "MUL331",
                    description: None,
                    value: 331,
                },
                EnumVariant {
                    name: "MUL332",
                    description: None,
                    value: 332,
                },
                EnumVariant {
                    name: "MUL333",
                    description: None,
                    value: 333,
                },
                EnumVariant {
                    name: "MUL334",
                    description: None,
                    value: 334,
                },
                EnumVariant {
                    name: "MUL335",
                    description: None,
                    value: 335,
                },
                EnumVariant {
                    name: "MUL336",
                    description: None,
                    value: 336,
                },
                EnumVariant {
                    name: "MUL337",
                    description: None,
                    value: 337,
                },
                EnumVariant {
                    name: "MUL338",
                    description: None,
                    value: 338,
                },
                EnumVariant {
                    name: "MUL339",
                    description: None,
                    value: 339,
                },
                EnumVariant {
                    name: "MUL340",
                    description: None,
                    value: 340,
                },
                EnumVariant {
                    name: "MUL341",
                    description: None,
                    value: 341,
                },
                EnumVariant {
                    name: "MUL342",
                    description: None,
                    value: 342,
                },
                EnumVariant {
                    name: "MUL343",
                    description: None,
                    value: 343,
                },
                EnumVariant {
                    name: "MUL344",
                    description: None,
                    value: 344,
                },
                EnumVariant {
                    name: "MUL345",
                    description: None,
                    value: 345,
                },
                EnumVariant {
                    name: "MUL346",
                    description: None,
                    value: 346,
                },
                EnumVariant {
                    name: "MUL347",
                    description: None,
                    value: 347,
                },
                EnumVariant {
                    name: "MUL348",
                    description: None,
                    value: 348,
                },
                EnumVariant {
                    name: "MUL349",
                    description: None,
                    value: 349,
                },
                EnumVariant {
                    name: "MUL350",
                    description: None,
                    value: 350,
                },
                EnumVariant {
                    name: "MUL351",
                    description: None,
                    value: 351,
                },
                EnumVariant {
                    name: "MUL352",
                    description: None,
                    value: 352,
                },
                EnumVariant {
                    name: "MUL353",
                    description: None,
                    value: 353,
                },
                EnumVariant {
                    name: "MUL354",
                    description: None,
                    value: 354,
                },
                EnumVariant {
                    name: "MUL355",
                    description: None,
                    value: 355,
                },
                EnumVariant {
                    name: "MUL356",
                    description: None,
                    value: 356,
                },
                EnumVariant {
                    name: "MUL357",
                    description: None,
                    value: 357,
                },
                EnumVariant {
                    name: "MUL358",
                    description: None,
                    value: 358,
                },
                EnumVariant {
                    name: "MUL359",
                    description: None,
                    value: 359,
                },
                EnumVariant {
                    name: "MUL360",
                    description: None,
                    value: 360,
                },
                EnumVariant {
                    name: "MUL361",
                    description: None,
                    value: 361,
                },
                EnumVariant {
                    name: "MUL362",
                    description: None,
                    value: 362,
                },
                EnumVariant {
                    name: "MUL363",
                    description: None,
                    value: 363,
                },
                EnumVariant {
                    name: "MUL364",
                    description: None,
                    value: 364,
                },
                EnumVariant {
                    name: "MUL365",
                    description: None,
                    value: 365,
                },
                EnumVariant {
                    name: "MUL366",
                    description: None,
                    value: 366,
                },
                EnumVariant {
                    name: "MUL367",
                    description: None,
                    value: 367,
                },
                EnumVariant {
                    name: "MUL368",
                    description: None,
                    value: 368,
                },
                EnumVariant {
                    name: "MUL369",
                    description: None,
                    value: 369,
                },
                EnumVariant {
                    name: "MUL370",
                    description: None,
                    value: 370,
                },
                EnumVariant {
                    name: "MUL371",
                    description: None,
                    value: 371,
                },
                EnumVariant {
                    name: "MUL372",
                    description: None,
                    value: 372,
                },
                EnumVariant {
                    name: "MUL373",
                    description: None,
                    value: 373,
                },
                EnumVariant {
                    name: "MUL374",
                    description: None,
                    value: 374,
                },
                EnumVariant {
                    name: "MUL375",
                    description: None,
                    value: 375,
                },
                EnumVariant {
                    name: "MUL376",
                    description: None,
                    value: 376,
                },
                EnumVariant {
                    name: "MUL377",
                    description: None,
                    value: 377,
                },
                EnumVariant {
                    name: "MUL378",
                    description: None,
                    value: 378,
                },
                EnumVariant {
                    name: "MUL379",
                    description: None,
                    value: 379,
                },
                EnumVariant {
                    name: "MUL380",
                    description: None,
                    value: 380,
                },
                EnumVariant {
                    name: "MUL381",
                    description: None,
                    value: 381,
                },
                EnumVariant {
                    name: "MUL382",
                    description: None,
                    value: 382,
                },
                EnumVariant {
                    name: "MUL383",
                    description: None,
                    value: 383,
                },
                EnumVariant {
                    name: "MUL384",
                    description: None,
                    value: 384,
                },
                EnumVariant {
                    name: "MUL385",
                    description: None,
                    value: 385,
                },
                EnumVariant {
                    name: "MUL386",
                    description: None,
                    value: 386,
                },
                EnumVariant {
                    name: "MUL387",
                    description: None,
                    value: 387,
                },
                EnumVariant {
                    name: "MUL388",
                    description: None,
                    value: 388,
                },
                EnumVariant {
                    name: "MUL389",
                    description: None,
                    value: 389,
                },
                EnumVariant {
                    name: "MUL390",
                    description: None,
                    value: 390,
                },
                EnumVariant {
                    name: "MUL391",
                    description: None,
                    value: 391,
                },
                EnumVariant {
                    name: "MUL392",
                    description: None,
                    value: 392,
                },
                EnumVariant {
                    name: "MUL393",
                    description: None,
                    value: 393,
                },
                EnumVariant {
                    name: "MUL394",
                    description: None,
                    value: 394,
                },
                EnumVariant {
                    name: "MUL395",
                    description: None,
                    value: 395,
                },
                EnumVariant {
                    name: "MUL396",
                    description: None,
                    value: 396,
                },
                EnumVariant {
                    name: "MUL397",
                    description: None,
                    value: 397,
                },
                EnumVariant {
                    name: "MUL398",
                    description: None,
                    value: 398,
                },
                EnumVariant {
                    name: "MUL399",
                    description: None,
                    value: 399,
                },
                EnumVariant {
                    name: "MUL400",
                    description: None,
                    value: 400,
                },
                EnumVariant {
                    name: "MUL401",
                    description: None,
                    value: 401,
                },
                EnumVariant {
                    name: "MUL402",
                    description: None,
                    value: 402,
                },
                EnumVariant {
                    name: "MUL403",
                    description: None,
                    value: 403,
                },
                EnumVariant {
                    name: "MUL404",
                    description: None,
                    value: 404,
                },
                EnumVariant {
                    name: "MUL405",
                    description: None,
                    value: 405,
                },
                EnumVariant {
                    name: "MUL406",
                    description: None,
                    value: 406,
                },
                EnumVariant {
                    name: "MUL407",
                    description: None,
                    value: 407,
                },
                EnumVariant {
                    name: "MUL408",
                    description: None,
                    value: 408,
                },
                EnumVariant {
                    name: "MUL409",
                    description: None,
                    value: 409,
                },
                EnumVariant {
                    name: "MUL410",
                    description: None,
                    value: 410,
                },
                EnumVariant {
                    name: "MUL411",
                    description: None,
                    value: 411,
                },
                EnumVariant {
                    name: "MUL412",
                    description: None,
                    value: 412,
                },
                EnumVariant {
                    name: "MUL413",
                    description: None,
                    value: 413,
                },
                EnumVariant {
                    name: "MUL414",
                    description: None,
                    value: 414,
                },
                EnumVariant {
                    name: "MUL415",
                    description: None,
                    value: 415,
                },
                EnumVariant {
                    name: "MUL416",
                    description: None,
                    value: 416,
                },
                EnumVariant {
                    name: "MUL417",
                    description: None,
                    value: 417,
                },
                EnumVariant {
                    name: "MUL418",
                    description: None,
                    value: 418,
                },
                EnumVariant {
                    name: "MUL419",
                    description: None,
                    value: 419,
                },
                EnumVariant {
                    name: "MUL420",
                    description: None,
                    value: 420,
                },
                EnumVariant {
                    name: "MUL421",
                    description: None,
                    value: 421,
                },
                EnumVariant {
                    name: "MUL422",
                    description: None,
                    value: 422,
                },
                EnumVariant {
                    name: "MUL423",
                    description: None,
                    value: 423,
                },
                EnumVariant {
                    name: "MUL424",
                    description: None,
                    value: 424,
                },
                EnumVariant {
                    name: "MUL425",
                    description: None,
                    value: 425,
                },
                EnumVariant {
                    name: "MUL426",
                    description: None,
                    value: 426,
                },
                EnumVariant {
                    name: "MUL427",
                    description: None,
                    value: 427,
                },
                EnumVariant {
                    name: "MUL428",
                    description: None,
                    value: 428,
                },
                EnumVariant {
                    name: "MUL429",
                    description: None,
                    value: 429,
                },
                EnumVariant {
                    name: "MUL430",
                    description: None,
                    value: 430,
                },
                EnumVariant {
                    name: "MUL431",
                    description: None,
                    value: 431,
                },
                EnumVariant {
                    name: "MUL432",
                    description: None,
                    value: 432,
                },
            ],
        },
        Enum {
            name: "Pllp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLP=2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLP=4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLP=6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLP=8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllq",
            description: None,
            bit_size: 4,
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
            ],
        },
        Enum {
            name: "Pllr",
            description: None,
            bit_size: 3,
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
            ],
        },
        Enum {
            name: "Pllsaidivq",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "PLLSAIDIVQ = /1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLSAIDIVQ = /2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "PLLSAIDIVQ = /3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLSAIDIVQ = /4",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "PLLSAIDIVQ = /5",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLSAIDIVQ = /6",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "PLLSAIDIVQ = /7",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLSAIDIVQ = /8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "PLLSAIDIVQ = /9",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "PLLSAIDIVQ = /10",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "PLLSAIDIVQ = /11",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "PLLSAIDIVQ = /12",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "PLLSAIDIVQ = /13",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "PLLSAIDIVQ = /14",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "PLLSAIDIVQ = /15",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLSAIDIVQ = /16",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: Some(
                        "PLLSAIDIVQ = /17",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: Some(
                        "PLLSAIDIVQ = /18",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: Some(
                        "PLLSAIDIVQ = /19",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: Some(
                        "PLLSAIDIVQ = /20",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: Some(
                        "PLLSAIDIVQ = /21",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: Some(
                        "PLLSAIDIVQ = /22",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: Some(
                        "PLLSAIDIVQ = /23",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: Some(
                        "PLLSAIDIVQ = /24",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: Some(
                        "PLLSAIDIVQ = /25",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: Some(
                        "PLLSAIDIVQ = /26",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: Some(
                        "PLLSAIDIVQ = /27",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: Some(
                        "PLLSAIDIVQ = /28",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: Some(
                        "PLLSAIDIVQ = /29",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: Some(
                        "PLLSAIDIVQ = /30",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: Some(
                        "PLLSAIDIVQ = /31",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "PLLSAIDIVQ = /32",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Pllsaidivr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLSAIDIVR = /2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLSAIDIVR = /4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLSAIDIVR = /8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLSAIDIVR = /16",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected as PLL and PLLI2S clock entry",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected as PLL and PLLI2S clock entry",
                    ),
                    value: 1,
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
                        "HSE oscillator clock divided by a prescaler used as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Saisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLLSAI1_Q",
                    description: Some(
                        "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLI2S1_Q",
                    description: Some(
                        "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "AFIF",
                    description: Some(
                        "SAI2 clock frequency = Alternate function input frequency",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI_HSE",
                    description: Some(
                        "SAI2 clock frequency = HSI or HSE",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLK48",
                    description: Some(
                        "48 MHz clock is selected as SD clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock is selected as SD clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spreadsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CENTER",
                    description: Some(
                        "Center spread",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Down spread",
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
                    name: "HSI",
                    description: Some(
                        "HSI oscillator used as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator used as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL used as system clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Timpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MUL2",
                    description: Some(
                        "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUL4",
                    description: Some(
                        "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx",
                    ),
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
                        "APB2 clock (PCLK2) is selected as USART clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock is selected as USART clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as USART clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as USART clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usart2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "APB1 clock (PCLK1) is selected as USART clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock is selected as USART clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as USART clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as USART clock",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
