
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "AdcCommon",
        extends: None,
        description: Some("ADC common registers"),
        items: &[BlockItem {
            name: "ccr",
            description: Some("common configuration register"),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Ccr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Ccr",
        extends: None,
        description: Some("common configuration register"),
        bit_size: 32,
        fields: &[
            Field {
                name: "presc",
                description: Some("prescaler"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 4,
                array: None,
                enumm: Some("Presc"),
            },
            Field {
                name: "vrefen",
                description: Some("VREFINT enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "tsen",
                description: Some("Temperature sensor enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[Enum {
        name: "Presc",
        description: None,
        bit_size: 4,
        variants: &[
            EnumVariant {
                name: "Div1",
                description: Some("adc_ker_ck_input not divided"),
                value: 0,
            },
            EnumVariant {
                name: "Div10",
                description: Some("adc_ker_ck_input divided by 10"),
                value: 5,
            },
            EnumVariant {
                name: "Div12",
                description: Some("adc_ker_ck_input divided by 12"),
                value: 6,
            },
            EnumVariant {
                name: "Div128",
                description: Some("adc_ker_ck_input divided by 128"),
                value: 10,
            },
            EnumVariant {
                name: "Div16",
                description: Some("adc_ker_ck_input divided by 16"),
                value: 7,
            },
            EnumVariant {
                name: "Div2",
                description: Some("adc_ker_ck_input divided by 2"),
                value: 1,
            },
            EnumVariant {
                name: "Div256",
                description: Some("adc_ker_ck_input divided by 256"),
                value: 11,
            },
            EnumVariant {
                name: "Div32",
                description: Some("adc_ker_ck_input divided by 32"),
                value: 8,
            },
            EnumVariant {
                name: "Div4",
                description: Some("adc_ker_ck_input divided by 4"),
                value: 2,
            },
            EnumVariant {
                name: "Div6",
                description: Some("adc_ker_ck_input divided by 6"),
                value: 3,
            },
            EnumVariant {
                name: "Div64",
                description: Some("adc_ker_ck_input divided by 64"),
                value: 9,
            },
            EnumVariant {
                name: "Div8",
                description: Some("adc_ker_ck_input divided by 8"),
                value: 4,
            },
        ],
    }],
};
