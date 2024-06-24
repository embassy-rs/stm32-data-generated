
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
                    name: "hsicfgr",
                    description: Some(
                        "RCC HSI configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                        "RCC Clock Recovery RC Register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RCC CSI configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                        "RCC Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "d1cfgr",
                    description: Some(
                        "RCC Domain 1 Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D1cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d2cfgr",
                    description: Some(
                        "RCC Domain 2 Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D2cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d3cfgr",
                    description: Some(
                        "RCC Domain 3 Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D3cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllckselr",
                    description: Some(
                        "RCC PLLs Clock Source Selection Register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllckselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "RCC PLLs Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                        "RCC PLL1 Dividers Configuration Register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
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
                        "RCC PLL1 Fractional Divider Register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
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
                                "Pllfracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d1ccipr",
                    description: Some(
                        "RCC Domain 1 Kernel Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D1ccipr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d2ccip1r",
                    description: Some(
                        "RCC Domain 2 Kernel Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D2ccip1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d2ccip2r",
                    description: Some(
                        "RCC Domain 2 Kernel Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D2ccip2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d3ccipr",
                    description: Some(
                        "RCC Domain 3 Kernel Clock Configuration Register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D3ccipr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC Clock Source Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                        "RCC Clock Source Interrupt Flag Register",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                        "RCC Clock Source Interrupt Clear Register",
                    ),
                    array: None,
                    byte_offset: 0x68,
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
                    name: "bdcr",
                    description: Some(
                        "RCC Backup Domain Control Register",
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
                        "RCC Clock Control and Status Register",
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
                    name: "ahb3rstr",
                    description: Some(
                        "RCC AHB3 Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
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
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x80,
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
                        "RCC AHB2 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x84,
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
                    name: "ahb4rstr",
                    description: Some(
                        "RCC AHB4 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3rstr",
                    description: Some(
                        "RCC APB3 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
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
                    name: "apb1lrstr",
                    description: Some(
                        "RCC APB1 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x90,
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
                        "RCC APB1 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x94,
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
                        "RCC APB2 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x98,
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
                    name: "apb4rstr",
                    description: Some(
                        "RCC APB4 Peripheral Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "Global Control Register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "d3amr",
                    description: Some(
                        "RCC D3 Autonomous mode Register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "D3amr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsr",
                    description: Some(
                        "RCC Reset Status Register",
                    ),
                    array: None,
                    byte_offset: 0x130,
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
                BlockItem {
                    name: "ahb3enr",
                    description: Some(
                        "RCC AHB3 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x134,
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
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x138,
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
                        "RCC AHB2 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x13c,
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
                    name: "ahb4enr",
                    description: Some(
                        "RCC AHB4 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3enr",
                    description: Some(
                        "RCC APB3 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x144,
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
                    name: "apb1lenr",
                    description: Some(
                        "RCC APB1 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x148,
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
                        "RCC APB1 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x14c,
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
                        "RCC APB2 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x150,
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
                    name: "apb4enr",
                    description: Some(
                        "RCC APB4 Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3lpenr",
                    description: Some(
                        "RCC AHB3 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x15c,
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
                    name: "ahb1lpenr",
                    description: Some(
                        "RCC AHB1 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x160,
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
                        "RCC AHB2 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x164,
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
                    name: "ahb4lpenr",
                    description: Some(
                        "RCC AHB4 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3lpenr",
                    description: Some(
                        "RCC APB3 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x16c,
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
                    name: "apb1llpenr",
                    description: Some(
                        "RCC APB1 Low Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x170,
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
                        "RCC APB1 High Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x174,
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
                        "RCC APB2 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x178,
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
                    name: "apb4lpenr",
                    description: Some(
                        "RCC APB4 Sleep Clock Register",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb4lpenr",
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
                "RCC AHB1 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA1 Clock Enable",
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
                        "DMA2 Clock Enable",
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
                    name: "adc12en",
                    description: Some(
                        "ADC1/2 Peripheral Clocks Enable",
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
                    name: "arten",
                    description: Some(
                        "ART Clock Enable",
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
                    name: "ethen",
                    description: Some(
                        "Ethernet MAC bus interface Clock Enable",
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
                    name: "ethtxen",
                    description: Some(
                        "Ethernet Transmission Clock Enable",
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
                    name: "ethrxen",
                    description: Some(
                        "Ethernet Reception Clock Enable",
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
                    name: "usb_otg_hsen",
                    description: Some(
                        "USB_OTG_HS Peripheral Clocks Enable",
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
                    name: "usb_otg_hs_ulpien",
                    description: Some(
                        "USB_OTG_HS ULPI clock enable",
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
                    name: "usb_otg_fsen",
                    description: Some(
                        "USB_OTG_FS Peripheral Clocks Enable",
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
                    name: "usb_otg_fs_ulpien",
                    description: Some(
                        "USB_OTG_FS ULPI clock enable",
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
            ],
        },
        FieldSet {
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "RCC AHB1 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1lpen",
                    description: Some(
                        "DMA1 Clock Enable During CSleep Mode",
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
                    name: "dma2lpen",
                    description: Some(
                        "DMA2 Clock Enable During CSleep Mode",
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
                    name: "adc12lpen",
                    description: Some(
                        "ADC1/2 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "artlpen",
                    description: Some(
                        "ART Clock Enable During CSleep Mode",
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
                    name: "ethlpen",
                    description: Some(
                        "Ethernet MAC bus interface Clock Enable During CSleep Mode",
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
                    name: "ethtxlpen",
                    description: Some(
                        "Ethernet Transmission Clock Enable During CSleep Mode",
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
                    name: "ethrxlpen",
                    description: Some(
                        "Ethernet Reception Clock Enable During CSleep Mode",
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
                    name: "usb_otg_hslpen",
                    description: Some(
                        "USB_OTG_HS peripheral clock enable during CSleep mode",
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
                    name: "usb_otg_hs_ulpilpen",
                    description: Some(
                        "USB_PHY1 clock enable during CSleep mode",
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
                    name: "usb_otg_fslpen",
                    description: Some(
                        "USB_OTG_FS peripheral clock enable during CSleep mode",
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
                    name: "usb_otg_fs_ulpilpen",
                    description: Some(
                        "USB_PHY2 clocks enable during CSleep mode",
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
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "RCC AHB1 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA1 block reset",
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
                        "DMA2 block reset",
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
                    name: "adc12rst",
                    description: Some(
                        "ADC1&2 block reset",
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
                    name: "artrst",
                    description: Some(
                        "ART block reset",
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
                    name: "ethrst",
                    description: Some(
                        "ETH block reset",
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
                    name: "usb_otg_hsrst",
                    description: Some(
                        "USB_OTG_HS block reset",
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
                    name: "usb_otg_fsrst",
                    description: Some(
                        "USB_OTG_FS block reset",
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
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmien",
                    description: Some(
                        "DCMI peripheral clock",
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
                    name: "crypen",
                    description: Some(
                        "CRYP peripheral clock enable",
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
                        "HASH peripheral clock enable",
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
                        "RNG peripheral clocks enable",
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
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 and SDMMC2 delay clock enable",
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
                    name: "bdma1en",
                    description: Some(
                        "BDMA1 clock enable",
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
                    name: "fmacen",
                    description: Some(
                        "FMAC enable",
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
                    name: "cordicen",
                    description: Some(
                        "CORDIC enable",
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
                    name: "sram1en",
                    description: Some(
                        "SRAM1 block enable",
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
                    name: "sram2en",
                    description: Some(
                        "SRAM2 block enable",
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
                    name: "sram3en",
                    description: Some(
                        "SRAM3 block enable",
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
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "RCC AHB2 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmilpen",
                    description: Some(
                        "DCMI peripheral clock enable during csleep mode",
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
                    name: "cryplpen",
                    description: Some(
                        "CRYP peripheral clock enable during CSleep mode",
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
                        "HASH peripheral clock enable during CSleep mode",
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
                        "RNG peripheral clock enable during CSleep mode",
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
                    name: "sdmmc2lpen",
                    description: Some(
                        "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode",
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
                    name: "bdma1lpen",
                    description: Some(
                        "BDMA1 Clock Enable During CSleep Mode",
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
                    name: "fmaclpen",
                    description: Some(
                        "FMAC enable during CSleep Mode",
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
                    name: "cordiclpen",
                    description: Some(
                        "CORDIC enable during CSleep Mode",
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
                    name: "sram1lpen",
                    description: Some(
                        "SRAM1 Clock Enable During CSleep Mode",
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
                    name: "sram2lpen",
                    description: Some(
                        "SRAM2 Clock Enable During CSleep Mode",
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
                    name: "sram3lpen",
                    description: Some(
                        "SRAM3 Clock Enable During CSleep Mode",
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
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "RCC AHB2 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmirst",
                    description: Some(
                        "DCMI block reset",
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
                    name: "cryprst",
                    description: Some(
                        "CRYPography block reset",
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
                    name: "hashrst",
                    description: Some(
                        "Hash block reset",
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
                        "Random Number Generator block reset",
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
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 and SDMMC2 Delay block reset",
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
                    name: "bdma1rst",
                    description: Some(
                        "BDMA1 block reset",
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
                    name: "fmacrst",
                    description: Some(
                        "FMAC reset",
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
                    name: "cordicrst",
                    description: Some(
                        "CORDIC reset",
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
            name: "Ahb3enr",
            extends: None,
            description: Some(
                "RCC AHB3 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdmaen",
                    description: Some(
                        "MDMA Peripheral Clock Enable",
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
                    name: "dma2den",
                    description: Some(
                        "DMA2D Peripheral Clock Enable",
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
                    name: "jpgdecen",
                    description: Some(
                        "JPGDEC Peripheral Clock Enable",
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
                    name: "fmcen",
                    description: Some(
                        "FMC Peripheral Clocks Enable",
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
                    name: "octospi1en",
                    description: Some(
                        "OCTOSPI1 and OCTOSPI1 Delay Clock Enable",
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
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 and SDMMC1 Delay Clock Enable",
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
                    name: "octospi2en",
                    description: Some(
                        "OCTOSPI2 and OCTOSPI2 delay block enable",
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
                    name: "iomngren",
                    description: Some(
                        "OCTOSPI IO manager enable",
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
                    name: "otfd1en",
                    description: Some(
                        "OTFDEC1 enable",
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
                    name: "otfd2en",
                    description: Some(
                        "OTFDEC2 enable",
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
                    name: "dtcm1en",
                    description: Some(
                        "D1 DTCM1 block enable",
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
                    name: "dtcm2en",
                    description: Some(
                        "D1 DTCM2 block enable",
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
                    name: "itcm1en",
                    description: Some(
                        "D1 ITCM block enable",
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
                    name: "axisramen",
                    description: Some(
                        "AXISRAM block enable",
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
            name: "Ahb3lpenr",
            extends: None,
            description: Some(
                "RCC AHB3 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdmalpen",
                    description: Some(
                        "MDMA Clock Enable During CSleep Mode",
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
                    name: "dma2dlpen",
                    description: Some(
                        "DMA2D Clock Enable During CSleep Mode",
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
                    name: "jpgdeclpen",
                    description: Some(
                        "JPGDEC Clock Enable During CSleep Mode",
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
                    name: "flashlpen",
                    description: Some(
                        "FLASH Clock Enable During CSleep Mode",
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
                    name: "fmclpen",
                    description: Some(
                        "FMC Peripheral Clocks Enable During CSleep Mode",
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
                    name: "octospi1lpen",
                    description: Some(
                        "OCTOSPI1 and OCTOSPI1 Delay Clock Enable During CSleep Mode",
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
                    name: "sdmmc1lpen",
                    description: Some(
                        "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode",
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
                    name: "octospi2lpen",
                    description: Some(
                        "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode",
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
                    name: "iomngrlpen",
                    description: Some(
                        "OCTOSPI IO manager enable during CSleep Mode",
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
                    name: "otfd1lpen",
                    description: Some(
                        "OTFDEC1 enable during CSleep Mode",
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
                    name: "otfd2lpen",
                    description: Some(
                        "OTFDEC2 enable during CSleep Mode",
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
                    name: "d1dtcm1lpen",
                    description: Some(
                        "D1DTCM1 Block Clock Enable During CSleep mode",
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
                    name: "dtcm2lpen",
                    description: Some(
                        "D1 DTCM2 Block Clock Enable During CSleep mode",
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
                    name: "itcmlpen",
                    description: Some(
                        "D1ITCM Block Clock Enable During CSleep mode",
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
                    name: "axisramlpen",
                    description: Some(
                        "AXISRAM Block Clock Enable During CSleep mode",
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
            name: "Ahb3rstr",
            extends: None,
            description: Some(
                "RCC AHB3 Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdmarst",
                    description: Some(
                        "MDMA block reset",
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
                    name: "dma2drst",
                    description: Some(
                        "DMA2D block reset",
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
                    name: "jpgdecrst",
                    description: Some(
                        "JPGDEC block reset",
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
                    name: "fmcrst",
                    description: Some(
                        "FMC block reset",
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
                    name: "octospi1rst",
                    description: Some(
                        "OCTOSPI1 and OCTOSPI1 delay block reset",
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
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 and SDMMC1 delay block reset",
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
                    name: "octospi2rst",
                    description: Some(
                        "OCTOSPI2 and OCTOSPI2 delay block reset",
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
                    name: "iomngrrst",
                    description: Some(
                        "OCTOSPI IO manager reset",
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
                    name: "otfd1rst",
                    description: Some(
                        "OTFDEC1 reset",
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
                    name: "otfd2rst",
                    description: Some(
                        "OTFDEC2 reset",
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
                    name: "cpurst",
                    description: Some(
                        "CPU reset",
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
            name: "Ahb4enr",
            extends: None,
            description: Some(
                "RCC AHB4 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "0GPIO peripheral clock enable",
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
                        "CRC peripheral clock enable",
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
                    name: "bdma2en",
                    description: Some(
                        "BDMA2 and DMAMUX2 Clock Enable",
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
                    name: "bdmaen",
                    description: Some(
                        "BDMA and DMAMUX2 Clock Enable",
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
                    name: "adc3en",
                    description: Some(
                        "ADC3 Peripheral Clocks Enable",
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
                    name: "hsemen",
                    description: Some(
                        "HSEM peripheral clock enable",
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
                    name: "bkpsramen",
                    description: Some(
                        "Backup RAM Clock Enable",
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
            ],
        },
        FieldSet {
            name: "Ahb4lpenr",
            extends: None,
            description: Some(
                "RCC AHB4 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "GPIO peripheral clock enable during CSleep mode",
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
                        "CRC peripheral clock enable during CSleep mode",
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
                    name: "bdma2lpen",
                    description: Some(
                        "BDMA2 Clock Enable During CSleep Mode",
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
                    name: "bdmalpen",
                    description: Some(
                        "BDMA Clock Enable During CSleep Mode",
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
                    name: "adc3lpen",
                    description: Some(
                        "ADC3 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "bkpsramlpen",
                    description: Some(
                        "Backup RAM Clock Enable During CSleep Mode",
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
                    name: "sram4lpen",
                    description: Some(
                        "SRAM4 Clock Enable During CSleep Mode",
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
            name: "Ahb4rstr",
            extends: None,
            description: Some(
                "RCC AHB4 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "GPIO block reset",
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
                        "CRC block reset",
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
                    name: "bdma2rst",
                    description: Some(
                        "BDMA2 block reset",
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
                    name: "bdmarst",
                    description: Some(
                        "BDMA block reset",
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
                    name: "adc3rst",
                    description: Some(
                        "ADC3 block reset",
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
                    name: "hsemrst",
                    description: Some(
                        "HSEM block reset",
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
            ],
        },
        FieldSet {
            name: "Apb1henr",
            extends: None,
            description: Some(
                "RCC APB1 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crsen",
                    description: Some(
                        "Clock Recovery System peripheral clock enable",
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
                    name: "swpmien",
                    description: Some(
                        "SWPMI Peripheral Clocks Enable",
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
                    name: "opampen",
                    description: Some(
                        "OPAMP peripheral clock enable",
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
                    name: "mdiosen",
                    description: Some(
                        "MDIOS peripheral clock enable",
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
                    name: "fdcanen",
                    description: Some(
                        "FDCAN Peripheral Clocks Enable",
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
                    name: "tim23en",
                    description: Some(
                        "TIM23 block enable",
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
                    name: "tim24en",
                    description: Some(
                        "TIM24 block enable",
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
            ],
        },
        FieldSet {
            name: "Apb1hlpenr",
            extends: None,
            description: Some(
                "RCC APB1 High Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crslpen",
                    description: Some(
                        "Clock Recovery System peripheral clock enable during CSleep mode",
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
                    name: "swpmilpen",
                    description: Some(
                        "SWPMI Peripheral Clocks Enable During CSleep Mode",
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
                    name: "opamplpen",
                    description: Some(
                        "OPAMP peripheral clock enable during CSleep mode",
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
                    name: "mdioslpen",
                    description: Some(
                        "MDIOS peripheral clock enable during CSleep mode",
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
                    name: "fdcanlpen",
                    description: Some(
                        "FDCAN Peripheral Clocks Enable During CSleep Mode",
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
                    name: "tim23lpen",
                    description: Some(
                        "TIM23 block enable during CSleep Mode",
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
                    name: "tim24lpen",
                    description: Some(
                        "TIM24 block enable during CSleep Mode",
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
            ],
        },
        FieldSet {
            name: "Apb1hrstr",
            extends: None,
            description: Some(
                "RCC APB1 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crsrst",
                    description: Some(
                        "Clock Recovery System reset",
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
                    name: "swpmirst",
                    description: Some(
                        "SWPMI block reset",
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
                    name: "opamprst",
                    description: Some(
                        "OPAMP block reset",
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
                    name: "mdiosrst",
                    description: Some(
                        "MDIOS block reset",
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
                    name: "fdcanrst",
                    description: Some(
                        "FDCAN block reset",
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
                    name: "tim23rst",
                    description: Some(
                        "TIM23 block reset",
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
                    name: "tim24rst",
                    description: Some(
                        "TIM24 block reset",
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
            ],
        },
        FieldSet {
            name: "Apb1lenr",
            extends: None,
            description: Some(
                "RCC APB1 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "TIM peripheral clock enable",
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
                        "LPTIM1 Peripheral Clocks Enable",
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
                    name: "wwdg2en",
                    description: Some(
                        "WWDG2 peripheral clock enable",
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
                        "SPI2 Peripheral Clocks Enable",
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
                        "SPI3 Peripheral Clocks Enable",
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
                        "SPDIFRX Peripheral Clocks Enable",
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
                        "USART2 Peripheral Clocks Enable",
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
                        "USART3 Peripheral Clocks Enable",
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
                        "UART4 Peripheral Clocks Enable",
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
                        "UART5 Peripheral Clocks Enable",
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
                        "I2C1 Peripheral Clocks Enable",
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
                        "I2C2 Peripheral Clocks Enable",
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
                        "I2C3 Peripheral Clocks Enable",
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
                    name: "i2c5en",
                    description: Some(
                        "I2C5 Peripheral Clocks\r Enable",
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
                    name: "cecen",
                    description: Some(
                        "HDMI-CEC peripheral clock enable",
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
                    name: "dac1en",
                    description: Some(
                        "DAC1 (containing two converters) peripheral clock enable",
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
                        "UART7 Peripheral Clocks Enable",
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
                        "UART8 Peripheral Clocks Enable",
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
            name: "Apb1llpenr",
            extends: None,
            description: Some(
                "RCC APB1 Low Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 peripheral clock enable during CSleep mode",
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
                        "TIM3 peripheral clock enable during CSleep mode",
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
                        "TIM4 peripheral clock enable during CSleep mode",
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
                        "TIM5 peripheral clock enable during CSleep mode",
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
                        "TIM6 peripheral clock enable during CSleep mode",
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
                        "TIM7 peripheral clock enable during CSleep mode",
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
                        "TIM12 peripheral clock enable during CSleep mode",
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
                        "TIM13 peripheral clock enable during CSleep mode",
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
                        "TIM14 peripheral clock enable during CSleep mode",
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
                        "LPTIM1 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "wwdg2lpen",
                    description: Some(
                        "WWDG2 peripheral Clocks Enable During CSleep Mode",
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
                    name: "spi2lpen",
                    description: Some(
                        "SPI2 Peripheral Clocks Enable During CSleep Mode",
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
                        "SPI3 Peripheral Clocks Enable During CSleep Mode",
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
                        "SPDIFRX Peripheral Clocks Enable During CSleep Mode",
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
                        "USART2 Peripheral Clocks Enable During CSleep Mode",
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
                        "USART3 Peripheral Clocks Enable During CSleep Mode",
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
                        "UART4 Peripheral Clocks Enable During CSleep Mode",
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
                        "UART5 Peripheral Clocks Enable During CSleep Mode",
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
                        "I2C1 Peripheral Clocks Enable During CSleep Mode",
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
                        "I2C2 Peripheral Clocks Enable During CSleep Mode",
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
                        "I2C3 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "i2c5lpen",
                    description: Some(
                        "I2C5 block enable during CSleep Mode",
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
                    name: "ceclpen",
                    description: Some(
                        "HDMI-CEC Peripheral Clocks Enable During CSleep Mode",
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
                    name: "dac1lpen",
                    description: Some(
                        "DAC1 (containing two converters) peripheral clock enable during CSleep mode",
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
                        "UART7 Peripheral Clocks Enable During CSleep Mode",
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
                        "UART8 Peripheral Clocks Enable During CSleep Mode",
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
            name: "Apb1lrstr",
            extends: None,
            description: Some(
                "RCC APB1 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "TIM block reset",
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
                        "SPI2 block reset",
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
                        "SPI3 block reset",
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
                        "SPDIFRX block reset",
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
                        "USART2 block reset",
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
                        "USART3 block reset",
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
                        "UART4 block reset",
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
                        "UART5 block reset",
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
                        "I2C1 block reset",
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
                        "I2C2 block reset",
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
                        "I2C3 block reset",
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
                    name: "i2c5rst",
                    description: Some(
                        "I2C5 block reset",
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
                    name: "cecrst",
                    description: Some(
                        "HDMI-CEC block reset",
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
                    name: "dac1rst",
                    description: Some(
                        "DAC1 (containing two converters) reset",
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
                        "UART7 block reset",
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
                        "UART8 block reset",
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
                "RCC APB2 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 peripheral clock enable",
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
                        "TIM8 peripheral clock enable",
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
                        "USART1 Peripheral Clocks Enable",
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
                        "USART6 Peripheral Clocks Enable",
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
                    name: "uart9en",
                    description: Some(
                        "UART9 Peripheral Clocks\r Enable",
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
                    name: "usart10en",
                    description: Some(
                        "USART10 Peripheral Clocks\r Enable",
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
                    name: "spi1en",
                    description: Some(
                        "SPI1 Peripheral Clocks Enable",
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
                        "SPI4 Peripheral Clocks Enable",
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
                    name: "tim15en",
                    description: Some(
                        "TIM15 peripheral clock enable",
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
                        "TIM16 peripheral clock enable",
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
                        "TIM17 peripheral clock enable",
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
                        "SPI5 Peripheral Clocks Enable",
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
                    name: "sai1en",
                    description: Some(
                        "SAI1 Peripheral Clocks Enable",
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
                        "SAI2 Peripheral Clocks Enable",
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
                    name: "sai3en",
                    description: Some(
                        "SAI3 Peripheral Clocks Enable",
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
                    name: "dfsdm1en",
                    description: Some(
                        "DFSDM1 Peripheral Clocks Enable",
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
                    name: "hrtimen",
                    description: Some(
                        "HRTIM peripheral clock enable",
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
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "RCC APB2 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 peripheral clock enable during CSleep mode",
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
                        "TIM8 peripheral clock enable during CSleep mode",
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
                        "USART1 Peripheral Clocks Enable During CSleep Mode",
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
                        "USART6 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "spi1lpen",
                    description: Some(
                        "SPI1 Peripheral Clocks Enable During CSleep Mode",
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
                        "SPI4 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "tim15lpen",
                    description: Some(
                        "TIM15 peripheral clock enable during CSleep mode",
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
                    name: "tim16lpen",
                    description: Some(
                        "TIM16 peripheral clock enable during CSleep mode",
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
                    name: "tim17lpen",
                    description: Some(
                        "TIM17 peripheral clock enable during CSleep mode",
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
                        "SPI5 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "sai1lpen",
                    description: Some(
                        "SAI1 Peripheral Clocks Enable During CSleep Mode",
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
                        "SAI2 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "sai3lpen",
                    description: Some(
                        "SAI3 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "dfsdm1lpen",
                    description: Some(
                        "DFSDM1 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "hrtimlpen",
                    description: Some(
                        "HRTIM peripheral clock enable during CSleep mode",
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
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "RCC APB2 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 block reset",
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
                        "TIM8 block reset",
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
                        "USART1 block reset",
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
                        "USART6 block reset",
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
                    name: "uart9rst",
                    description: Some(
                        "UART9 block reset",
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
                    name: "usart10rst",
                    description: Some(
                        "USART10 block reset",
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
                    name: "spi1rst",
                    description: Some(
                        "SPI1 block reset",
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
                        "SPI4 block reset",
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
                    name: "tim15rst",
                    description: Some(
                        "TIM15 block reset",
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
                        "TIM16 block reset",
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
                        "TIM17 block reset",
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
                        "SPI5 block reset",
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
                    name: "sai1rst",
                    description: Some(
                        "SAI1 block reset",
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
                        "SAI2 block reset",
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
                    name: "sai3rst",
                    description: Some(
                        "SAI3 block reset",
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
                    name: "dfsdm1rst",
                    description: Some(
                        "DFSDM1 block reset",
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
                    name: "hrtimrst",
                    description: Some(
                        "HRTIM block reset",
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
            name: "Apb3enr",
            extends: None,
            description: Some(
                "RCC APB3 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LTDC peripheral clock enable",
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
                    name: "dsien",
                    description: Some(
                        "DSI Peripheral clocks enable",
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
                    name: "wwdg1en",
                    description: Some(
                        "WWDG1 Clock Enable",
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
            ],
        },
        FieldSet {
            name: "Apb3lpenr",
            extends: None,
            description: Some(
                "RCC APB3 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdclpen",
                    description: Some(
                        "LTDC peripheral clock enable during CSleep mode",
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
                    name: "dsilpen",
                    description: Some(
                        "DSI Peripheral Clock Enable During CSleep Mode",
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
                    name: "wwdg1lpen",
                    description: Some(
                        "WWDG1 Clock Enable During CSleep Mode",
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
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "RCC APB3 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcrst",
                    description: Some(
                        "LTDC block reset",
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
                    name: "dsirst",
                    description: Some(
                        "DSI block reset",
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
            ],
        },
        FieldSet {
            name: "Apb4enr",
            extends: None,
            description: Some(
                "RCC APB4 Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG peripheral clock enable",
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
                        "LPUART1 Peripheral Clocks Enable",
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
                    name: "spi6en",
                    description: Some(
                        "SPI6 Peripheral Clocks Enable",
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
                    name: "i2c4en",
                    description: Some(
                        "I2C4 Peripheral Clocks Enable",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 Peripheral Clocks Enable",
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
                    name: "lptim3en",
                    description: Some(
                        "LPTIM3 Peripheral Clocks Enable",
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
                    name: "lptim4en",
                    description: Some(
                        "LPTIM4 Peripheral Clocks Enable",
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
                    name: "lptim5en",
                    description: Some(
                        "LPTIM5 Peripheral Clocks Enable",
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
                    name: "dac2en",
                    description: Some(
                        "DAC2 (containing one converter) peripheral clock enable",
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
                    name: "comp12en",
                    description: Some(
                        "COMP1/2 peripheral clock enable",
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
                    name: "vrefen",
                    description: Some(
                        "VREF peripheral clock enable",
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
                    name: "rtcapben",
                    description: Some(
                        "RTC APB Clock Enable",
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
                    name: "sai4en",
                    description: Some(
                        "SAI4 Peripheral Clocks Enable",
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
                    name: "dtsen",
                    description: Some(
                        "Digital temperature sensor block enable",
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
            ],
        },
        FieldSet {
            name: "Apb4lpenr",
            extends: None,
            description: Some(
                "RCC APB4 Sleep Clock Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfglpen",
                    description: Some(
                        "SYSCFG peripheral clock enable during CSleep mode",
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
                        "LPUART1 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "spi6lpen",
                    description: Some(
                        "SPI6 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "i2c4lpen",
                    description: Some(
                        "I2C4 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "lptim2lpen",
                    description: Some(
                        "LPTIM2 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "lptim3lpen",
                    description: Some(
                        "LPTIM3 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "lptim4lpen",
                    description: Some(
                        "LPTIM4 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "lptim5lpen",
                    description: Some(
                        "LPTIM5 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "dac2lpen",
                    description: Some(
                        "DAC2 (containing one converter) peripheral clock enable during CSleep mode",
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
                    name: "comp12lpen",
                    description: Some(
                        "COMP1/2 peripheral clock enable during CSleep mode",
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
                    name: "vreflpen",
                    description: Some(
                        "VREF peripheral clock enable during CSleep mode",
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
                    name: "rtcapblpen",
                    description: Some(
                        "RTC APB Clock Enable During CSleep Mode",
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
                    name: "sai4lpen",
                    description: Some(
                        "SAI4 Peripheral Clocks Enable During CSleep Mode",
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
                    name: "dtslpen",
                    description: Some(
                        "Digital temperature sensor block enable during CSleep Mode",
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
            ],
        },
        FieldSet {
            name: "Apb4rstr",
            extends: None,
            description: Some(
                "RCC APB4 Peripheral Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SYSCFG block reset",
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
                        "LPUART1 block reset",
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
                    name: "spi6rst",
                    description: Some(
                        "SPI6 block reset",
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
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 block reset",
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
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 block reset",
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
                    name: "lptim3rst",
                    description: Some(
                        "LPTIM3 block reset",
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
                    name: "lptim4rst",
                    description: Some(
                        "LPTIM4 block reset",
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
                    name: "lptim5rst",
                    description: Some(
                        "LPTIM5 block reset",
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
                    name: "dac2rst",
                    description: Some(
                        "DAC2 (containing one converter) reset",
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
                    name: "comp12rst",
                    description: Some(
                        "COMP12 Blocks Reset",
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
                    name: "vrefrst",
                    description: Some(
                        "VREF block reset",
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
                    name: "sai4rst",
                    description: Some(
                        "SAI4 block reset",
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
                    name: "dtsrst",
                    description: Some(
                        "Digital temperature sensor block reset",
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
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup Domain Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enabled",
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
                        "LSE oscillator driving capability",
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
                        "LSE clock security system enable",
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
                        "LSE clock security system failure detection",
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
                        "VSwitch domain software reset",
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
                "RCC Clock Configuration Register",
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
                    bit_size: 3,
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
                        "System clock selection after a wake up from system Stop",
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
                        "Kernel clock selection after a wake up from system Stop",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock",
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
                    name: "hrtimsel",
                    description: Some(
                        "High Resolution Timer clock prescaler selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hrtimsel",
                    ),
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "Timers clocks prescaler selection",
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
                        "MCO1 prescaler",
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
                        "Micro-controller clock output 1",
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
                        "MCO2 prescaler",
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
                        "Micro-controller clock output 2",
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
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC Clock Source Interrupt Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready Interrupt Clear",
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
                        "LSE ready Interrupt Clear",
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
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready Interrupt Clear",
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
                    name: "hserdyc",
                    description: Some(
                        "HSE ready Interrupt Clear",
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
                    name: "hse_ready_interrupt_clear",
                    description: Some(
                        "CSI ready Interrupt Clear",
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
                        "RC48 ready Interrupt Clear",
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
                        "PLL1 ready Interrupt Clear",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssc",
                    description: Some(
                        "LSE clock security system Interrupt Clear",
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
                    name: "hsecssc",
                    description: Some(
                        "HSE clock security system Interrupt Clear",
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
                "RCC Clock Source Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready Interrupt Enable",
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
                        "LSE ready Interrupt Enable",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready Interrupt Enable",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE ready Interrupt Enable",
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
                    name: "csirdyie",
                    description: Some(
                        "CSI ready Interrupt Enable",
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
                        "RC48 ready Interrupt Enable",
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
                        "PLL1 ready Interrupt Enable",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssie",
                    description: Some(
                        "LSE clock security system Interrupt Enable",
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
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC Clock Source Interrupt Flag Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready Interrupt Flag",
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
                        "LSE ready Interrupt Flag",
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
                        "HSI ready Interrupt Flag",
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
                        "HSE ready Interrupt Flag",
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
                    name: "csirdyf",
                    description: Some(
                        "CSI ready Interrupt Flag",
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
                        "RC48 ready Interrupt Flag",
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
                        "PLL1 ready Interrupt Flag",
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
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lsecssf",
                    description: Some(
                        "LSE clock security system Interrupt Flag",
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
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system Interrupt Flag",
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
                    name: "hsikeron",
                    description: Some(
                        "High Speed Internal clock enable in Stop mode",
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
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag",
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
                        "HSI clock divider",
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
                        "HSI divider flag",
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
                        "CSI clock enable",
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
                    name: "csirdy",
                    description: Some(
                        "CSI clock ready flag",
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
                    name: "csikeron",
                    description: Some(
                        "CSI clock enable in Stop mode",
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
                    name: "hsi48on",
                    description: Some(
                        "RC48 clock enable",
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
                        "RC48 clock ready flag",
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
                    name: "d1ckrdy",
                    description: Some(
                        "D1 domain clocks ready flag",
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
                    name: "d2ckrdy",
                    description: Some(
                        "D2 domain clocks ready flag",
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
                    name: "hsecsson",
                    description: Some(
                        "HSE Clock Security System enable",
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
                        "PLL1 enable",
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
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag",
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
                                len: 3,
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
                "RCC Clock Recovery RC Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "Internal RC 48 MHz clock calibration",
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
                "RCC CSI configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csical",
                    description: Some(
                        "CSI clock calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csitrim",
                    description: Some(
                        "CSI clock trimming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "RCC Clock Control and Status Register",
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
            ],
        },
        FieldSet {
            name: "D1ccipr",
            extends: None,
            description: Some(
                "RCC Domain 1 Kernel Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcsel",
                    description: Some(
                        "FMC kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fmcsel",
                    ),
                },
                Field {
                    name: "octospisel",
                    description: Some(
                        "OCTOSPI kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fmcsel",
                    ),
                },
                Field {
                    name: "sdmmcsel",
                    description: Some(
                        "SDMMC kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "persel",
                    description: Some(
                        "per_ck clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
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
            name: "D1cfgr",
            extends: None,
            description: Some(
                "RCC Domain 1 Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "D1 domain AHB prescaler",
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
                    name: "d1ppre",
                    description: Some(
                        "D1 domain APB3 prescaler",
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
                    name: "d1cpre",
                    description: Some(
                        "D1 domain Core prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "D2ccip1r",
            extends: None,
            description: Some(
                "RCC Domain 2 Kernel Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 and DFSDM1 kernel Aclk clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "sai2asel",
                    description: Some(
                        "SAI2 kernel clock source A source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saiasel",
                    ),
                },
                Field {
                    name: "sai2bsel",
                    description: Some(
                        "SAI2 kernel clock source B source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saiasel",
                    ),
                },
                Field {
                    name: "spi123sel",
                    description: Some(
                        "SPI/I2S1,2 and 3 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "spi45sel",
                    description: Some(
                        "SPI4 and 5 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi45sel",
                    ),
                },
                Field {
                    name: "spdifrxsel",
                    description: Some(
                        "SPDIFRX kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spdifrxsel",
                    ),
                },
                Field {
                    name: "dfsdm1sel",
                    description: Some(
                        "DFSDM1 kernel Clk clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dfsdmsel",
                    ),
                },
                Field {
                    name: "fdcansel",
                    description: Some(
                        "FDCAN kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "swpmisel",
                    description: Some(
                        "SWPMI kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Swpmisel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "D2ccip2r",
            extends: None,
            description: Some(
                "RCC Domain 2 Kernel Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart234578sel",
                    description: Some(
                        "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usart234578sel",
                    ),
                },
                Field {
                    name: "usart16910sel",
                    description: Some(
                        "USART1, 6, 9 and 10 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usart16910sel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNG kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "i2c1235sel",
                    description: Some(
                        "I2C1,2,3 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1235sel",
                    ),
                },
                Field {
                    name: "usbsel",
                    description: Some(
                        "USBOTG 1 and 2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbsel",
                    ),
                },
                Field {
                    name: "cecsel",
                    description: Some(
                        "HDMI-CEC kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cecsel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "D2cfgr",
            extends: None,
            description: Some(
                "RCC Domain 2 Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "d2ppre1",
                    description: Some(
                        "D2 domain APB1 prescaler",
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
                    name: "d2ppre2",
                    description: Some(
                        "D2 domain APB2 prescaler",
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
            ],
        },
        FieldSet {
            name: "D3amr",
            extends: None,
            description: Some(
                "RCC D3 Autonomous mode Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bdma2amen",
                    description: Some(
                        "BDMA2 and DMAMUX Autonomous mode enable",
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
                    name: "bdmaamen",
                    description: Some(
                        "BDMA and DMAMUX Autonomous mode enable",
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
                    name: "lpuart1amen",
                    description: Some(
                        "LPUART1 Autonomous mode enable",
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
                    name: "spi6amen",
                    description: Some(
                        "SPI6 Autonomous mode enable",
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
                    name: "i2c4amen",
                    description: Some(
                        "I2C4 Autonomous mode enable",
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
                    name: "lptim2amen",
                    description: Some(
                        "LPTIM2 Autonomous mode enable",
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
                    name: "lptim3amen",
                    description: Some(
                        "LPTIM3 Autonomous mode enable",
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
                    name: "lptim4amen",
                    description: Some(
                        "LPTIM4 Autonomous mode enable",
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
                    name: "lptim5amen",
                    description: Some(
                        "LPTIM5 Autonomous mode enable",
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
                    name: "dac2amen",
                    description: Some(
                        "DAC2 (containing one converter) Autonomous mode enable",
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
                    name: "comp12amen",
                    description: Some(
                        "COMP12 Autonomous mode enable",
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
                    name: "vrefamen",
                    description: Some(
                        "VREF Autonomous mode enable",
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
                    name: "rtcamen",
                    description: Some(
                        "RTC Autonomous mode enable",
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
                    name: "crcamen",
                    description: Some(
                        "CRC Autonomous mode enable",
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
                    name: "sai4amen",
                    description: Some(
                        "SAI4 Autonomous mode enable",
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
                    name: "adc3amen",
                    description: Some(
                        "ADC3 Autonomous mode enable",
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
                    name: "dtsamen",
                    description: Some(
                        "Digital temperature sensor Autonomous mode enable",
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
                    name: "bkpsramamen",
                    description: Some(
                        "Backup RAM Autonomous mode enable",
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
                    name: "sram4amen",
                    description: Some(
                        "SRAM4 Autonomous mode enable",
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
            name: "D3ccipr",
            extends: None,
            description: Some(
                "RCC Domain 3 Kernel Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c4sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "LPTIM2 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "lptim345sel",
                    description: Some(
                        "LPTIM3,4,5 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "SAR ADC kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
                Field {
                    name: "dfsdm2sel",
                    description: Some(
                        "DFSDM2 kernel clock source selection",
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
                    name: "spi6sel",
                    description: Some(
                        "SPI6 kernel clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spi6sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "D3cfgr",
            extends: None,
            description: Some(
                "RCC Domain 3 Clock Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "d3ppre",
                    description: Some(
                        "D3 domain APB4 prescaler",
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
            ],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some(
                "Global Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ww1rsc",
                    description: Some(
                        "WWDG1 reset scope control",
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
            ],
        },
        FieldSet {
            name: "Hsicfgr",
            extends: None,
            description: Some(
                "RCC HSI configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration",
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
                "RCC PLLs Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllvcosel",
                    description: Some(
                        "PLL1 VCO selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllvcosel",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "divpen",
                    description: Some(
                        "PLL1 DIVP divider output enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "divren",
                    description: Some(
                        "PLL1 DIVR divider output enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllckselr",
            extends: None,
            description: Some(
                "RCC PLLs Clock Source Selection Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "DIVMx and PLLs clock source selection",
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
                    name: "divm",
                    description: Some(
                        "Prescaler for PLL1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 6,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pllm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Plldivr",
            extends: None,
            description: Some(
                "RCC PLL1 Dividers Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1 VCO",
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
                        "PLL DIVP division factor",
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
                        "PLL DIVQ division factor",
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
                        "PLL DIVR division factor",
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
                "RCC PLL Fractional Divider Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL VCO",
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
                "RCC Reset Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
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
                    name: "cpurstf",
                    description: Some(
                        "CPU reset flag",
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
                    name: "d1rstf",
                    description: Some(
                        "D1 domain power switch reset flag",
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
                    name: "d2rstf",
                    description: Some(
                        "D2 domain power switch reset flag",
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
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag",
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
                    name: "pinrstf",
                    description: Some(
                        "Pin reset flag (NRST)",
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
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag",
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
                    name: "sftrstf",
                    description: Some(
                        "System reset from CPU reset flag",
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
                    name: "iwdg1rstf",
                    description: Some(
                        "Independent Watchdog reset flag",
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
                    name: "wwdg1rstf",
                    description: Some(
                        "Window Watchdog reset flag",
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
                    name: "lpwrrstf",
                    description: Some(
                        "Reset due to illegal D1 DStandby or CPU CStop flag",
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
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Cecsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 2,
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
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected as peripheral clock",
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
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Fmcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK3",
                    description: Some(
                        "rcc_hclk3 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 3,
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
                        "sys_ck not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "sys_ck divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "sys_ck divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "sys_ck divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "sys_ck divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "sys_ck divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "sys_ck divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "sys_ck divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "sys_ck divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Hrtimsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMY_KER",
                    description: Some(
                        "The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "C_CK",
                    description: Some(
                        "The HRTIM prescaler clock source is the CPU clock (c_ck)",
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
            ],
        },
        Enum {
            name: "I2c1235sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2c4sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lptim2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk_d4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
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
            name: "Mco1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected for micro-controller clock output",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1_p selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected for micro-controller clock output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected for micro-controller clock output",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Divide by 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Divide by 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Divide by 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Divide by 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Divide by 5",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "Divide by 6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "Divide by 7",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Divide by 8",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "Divide by 9",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "Divide by 10",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "Divide by 11",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "Divide by 12",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "Divide by 13",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "Divide by 14",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "Divide by 15",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Persel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Plldiv",
            description: None,
            bit_size: 7,
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
                EnumVariant {
                    name: "DIV9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "DIV33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "DIV34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "DIV35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "DIV36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "DIV37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "DIV38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "DIV39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "DIV40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "DIV41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "DIV42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "DIV43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "DIV44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "DIV45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "DIV46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "DIV47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "DIV48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "DIV49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "DIV50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "DIV51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "DIV52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "DIV53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "DIV54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "DIV55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "DIV56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "DIV57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "DIV58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "DIV59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "DIV60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "DIV61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "DIV62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "DIV63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "DIV64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "DIV65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "DIV66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "DIV67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "DIV68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "DIV69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "DIV70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "DIV71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "DIV72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "DIV73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "DIV74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "DIV75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "DIV76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "DIV77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "DIV78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "DIV79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "DIV80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "DIV81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "DIV82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "DIV83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "DIV84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "DIV85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "DIV86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "DIV87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "DIV88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "DIV89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "DIV90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "DIV91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "DIV92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "DIV93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "DIV94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "DIV95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "DIV96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "DIV97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "DIV98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "DIV99",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "DIV100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "DIV101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "DIV102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "DIV103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "DIV104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "DIV105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "DIV106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "DIV107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "DIV108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "DIV109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "DIV110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "DIV111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "DIV112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "DIV113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "DIV114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "DIV115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "DIV116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "DIV117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "DIV118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "DIV119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "DIV120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "DIV121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "DIV122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "DIV123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "DIV124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "DIV125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "DIV126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "DIV127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 127,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 1,
                },
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
            ],
        },
        Enum {
            name: "Plln",
            description: None,
            bit_size: 9,
            variants: &[
                EnumVariant {
                    name: "MUL4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "MUL5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "MUL6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "MUL7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "MUL8",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "MUL9",
                    description: None,
                    value: 8,
                },
                EnumVariant {
                    name: "MUL10",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "MUL11",
                    description: None,
                    value: 10,
                },
                EnumVariant {
                    name: "MUL12",
                    description: None,
                    value: 11,
                },
                EnumVariant {
                    name: "MUL13",
                    description: None,
                    value: 12,
                },
                EnumVariant {
                    name: "MUL14",
                    description: None,
                    value: 13,
                },
                EnumVariant {
                    name: "MUL15",
                    description: None,
                    value: 14,
                },
                EnumVariant {
                    name: "MUL16",
                    description: None,
                    value: 15,
                },
                EnumVariant {
                    name: "MUL17",
                    description: None,
                    value: 16,
                },
                EnumVariant {
                    name: "MUL18",
                    description: None,
                    value: 17,
                },
                EnumVariant {
                    name: "MUL19",
                    description: None,
                    value: 18,
                },
                EnumVariant {
                    name: "MUL20",
                    description: None,
                    value: 19,
                },
                EnumVariant {
                    name: "MUL21",
                    description: None,
                    value: 20,
                },
                EnumVariant {
                    name: "MUL22",
                    description: None,
                    value: 21,
                },
                EnumVariant {
                    name: "MUL23",
                    description: None,
                    value: 22,
                },
                EnumVariant {
                    name: "MUL24",
                    description: None,
                    value: 23,
                },
                EnumVariant {
                    name: "MUL25",
                    description: None,
                    value: 24,
                },
                EnumVariant {
                    name: "MUL26",
                    description: None,
                    value: 25,
                },
                EnumVariant {
                    name: "MUL27",
                    description: None,
                    value: 26,
                },
                EnumVariant {
                    name: "MUL28",
                    description: None,
                    value: 27,
                },
                EnumVariant {
                    name: "MUL29",
                    description: None,
                    value: 28,
                },
                EnumVariant {
                    name: "MUL30",
                    description: None,
                    value: 29,
                },
                EnumVariant {
                    name: "MUL31",
                    description: None,
                    value: 30,
                },
                EnumVariant {
                    name: "MUL32",
                    description: None,
                    value: 31,
                },
                EnumVariant {
                    name: "MUL33",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "MUL34",
                    description: None,
                    value: 33,
                },
                EnumVariant {
                    name: "MUL35",
                    description: None,
                    value: 34,
                },
                EnumVariant {
                    name: "MUL36",
                    description: None,
                    value: 35,
                },
                EnumVariant {
                    name: "MUL37",
                    description: None,
                    value: 36,
                },
                EnumVariant {
                    name: "MUL38",
                    description: None,
                    value: 37,
                },
                EnumVariant {
                    name: "MUL39",
                    description: None,
                    value: 38,
                },
                EnumVariant {
                    name: "MUL40",
                    description: None,
                    value: 39,
                },
                EnumVariant {
                    name: "MUL41",
                    description: None,
                    value: 40,
                },
                EnumVariant {
                    name: "MUL42",
                    description: None,
                    value: 41,
                },
                EnumVariant {
                    name: "MUL43",
                    description: None,
                    value: 42,
                },
                EnumVariant {
                    name: "MUL44",
                    description: None,
                    value: 43,
                },
                EnumVariant {
                    name: "MUL45",
                    description: None,
                    value: 44,
                },
                EnumVariant {
                    name: "MUL46",
                    description: None,
                    value: 45,
                },
                EnumVariant {
                    name: "MUL47",
                    description: None,
                    value: 46,
                },
                EnumVariant {
                    name: "MUL48",
                    description: None,
                    value: 47,
                },
                EnumVariant {
                    name: "MUL49",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "MUL50",
                    description: None,
                    value: 49,
                },
                EnumVariant {
                    name: "MUL51",
                    description: None,
                    value: 50,
                },
                EnumVariant {
                    name: "MUL52",
                    description: None,
                    value: 51,
                },
                EnumVariant {
                    name: "MUL53",
                    description: None,
                    value: 52,
                },
                EnumVariant {
                    name: "MUL54",
                    description: None,
                    value: 53,
                },
                EnumVariant {
                    name: "MUL55",
                    description: None,
                    value: 54,
                },
                EnumVariant {
                    name: "MUL56",
                    description: None,
                    value: 55,
                },
                EnumVariant {
                    name: "MUL57",
                    description: None,
                    value: 56,
                },
                EnumVariant {
                    name: "MUL58",
                    description: None,
                    value: 57,
                },
                EnumVariant {
                    name: "MUL59",
                    description: None,
                    value: 58,
                },
                EnumVariant {
                    name: "MUL60",
                    description: None,
                    value: 59,
                },
                EnumVariant {
                    name: "MUL61",
                    description: None,
                    value: 60,
                },
                EnumVariant {
                    name: "MUL62",
                    description: None,
                    value: 61,
                },
                EnumVariant {
                    name: "MUL63",
                    description: None,
                    value: 62,
                },
                EnumVariant {
                    name: "MUL64",
                    description: None,
                    value: 63,
                },
                EnumVariant {
                    name: "MUL65",
                    description: None,
                    value: 64,
                },
                EnumVariant {
                    name: "MUL66",
                    description: None,
                    value: 65,
                },
                EnumVariant {
                    name: "MUL67",
                    description: None,
                    value: 66,
                },
                EnumVariant {
                    name: "MUL68",
                    description: None,
                    value: 67,
                },
                EnumVariant {
                    name: "MUL69",
                    description: None,
                    value: 68,
                },
                EnumVariant {
                    name: "MUL70",
                    description: None,
                    value: 69,
                },
                EnumVariant {
                    name: "MUL71",
                    description: None,
                    value: 70,
                },
                EnumVariant {
                    name: "MUL72",
                    description: None,
                    value: 71,
                },
                EnumVariant {
                    name: "MUL73",
                    description: None,
                    value: 72,
                },
                EnumVariant {
                    name: "MUL74",
                    description: None,
                    value: 73,
                },
                EnumVariant {
                    name: "MUL75",
                    description: None,
                    value: 74,
                },
                EnumVariant {
                    name: "MUL76",
                    description: None,
                    value: 75,
                },
                EnumVariant {
                    name: "MUL77",
                    description: None,
                    value: 76,
                },
                EnumVariant {
                    name: "MUL78",
                    description: None,
                    value: 77,
                },
                EnumVariant {
                    name: "MUL79",
                    description: None,
                    value: 78,
                },
                EnumVariant {
                    name: "MUL80",
                    description: None,
                    value: 79,
                },
                EnumVariant {
                    name: "MUL81",
                    description: None,
                    value: 80,
                },
                EnumVariant {
                    name: "MUL82",
                    description: None,
                    value: 81,
                },
                EnumVariant {
                    name: "MUL83",
                    description: None,
                    value: 82,
                },
                EnumVariant {
                    name: "MUL84",
                    description: None,
                    value: 83,
                },
                EnumVariant {
                    name: "MUL85",
                    description: None,
                    value: 84,
                },
                EnumVariant {
                    name: "MUL86",
                    description: None,
                    value: 85,
                },
                EnumVariant {
                    name: "MUL87",
                    description: None,
                    value: 86,
                },
                EnumVariant {
                    name: "MUL88",
                    description: None,
                    value: 87,
                },
                EnumVariant {
                    name: "MUL89",
                    description: None,
                    value: 88,
                },
                EnumVariant {
                    name: "MUL90",
                    description: None,
                    value: 89,
                },
                EnumVariant {
                    name: "MUL91",
                    description: None,
                    value: 90,
                },
                EnumVariant {
                    name: "MUL92",
                    description: None,
                    value: 91,
                },
                EnumVariant {
                    name: "MUL93",
                    description: None,
                    value: 92,
                },
                EnumVariant {
                    name: "MUL94",
                    description: None,
                    value: 93,
                },
                EnumVariant {
                    name: "MUL95",
                    description: None,
                    value: 94,
                },
                EnumVariant {
                    name: "MUL96",
                    description: None,
                    value: 95,
                },
                EnumVariant {
                    name: "MUL97",
                    description: None,
                    value: 96,
                },
                EnumVariant {
                    name: "MUL98",
                    description: None,
                    value: 97,
                },
                EnumVariant {
                    name: "MUL99",
                    description: None,
                    value: 98,
                },
                EnumVariant {
                    name: "MUL100",
                    description: None,
                    value: 99,
                },
                EnumVariant {
                    name: "MUL101",
                    description: None,
                    value: 100,
                },
                EnumVariant {
                    name: "MUL102",
                    description: None,
                    value: 101,
                },
                EnumVariant {
                    name: "MUL103",
                    description: None,
                    value: 102,
                },
                EnumVariant {
                    name: "MUL104",
                    description: None,
                    value: 103,
                },
                EnumVariant {
                    name: "MUL105",
                    description: None,
                    value: 104,
                },
                EnumVariant {
                    name: "MUL106",
                    description: None,
                    value: 105,
                },
                EnumVariant {
                    name: "MUL107",
                    description: None,
                    value: 106,
                },
                EnumVariant {
                    name: "MUL108",
                    description: None,
                    value: 107,
                },
                EnumVariant {
                    name: "MUL109",
                    description: None,
                    value: 108,
                },
                EnumVariant {
                    name: "MUL110",
                    description: None,
                    value: 109,
                },
                EnumVariant {
                    name: "MUL111",
                    description: None,
                    value: 110,
                },
                EnumVariant {
                    name: "MUL112",
                    description: None,
                    value: 111,
                },
                EnumVariant {
                    name: "MUL113",
                    description: None,
                    value: 112,
                },
                EnumVariant {
                    name: "MUL114",
                    description: None,
                    value: 113,
                },
                EnumVariant {
                    name: "MUL115",
                    description: None,
                    value: 114,
                },
                EnumVariant {
                    name: "MUL116",
                    description: None,
                    value: 115,
                },
                EnumVariant {
                    name: "MUL117",
                    description: None,
                    value: 116,
                },
                EnumVariant {
                    name: "MUL118",
                    description: None,
                    value: 117,
                },
                EnumVariant {
                    name: "MUL119",
                    description: None,
                    value: 118,
                },
                EnumVariant {
                    name: "MUL120",
                    description: None,
                    value: 119,
                },
                EnumVariant {
                    name: "MUL121",
                    description: None,
                    value: 120,
                },
                EnumVariant {
                    name: "MUL122",
                    description: None,
                    value: 121,
                },
                EnumVariant {
                    name: "MUL123",
                    description: None,
                    value: 122,
                },
                EnumVariant {
                    name: "MUL124",
                    description: None,
                    value: 123,
                },
                EnumVariant {
                    name: "MUL125",
                    description: None,
                    value: 124,
                },
                EnumVariant {
                    name: "MUL126",
                    description: None,
                    value: 125,
                },
                EnumVariant {
                    name: "MUL127",
                    description: None,
                    value: 126,
                },
                EnumVariant {
                    name: "MUL128",
                    description: None,
                    value: 127,
                },
                EnumVariant {
                    name: "MUL129",
                    description: None,
                    value: 128,
                },
                EnumVariant {
                    name: "MUL130",
                    description: None,
                    value: 129,
                },
                EnumVariant {
                    name: "MUL131",
                    description: None,
                    value: 130,
                },
                EnumVariant {
                    name: "MUL132",
                    description: None,
                    value: 131,
                },
                EnumVariant {
                    name: "MUL133",
                    description: None,
                    value: 132,
                },
                EnumVariant {
                    name: "MUL134",
                    description: None,
                    value: 133,
                },
                EnumVariant {
                    name: "MUL135",
                    description: None,
                    value: 134,
                },
                EnumVariant {
                    name: "MUL136",
                    description: None,
                    value: 135,
                },
                EnumVariant {
                    name: "MUL137",
                    description: None,
                    value: 136,
                },
                EnumVariant {
                    name: "MUL138",
                    description: None,
                    value: 137,
                },
                EnumVariant {
                    name: "MUL139",
                    description: None,
                    value: 138,
                },
                EnumVariant {
                    name: "MUL140",
                    description: None,
                    value: 139,
                },
                EnumVariant {
                    name: "MUL141",
                    description: None,
                    value: 140,
                },
                EnumVariant {
                    name: "MUL142",
                    description: None,
                    value: 141,
                },
                EnumVariant {
                    name: "MUL143",
                    description: None,
                    value: 142,
                },
                EnumVariant {
                    name: "MUL144",
                    description: None,
                    value: 143,
                },
                EnumVariant {
                    name: "MUL145",
                    description: None,
                    value: 144,
                },
                EnumVariant {
                    name: "MUL146",
                    description: None,
                    value: 145,
                },
                EnumVariant {
                    name: "MUL147",
                    description: None,
                    value: 146,
                },
                EnumVariant {
                    name: "MUL148",
                    description: None,
                    value: 147,
                },
                EnumVariant {
                    name: "MUL149",
                    description: None,
                    value: 148,
                },
                EnumVariant {
                    name: "MUL150",
                    description: None,
                    value: 149,
                },
                EnumVariant {
                    name: "MUL151",
                    description: None,
                    value: 150,
                },
                EnumVariant {
                    name: "MUL152",
                    description: None,
                    value: 151,
                },
                EnumVariant {
                    name: "MUL153",
                    description: None,
                    value: 152,
                },
                EnumVariant {
                    name: "MUL154",
                    description: None,
                    value: 153,
                },
                EnumVariant {
                    name: "MUL155",
                    description: None,
                    value: 154,
                },
                EnumVariant {
                    name: "MUL156",
                    description: None,
                    value: 155,
                },
                EnumVariant {
                    name: "MUL157",
                    description: None,
                    value: 156,
                },
                EnumVariant {
                    name: "MUL158",
                    description: None,
                    value: 157,
                },
                EnumVariant {
                    name: "MUL159",
                    description: None,
                    value: 158,
                },
                EnumVariant {
                    name: "MUL160",
                    description: None,
                    value: 159,
                },
                EnumVariant {
                    name: "MUL161",
                    description: None,
                    value: 160,
                },
                EnumVariant {
                    name: "MUL162",
                    description: None,
                    value: 161,
                },
                EnumVariant {
                    name: "MUL163",
                    description: None,
                    value: 162,
                },
                EnumVariant {
                    name: "MUL164",
                    description: None,
                    value: 163,
                },
                EnumVariant {
                    name: "MUL165",
                    description: None,
                    value: 164,
                },
                EnumVariant {
                    name: "MUL166",
                    description: None,
                    value: 165,
                },
                EnumVariant {
                    name: "MUL167",
                    description: None,
                    value: 166,
                },
                EnumVariant {
                    name: "MUL168",
                    description: None,
                    value: 167,
                },
                EnumVariant {
                    name: "MUL169",
                    description: None,
                    value: 168,
                },
                EnumVariant {
                    name: "MUL170",
                    description: None,
                    value: 169,
                },
                EnumVariant {
                    name: "MUL171",
                    description: None,
                    value: 170,
                },
                EnumVariant {
                    name: "MUL172",
                    description: None,
                    value: 171,
                },
                EnumVariant {
                    name: "MUL173",
                    description: None,
                    value: 172,
                },
                EnumVariant {
                    name: "MUL174",
                    description: None,
                    value: 173,
                },
                EnumVariant {
                    name: "MUL175",
                    description: None,
                    value: 174,
                },
                EnumVariant {
                    name: "MUL176",
                    description: None,
                    value: 175,
                },
                EnumVariant {
                    name: "MUL177",
                    description: None,
                    value: 176,
                },
                EnumVariant {
                    name: "MUL178",
                    description: None,
                    value: 177,
                },
                EnumVariant {
                    name: "MUL179",
                    description: None,
                    value: 178,
                },
                EnumVariant {
                    name: "MUL180",
                    description: None,
                    value: 179,
                },
                EnumVariant {
                    name: "MUL181",
                    description: None,
                    value: 180,
                },
                EnumVariant {
                    name: "MUL182",
                    description: None,
                    value: 181,
                },
                EnumVariant {
                    name: "MUL183",
                    description: None,
                    value: 182,
                },
                EnumVariant {
                    name: "MUL184",
                    description: None,
                    value: 183,
                },
                EnumVariant {
                    name: "MUL185",
                    description: None,
                    value: 184,
                },
                EnumVariant {
                    name: "MUL186",
                    description: None,
                    value: 185,
                },
                EnumVariant {
                    name: "MUL187",
                    description: None,
                    value: 186,
                },
                EnumVariant {
                    name: "MUL188",
                    description: None,
                    value: 187,
                },
                EnumVariant {
                    name: "MUL189",
                    description: None,
                    value: 188,
                },
                EnumVariant {
                    name: "MUL190",
                    description: None,
                    value: 189,
                },
                EnumVariant {
                    name: "MUL191",
                    description: None,
                    value: 190,
                },
                EnumVariant {
                    name: "MUL192",
                    description: None,
                    value: 191,
                },
                EnumVariant {
                    name: "MUL193",
                    description: None,
                    value: 192,
                },
                EnumVariant {
                    name: "MUL194",
                    description: None,
                    value: 193,
                },
                EnumVariant {
                    name: "MUL195",
                    description: None,
                    value: 194,
                },
                EnumVariant {
                    name: "MUL196",
                    description: None,
                    value: 195,
                },
                EnumVariant {
                    name: "MUL197",
                    description: None,
                    value: 196,
                },
                EnumVariant {
                    name: "MUL198",
                    description: None,
                    value: 197,
                },
                EnumVariant {
                    name: "MUL199",
                    description: None,
                    value: 198,
                },
                EnumVariant {
                    name: "MUL200",
                    description: None,
                    value: 199,
                },
                EnumVariant {
                    name: "MUL201",
                    description: None,
                    value: 200,
                },
                EnumVariant {
                    name: "MUL202",
                    description: None,
                    value: 201,
                },
                EnumVariant {
                    name: "MUL203",
                    description: None,
                    value: 202,
                },
                EnumVariant {
                    name: "MUL204",
                    description: None,
                    value: 203,
                },
                EnumVariant {
                    name: "MUL205",
                    description: None,
                    value: 204,
                },
                EnumVariant {
                    name: "MUL206",
                    description: None,
                    value: 205,
                },
                EnumVariant {
                    name: "MUL207",
                    description: None,
                    value: 206,
                },
                EnumVariant {
                    name: "MUL208",
                    description: None,
                    value: 207,
                },
                EnumVariant {
                    name: "MUL209",
                    description: None,
                    value: 208,
                },
                EnumVariant {
                    name: "MUL210",
                    description: None,
                    value: 209,
                },
                EnumVariant {
                    name: "MUL211",
                    description: None,
                    value: 210,
                },
                EnumVariant {
                    name: "MUL212",
                    description: None,
                    value: 211,
                },
                EnumVariant {
                    name: "MUL213",
                    description: None,
                    value: 212,
                },
                EnumVariant {
                    name: "MUL214",
                    description: None,
                    value: 213,
                },
                EnumVariant {
                    name: "MUL215",
                    description: None,
                    value: 214,
                },
                EnumVariant {
                    name: "MUL216",
                    description: None,
                    value: 215,
                },
                EnumVariant {
                    name: "MUL217",
                    description: None,
                    value: 216,
                },
                EnumVariant {
                    name: "MUL218",
                    description: None,
                    value: 217,
                },
                EnumVariant {
                    name: "MUL219",
                    description: None,
                    value: 218,
                },
                EnumVariant {
                    name: "MUL220",
                    description: None,
                    value: 219,
                },
                EnumVariant {
                    name: "MUL221",
                    description: None,
                    value: 220,
                },
                EnumVariant {
                    name: "MUL222",
                    description: None,
                    value: 221,
                },
                EnumVariant {
                    name: "MUL223",
                    description: None,
                    value: 222,
                },
                EnumVariant {
                    name: "MUL224",
                    description: None,
                    value: 223,
                },
                EnumVariant {
                    name: "MUL225",
                    description: None,
                    value: 224,
                },
                EnumVariant {
                    name: "MUL226",
                    description: None,
                    value: 225,
                },
                EnumVariant {
                    name: "MUL227",
                    description: None,
                    value: 226,
                },
                EnumVariant {
                    name: "MUL228",
                    description: None,
                    value: 227,
                },
                EnumVariant {
                    name: "MUL229",
                    description: None,
                    value: 228,
                },
                EnumVariant {
                    name: "MUL230",
                    description: None,
                    value: 229,
                },
                EnumVariant {
                    name: "MUL231",
                    description: None,
                    value: 230,
                },
                EnumVariant {
                    name: "MUL232",
                    description: None,
                    value: 231,
                },
                EnumVariant {
                    name: "MUL233",
                    description: None,
                    value: 232,
                },
                EnumVariant {
                    name: "MUL234",
                    description: None,
                    value: 233,
                },
                EnumVariant {
                    name: "MUL235",
                    description: None,
                    value: 234,
                },
                EnumVariant {
                    name: "MUL236",
                    description: None,
                    value: 235,
                },
                EnumVariant {
                    name: "MUL237",
                    description: None,
                    value: 236,
                },
                EnumVariant {
                    name: "MUL238",
                    description: None,
                    value: 237,
                },
                EnumVariant {
                    name: "MUL239",
                    description: None,
                    value: 238,
                },
                EnumVariant {
                    name: "MUL240",
                    description: None,
                    value: 239,
                },
                EnumVariant {
                    name: "MUL241",
                    description: None,
                    value: 240,
                },
                EnumVariant {
                    name: "MUL242",
                    description: None,
                    value: 241,
                },
                EnumVariant {
                    name: "MUL243",
                    description: None,
                    value: 242,
                },
                EnumVariant {
                    name: "MUL244",
                    description: None,
                    value: 243,
                },
                EnumVariant {
                    name: "MUL245",
                    description: None,
                    value: 244,
                },
                EnumVariant {
                    name: "MUL246",
                    description: None,
                    value: 245,
                },
                EnumVariant {
                    name: "MUL247",
                    description: None,
                    value: 246,
                },
                EnumVariant {
                    name: "MUL248",
                    description: None,
                    value: 247,
                },
                EnumVariant {
                    name: "MUL249",
                    description: None,
                    value: 248,
                },
                EnumVariant {
                    name: "MUL250",
                    description: None,
                    value: 249,
                },
                EnumVariant {
                    name: "MUL251",
                    description: None,
                    value: 250,
                },
                EnumVariant {
                    name: "MUL252",
                    description: None,
                    value: 251,
                },
                EnumVariant {
                    name: "MUL253",
                    description: None,
                    value: 252,
                },
                EnumVariant {
                    name: "MUL254",
                    description: None,
                    value: 253,
                },
                EnumVariant {
                    name: "MUL255",
                    description: None,
                    value: 254,
                },
                EnumVariant {
                    name: "MUL256",
                    description: None,
                    value: 255,
                },
                EnumVariant {
                    name: "MUL257",
                    description: None,
                    value: 256,
                },
                EnumVariant {
                    name: "MUL258",
                    description: None,
                    value: 257,
                },
                EnumVariant {
                    name: "MUL259",
                    description: None,
                    value: 258,
                },
                EnumVariant {
                    name: "MUL260",
                    description: None,
                    value: 259,
                },
                EnumVariant {
                    name: "MUL261",
                    description: None,
                    value: 260,
                },
                EnumVariant {
                    name: "MUL262",
                    description: None,
                    value: 261,
                },
                EnumVariant {
                    name: "MUL263",
                    description: None,
                    value: 262,
                },
                EnumVariant {
                    name: "MUL264",
                    description: None,
                    value: 263,
                },
                EnumVariant {
                    name: "MUL265",
                    description: None,
                    value: 264,
                },
                EnumVariant {
                    name: "MUL266",
                    description: None,
                    value: 265,
                },
                EnumVariant {
                    name: "MUL267",
                    description: None,
                    value: 266,
                },
                EnumVariant {
                    name: "MUL268",
                    description: None,
                    value: 267,
                },
                EnumVariant {
                    name: "MUL269",
                    description: None,
                    value: 268,
                },
                EnumVariant {
                    name: "MUL270",
                    description: None,
                    value: 269,
                },
                EnumVariant {
                    name: "MUL271",
                    description: None,
                    value: 270,
                },
                EnumVariant {
                    name: "MUL272",
                    description: None,
                    value: 271,
                },
                EnumVariant {
                    name: "MUL273",
                    description: None,
                    value: 272,
                },
                EnumVariant {
                    name: "MUL274",
                    description: None,
                    value: 273,
                },
                EnumVariant {
                    name: "MUL275",
                    description: None,
                    value: 274,
                },
                EnumVariant {
                    name: "MUL276",
                    description: None,
                    value: 275,
                },
                EnumVariant {
                    name: "MUL277",
                    description: None,
                    value: 276,
                },
                EnumVariant {
                    name: "MUL278",
                    description: None,
                    value: 277,
                },
                EnumVariant {
                    name: "MUL279",
                    description: None,
                    value: 278,
                },
                EnumVariant {
                    name: "MUL280",
                    description: None,
                    value: 279,
                },
                EnumVariant {
                    name: "MUL281",
                    description: None,
                    value: 280,
                },
                EnumVariant {
                    name: "MUL282",
                    description: None,
                    value: 281,
                },
                EnumVariant {
                    name: "MUL283",
                    description: None,
                    value: 282,
                },
                EnumVariant {
                    name: "MUL284",
                    description: None,
                    value: 283,
                },
                EnumVariant {
                    name: "MUL285",
                    description: None,
                    value: 284,
                },
                EnumVariant {
                    name: "MUL286",
                    description: None,
                    value: 285,
                },
                EnumVariant {
                    name: "MUL287",
                    description: None,
                    value: 286,
                },
                EnumVariant {
                    name: "MUL288",
                    description: None,
                    value: 287,
                },
                EnumVariant {
                    name: "MUL289",
                    description: None,
                    value: 288,
                },
                EnumVariant {
                    name: "MUL290",
                    description: None,
                    value: 289,
                },
                EnumVariant {
                    name: "MUL291",
                    description: None,
                    value: 290,
                },
                EnumVariant {
                    name: "MUL292",
                    description: None,
                    value: 291,
                },
                EnumVariant {
                    name: "MUL293",
                    description: None,
                    value: 292,
                },
                EnumVariant {
                    name: "MUL294",
                    description: None,
                    value: 293,
                },
                EnumVariant {
                    name: "MUL295",
                    description: None,
                    value: 294,
                },
                EnumVariant {
                    name: "MUL296",
                    description: None,
                    value: 295,
                },
                EnumVariant {
                    name: "MUL297",
                    description: None,
                    value: 296,
                },
                EnumVariant {
                    name: "MUL298",
                    description: None,
                    value: 297,
                },
                EnumVariant {
                    name: "MUL299",
                    description: None,
                    value: 298,
                },
                EnumVariant {
                    name: "MUL300",
                    description: None,
                    value: 299,
                },
                EnumVariant {
                    name: "MUL301",
                    description: None,
                    value: 300,
                },
                EnumVariant {
                    name: "MUL302",
                    description: None,
                    value: 301,
                },
                EnumVariant {
                    name: "MUL303",
                    description: None,
                    value: 302,
                },
                EnumVariant {
                    name: "MUL304",
                    description: None,
                    value: 303,
                },
                EnumVariant {
                    name: "MUL305",
                    description: None,
                    value: 304,
                },
                EnumVariant {
                    name: "MUL306",
                    description: None,
                    value: 305,
                },
                EnumVariant {
                    name: "MUL307",
                    description: None,
                    value: 306,
                },
                EnumVariant {
                    name: "MUL308",
                    description: None,
                    value: 307,
                },
                EnumVariant {
                    name: "MUL309",
                    description: None,
                    value: 308,
                },
                EnumVariant {
                    name: "MUL310",
                    description: None,
                    value: 309,
                },
                EnumVariant {
                    name: "MUL311",
                    description: None,
                    value: 310,
                },
                EnumVariant {
                    name: "MUL312",
                    description: None,
                    value: 311,
                },
                EnumVariant {
                    name: "MUL313",
                    description: None,
                    value: 312,
                },
                EnumVariant {
                    name: "MUL314",
                    description: None,
                    value: 313,
                },
                EnumVariant {
                    name: "MUL315",
                    description: None,
                    value: 314,
                },
                EnumVariant {
                    name: "MUL316",
                    description: None,
                    value: 315,
                },
                EnumVariant {
                    name: "MUL317",
                    description: None,
                    value: 316,
                },
                EnumVariant {
                    name: "MUL318",
                    description: None,
                    value: 317,
                },
                EnumVariant {
                    name: "MUL319",
                    description: None,
                    value: 318,
                },
                EnumVariant {
                    name: "MUL320",
                    description: None,
                    value: 319,
                },
                EnumVariant {
                    name: "MUL321",
                    description: None,
                    value: 320,
                },
                EnumVariant {
                    name: "MUL322",
                    description: None,
                    value: 321,
                },
                EnumVariant {
                    name: "MUL323",
                    description: None,
                    value: 322,
                },
                EnumVariant {
                    name: "MUL324",
                    description: None,
                    value: 323,
                },
                EnumVariant {
                    name: "MUL325",
                    description: None,
                    value: 324,
                },
                EnumVariant {
                    name: "MUL326",
                    description: None,
                    value: 325,
                },
                EnumVariant {
                    name: "MUL327",
                    description: None,
                    value: 326,
                },
                EnumVariant {
                    name: "MUL328",
                    description: None,
                    value: 327,
                },
                EnumVariant {
                    name: "MUL329",
                    description: None,
                    value: 328,
                },
                EnumVariant {
                    name: "MUL330",
                    description: None,
                    value: 329,
                },
                EnumVariant {
                    name: "MUL331",
                    description: None,
                    value: 330,
                },
                EnumVariant {
                    name: "MUL332",
                    description: None,
                    value: 331,
                },
                EnumVariant {
                    name: "MUL333",
                    description: None,
                    value: 332,
                },
                EnumVariant {
                    name: "MUL334",
                    description: None,
                    value: 333,
                },
                EnumVariant {
                    name: "MUL335",
                    description: None,
                    value: 334,
                },
                EnumVariant {
                    name: "MUL336",
                    description: None,
                    value: 335,
                },
                EnumVariant {
                    name: "MUL337",
                    description: None,
                    value: 336,
                },
                EnumVariant {
                    name: "MUL338",
                    description: None,
                    value: 337,
                },
                EnumVariant {
                    name: "MUL339",
                    description: None,
                    value: 338,
                },
                EnumVariant {
                    name: "MUL340",
                    description: None,
                    value: 339,
                },
                EnumVariant {
                    name: "MUL341",
                    description: None,
                    value: 340,
                },
                EnumVariant {
                    name: "MUL342",
                    description: None,
                    value: 341,
                },
                EnumVariant {
                    name: "MUL343",
                    description: None,
                    value: 342,
                },
                EnumVariant {
                    name: "MUL344",
                    description: None,
                    value: 343,
                },
                EnumVariant {
                    name: "MUL345",
                    description: None,
                    value: 344,
                },
                EnumVariant {
                    name: "MUL346",
                    description: None,
                    value: 345,
                },
                EnumVariant {
                    name: "MUL347",
                    description: None,
                    value: 346,
                },
                EnumVariant {
                    name: "MUL348",
                    description: None,
                    value: 347,
                },
                EnumVariant {
                    name: "MUL349",
                    description: None,
                    value: 348,
                },
                EnumVariant {
                    name: "MUL350",
                    description: None,
                    value: 349,
                },
                EnumVariant {
                    name: "MUL351",
                    description: None,
                    value: 350,
                },
                EnumVariant {
                    name: "MUL352",
                    description: None,
                    value: 351,
                },
                EnumVariant {
                    name: "MUL353",
                    description: None,
                    value: 352,
                },
                EnumVariant {
                    name: "MUL354",
                    description: None,
                    value: 353,
                },
                EnumVariant {
                    name: "MUL355",
                    description: None,
                    value: 354,
                },
                EnumVariant {
                    name: "MUL356",
                    description: None,
                    value: 355,
                },
                EnumVariant {
                    name: "MUL357",
                    description: None,
                    value: 356,
                },
                EnumVariant {
                    name: "MUL358",
                    description: None,
                    value: 357,
                },
                EnumVariant {
                    name: "MUL359",
                    description: None,
                    value: 358,
                },
                EnumVariant {
                    name: "MUL360",
                    description: None,
                    value: 359,
                },
                EnumVariant {
                    name: "MUL361",
                    description: None,
                    value: 360,
                },
                EnumVariant {
                    name: "MUL362",
                    description: None,
                    value: 361,
                },
                EnumVariant {
                    name: "MUL363",
                    description: None,
                    value: 362,
                },
                EnumVariant {
                    name: "MUL364",
                    description: None,
                    value: 363,
                },
                EnumVariant {
                    name: "MUL365",
                    description: None,
                    value: 364,
                },
                EnumVariant {
                    name: "MUL366",
                    description: None,
                    value: 365,
                },
                EnumVariant {
                    name: "MUL367",
                    description: None,
                    value: 366,
                },
                EnumVariant {
                    name: "MUL368",
                    description: None,
                    value: 367,
                },
                EnumVariant {
                    name: "MUL369",
                    description: None,
                    value: 368,
                },
                EnumVariant {
                    name: "MUL370",
                    description: None,
                    value: 369,
                },
                EnumVariant {
                    name: "MUL371",
                    description: None,
                    value: 370,
                },
                EnumVariant {
                    name: "MUL372",
                    description: None,
                    value: 371,
                },
                EnumVariant {
                    name: "MUL373",
                    description: None,
                    value: 372,
                },
                EnumVariant {
                    name: "MUL374",
                    description: None,
                    value: 373,
                },
                EnumVariant {
                    name: "MUL375",
                    description: None,
                    value: 374,
                },
                EnumVariant {
                    name: "MUL376",
                    description: None,
                    value: 375,
                },
                EnumVariant {
                    name: "MUL377",
                    description: None,
                    value: 376,
                },
                EnumVariant {
                    name: "MUL378",
                    description: None,
                    value: 377,
                },
                EnumVariant {
                    name: "MUL379",
                    description: None,
                    value: 378,
                },
                EnumVariant {
                    name: "MUL380",
                    description: None,
                    value: 379,
                },
                EnumVariant {
                    name: "MUL381",
                    description: None,
                    value: 380,
                },
                EnumVariant {
                    name: "MUL382",
                    description: None,
                    value: 381,
                },
                EnumVariant {
                    name: "MUL383",
                    description: None,
                    value: 382,
                },
                EnumVariant {
                    name: "MUL384",
                    description: None,
                    value: 383,
                },
                EnumVariant {
                    name: "MUL385",
                    description: None,
                    value: 384,
                },
                EnumVariant {
                    name: "MUL386",
                    description: None,
                    value: 385,
                },
                EnumVariant {
                    name: "MUL387",
                    description: None,
                    value: 386,
                },
                EnumVariant {
                    name: "MUL388",
                    description: None,
                    value: 387,
                },
                EnumVariant {
                    name: "MUL389",
                    description: None,
                    value: 388,
                },
                EnumVariant {
                    name: "MUL390",
                    description: None,
                    value: 389,
                },
                EnumVariant {
                    name: "MUL391",
                    description: None,
                    value: 390,
                },
                EnumVariant {
                    name: "MUL392",
                    description: None,
                    value: 391,
                },
                EnumVariant {
                    name: "MUL393",
                    description: None,
                    value: 392,
                },
                EnumVariant {
                    name: "MUL394",
                    description: None,
                    value: 393,
                },
                EnumVariant {
                    name: "MUL395",
                    description: None,
                    value: 394,
                },
                EnumVariant {
                    name: "MUL396",
                    description: None,
                    value: 395,
                },
                EnumVariant {
                    name: "MUL397",
                    description: None,
                    value: 396,
                },
                EnumVariant {
                    name: "MUL398",
                    description: None,
                    value: 397,
                },
                EnumVariant {
                    name: "MUL399",
                    description: None,
                    value: 398,
                },
                EnumVariant {
                    name: "MUL400",
                    description: None,
                    value: 399,
                },
                EnumVariant {
                    name: "MUL401",
                    description: None,
                    value: 400,
                },
                EnumVariant {
                    name: "MUL402",
                    description: None,
                    value: 401,
                },
                EnumVariant {
                    name: "MUL403",
                    description: None,
                    value: 402,
                },
                EnumVariant {
                    name: "MUL404",
                    description: None,
                    value: 403,
                },
                EnumVariant {
                    name: "MUL405",
                    description: None,
                    value: 404,
                },
                EnumVariant {
                    name: "MUL406",
                    description: None,
                    value: 405,
                },
                EnumVariant {
                    name: "MUL407",
                    description: None,
                    value: 406,
                },
                EnumVariant {
                    name: "MUL408",
                    description: None,
                    value: 407,
                },
                EnumVariant {
                    name: "MUL409",
                    description: None,
                    value: 408,
                },
                EnumVariant {
                    name: "MUL410",
                    description: None,
                    value: 409,
                },
                EnumVariant {
                    name: "MUL411",
                    description: None,
                    value: 410,
                },
                EnumVariant {
                    name: "MUL412",
                    description: None,
                    value: 411,
                },
                EnumVariant {
                    name: "MUL413",
                    description: None,
                    value: 412,
                },
                EnumVariant {
                    name: "MUL414",
                    description: None,
                    value: 413,
                },
                EnumVariant {
                    name: "MUL415",
                    description: None,
                    value: 414,
                },
                EnumVariant {
                    name: "MUL416",
                    description: None,
                    value: 415,
                },
                EnumVariant {
                    name: "MUL417",
                    description: None,
                    value: 416,
                },
                EnumVariant {
                    name: "MUL418",
                    description: None,
                    value: 417,
                },
                EnumVariant {
                    name: "MUL419",
                    description: None,
                    value: 418,
                },
                EnumVariant {
                    name: "MUL420",
                    description: None,
                    value: 419,
                },
                EnumVariant {
                    name: "MUL421",
                    description: None,
                    value: 420,
                },
                EnumVariant {
                    name: "MUL422",
                    description: None,
                    value: 421,
                },
                EnumVariant {
                    name: "MUL423",
                    description: None,
                    value: 422,
                },
                EnumVariant {
                    name: "MUL424",
                    description: None,
                    value: 423,
                },
                EnumVariant {
                    name: "MUL425",
                    description: None,
                    value: 424,
                },
                EnumVariant {
                    name: "MUL426",
                    description: None,
                    value: 425,
                },
                EnumVariant {
                    name: "MUL427",
                    description: None,
                    value: 426,
                },
                EnumVariant {
                    name: "MUL428",
                    description: None,
                    value: 427,
                },
                EnumVariant {
                    name: "MUL429",
                    description: None,
                    value: 428,
                },
                EnumVariant {
                    name: "MUL430",
                    description: None,
                    value: 429,
                },
                EnumVariant {
                    name: "MUL431",
                    description: None,
                    value: 430,
                },
                EnumVariant {
                    name: "MUL432",
                    description: None,
                    value: 431,
                },
                EnumVariant {
                    name: "MUL433",
                    description: None,
                    value: 432,
                },
                EnumVariant {
                    name: "MUL434",
                    description: None,
                    value: 433,
                },
                EnumVariant {
                    name: "MUL435",
                    description: None,
                    value: 434,
                },
                EnumVariant {
                    name: "MUL436",
                    description: None,
                    value: 435,
                },
                EnumVariant {
                    name: "MUL437",
                    description: None,
                    value: 436,
                },
                EnumVariant {
                    name: "MUL438",
                    description: None,
                    value: 437,
                },
                EnumVariant {
                    name: "MUL439",
                    description: None,
                    value: 438,
                },
                EnumVariant {
                    name: "MUL440",
                    description: None,
                    value: 439,
                },
                EnumVariant {
                    name: "MUL441",
                    description: None,
                    value: 440,
                },
                EnumVariant {
                    name: "MUL442",
                    description: None,
                    value: 441,
                },
                EnumVariant {
                    name: "MUL443",
                    description: None,
                    value: 442,
                },
                EnumVariant {
                    name: "MUL444",
                    description: None,
                    value: 443,
                },
                EnumVariant {
                    name: "MUL445",
                    description: None,
                    value: 444,
                },
                EnumVariant {
                    name: "MUL446",
                    description: None,
                    value: 445,
                },
                EnumVariant {
                    name: "MUL447",
                    description: None,
                    value: 446,
                },
                EnumVariant {
                    name: "MUL448",
                    description: None,
                    value: 447,
                },
                EnumVariant {
                    name: "MUL449",
                    description: None,
                    value: 448,
                },
                EnumVariant {
                    name: "MUL450",
                    description: None,
                    value: 449,
                },
                EnumVariant {
                    name: "MUL451",
                    description: None,
                    value: 450,
                },
                EnumVariant {
                    name: "MUL452",
                    description: None,
                    value: 451,
                },
                EnumVariant {
                    name: "MUL453",
                    description: None,
                    value: 452,
                },
                EnumVariant {
                    name: "MUL454",
                    description: None,
                    value: 453,
                },
                EnumVariant {
                    name: "MUL455",
                    description: None,
                    value: 454,
                },
                EnumVariant {
                    name: "MUL456",
                    description: None,
                    value: 455,
                },
                EnumVariant {
                    name: "MUL457",
                    description: None,
                    value: 456,
                },
                EnumVariant {
                    name: "MUL458",
                    description: None,
                    value: 457,
                },
                EnumVariant {
                    name: "MUL459",
                    description: None,
                    value: 458,
                },
                EnumVariant {
                    name: "MUL460",
                    description: None,
                    value: 459,
                },
                EnumVariant {
                    name: "MUL461",
                    description: None,
                    value: 460,
                },
                EnumVariant {
                    name: "MUL462",
                    description: None,
                    value: 461,
                },
                EnumVariant {
                    name: "MUL463",
                    description: None,
                    value: 462,
                },
                EnumVariant {
                    name: "MUL464",
                    description: None,
                    value: 463,
                },
                EnumVariant {
                    name: "MUL465",
                    description: None,
                    value: 464,
                },
                EnumVariant {
                    name: "MUL466",
                    description: None,
                    value: 465,
                },
                EnumVariant {
                    name: "MUL467",
                    description: None,
                    value: 466,
                },
                EnumVariant {
                    name: "MUL468",
                    description: None,
                    value: 467,
                },
                EnumVariant {
                    name: "MUL469",
                    description: None,
                    value: 468,
                },
                EnumVariant {
                    name: "MUL470",
                    description: None,
                    value: 469,
                },
                EnumVariant {
                    name: "MUL471",
                    description: None,
                    value: 470,
                },
                EnumVariant {
                    name: "MUL472",
                    description: None,
                    value: 471,
                },
                EnumVariant {
                    name: "MUL473",
                    description: None,
                    value: 472,
                },
                EnumVariant {
                    name: "MUL474",
                    description: None,
                    value: 473,
                },
                EnumVariant {
                    name: "MUL475",
                    description: None,
                    value: 474,
                },
                EnumVariant {
                    name: "MUL476",
                    description: None,
                    value: 475,
                },
                EnumVariant {
                    name: "MUL477",
                    description: None,
                    value: 476,
                },
                EnumVariant {
                    name: "MUL478",
                    description: None,
                    value: 477,
                },
                EnumVariant {
                    name: "MUL479",
                    description: None,
                    value: 478,
                },
                EnumVariant {
                    name: "MUL480",
                    description: None,
                    value: 479,
                },
                EnumVariant {
                    name: "MUL481",
                    description: None,
                    value: 480,
                },
                EnumVariant {
                    name: "MUL482",
                    description: None,
                    value: 481,
                },
                EnumVariant {
                    name: "MUL483",
                    description: None,
                    value: 482,
                },
                EnumVariant {
                    name: "MUL484",
                    description: None,
                    value: 483,
                },
                EnumVariant {
                    name: "MUL485",
                    description: None,
                    value: 484,
                },
                EnumVariant {
                    name: "MUL486",
                    description: None,
                    value: 485,
                },
                EnumVariant {
                    name: "MUL487",
                    description: None,
                    value: 486,
                },
                EnumVariant {
                    name: "MUL488",
                    description: None,
                    value: 487,
                },
                EnumVariant {
                    name: "MUL489",
                    description: None,
                    value: 488,
                },
                EnumVariant {
                    name: "MUL490",
                    description: None,
                    value: 489,
                },
                EnumVariant {
                    name: "MUL491",
                    description: None,
                    value: 490,
                },
                EnumVariant {
                    name: "MUL492",
                    description: None,
                    value: 491,
                },
                EnumVariant {
                    name: "MUL493",
                    description: None,
                    value: 492,
                },
                EnumVariant {
                    name: "MUL494",
                    description: None,
                    value: 493,
                },
                EnumVariant {
                    name: "MUL495",
                    description: None,
                    value: 494,
                },
                EnumVariant {
                    name: "MUL496",
                    description: None,
                    value: 495,
                },
                EnumVariant {
                    name: "MUL497",
                    description: None,
                    value: 496,
                },
                EnumVariant {
                    name: "MUL498",
                    description: None,
                    value: 497,
                },
                EnumVariant {
                    name: "MUL499",
                    description: None,
                    value: 498,
                },
                EnumVariant {
                    name: "MUL500",
                    description: None,
                    value: 499,
                },
                EnumVariant {
                    name: "MUL501",
                    description: None,
                    value: 500,
                },
                EnumVariant {
                    name: "MUL502",
                    description: None,
                    value: 501,
                },
                EnumVariant {
                    name: "MUL503",
                    description: None,
                    value: 502,
                },
                EnumVariant {
                    name: "MUL504",
                    description: None,
                    value: 503,
                },
                EnumVariant {
                    name: "MUL505",
                    description: None,
                    value: 504,
                },
                EnumVariant {
                    name: "MUL506",
                    description: None,
                    value: 505,
                },
                EnumVariant {
                    name: "MUL507",
                    description: None,
                    value: 506,
                },
                EnumVariant {
                    name: "MUL508",
                    description: None,
                    value: 507,
                },
                EnumVariant {
                    name: "MUL509",
                    description: None,
                    value: 508,
                },
                EnumVariant {
                    name: "MUL510",
                    description: None,
                    value: 509,
                },
                EnumVariant {
                    name: "MUL511",
                    description: None,
                    value: 510,
                },
                EnumVariant {
                    name: "MUL512",
                    description: None,
                    value: 511,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Frequency is between 1 and 2 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Frequency is between 2 and 4 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE4",
                    description: Some(
                        "Frequency is between 4 and 8 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE8",
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
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as PLL clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock sent to DIVMx dividers and PLLs",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllvcosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "WIDEVCO",
                    description: Some(
                        "VCO frequency range 192 to 836 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMVCO",
                    description: Some(
                        "VCO frequency range 150 to 420 MHz",
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
                        "rcc_hclk not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "rcc_hclk divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "rcc_hclk divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "rcc_hclk divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "rcc_hclk divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 3,
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
            name: "Saiasel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "I2S_CKIN",
                    description: Some(
                        "i2s_ckin selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Saisel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "pll3_p selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "I2S_CKIN",
                    description: Some(
                        "I2S_CKIN selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r selected as peripheral clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spdifrxsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Spi45sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "APB2 clock selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Spi6sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as wake up clock from system Stop",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wake up clock from system Stop",
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
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as system clock",
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
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Swpmisel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "pclk selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
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
                    name: "DEFAULTX2",
                    description: Some(
                        "Timer kernel clock equal to 2x pclk by default",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEFAULTX4",
                    description: Some(
                        "Timer kernel clock equal to 4x pclk by default",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart16910sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Usart234578sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Usbsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "Disable the kernel clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "pll3_q selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                