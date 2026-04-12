
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Comp",
        extends: None,
        description: Some("Comparator v1. (RM0444 18)"),
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
                description: Some("Comparator signal selector for inverting input INM."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 4,
                array: None,
                enumm: Some("Inm"),
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
                name: "winmode",
                description: Some("Comparator non-inverting input selector for window mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                bit_size: 1,
                array: None,
                enumm: Some("WindowMode"),
            },
            Field {
                name: "winout",
                description: Some("Comparator output selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                bit_size: 1,
                array: None,
                enumm: Some("WindowOut"),
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
                bit_size: 2,
                array: None,
                enumm: Some("Hysteresis"),
            },
            Field {
                name: "pwrmode",
                description: Some("Comparator power mode selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 2,
                array: None,
                enumm: Some("PowerMode"),
            },
            Field {
                name: "blanksel",
                description: Some("Comparator blanking source selector."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                bit_size: 5,
                array: None,
                enumm: Some("Blanking"),
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
            bit_size: 5,
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
                    value: 4,
                },
                EnumVariant {
                    name: "Blank4",
                    description: Some("Check data sheet for blanking options"),
                    value: 8,
                },
                EnumVariant {
                    name: "Blank5",
                    description: Some("Check data sheet for blanking options"),
                    value: 16,
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
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "High",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Low",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Medium",
                    description: None,
                    value: 2,
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
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Dac1",
                    description: Some("Inverting input set to DAC1 output"),
                    value: 4,
                },
                EnumVariant {
                    name: "Dac2",
                    description: Some("Inverting input set to DAC2 output"),
                    value: 5,
                },
                EnumVariant {
                    name: "HalfVRef",
                    description: Some("Inverting input set to 1/2 VRef"),
                    value: 1,
                },
                EnumVariant {
                    name: "Inm1",
                    description: Some("Inverting input set to IO1"),
                    value: 6,
                },
                EnumVariant {
                    name: "Inm2",
                    description: Some("Inverting input set to IO2"),
                    value: 7,
                },
                EnumVariant {
                    name: "Inm3",
                    description: Some("Inverting input set to IO3"),
                    value: 8,
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
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "NotInverted",
                    description: None,
                    value: 0,
                },
            ],
        },
        Enum {
            name: "PowerMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HighSpeed",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MediumSpeed",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WindowMode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OtherInpsel",
                    description: Some(
                        "Signal selected with INPSEL[1:0] bitfield of the other register (required for window mode).",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ThisInpsel",
                    description: Some("Signal selected with INPSEL[1:0] bitfield of this register."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "WindowOut",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Comp1Value",
                    description: Some("Comparator 1 value."),
                    value: 0,
                },
                EnumVariant {
                    name: "Comp1ValueXorComp2Value",
                    description: Some("Comparator 1 value XOR comparator 2 value (required for window mode)."),
                    value: 1,
                },
            ],
        },
    ],
};
