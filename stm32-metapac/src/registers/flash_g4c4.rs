
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Flash",
        extends: None,
        description: Some("Flash"),
        items: &[
            BlockItem {
                name: "acr",
                description: Some("Access control register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Acr"),
                }),
            },
            BlockItem {
                name: "pdkeyr",
                description: Some("Power down key register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "keyr",
                description: Some("Flash key register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "optkeyr",
                description: Some("Option byte key register"),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Flash control register"),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "eccr",
                description: Some("Flash ECC register"),
                array: None,
                byte_offset: 0x18,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Eccr"),
                }),
            },
            BlockItem {
                name: "optr",
                description: Some("Flash option register"),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Optr"),
                }),
            },
            BlockItem {
                name: "pcrop1sr",
                description: Some("Flash Bank 1 PCROP Start address register"),
                array: None,
                byte_offset: 0x24,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pcrop1sr"),
                }),
            },
            BlockItem {
                name: "pcrop1er",
                description: Some("Flash Bank 1 PCROP End address register"),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pcrop1er"),
                }),
            },
            BlockItem {
                name: "wrp1ar",
                description: Some("Flash Bank 1 WRP area A address register"),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Wrp1ar"),
                }),
            },
            BlockItem {
                name: "wrp1br",
                description: Some("Flash Bank 1 WRP area B address register"),
                array: None,
                byte_offset: 0x30,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Wrp1br"),
                }),
            },
            BlockItem {
                name: "sec1r",
                description: Some("securable area bank1 register"),
                array: None,
                byte_offset: 0x70,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sec1r"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some("Access control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some("Latency"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Latency"),
                },
                Field {
                    name: "prften",
                    description: Some("Prefetch enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icen",
                    description: Some("Instruction cache enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcen",
                    description: Some("Data cache enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icrst",
                    description: Some("Instruction cache reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrst",
                    description: Some("Data cache reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "run_pd",
                    description: Some("Flash Power-down mode during Low-power run mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sleep_pd",
                    description: Some("Flash Power-down mode during Low-power sleep mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_swen",
                    description: Some("Debug software enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Flash control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some("Programming"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some("Page erase"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer1",
                    description: Some("Bank 1 Mass erase"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pnb",
                    description: Some("Page number"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some("Start"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optstrt",
                    description: Some("Options modification start"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fstpg",
                    description: Some("Fast programming"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some("End of operation interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errie",
                    description: Some("Error interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderrie",
                    description: Some("PCROP read error interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "obl_launch",
                    description: Some("Force the option byte loading"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 27 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sec_prot1",
                    description: Some("Securable memory area protection enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optlock",
                    description: Some("Options Lock"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("FLASH_CR Lock"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccr",
            extends: None,
            description: Some("Flash ECC register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some("ECC fail address"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bk_ecc",
                    description: Some("ECC fail for Corrected ECC Error or Double ECC Error in info block"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_ecc",
                    description: Some("ECC fail for Corrected ECC Error or Double ECC Error in info block"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccie",
                    description: Some("ECC correction interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccc2",
                    description: Some("ECC correction"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccd2",
                    description: Some("ECC2 detection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccc",
                    description: Some("ECC correction"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccd",
                    description: Some("ECC detection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some("Flash option register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdp",
                    description: Some("Read protection level"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: Some("Rdp"),
                },
                Field {
                    name: "bor_lev",
                    description: Some("BOR reset Level"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stop",
                    description: Some("nRST_STOP"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_stdby",
                    description: Some("nRST_STDBY"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_rst_shdw",
                    description: Some("nRST_SHDW"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idwg_sw",
                    description: Some("Independent watchdog selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stop",
                    description: Some("Independent watchdog counter freeze in Stop mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some("Independent watchdog counter freeze in Standby mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdg_sw",
                    description: Some("Window watchdog selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb4_pupen",
                    description: Some("PB4 pull-up enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot1",
                    description: Some("Boot configuration"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_pe",
                    description: Some("SRAM2 parity check enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_rst",
                    description: Some("SRAM2 Erase when system reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_swboot0",
                    description: Some("nSWBOOT0"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_boot0",
                    description: Some("nBOOT0 option bit"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 27 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nrst_mode",
                    description: Some("NRST_MODE"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("NrstMode"),
                },
                Field {
                    name: "irhen",
                    description: Some("Internal reset holder enable bit"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1er",
            extends: None,
            description: Some("Flash Bank 1 PCROP End address register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcrop1_end",
                    description: Some("Bank 1 PCROP area end offset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcrop_rdp",
                    description: Some("PCROP area preserved when RDP level decreased"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcrop1sr",
            extends: None,
            description: Some("Flash Bank 1 PCROP Start address register"),
            bit_size: 32,
            fields: &[Field {
                name: "pcrop1_strt",
                description: Some("Bank 1 PCROP area start offset"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 15,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sec1r",
            extends: None,
            description: Some("securable area bank1 register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_size1",
                    description: Some("SEC_SIZE1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boot_lock",
                    description: Some("used to force boot from user area"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some("End of operation"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "operr",
                    description: Some("Operation error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "progerr",
                    description: Some("Programming error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrperr",
                    description: Some("Write protected error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgaerr",
                    description: Some("Programming alignment error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sizerr",
                    description: Some("Size error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgserr",
                    description: Some("Programming sequence error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "miserr",
                    description: Some("Fast programming data miss error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fasterr",
                    description: Some("Fast programming error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some("PCROP read error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optverr",
                    description: Some("Option validity error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsy",
                    description: Some("Busy"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1ar",
            extends: None,
            description: Some("Flash Bank 1 WRP area A address register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1a_strt",
                    description: Some("Bank 1 WRP first area start offset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1a_end",
                    description: Some("Bank 1 WRP first area A end offset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp1br",
            extends: None,
            description: Some("Flash Bank 1 WRP area B address register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1b_strt",
                    description: Some("Bank 1 WRP second area B end offset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1b_end",
                    description: Some("Bank 1 WRP second area B start offset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Latency",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "WS0",
                    description: Some("Zero wait states"),
                    value: 0,
                },
                EnumVariant {
                    name: "WS1",
                    description: Some("One wait state"),
                    value: 1,
                },
                EnumVariant {
                    name: "WS2",
                    description: Some("Two wait states"),
                    value: 2,
                },
                EnumVariant {
                    name: "WS3",
                    description: Some("Three wait states"),
                    value: 3,
                },
                EnumVariant {
                    name: "WS4",
                    description: Some("Four wait states"),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "NrstMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INPUT_ONLY",
                    description: Some("Reset pin is in reset input mode only"),
                    value: 1,
                },
                EnumVariant {
                    name: "GPIO",
                    description: Some("Reset pin is in GPIO mode only"),
                    value: 2,
                },
                EnumVariant {
                    name: "INPUT_OUTPUT",
                    description: Some("Reset pin is in reset input and output mode"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rdp",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "LEVEL_0",
                    description: Some("Read protection not active"),
                    value: 170,
                },
                EnumVariant {
                    name: "LEVEL_1",
                    description: Some("Memories read protection active"),
                    value: 187,
                },
                EnumVariant {
                    name: "LEVEL_2",
                    description: Some("Chip read protection active"),
                    value: 204,
                },
            ],
        },
    ],
};
