
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Gpu2d",
        extends: None,
        description: Some("2D graphics accelerator."),
        items: &[
            BlockItem {
                name: "breakpoint",
                description: Some("Breakpoint register."),
                array: None,
                byte_offset: 0x80,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Breakpoint"),
                }),
            },
            BlockItem {
                name: "itctrl",
                description: Some("Interrupt control register."),
                array: None,
                byte_offset: 0xf8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Itctrl"),
                }),
            },
            BlockItem {
                name: "clid",
                description: Some("Last command list identifier register."),
                array: None,
                byte_offset: 0x148,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Clid"),
                }),
            },
            BlockItem {
                name: "sys_interrupt",
                description: Some("System interrupt register."),
                array: None,
                byte_offset: 0xff8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("SysInterrupt"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Breakpoint",
            extends: None,
            description: Some("Breakpoint register."),
            bit_size: 32,
            fields: &[Field {
                name: "value",
                description: Some("Breakpoint value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Clid",
            extends: None,
            description: Some("Last command list identifier register."),
            bit_size: 32,
            fields: &[Field {
                name: "id",
                description: Some("Last completed command list identifier."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Itctrl",
            extends: None,
            description: Some("Interrupt control register."),
            bit_size: 32,
            fields: &[Field {
                name: "clc",
                description: Some("Command list complete interrupt and flag."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "SysInterrupt",
            extends: None,
            description: Some("System interrupt register."),
            bit_size: 32,
            fields: &[Field {
                name: "er",
                description: Some("Error interrupt status."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
