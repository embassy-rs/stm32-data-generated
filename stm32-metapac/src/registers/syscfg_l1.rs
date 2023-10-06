
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Syscfg",
        extends: None,
        description: Some("System configuration controller"),
        items: &[
            BlockItem {
                name: "memrmp",
                description: Some("memory remap register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Memrmp"),
                }),
            },
            BlockItem {
                name: "pmc",
                description: Some("peripheral mode configuration register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pmc"),
                }),
            },
            BlockItem {
                name: "exticr",
                description: Some("external interrupt configuration register 1"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Exticr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Pmc",
            extends: None,
            description: Some("peripheral mode configuration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb_pu",
                    description: Some("USB pull-up"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lcd_capa",
                    description: Some("USB pull-up enable on DP line"),
                    bit_offset: 1,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exticr",
            extends: None,
            description: Some("external interrupt configuration register 3"),
            bit_size: 32,
            fields: &[Field {
                name: "exti",
                description: Some("EXTI x configuration (x = 8 to 11)"),
                bit_offset: 0,
                bit_size: 4,
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Memrmp",
            extends: None,
            description: Some("memory remap register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some("MEM_MODE"),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boot_mode",
                    description: Some("BOOT_MODE"),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
