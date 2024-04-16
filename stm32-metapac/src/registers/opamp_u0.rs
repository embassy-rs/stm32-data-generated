
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opamp",
            extends: None,
            description: Some(
                "OPAMP address block description.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "OPAMP control/status register.",
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
                BlockItem {
                    name: "otr",
                    description: Some(
                        "OPAMP offset trimming register in normal mode.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpotr",
                    description: Some(
                        "OPAMP offset trimming register in low-power mode.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpotr",
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
                "OPAMP control/status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
                    description: Some(
                        "Operational amplifier Enable.",
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
                    name: "opalpm",
                    description: Some(
                        "Operational amplifier Low Power Mode. The operational amplifier must be disable to change this configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Opalpm",
                    ),
                },
                Field {
                    name: "opamode",
                    description: Some(
                        "Operational amplifier PGA mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Opamode",
                    ),
                },
                Field {
                    name: "pga_gain",
                    description: Some(
                        "Operational amplifier Programmable amplifier gain value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PgaGain",
                    ),
                },
                Field {
                    name: "vm_sel",
                    description: Some(
                        "Inverting input selection. These bits are used only when OPAMODE = 00, 01 or 10. 1x: Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "VmSel",
                    ),
                },
                Field {
                    name: "vp_sel",
                    description: Some(
                        "Non inverted input selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "VpSel",
                    ),
                },
                Field {
                    name: "calon",
                    description: Some(
                        "Calibration mode enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calon",
                    ),
                },
                Field {
                    name: "calsel",
                    description: Some(
                        "Calibration selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calsel",
                    ),
                },
                Field {
                    name: "usertrim",
                    description: Some(
                        "allows to switch from factory AOP offset trimmed values to AOP offset user trimmed values This bit is active for both mode normal and low-power.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usertrim",
                    ),
                },
                Field {
                    name: "calout",
                    description: Some(
                        "Operational amplifier calibration output During calibration mode offset is trimmed when this signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opa_range",
                    description: Some(
                        "Operational amplifier power supply range for stability All AOP must be in power down to allow AOP-RANGE bit write. It applies to all AOP embedded in the product.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OpaRange",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Lpotr",
            extends: None,
            description: Some(
                "OPAMP offset trimming register in low-power mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimlpoffsetn",
                    description: Some(
                        "Low-power mode trim for NMOS differential pairs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimlpoffsetp",
                    description: Some(
                        "Low-power mode trim for PMOS differential pairs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Otr",
            extends: None,
            description: Some(
                "OPAMP offset trimming register in normal mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimoffsetn",
                    description: Some(
                        "Trim for NMOS differential pairs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimoffsetp",
                    description: Some(
                        "Trim for PMOS differential pairs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Calon",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CALIBRATION",
                    description: Some(
                        "Calibration mode (all switches opened by HW).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NMOS",
                    description: Some(
                        "NMOS calibration (200mV applied on OPAMP inputs).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PMOS",
                    description: Some(
                        "PMOS calibration (VDDA-200mV applied on OPAMP inputs).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OpaRange",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low range (VDDA < 2.4V).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High range (VDDA > 2.4V).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Opalpm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "operational amplifier in normal mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOWPOWER",
                    description: Some(
                        "operational amplifier in low-power mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Opamode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "internal PGA disable.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLE2",
                    description: Some(
                        "internal PGA disable. (Duplicate)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "internal PGA enable, gain programmed in PGA_GAIN.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FOLLOWER",
                    description: Some(
                        "internal follower.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GAIN2",
                    description: Some(
                        "internal PGA Gain 2.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some(
                        "internal PGA Gain 4.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some(
                        "internal PGA Gain 8.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some(
                        "internal PGA Gain 16.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Usertrim",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FACTORY",
                    description: Some(
                        "Factory trim code used.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USER",
                    description: Some(
                        "User trim code used.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "VmSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VINM",
                    description: Some(
                        "GPIO connected to VINM (valid also in PGA mode for filtering).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOTCONNECTED",
                    description: Some(
                        "Inverting input not externally connected. These configurations are valid only when OPAMODE = 10 (PGA mode)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "VINP",
                    description: Some(
                        "GPIO connected to VINP.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DAC",
                    description: Some(
                        "DAC connected to VINP.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                