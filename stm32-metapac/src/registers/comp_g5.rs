
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("Comparator."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("Comparator control and status register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "fltr",
                description: Some("Comparator filter configuration register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Fltr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Comparator control and status register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("COMP enable bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inmsel",
                    description: Some("Comparator signal selector for inverting input INM."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inpsel",
                    description: Some("Comparator signal selector for non-inverting input INP."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "winout",
                    description: Some("Comparator output selector."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "polarity",
                    description: Some("Comparator polarity selector."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Polarity"),
                },
                Field {
                    name: "hyst",
                    description: Some("Comparator hysteresis selector."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Hysteresis"),
                },
                Field {
                    name: "blanksel",
                    description: Some("Comparator blanking source selector."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brgen",
                    description: Some("Vrefint resistor bridge enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scalen",
                    description: Some("Vrefint scaled input enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dglt",
                    description: Some("Deglitcher enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "value",
                    description: Some("Comparator output status."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("CSR and FLTR register lock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fltr",
            extends: None,
            description: Some("Comparator filter configuration register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpsc",
                    description: Some("Digital filter clock prescaler selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Fpsc"),
                },
                Field {
                    name: "fsel",
                    description: Some("Digital filter sampling and sample count selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Fsel"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Fpsc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Div1",
                    description: Some("filter_clk = comp_pclk."),
                    value: 0,
                },
                EnumVariant {
                    name: "Div2",
                    description: Some("filter_clk = comp_pclk / 2."),
                    value: 1,
                },
                EnumVariant {
                    name: "Div4",
                    description: Some("filter_clk = comp_pclk / 4."),
                    value: 2,
                },
                EnumVariant {
                    name: "Div8",
                    description: Some("filter_clk = comp_pclk / 8."),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Fsel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Disabled",
                    description: Some("Filter disabled."),
                    value: 0,
                },
                EnumVariant {
                    name: "Div16n5",
                    description: Some("filter_sampling = filter_clk / 16, N = 5."),
                    value: 10,
                },
                EnumVariant {
                    name: "Div16n6",
                    description: Some("filter_sampling = filter_clk / 16, N = 6."),
                    value: 11,
                },
                EnumVariant {
                    name: "Div16n8",
                    description: Some("filter_sampling = filter_clk / 16, N = 8."),
                    value: 12,
                },
                EnumVariant {
                    name: "Div2n6",
                    description: Some("filter_sampling = filter_clk / 2, N = 6."),
                    value: 4,
                },
                EnumVariant {
                    name: "Div2n8",
                    description: Some("filter_sampling = filter_clk / 2, N = 8."),
                    value: 5,
                },
                EnumVariant {
                    name: "Div32n5",
                    description: Some("filter_sampling = filter_clk / 32, N = 5."),
                    value: 13,
                },
                EnumVariant {
                    name: "Div32n6",
                    description: Some("filter_sampling = filter_clk / 32, N = 6."),
                    value: 14,
                },
                EnumVariant {
                    name: "Div32n8",
                    description: Some("filter_sampling = filter_clk / 32, N = 8."),
                    value: 15,
                },
                EnumVariant {
                    name: "Div4n6",
                    description: Some("filter_sampling = filter_clk / 4, N = 6."),
                    value: 6,
                },
                EnumVariant {
                    name: "Div4n8",
                    description: Some("filter_sampling = filter_clk / 4, N = 8."),
                    value: 7,
                },
                EnumVariant {
                    name: "Div8n6",
                    description: Some("filter_sampling = filter_clk / 8, N = 6."),
                    value: 8,
                },
                EnumVariant {
                    name: "Div8n8",
                    description: Some("filter_sampling = filter_clk / 8, N = 8."),
                    value: 9,
                },
                EnumVariant {
                    name: "PclkN2",
                    description: Some("filter_sampling = comp_pclk, N = 2."),
                    value: 1,
                },
                EnumVariant {
                    name: "PclkN4",
                    description: Some("filter_sampling = comp_pclk, N = 4."),
                    value: 2,
                },
                EnumVariant {
                    name: "PclkN8",
                    description: Some("filter_sampling = comp_pclk, N = 8."),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Hysteresis",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Hyst10m",
                    description: Some("10 mV hysteresis."),
                    value: 1,
                },
                EnumVariant {
                    name: "Hyst20m",
                    description: Some("20 mV hysteresis."),
                    value: 2,
                },
                EnumVariant {
                    name: "Hyst30m",
                    description: Some("30 mV hysteresis."),
                    value: 3,
                },
                EnumVariant {
                    name: "Hyst40m",
                    description: Some("40 mV hysteresis."),
                    value: 4,
                },
                EnumVariant {
                    name: "Hyst50m",
                    description: Some("50 mV hysteresis."),
                    value: 5,
                },
                EnumVariant {
                    name: "Hyst60m",
                    description: Some("60 mV hysteresis."),
                    value: 6,
                },
                EnumVariant {
                    name: "Hyst70m",
                    description: Some("70 mV hysteresis."),
                    value: 7,
                },
                EnumVariant {
                    name: "None",
                    description: Some("No hysteresis."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Polarity",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Inverted",
                    description: Some("Output is inverted."),
                    value: 1,
                },
                EnumVariant {
                    name: "NotInverted",
                    description: Some("Output is not inverted."),
                    value: 0,
                },
            ],
        },
    ],
};
