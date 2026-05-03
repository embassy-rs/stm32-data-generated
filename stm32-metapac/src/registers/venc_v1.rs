
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Venc",
        extends: None,
        description: Some("Video encoder."),
        items: &[BlockItem {
            name: "swreg",
            description: Some("VENC ID register."),
            array: Some(Array::Regular(RegularArray { len: 499, stride: 4 })),
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Swreg"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Swreg",
        extends: None,
        description: Some("VENC ID register."),
        bit_size: 32,
        fields: &[Field {
            name: "swreg_field",
            description: Some("Interrupt register (all format mode)."),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 32,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
