
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Vrefintcal",
        extends: None,
        description: Some("VREFINT Factory Calibration"),
        items: &[BlockItem {
            name: "data",
            description: Some("Factory calibration"),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::Read,
                bit_size: 32,
                fieldset: Some("Data"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Data",
        extends: None,
        description: Some("Factory calibration data"),
        bit_size: 32,
        fields: &[Field {
            name: "value",
            description: Some("Calibration value"),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 16,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
