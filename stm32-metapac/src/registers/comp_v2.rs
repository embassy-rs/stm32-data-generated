
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("Comparator v2. (RM0440 24)"),
        items: &[BlockItem {
            name: "csr",
            description: Some("Comparator control and status register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Csr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
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
                description: Some("Comparator signal selector for inverting input INM. (RM0440 24.3.2 Table 197)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 3,
                array: None,
                enumm: Some("Inm"),
            },
            Field {
                name: "inpsel",
                description: Some("Comparator signal selector for non-inverting input INP. (RM0440 24.3.2 Table 196)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
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
                description: Some("Comparator blanking source selector. (RM0440 24.3.6 Table 198)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                bit_size: 3,
                array: None,
                enumm: Some("Blanking"),
            },
            Field {
                name: "brgen",
                description: Some("Vrefint resistor bridge enable. (RM0440 24.6)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "scalen",
                description: Some("Vrefint scaled input enable. (RM0440 24.6)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "value",
                description: Some("Comparator output status. (READ ONLY)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lock",
                description: Some("CSR register lock."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Blanking",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Blank1",
                    description: Some("Check data sheet for blanking options"),
                    value: 1,
                },
                EnumVariant {
                    name: "Blank2",
                    description: Some("Check data sheet for blanking options"),
                    value: 2,
                },
                EnumVariant {
                    name: "Blank3",
                    description: Some("Check data sheet for blanking options"),
                    value: 3,
                },
                EnumVariant {
                    name: "Blank4",
                    description: Some("Check data sheet for blanking options"),
                    value: 4,
                },
                EnumVariant {
                    name: "Blank5",
                    description: Some("Check data sheet for blanking options"),
                    value: 5,
                },
                EnumVariant {
                    name: "Blank6",
                    description: Some("Check data sheet for blanking options"),
                    value: 6,
                },
                EnumVariant {
                    name: "Blank7",
                    description: Some("Check data sheet for blanking options"),
                    value: 7,
                },
                EnumVariant {
                    name: "NoBlanking",
                    description: Some("No blanking."),
                    value: 0,
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
                    description: Some("10mV hysteresis"),
                    value: 1,
                },
                EnumVariant {
                    name: "Hyst20m",
                    description: Some("20mV hysteresis"),
                    value: 2,
                },
                EnumVariant {
                    name: "Hyst30m",
                    description: Some("30mV hysteresis"),
                    value: 3,
                },
                EnumVariant {
                    name: "Hyst40m",
                    description: Some("40mV hysteresis"),
                    value: 4,
                },
                EnumVariant {
                    name: "Hyst50m",
                    description: Some("50mV hysteresis"),
                    value: 5,
                },
                EnumVariant {
                    name: "Hyst60m",
                    description: Some("60mV hysteresis"),
                    value: 6,
                },
                EnumVariant {
                    name: "Hyst70m",
                    description: Some("70mV hysteresis"),
                    value: 7,
                },
                EnumVariant {
                    name: "None",
                    description: None,
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Inm",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "Daca",
                    description: Some("Inverting input set to DAC output (RM0440 24.3.2 Table)"),
                    value: 4,
                },
                EnumVariant {
                    name: "Dacb",
                    description: Some("Inverting input set to DAC output (RM0440 24.3.2 Table)"),
                    value: 5,
                },
                EnumVariant {
                    name: "HalfVRef",
                    description: Some("Inverting input set to 1/2 VRef"),
                    value: 1,
                },
                EnumVariant {
                    name: "Inm1",
                    description: Some("Inverting input set to IO (RM0440 24.3.2 Table)"),
                    value: 6,
                },
                EnumVariant {
                    name: "Inm2",
                    description: Some("Inverting input set to IO (RM0440 24.3.2 Table)"),
                    value: 7,
                },
                EnumVariant {
                    name: "QuarterVRef",
                    description: Some("Inverting input set to 1/4 VRef"),
                    value: 0,
                },
                EnumVariant {
                    name: "ThreeQuarterVRef",
                    description: Some("Inverting input set to 3/4 VRef"),
                    value: 2,
                },
                EnumVariant {
                    name: "VRef",
                    description: Some("Inverting input set to VRef"),
                    value: 3,
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
