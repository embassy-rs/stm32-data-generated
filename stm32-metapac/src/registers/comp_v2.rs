
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Comp",
            extends: None,
            description: Some(
                "Comparator v2. (RM0440 24)",
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
                "Comparator control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "COMP enable bit.",
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
                        "Comparator signal selector for inverting input INM. (RM0440 24.3.2 Table 197)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inpsel",
                    description: Some(
                        "Comparator signal selector for non-inverting input INP. (RM0440 24.3.2 Table 196)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "polarity",
                    description: Some(
                        "Comparator polarity selector.",
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
                        "Comparator hysteresis selector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Hyst",
                    ),
                },
                Field {
                    name: "blanksel",
                    description: Some(
                        "Comparator blanking source selector. (RM0440 24.3.6 Table 198)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brgen",
                    description: Some(
                        "Vrefint resistor bridge enable. (RM0440 24.6)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scalen",
                    description: Some(
                        "Vrefint scaled input enable. (RM0440 24.6)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "value_do_not_set",
                    description: Some(
                        "Comparator output status. (READ ONLY)",
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
                        "CSR register lock.",
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
            name: "Hyst",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "HYST10M",
                    description: Some(
                        "10mV hysteresis",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HYST20M",
                    description: Some(
                        "20mV hysteresis",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HYST30M",
                    description: Some(
                        "30mV hysteresis",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HYST40M",
                    description: Some(
                        "40mV hysteresis",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HYST50M",
                    description: Some(
                        "50mV hysteresis",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HYST60M",
                    description: Some(
                        "60mV hysteresis",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "HYST70M",
                    description: Some(
                        "70mV hysteresis",
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
                    name: "NONINVERTED",
                    description: Some(
                        "Non-inverted polarity",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERTED",
                    description: Some(
                        "Inverted polarity",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                