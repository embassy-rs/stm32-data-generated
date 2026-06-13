
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Ccb",
        extends: None,
        description: Some("Coupling and chaining bridge"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("CCB control register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("CCB status register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "reftagr",
                description: Some("CCB reference tag register"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("CCB control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccop",
                    description: Some("Coupling and chaining operation"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iprst",
                    description: Some("CCB reset"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("CCB status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opstep",
                    description: Some("Operation step"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "operr",
                    description: Some("Operation error"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some("CCB busy"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tamp_evt",
                    description: Some("Tamper event flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
