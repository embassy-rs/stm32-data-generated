
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Comp",
            extends: None,
            description: Some(
                "Comparator.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "Comparator control and status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inmsel",
                    description: Some(
                        "Input minus selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Inm",
                    ),
                },
                Field {
                    name: "inpsel",
                    description: Some(
                        "Input plus selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "winmode",
                    description: Some(
                        "Comparator 1 noninverting input selector for window mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WindowMode",
                    ),
                },
                Field {
                    name: "winout",
                    description: Some(
                        "Comparator 1 output selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WindowOut",
                    ),
                },
                Field {
                    name: "polarity",
                    description: Some(
                        "Polarity selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Polarity",
                    ),
                },
                Field {
                    name: "hyst",
                    description: Some(
                        "Hysteresis selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hysteresis",
                    ),
                },
                Field {
                    name: "pwrmode",
                    description: Some(
                        "Power Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PowerMode",
                    ),
                },
                Field {
                    name: "blanksel",
                    description: Some(
                        "Blanking source selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Blanking",
                    ),
                },
                Field {
                    name: "value",
                    description: Some(
                        "Output status bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Register lock bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Blanking",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "NOBLANKING",
                    description: Some(
                        "No blanking.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BLANK1",
                    description: Some(
                        "Check data sheet for blanking options",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BLANK2",
                    description: Some(
                        "Check data sheet for blanking options",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BLANK3",
                    description: Some(
                        "Check data sheet for blanking options",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Hysteresis",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Inm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "QUARTERVREF",
                    description: Some(
                        "Inverting input set to 1/4 VRef",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALFVREF",
                    description: Some(
                        "Inverting input set to 1/2 VRef",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "THREEQUARTERVREF",
                    description: Some(
                        "Inverting input set to 3/4 VRef",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VREF",
                    description: Some(
                        "Inverting input set to VRef",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DAC1",
                    description: Some(
                        "Inverting input set to DAC1 output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DAC2",
                    description: Some(
                        "Inverting input set to DAC2 output",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "INM1",
                    description: Some(
                        "Inverting input set to IO1 (PB7)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "INM2",
                    description: Some(
                        "Inverting input set to IO2 (PB3)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Polarity",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTINVERTED",
                    description: Some(
                        "Output is not inverted.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some(
                        "Output is inverted.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PowerMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HIGHSPEED",
                    description: Some(
                        "High speed / full power.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMSPEED",
                    description: Some(
                        "Medium speed / medium power.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ULTRALOW",
                    description: Some(
                        "Very-low speed / ultra-low power.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "WindowMode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "THISINPSEL",
                    description: Some(
                        "Signal selected with INPSEL[2:0] bitfield of this register.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OTHERINPSEL",
                    description: Some(
                        "Signal selected with INPSEL[2:0] bitfield of the other register (required for window mode).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WindowOut",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "COMP1_VALUE",
                    description: Some(
                        "Comparator 1 value.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "COMP1_VALUEXORCOMP2_VALUE",
                    description: Some(
                        "Comparator 1 value XOR comparator 2 value (required for window mode).",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                