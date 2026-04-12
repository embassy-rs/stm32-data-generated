
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
                    name: "icscr3",
                    description: Some(
                        "RCC internal clock sources calibration register 3",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "RCC clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "cfgr2",
                    description: Some(
                        "RCC clock configuration register 2",
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
                    name: "cfgr3",
                    description: Some(
                        "RCC clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                BlockItem {
                    name: "pll1cfgr",
                    description: Some(
                        "RCC PLL1 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1divr",
                    description: Some(
                        "RCC PLL1 dividers register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1divr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1fracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1fracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock interrupt enable register",
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
                        "RCC clock interrupt flag register",
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
                        "RCC clock interrupt clear register",
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
                        "RCC AHB1 peripheral reset register",
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
                    name: "ahb4rstr",
                    description: Some(
                        "RCC AHB4 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
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
                    name: "ahb5rstr",
                    description: Some(
                        "RCC AHB5 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr1",
                    description: Some(
                        "RCC APB1 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 0x74,
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
                        "RCC APB1 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 0x78,
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
                    name: "apb7rstr",
                    description: Some(
                        "RCC APB7 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripheral clock enable register",
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
                        "RCC AHB2 peripheral clock enable register",
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
                    name: "ahb4enr",
                    description: Some(
                        "RCC AHB4 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x94,
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
                    name: "ahb5enr",
                    description: Some(
                        "RCC AHB5 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr1",
                    description: Some(
                        "RCC APB1 peripheral clock enable register 1",
                    ),
                    array: None,
                    byte_offset: 0x9c,
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
                        "RCC APB1 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 0xa0,
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
                        "RCC APB2 peripheral clock enable register",
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
                    name: "apb7enr",
                    description: Some(
                        "RCC APB7 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1smenr",
                    description: Some(
                        "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
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
                        "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
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
                    name: "ahb4smenr",
                    description: Some(
                        "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb4smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb5smenr",
                    description: Some(
                        "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb5smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr1",
                    description: Some(
                        "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
                    ),
                    array: None,
                    byte_offset: 0xc4,
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
                        "RCC APB1 peripheral clocks enable in Sleep and Stop modes \tregister 2",
                    ),
                    array: None,
                    byte_offset: 0xc8,
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
                        "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
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
                    name: "apb7smenr",
                    description: Some(
                        "RCC APB7 peripheral clock enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb7smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC peripherals independent clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0xe0,
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
                        "RCC peripherals independent clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0xe4,
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
                        "RCC peripherals independent clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0xe8,
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
                    name: "bdcr",
                    description: Some(
                        "RCC backup domain control register",
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
                    name: "csr",
                    description: Some(
                        "RCC control/status register",
                    ),
                    array: None,
                    byte_offset: 0xf4,
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
                    name: "bdcr2",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "RCC secure configuration register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "RCC privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascr",
                    description: Some(
                        "RCC audio synchronization control register",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "asier",
                    description: Some(
                        "RCC audio synchronization interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Asier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "assr",
                    description: Some(
                        "RCC audio synchronization status register",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Assr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascntr",
                    description: Some(
                        "RCC audio synchronization counter register",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "asarr",
                    description: Some(
                        "RCC audio synchronization auto-reload register",
                    ),
                    array: None,
                    byte_offset: 0x1d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Asarr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascar",
                    description: Some(
                        "RCC audio synchronization capture register",
                    ),
                    array: None,
                    byte_offset: 0x1d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ascor",
                    description: Some(
                        "RCC audio synchronization compare register",
                    ),
                    array: None,
                    byte_offset: 0x1d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ascor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr4",
                    description: Some(
                        "RCC clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x200,
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
                    name: "radioenr",
                    description: Some(
                        "RCC RADIO peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Radioenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecscr1",
                    description: Some(
                        "RCC external clock sources calibration register 1",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecscr1",
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
                "RCC AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 bus clock enable\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "flashen",
                    description: Some(
                        "FLASH bus clock enable\r Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.\r Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "CRC bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Touch sensing controller bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gtzc1en",
                    description: Some(
                        "GTZC1 bus clock enable \r Set and reset by software.\r Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram1en",
                    description: Some(
                        "SRAM1 bus clock enable \r Set and reset by software.\r Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 reset\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "crcrst",
                    description: Some(
                        "CRC reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TSC reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb1smenr",
            extends: None,
            description: Some(
                "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1smen",
                    description: Some(
                        "GPDMA1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "flashsmen",
                    description: Some(
                        "FLASH bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "crcsmen",
                    description: Some(
                        "CRC bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TSC bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                    name: "ramcfgsmen",
                    description: Some(
                        "RAMCFG bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gtzc1smen",
                    description: Some(
                        "GTZC1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "icachesmen",
                    description: Some(
                        "ICACHE bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "IO port A bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port B bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port C bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port D bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port E bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogen",
                    description: Some(
                        "IO port G bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port H bus clock enable\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hsen",
                    description: Some(
                        "USB OTG_HS bus and kernel clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hs_phyen",
                    description: Some(
                        "USB OTG_HS PHY kernel clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "aesen",
                    description: Some(
                        "AES bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HASH bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "RNG bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saesen",
                    description: Some(
                        "SAES bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsemen",
                    description: Some(
                        "HSEM bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkaen",
                    description: Some(
                        "PKA bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram2en",
                    description: Some(
                        "SRAM2 bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port A reset\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port B reset\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port C reset\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port D reset\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port E reset\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port H reset\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hsrst",
                    description: Some(
                        "USB OTG_HS reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "AES hardware accelerator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Hash reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Random number generator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saesrst",
                    description: Some(
                        "SAES hardware accelerator reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsemrst",
                    description: Some(
                        "HSEM hardware accelerator reset\r Set and cleared by software.\r Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkarst",
                    description: Some(
                        "PKA reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb2smenr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port B bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port C bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port D bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port E bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "gpiogsmen",
                    description: Some(
                        "IO port G bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "IO port H bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hssmen",
                    description: Some(
                        "USB OTG_HS bus and kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usb_otg_hs_physmen",
                    description: Some(
                        "USB OTG_HS PHY kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "aessmen",
                    description: Some(
                        "AES bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HASH bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "saessmen",
                    description: Some(
                        "SAES accelerator bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "pkasmen",
                    description: Some(
                        "PKA bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb4enr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwren",
                    description: Some(
                        "PWR bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "adc4en",
                    description: Some(
                        "ADC4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb4rstr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc4rst",
                    description: Some(
                        "ADC4 reset\r Set and cleared by software.\r Access can be secred by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Ahb4smenr",
            extends: None,
            description: Some(
                "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwrsmen",
                    description: Some(
                        "PWR bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "adc4smen",
                    description: Some(
                        "ADC4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            name: "Ahb5enr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radioen",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked.\r Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop).",
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
                    name: "ptaconven",
                    description: Some(
                        "PTACONV bus clock enable",
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
            name: "Ahb5rstr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radiorst",
                    description: Some(
                        "2.4 GHz RADIO reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ptaconvrst",
                    description: Some(
                        "PTACONV reset.",
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
            name: "Ahb5smenr",
            extends: None,
            description: Some(
                "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "radiosmen",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active.\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "ptaconvsmen",
                    description: Some(
                        "PTACONV bus clock enable during Sleep and Stop modes.",
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
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "wwdgen",
                    description: Some(
                        "WWDG bus clock enable\r Set by software to enable the window watchdog bus clock. Reset by hardware system reset.\r This bit can also be set by hardware if the WWDG_SW option bit is reset.\r Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2en",
                    description: Some(
                        "USART2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                        "USART3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART3SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..",
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
                        "I2C1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4en",
                    description: Some(
                        "I2C4 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC APB1 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM4 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2rst",
                    description: Some(
                        "USART2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "USART3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC UART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "I2C2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset.",
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
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "wwdgsmen",
                    description: Some(
                        "Window watchdog bus clock enable during Sleep and Stop modes\r Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.\r Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI2 bus and kernel clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "usart2smen",
                    description: Some(
                        "USART2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                        "USART3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                        "I2C2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            name: "Apb1smenr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clocks enable in Sleep and Stop modes \tregister 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4smen",
                    description: Some(
                        "I2C4 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lptim2smen",
                    description: Some(
                        "LPTIM2 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                "RCC APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "USART1bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim16en",
                    description: Some(
                        "TIM16 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM17 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SAI1 bus and kernel clocks enable",
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
                        "TIM1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "USART1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "tim16rst",
                    description: Some(
                        "TIM16 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM17 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SAI1 reset.",
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
            name: "Apb2smenr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1smen",
                    description: Some(
                        "TIM1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SPI1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "usart1smen",
                    description: Some(
                        "USART1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "tim16smen",
                    description: Some(
                        "TIM16 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "TIM17 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "SAI1 bus and kernel clocks enable during Sleep and Stop modes.",
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
            name: "Apb7enr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG bus clock enable\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3en",
                    description: Some(
                        "SPI3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "i2c3en",
                    description: Some(
                        "I2C3 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 bus and kernel clocks enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "compen",
                    description: Some(
                        "COMP bus clock enable",
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
                    name: "vrefen",
                    description: Some(
                        "VREFBUF bus clock enable\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "RTC and TAMP bus clock enable\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb7rstr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SYSCFG reset\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "comprst",
                    description: Some(
                        "COMP reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC COMPSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "vrefrst",
                    description: Some(
                        "VREF reset\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Apb7smenr",
            extends: None,
            description: Some(
                "RCC APB7 peripheral clock enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsmen",
                    description: Some(
                        "SYSCFG bus clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "spi3smen",
                    description: Some(
                        "SPI3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lpuart1smen",
                    description: Some(
                        "LPUART1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "lptim1smen",
                    description: Some(
                        "LPTIM1 bus and kernel clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "compsmen",
                    description: Some(
                        "COMP bus clock enable during Sleep and Stop modes.",
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
                    name: "vrefsmen",
                    description: Some(
                        "VREFBUF clock enable during Sleep and Stop modes\r Set and cleared by software.\r Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC and TAMP APB clock enable during Sleep and Stop modes\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
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
            name: "Asarr",
            extends: None,
            description: Some(
                "RCC audio synchronization auto-reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ar",
                    description: Some(
                        "Auto-reload value\r This field is set by software.\r CA[19:0] is the counter auto-reload value at which to restart the audio synchronization counter from value 0. It defines the counter period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascar",
            extends: None,
            description: Some(
                "RCC audio synchronization capture register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ca",
                    description: Some(
                        "Capture value\r This field is set by hardware.\r CA[26:20] is the capture period counter value loaded on the trigger event. CA[19:0] is the audio synchronization counter value loaded on the trigger event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 27,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascntr",
            extends: None,
            description: Some(
                "RCC audio synchronization counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value\r This field is set by hardware.\r CNT[19:0] is the counter value at the time this register is read.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascor",
            extends: None,
            description: Some(
                "RCC audio synchronization compare register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "co",
                    description: Some(
                        "Compare value\r This field is set by software.\r CO[19:0] is the value to be compared to the audio synchronization counter to generate an compare interrupt event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ascr",
            extends: None,
            description: Some(
                "RCC audio synchronization control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some(
                        "Counter enable\r This bit is set and cleared by software.\r Clearing this bit will reset the audio synchronization counter and capture prescaler and all associated registers ASCR, ASIER, ASSR, ASCNTR, ASARR, ASCAR, and ASCOR.",
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
                    name: "psc",
                    description: Some(
                        "Clock prescaler\r This field is set and cleared by software.\r Counter clock frequency = f_audiosync_ker_ck / (PSC + 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cps",
                    description: Some(
                        "Capture prescaler\r This field is set and cleared by software.\r Capture period in number of counter periods. Capture period = counter period * (TPS + 1)",
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
            name: "Asier",
            extends: None,
            description: Some(
                "RCC audio synchronization interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caie",
                    description: Some(
                        "Capture trigger interrupt enable\r This bit is set and cleared by software.",
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
                    name: "coie",
                    description: Some(
                        "Comparer interrupt enable\r This field is set and cleared by software.",
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
                    name: "caeie",
                    description: Some(
                        "Capture error interrupt enable\r This field is set and cleared by software.",
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
            ],
        },
        FieldSet {
            name: "Assr",
            extends: None,
            description: Some(
                "RCC audio synchronization status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caf",
                    description: Some(
                        "Capture trigger interrupt flag\r This field is set by hardware, only when CAIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAIE is 0.",
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
                    name: "cof",
                    description: Some(
                        "Comparer interrupt flag\r This field is set by hardware, only when COIE is enabled. This bit is cleared by software by writing it to 0 or masked when COIE is 0.",
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
                    name: "caef",
                    description: Some(
                        "Capture error interrupt flag\r This field is set by hardware, only when CAEIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAEIE is 0.",
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
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable\r Set and cleared by software.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator ready\r Set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator bypass\r Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE oscillator drive capability\r Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in ‘Xtal’ mode.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode.",
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
                        "Low speed external clock security enable\r Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable the LSECSSON bit.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Low speed external clock security, LSE failure Detection\r Set by hardware to indicate when a failure is detected by the LSECCS on the external 32�kHz oscillator.\r Reset when LSCSSON bit is cleared.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsesysen",
                    description: Some(
                        "LSE system clock (LSESYS) enable\r Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "rtcsel",
                    description: Some(
                        "RTC and TAMP kernel clock source enable and selection\r Set by software to enable and select the clock source for the RTC.\r Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsesysrdy",
                    description: Some(
                        "LSE system clock (LSESYS) ready\r Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles.\r The LSE clock must be already enabled and stable (LSEON and LSERDY are set). \r When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsegfon",
                    description: Some(
                        "LSE clock glitch filter enable\r Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsetrim",
                    description: Some(
                        "LSE trimming\r These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value.\r Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY�= 0).\r Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsetrim",
                    ),
                },
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset\r Set and cleared by software.\r Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "radiostsel",
                    description: Some(
                        "2.4 GHz RADIO sleep timer kernel clock enable and selection\r Set and cleared by software.\r Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Radiostsel",
                    ),
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.\r Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "Low-speed clock output selection\r Set and cleared by software.\r Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1on",
                    description: Some(
                        "LSI1 oscillator enable\r Set and cleared by software.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1rdy",
                    description: Some(
                        "LSI1 oscillator ready\r Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi1prediv",
                    description: Some(
                        "LSI1 Low-speed clock divider configuration\r Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC.\r Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsiprediv",
                    ),
                },
                Field {
                    name: "lsi2on",
                    description: Some(
                        "LSI2 oscillator enable",
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
                    name: "lsi2rdy",
                    description: Some(
                        "LSI2 oscillator ready.",
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
            name: "Bdcr2",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi2mode",
                    description: Some(
                        "LSI2 oscillator operating mode configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lsi2mode",
                    ),
                },
                Field {
                    name: "lsi2cfg",
                    description: Some(
                        "LSI2 oscillator configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Lsi2cfg",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r This bits are used to select the USART1 kernel clock source.\r Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
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
                        "USART2 kernel clock source selection\r This bits are used to select the USART2 kernel clock source.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
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
                        "USART3 kernel clock source selection\r This bits are used to select the USART3 kernel clock source.\r Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE.",
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
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection\r These bits are used to select the I2C1 kernel clock source.\r Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
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
                        "I2C2 kernel clock source selection\r These bits are used to select the I2C2 kernel clock source.\r Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
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
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 kernel clock source selection\r These bits are used to select the I2C4 kernel clock source.\r Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2c1sel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r These bits are used to select the SPI2 kernel clock source.\r Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi2sel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "Low-power timer 2 kernel clock source selection\r These bits are used to select the LPTIM2 kernel clock source.\r Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim2sel",
                    ),
                },
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r These bits are used to select the SPI1 kernel clock source.\r Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi1sel",
                    ),
                },
                Field {
                    name: "systicksel",
                    description: Some(
                        "SysTick clock source selection\r These bits are used to select the SysTick clock source.\r Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture \r When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI/256. \r When TIMICSEL is cleared, the HSI, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture.\r Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.",
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
                "RCC peripherals independent clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 kernel clock source selection\r These bits allow to select the SAI1 kernel clock source.\r Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sai1sel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNGSEL kernel clock source selection\r These bits allow to select the RNG kernel clock source.\r Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "otghssel",
                    description: Some(
                        "USB OTG_HS PHY kernel clock source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Otghssel",
                    ),
                },
                Field {
                    name: "assel",
                    description: Some(
                        "RCC audio synchronization kernel clock source selection\r This bit allows to select the audio synchronization kernel clock source.\r Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Assel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r These bits are used to select the LPUART1 kernel clock source.\r Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPUART1 is functional in Stop modes only when the kernel clock is HSI or LSE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r These bits are used to select the SPI3 kernel clock source.\r Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The SPI3 is functional in Stop modes only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spi3sel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 kernel clock source selection\r These bits are used to select the I2C3 kernel clock source.\r Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The I2C3 is functional in Stop modes only when the kernel clock is HSI",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
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
                        "LPTIM1 kernel clock source selection\r These bits are used to select the LPTIM1 kernel clock source.\r Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: The LPTIM1 is functional in Stop modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptim1sel",
                    ),
                },
                Field {
                    name: "adcsel",
                    description: Some(
                        "ADC4 kernel clock source selection\r These bits are used to select the ADC4 kernel clock source.\r Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r others: reserved\r Note: The ADC4 is functional in Stop modes only when the kernel clock is HSI.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "RCC clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock switch\r Set and cleared by software to select system clock source (SYSCLK).\r Cleared by hardware when entering Stop and Standby modes\r When selecting HSE directly or indirectly as system clock and HSE oscillator clock security fails, cleared by hardware.",
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
                        "system clock switch status\r Set and cleared by hardware to indicate which clock source is used as system clock.",
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
                    name: "mcosel",
                    description: Some(
                        "microcontroller clock output\r Set and cleared by software.\r others: reserved\r Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.",
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
                        "microcontroller clock output prescaler\r Set and cleared by software.\r It is highly recommended to change this prescaler before MCO output is enabled.\r others: not allowed",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB1, AHB2 and AHB4 prescaler\r Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1).\r The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.\r 0xx: hclk1 = SYSCLK not divided",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB1 prescaler\r Set and cleared by software to control the division factor of the APB1 clock (pclk1).\r 0xx: pclk1 = hclk1 not divided",
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
                        "APB2 prescaler\r Set and cleared by software to control the division factor of the APB2 clock (pclk2).\r 0xx: pclk2 = hclk1 not divided",
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
            name: "Cfgr3",
            extends: None,
            description: Some(
                "RCC clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppre7",
                    description: Some(
                        "APB7 prescaler\r Set and cleared by software to control the division factor of the APB7 clock (pclk7).\r 0xx: hclk1 not divided",
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
            name: "Cfgr4",
            extends: None,
            description: Some(
                "RCC clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre5",
                    description: Some(
                        "AHB5 prescaler when SWS select PLL1\r Set and cleared by software to control the division factor of the AHB5 clock (hclk5).\r Must not be changed when SYSCLK source indicated by SWS is PLL1.\r When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account.\r When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs\r Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.\r 0xx: hclk5 = SYSCLK not divided",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hpre5",
                    ),
                },
                Field {
                    name: "hdiv5",
                    description: Some(
                        "AHB5 divider when SWS select HSI or HSE\r Set and reset by software.\r Set to 1 by hardware when entering Stop 1 mode.\r When SYSCLK source indicated by SWS is HSI or HSE: HDIV5 is taken into account\r When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account\r Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hdiv5",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyc",
                    description: Some(
                        "LSI1 ready interrupt clear\r Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt clear\r Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt clear\r Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.\\\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE ready interrupt clear\r Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "PLL1 ready interrupt clear\r Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecssc",
                    description: Some(
                        "High speed external clock security system interrupt clear\r Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyc",
                    description: Some(
                        "LSI2 ready interrupt clear.",
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
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyie",
                    description: Some(
                        "LSI1 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "PLL1 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by PLL1 lock.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyie",
                    description: Some(
                        "LSI2 ready interrupt enable",
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
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsi1rdyf",
                    description: Some(
                        "LSI1 ready interrupt flag\r Set by hardware when the LSI1 clock becomes stable and LSI1RDYIE is set.\r Cleared by software setting the LSI1RDYC bit.\r Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "LSE ready interrupt flag\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.\r Cleared by software setting the LSERDYC bit.\r Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI ready interrupt flag\r Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Cleared by software setting the HSIRDYC bit.",
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
                        "HSE ready interrupt flag\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.\r Cleared by software setting the HSERDYC bit.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "PLL1 ready interrupt flag\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.\r Cleared by software setting the PLL1RDYC bit.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag\r Set by hardware when a clock security failure is detected in the HSE oscillator.\r Cleared by software setting the HSECSSC bit.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "lsi2rdyf",
                    description: Some(
                        "LSI2 ready interrupt flag.",
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
                        "HSI clock enable\r Set and cleared by software.\r Cleared by hardware when entering Stop and Standby modes. \r Set by hardware to force the HSI oscillator on when exiting Stop and Standby modes.\r Set by hardware to force the HSI oscillator on in case of clock security failure of the HSE crystal oscillator.\r This bit is set by hardware if the HSI is used directly or indirectly as system clock.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI enable for some peripheral kernels\r Set and cleared by software to force HSI oscillator on even in Stop modes. \r Keeping the HSI oscillator on in Stop modes allows the communication speed not to be reduced by the HSI oscillator startup time. This bit has no effect on register bit HSION value.\r Cleared by hardware when entering Standby modes. \r Refer to Peripherals clock gating and autonomous mode for more details.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSI clock ready flag\r Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION.\r Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.\r Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles.",
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
                    name: "hseon",
                    description: Some(
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE clock for the CPU when entering Stop and Standby modes and on a HSECSS failure.\r When the HSE is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode.\r This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable. This bit is set both when HSE is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable\r Set by software to enable the HSE clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "hsepre",
                    description: Some(
                        "HSE clock for SYSCLK prescaler\r Set and cleared by software to control the division factor of the HSE clock for SYSCLK.\r Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsepre",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable the main PLL.\r Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE is selected as sysclk, on a HSECSS failure.\r This bit cannot be reset if the PLL1 clock is used as the system clock.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.\r Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
            name: "Csr",
            extends: None,
            description: Some(
                "RCC control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag\r Set by software to clear the reset flags.\r Access can be secured by RCC RMVFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.",
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
                    name: "oblrstf",
                    description: Some(
                        "Option byte loader reset flag\r Set by hardware when a reset from the option byte loading occurs.\r Cleared by writing to the RMVF bit.",
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
                        "NRST pin reset flag\r Set by hardware when a reset from the NRST pin occurs.\r Cleared by writing to the RMVF bit.",
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
                        "BOR flag\r Set by hardware when a BOR occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Software reset flag\r Set by hardware when a software reset occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Independent watchdog reset flag\r Set by hardware when an independent watchdog reset domain occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Window watchdog reset flag\r Set by hardware when a window watchdog reset occurs.\r Cleared by writing to the RMVF bit.",
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
                        "Low-power reset flag\r Set by hardware when a reset occurs due to illegal Stop and Standby modes entry.\r Cleared by writing to the RMVF bit.",
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
            name: "Ecscr1",
            extends: None,
            description: Some(
                "RCC external clock sources calibration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsetrim",
                    description: Some(
                        "HSE clock trimming \r These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE oscillator frequency.",
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
            name: "Icscr3",
            extends: None,
            description: Some(
                "RCC internal clock sources calibration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r These bits are initialized at startup with the factory-programmed HSI calibration value. When HSITRIM[4:0] is written, HSICAL[11:0] is updated with the sum of HSITRIM[4:0] and the initial factory trim value.",
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
                        "HSI clock trimming \r These bits provide an additional user-programmable trimming value that is added to the HSICAL[11:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.",
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
            ],
        },
        FieldSet {
            name: "Pll1cfgr",
            extends: None,
            description: Some(
                "RCC PLL1 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL1 entry clock source\r Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled.\r Cleared by hardware when entering Stop or Standby modes. \r Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0.",
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
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1.\r This bit must be written before enabling the PLL1.\r 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz",
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
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of PLL1FRACN into the ΣΔ modulator.\r In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set\u{a0}to\u{a0}0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details).",
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
                    name: "pllm",
                    description: Some(
                        "Prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). \r ...",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1pclk output of the PLL1.\r To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. \r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1qclk output of the PLL1.\r To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. \r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                        "PLL1 DIVR divider output enable\r Set and cleared by software to enable the pll1rclk output of the PLL1.\r To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
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
                    name: "pllrclkpre",
                    description: Some(
                        "pll1rclk clock for SYSCLK prescaler division enable\r Set and cleared by software to control the division of the pll1rclk clock for SYSCLK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllrclkpre",
                    ),
                },
                Field {
                    name: "pllrclkprestep",
                    description: Some(
                        "pll1rclk clock for SYSCLK prescaler division step selection\r Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllrclkprestep",
                    ),
                },
                Field {
                    name: "pllrclkprerdy",
                    description: Some(
                        "pll1rclkpre not divided ready.\r Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre.",
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
            name: "Pll1divr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1 VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r others: reserved\r VCO output frequency = F<sub>ref1_ck</sub> x multiplication factor for PLL1 VCO, when fractional value 0 has been loaded into PLL1FRACN, with: \r Multiplication factor for PLL1 VCO between 4 and 512\r input frequency F<sub>ref1_ck</sub> between 4 and 16�MHz",
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
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1pclk clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
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
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the PLl1QCLK clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1rclk clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
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
            name: "Pll1fracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO.\r These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r VCO output frequency = F<sub>ref1_ck</sub> x [multiplication factor for PLL1 VCO + (PLL1FRACN / 2<sup>13</sup>)], with: \r Multiplication factor for PLL1 VCO must be between 4 and 512.\r PLL1FRACN can be between 0 and 2<sup>13</sup>- 1.\r The input frequency F<sub>ref1_ck</sub> must be between 4 and 16 MHz. \r To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows:\r Set the bit PLL1FRACEN to 0. \r Write the new fractional value into PLL1FRACN. \r Set the bit PLL1FRACEN to 1.",
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
            name: "Privcfgr",
            extends: None,
            description: Some(
                "RCC privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "RCC secure functions privilege configuration\r Set and reset by software.\r This bit can be written only by a secure privileged access.",
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
                    name: "nspriv",
                    description: Some(
                        "RCC non-secure functions privilege configuration\r Set and reset by software.\r This bit can be written only by privileged access, secure or non-secure.",
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
            name: "Radioenr",
            extends: None,
            description: Some(
                "RCC RADIO peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bbclken",
                    description: Some(
                        "2.4 GHz RADIO baseband kernel clock (aclk) enable\r Set and cleared by software.\r Note: The HSE oscillator needs to be enabled by either HSEON or STRADIOCLKON.",
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
                    name: "stradioclkon",
                    description: Some(
                        "2.4 GHz RADIO bus clock enable and HSE oscillator enable by 2.4 GHz RADIO sleep timer wakeup event\r Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event.\r Cleared by software writing zero to this bit.\r Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked.",
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
                    name: "radioclkrdy",
                    description: Some(
                        "2.4 GHz RADIO bus clock ready.\r Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed.\r Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles.",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "RCC secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsisec",
                    description: Some(
                        "HSI clock configuration and status bits security\r Set and reset by software.",
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
                    name: "hsesec",
                    description: Some(
                        "HSE clock configuration bits, status bits and HSECSS security\r Set and reset by software.",
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
                    name: "lsisec",
                    description: Some(
                        "LSI clock configuration and status bits security\r Set and reset by software.",
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
                    name: "lsesec",
                    description: Some(
                        "LSE clock configuration and status bits security\r Set and reset by software.",
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
                    name: "sysclksec",
                    description: Some(
                        "SYSCLK selection, clock output on MCO configuration security\r Set and reset by software.",
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
                    name: "prescsec",
                    description: Some(
                        "AHBx/APBx prescaler configuration bits security\r Set and reset by software.",
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
                    name: "pllsec",
                    description: Some(
                        "PLL1 clock configuration and status bits security\r Set and reset by software.",
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
                    name: "rmvfsec",
                    description: Some(
                        "Remove reset flag security\r Set and reset by software.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Hclk4",
                    description: Some(
                        "hclk4 clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1pclk selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Assel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1pclk selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1qclk selected.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hdiv5",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "hclk5 = SYSCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "hclk5 = SYSCLK divided by 2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some(
                        "hclk = SYSCLK divided by 16",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "hclk = SYSCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "hclk = SYSCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "hclk = SYSCLK divided by 8",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Hpre5",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "hclk5 = SYSCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div3",
                    description: Some(
                        "hclk5 = SYSCLK divided by 3",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "hclk5 = SYSCLK divided by 4",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Div6",
                    description: Some(
                        "hclk5 = SYSCLK divided by 6",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Hsepre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "HSE not divided, SYSCLK = HSE",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "HSE divided, SYSCLK = HSE/2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2c3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lptim1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pclk7",
                    description: Some(
                        "pclk7 selected.",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lptim2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "pclk7 selected.",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pclk7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
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
            name: "Lsetrim",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "R",
                    description: Some(
                        "current source resistance R",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "R23",
                    description: Some(
                        "current source resistance 2/3 x R",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "R34",
                    description: Some(
                        "current source resistance 3/4 x R",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "R54",
                    description: Some(
                        "current source resistance 5/4 x R",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lsi2cfg",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Sensitivity0at20c",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +20 less thansup oless than/sup C.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Sensitivity0at50c",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +50 less thansup oless than/sup C.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Sensitivity0at80c",
                    description: Some(
                        "LSI2 frequency temperature sensitivity is close to 0 at +80 less thansup oless than/sup C.",
                    ),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lsi2mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "LowPower",
                    description: Some(
                        "low-power, medium accuracy.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NominalPower",
                    description: Some(
                        "nominal-power, high accuracy.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "UltraLowPower",
                    description: Some(
                        "ultra-low-power, low accuracy.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lsiprediv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "LSI not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div128",
                    description: Some(
                        "LSI divided by 128",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "MCO divided by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some(
                        "MCO divided by 16",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "MCO divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "MCO divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "MCO divided by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Disabled",
                    description: Some(
                        "MCO output disabled, no clock on MCO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hclk5",
                    description: Some(
                        "hclk5 clock selected",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1pclk clock selected",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1qclk clock selected",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "Pll1R",
                    description: Some(
                        "pll1rclk clock selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Sysclkpre",
                    description: Some(
                        "sysclkpre system clock after PLL1RCLKPRE division selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Otghssel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE32 selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HseDiv2",
                    description: Some(
                        "HSE32 divided by 2 selected.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1pclk selected,.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pll1PDiv2",
                    description: Some(
                        "pll1pclk divided by 2 selected.",
                    ),
                    value: 3,
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
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Div3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Div4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Div5",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Div6",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Div7",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Div8",
                    description: None,
                    value: 7,
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
            name: "Pllrclkpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some(
                        "pll1rclk not divided, sysclkpre = pll1rclk",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Divided",
                    description: Some(
                        "pll1rclk divided, sysclkpre = pll1rclk divided",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllrclkprestep",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Step2",
                    description: Some(
                        "pll1rclk 2-step division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Step3",
                    description: Some(
                        "pll1rclk 3-step division",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Freq4to8mhz",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Freq8to16mhz",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz",
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
                    name: "Disable",
                    description: Some(
                        "no clock sent to PLL1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE clock after HSEPRE divider selected as PLL1 clock entry",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI clock selected as PLL1 clock entry",
                    ),
                    value: 2,
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
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Div16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Radiostsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Disable",
                    description: Some(
                        "no clock selected, 2.4 GHz RADIO sleep timer kernel clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE oscillator clock divided by 1000 selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1qclk divide by 2 selected",
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
                    name: "Disable",
                    description: Some(
                        "no clock selected, RTC and TAMP kernel clock disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Hse",
                    description: Some(
                        "HSE oscillator clock divided by 32 selected, and enabled",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE oscillator clock selected, and enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI oscillator clock selected, and enabled",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Sai1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Audioclk",
                    description: Some(
                        "input pin AUDIOCLK selected.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI16 clock selected.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Pll1P",
                    description: Some(
                        "pll1pclk selected.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Pll1Q",
                    description: Some(
                        "pll1qclk selected.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Spi1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk2",
                    description: Some(
                        "pclk2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spi2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spi3sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Pclk7",
                    description: Some(
                        "pclk7 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
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
                    name: "Hse",
                    description: Some(
                        "HSE or HSE/2, as defined by HSEPRE, selected as system clock",
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
                    name: "Pll1R",
                    description: Some(
                        "pll1rclk selected as system clock",
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
                        "hclk1 divided by 8 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lsi",
                    description: Some(
                        "LSI selected",
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
                    name: "Hsi",
                    description: Some(
                        "HSI divider disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HsiDiv256",
                    description: Some(
                        "HSI/256 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
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
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pclk2",
                    description: Some(
                        "pclk2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Hsi",
                    description: Some(
                        "HSI selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Lse",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Pclk1",
                    description: Some(
                        "pclk1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Sys",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
