
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Tscal",
        extends: None,
        description: Some("Temperature Sensor Factory Calibration"),
        items: &[
            BlockItem {
                name: "tscal1",
                description: Some("Factory calibration point 1"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 16,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "tscal2",
                description: Some("Factory calibration point 2"),
                array: None,
                byte_offset: 0x22,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 16,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
