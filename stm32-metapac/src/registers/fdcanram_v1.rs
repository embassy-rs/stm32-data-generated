
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Fdcanram",
        extends: None,
        description: Some("FDCAN Message RAM"),
        items: &[
            BlockItem {
                name: "flssa",
                description: Some("11-bit filter"),
                array: Some(Array::Regular(RegularArray { len: 28, stride: 4 })),
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "flesa",
                description: Some("29-bit filter"),
                array: Some(Array::Regular(RegularArray { len: 16, stride: 4 })),
                byte_offset: 112,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "rxfifo0",
                description: Some("Rx FIFO 0"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 176,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "rxfifo1",
                description: Some("Rx FIFO 1"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 392,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "txefifo",
                description: Some("Tx event FIFO"),
                array: Some(Array::Regular(RegularArray { len: 6, stride: 4 })),
                byte_offset: 608,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "txbuf",
                description: Some("Tx buffer"),
                array: Some(Array::Regular(RegularArray { len: 54, stride: 4 })),
                byte_offset: 632,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
