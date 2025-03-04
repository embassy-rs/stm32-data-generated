
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Syscfg",
        extends: None,
        description: Some("System configuration controller"),
        items: &[
            BlockItem {
                name: "pmcr",
                description: Some("peripheral mode configuration register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pmcr"),
                }),
            },
            BlockItem {
                name: "exticr",
                description: Some("external interrupt configuration register"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Exticr"),
                }),
            },
            BlockItem {
                name: "cccsr",
                description: Some("compensation cell control/status register"),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cccsr"),
                }),
            },
            BlockItem {
                name: "ccvr",
                description: Some("SYSCFG compensation cell value register"),
                array: None,
                byte_offset: 0x24,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ccvr"),
                }),
            },
            BlockItem {
                name: "cccr",
                description: Some("SYSCFG compensation cell code register"),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cccr"),
                }),
            },
            BlockItem {
                name: "pkgr",
                description: Some("SYSCFG package register"),
                array: None,
                byte_offset: 0x124,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Pkgr"),
                }),
            },
            BlockItem {
                name: "ur0",
                description: Some("SYSCFG user register 0"),
                array: None,
                byte_offset: 0x300,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur0"),
                }),
            },
            BlockItem {
                name: "ur2",
                description: Some("SYSCFG user register 2"),
                array: None,
                byte_offset: 0x308,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ur2"),
                }),
            },
            BlockItem {
                name: "ur3",
                description: Some("SYSCFG user register 3"),
                array: None,
                byte_offset: 0x30c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ur3"),
                }),
            },
            BlockItem {
                name: "ur4",
                description: Some("SYSCFG user register 4"),
                array: None,
                byte_offset: 0x310,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur4"),
                }),
            },
            BlockItem {
                name: "ur5",
                description: Some("SYSCFG user register 5"),
                array: None,
                byte_offset: 0x314,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur5"),
                }),
            },
            BlockItem {
                name: "ur6",
                description: Some("SYSCFG user register 6"),
                array: None,
                byte_offset: 0x318,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur6"),
                }),
            },
            BlockItem {
                name: "ur7",
                description: Some("SYSCFG user register 7"),
                array: None,
                byte_offset: 0x31c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur7"),
                }),
            },
            BlockItem {
                name: "ur8",
                description: Some("SYSCFG user register 8"),
                array: None,
                byte_offset: 0x320,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur8"),
                }),
            },
            BlockItem {
                name: "ur9",
                description: Some("SYSCFG user register 9"),
                array: None,
                byte_offset: 0x324,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur9"),
                }),
            },
            BlockItem {
                name: "ur10",
                description: Some("SYSCFG user register 10"),
                array: None,
                byte_offset: 0x328,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur10"),
                }),
            },
            BlockItem {
                name: "ur11",
                description: Some("SYSCFG user register 11"),
                array: None,
                byte_offset: 0x32c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur11"),
                }),
            },
            BlockItem {
                name: "ur12",
                description: Some("SYSCFG user register 12"),
                array: None,
                byte_offset: 0x330,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur12"),
                }),
            },
            BlockItem {
                name: "ur13",
                description: Some("SYSCFG user register 13"),
                array: None,
                byte_offset: 0x334,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur13"),
                }),
            },
            BlockItem {
                name: "ur14",
                description: Some("SYSCFG user register 14"),
                array: None,
                byte_offset: 0x338,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ur14"),
                }),
            },
            BlockItem {
                name: "ur15",
                description: Some("SYSCFG user register 15"),
                array: None,
                byte_offset: 0x33c,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur15"),
                }),
            },
            BlockItem {
                name: "ur16",
                description: Some("SYSCFG user register 16"),
                array: None,
                byte_offset: 0x340,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur16"),
                }),
            },
            BlockItem {
                name: "ur17",
                description: Some("SYSCFG user register 17"),
                array: None,
                byte_offset: 0x344,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur17"),
                }),
            },
            BlockItem {
                name: "ur18",
                description: Some("SYSCFG user register 18"),
                array: None,
                byte_offset: 0x348,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ur18"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some("SYSCFG compensation cell code register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncc",
                    description: Some("NMOS compensation code"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcc",
                    description: Some("PMOS compensation code"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cccsr",
            extends: None,
            description: Some("compensation cell control/status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cs",
                    description: Some("Code selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdy",
                    description: Some("Compensation cell ready flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hslv",
                    description: Some("High-speed at low-voltage"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccvr",
            extends: None,
            description: Some("SYSCFG compensation cell value register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncv",
                    description: Some("NMOS compensation value"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcv",
                    description: Some("PMOS compensation value"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some("external interrupt configuration register 2"),
            bit_size: 32,
            fields: &[Field {
                name: "exti",
                description: Some("EXTI x configuration (x = 4 to 7)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pkgr",
            extends: None,
            description: Some("SYSCFG package register"),
            bit_size: 32,
            fields: &[Field {
                name: "pkg",
                description: Some("Package"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Pmcr",
            extends: None,
            description: Some("peripheral mode configuration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c1fmp",
                    description: Some("I2C1 Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2fmp",
                    description: Some("I2C2 Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3fmp",
                    description: Some("I2C3 Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c4fmp",
                    description: Some("I2C4 Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6fmp",
                    description: Some("PB(6) Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7fmp",
                    description: Some("PB(7) Fast Mode Plus"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8fmp",
                    description: Some("PB(8) Fast Mode Plus"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9fmp",
                    description: Some("PB(9) Fm+"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "booste",
                    description: Some("Booster Enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boostvddsel",
                    description: Some("Analog switch supply voltage selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eth_sel_phy",
                    description: Some("Ethernet PHY interface selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("EthSelPhy"),
                },
                Field {
                    name: "pa0so",
                    description: Some("PA0 Switch Open"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa1so",
                    description: Some("PA1 Switch Open"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pc2so",
                    description: Some("PC2 Switch Open"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pc3so",
                    description: Some("PC3 Switch Open"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 27 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur0",
            extends: None,
            description: Some("SYSCFG user register 0"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bks",
                    description: Some("Bank Swap"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdp",
                    description: Some("Readout protection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur10",
            extends: None,
            description: Some("SYSCFG user register 10"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_end_2",
                    description: Some("Protected area end address for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa_beg_2",
                    description: Some("Secured area start address for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur11",
            extends: None,
            description: Some("SYSCFG user register 11"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa_end_2",
                    description: Some("Secured area end address for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg1m",
                    description: Some("Independent Watchdog 1 mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur12",
            extends: None,
            description: Some("SYSCFG user register 12"),
            bit_size: 32,
            fields: &[Field {
                name: "secure",
                description: Some("Secure mode"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur13",
            extends: None,
            description: Some("SYSCFG user register 13"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdrs",
                    description: Some("Secured DTCM RAM Size"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "d1sbrst",
                    description: Some("D1 Standby reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur14",
            extends: None,
            description: Some("SYSCFG user register 14"),
            bit_size: 32,
            fields: &[Field {
                name: "d1stprst",
                description: Some("D1 Stop Reset"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur15",
            extends: None,
            description: Some("SYSCFG user register 15"),
            bit_size: 32,
            fields: &[Field {
                name: "fziwdgstb",
                description: Some("Freeze independent watchdog in Standby mode"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur16",
            extends: None,
            description: Some("SYSCFG user register 16"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fziwdgstp",
                    description: Some("Freeze independent watchdog in Stop mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pkp",
                    description: Some("Private key programmed"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur17",
            extends: None,
            description: Some("SYSCFG user register 17"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "io_hslv",
                    description: Some("I/O high speed / low voltage"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcm_axi_shared_cfg",
                    description: Some("ITCM-RAM / AXI-SRAM size"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("ItcmAxiRamSize"),
                },
            ],
        },
        FieldSet {
            name: "Ur18",
            extends: None,
            description: Some("SYSCFG user register 18"),
            bit_size: 32,
            fields: &[Field {
                name: "cpu_freq_boost",
                description: Some("CPU maximum frequency boost enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur2",
            extends: None,
            description: Some("SYSCFG user register 2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "borh",
                    description: Some("BOR_LVL Brownout Reset Threshold Level"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boot_add0",
                    description: Some("Boot Address 0"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur3",
            extends: None,
            description: Some("SYSCFG user register 3"),
            bit_size: 32,
            fields: &[Field {
                name: "boot_add1",
                description: Some("Boot Address 1"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur4",
            extends: None,
            description: Some("SYSCFG user register 4"),
            bit_size: 32,
            fields: &[Field {
                name: "mepad_1",
                description: Some("Mass Erase Protected Area Disabled for bank 1"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ur5",
            extends: None,
            description: Some("SYSCFG user register 5"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mesad_1",
                    description: Some("Mass erase secured area disabled for bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrpn_1",
                    description: Some("Write protection for flash bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur6",
            extends: None,
            description: Some("SYSCFG user register 6"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_beg_1",
                    description: Some("Protected area start address for bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa_end_1",
                    description: Some("Protected area end address for bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur7",
            extends: None,
            description: Some("SYSCFG user register 7"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa_beg_1",
                    description: Some("Secured area start address for bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa_end_1",
                    description: Some("Secured area end address for bank 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur8",
            extends: None,
            description: Some("SYSCFG user register 8"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mepad_2",
                    description: Some("Mass erase protected area disabled for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mesad_2",
                    description: Some("Mass erase secured area disabled for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ur9",
            extends: None,
            description: Some("SYSCFG user register 9"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpn_2",
                    description: Some("Write protection for flash bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa_beg_2",
                    description: Some("Protected area start address for bank 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "EthSelPhy",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MII_GMII",
                    description: Some("GMII or MII"),
                    value: 0,
                },
                EnumVariant {
                    name: "RMII",
                    description: Some("RMII"),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "ItcmAxiRamSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ITCM64AXI320",
                    description: Some("64 Kbyte ITCM-RAM / 320 Kbyte AXI-SRAM"),
                    value: 0,
                },
                EnumVariant {
                    name: "ITCM128AXI256",
                    description: Some("128 Kbyte ITCM-RAM / 256 Kbyte AXI-SRAM"),
                    value: 1,
                },
                EnumVariant {
                    name: "ITCM192AXI192",
                    description: Some("192 Kbyte ITCM-RAM / 192 Kbyte AXI-SRAM"),
                    value: 2,
                },
                EnumVariant {
                    name: "ITCM256AXI128",
                    description: Some("256 Kbyte ITCM-RAM / 128 Kbyte AXI-SRAM"),
                    value: 3,
                },
            ],
        },
    ],
};
