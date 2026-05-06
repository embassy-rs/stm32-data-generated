
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Dlybsd",
        extends: None,
        description: Some("DLYBSD address block description."),
        items: &[
            BlockItem {
                name: "cfg",
                description: Some("Delay block SDMMC DLL configuration."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfg"),
                }),
            },
            BlockItem {
                name: "status",
                description: Some("Delay block SDMMC DLL status."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Status"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some("Delay block SDMMC DLL configuration."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdmmc_dll_en",
                    description: Some("DLL enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc_rx_tap_sel",
                    description: Some("selection of RX delay."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc_dll_byp_en",
                    description: Some("DLL configuration."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc_dll_byp_cmd",
                    description: Some("Bypass command."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc_dll_antiglitch_en",
                    description: Some("Antiglitch logic enabled when 1."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some("Delay block SDMMC DLL status."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdmmc_dll_lock",
                    description: Some("SDMMC DLL lock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc_rx_tap_sel_ack",
                    description: Some("SDMMC RX delay selection acknowledge."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
