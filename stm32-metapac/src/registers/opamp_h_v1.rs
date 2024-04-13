
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opamp",
            extends: None,
            description: Some(
                "Operational amplifiers.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "OPAMP1 control/status register.",
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
                        "OPAMP1 offset trimming register in normal mode.",
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
                    name: "hsotr",
                    description: Some(
                        "OPAMP1 offset trimming register in low-power mode.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hsotr",
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
                "OPAMP1 control/status register.",
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
                    name: "force_vp",
                    description: Some(
                        "Force internal reference on VP (reserved for test.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ForceVp",
                    ),
                },
                Field {
                    name: "vp_sel",
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
                        "VpSel",
                    ),
                },
                Field {
                    name: "vm_sel",
                    description: Some(
                        "Inverting input selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "VmSel",
                    ),
                },
                Field {
                    name: "opahsm",
                    description: Some(
                        "Operational amplifier high-speed mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Opahsm",
                    ),
                },
                Field {
                    name: "calon",
                    description: Some(
                        "Calibration mode enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
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
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Calsel",
                    ),
                },
                Field {
                    name: "pga_gain",
                    description: Some(
                        "allows to switch from AOP offset trimmed values to AOP offset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "PgaGain",
                    ),
                },
                Field {
                    name: "usertrim",
                    description: Some(
                        "User trimming enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usertrim",
                    ),
                },
                Field {
                    name: "tstref",
                    description: Some(
                        "OPAMP calibration reference voltage output control (reserved for test).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calout",
                    description: Some(
                        "Operational amplifier calibration output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calout",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Hsotr",
            extends: None,
            description: Some(
                "OPAMP1 offset trimming register in low-power mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimlpoffsetn",
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
                    name: "trimlpoffsetp",
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
        FieldSet {
            name: "Otr",
            extends: None,
            description: Some(
                "OPAMP1 offset trimming register in normal mode.",
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
                        "Normal mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CALIBRATION",
                    description: Some(
                        "Calibration mode (all switches opened by HW)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calout",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LESS",
                    description: Some(
                        "Non-inverting < inverting",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GREATER",
                    description: Some(
                        "Non-inverting > inverting",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PERCENT3_3",
                    description: Some(
                        "VREFOPAMP=3.3% VDDA.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PERCENT10",
                    description: Some(
                        "VREFOPAMP=10% VDDA.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PERCENT50",
                    description: Some(
                        "VREFOPAMP=50% VDDA.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PERCENT90",
                    description: Some(
                        "VREFOPAMP=90% VDDA.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ForceVp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMALOPERATING",
                    description: Some(
                        "Normal operating mode. Non-inverting input connected to inputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CALIBRATIONVERIFICATION",
                    description: Some(
                        "Calibration verification mode. Non-inverting input connected to calibration reference voltage.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Opahsm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "operational amplifier in normal mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGHSPEED",
                    description: Some(
                        "operational amplifier in high-speed mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "GAIN2",
                    description: Some(
                        "Non-inverting internal Gain 2, VREF- referenced",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some(
                        "Non-inverting internal Gain 4, VREF- referenced",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some(
                        "Non-inverting internal Gain 8, VREF- referenced",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some(
                        "Non-inverting internal Gain 16, VREF- referenced",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "GAIN2_FILTERINGVINM0",
                    description: Some(
                        "Non-inverting internal Gain 2 with filtering on INM0, VREF- referenced",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "GAIN4_FILTERINGVINM0",
                    description: Some(
                        "Non-inverting internal Gain 4 with filtering on INM0, VREF- referenced",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "GAIN8_FILTERINGVINM0",
                    description: Some(
                        "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "GAIN16_FILTERINGVINM0",
                    description: Some(
                        "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "GAIN2INVGAINNEG1_INPUTVINM0",
                    description: Some(
                        "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "GAIN4INVGAINNEG3_INPUTVINM0",
                    description: Some(
                        "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "GAIN8INVGAINNEG7_INPUTVINM0",
                    description: Some(
                        "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "GAIN16INVGAINNEG15_INPUTVINM0",
                    description: Some(
                        "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "GAIN2INVGAINNEG1_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "GAIN4INVGAINNEG3_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "GAIN8INVGAINNEG7_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "GAIN16INVGAINNEG15_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 15,
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
                        "\\'factory\\' trim code used",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USER",
                    description: Some(
                        "\\'user\\' trim code used",
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
                    name: "INM0",
                    description: Some(
                        "INM0 connected to OPAMP_VINM input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INM1",
                    description: Some(
                        "INM1 connected to OPAMP_VINM input",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PGA",
                    description: Some(
                        "Feedback resistor is connected to the OPAMP_VINM input (PGA mode), Inverting input selection depends on the PGA_GAIN setting",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FOLLOWER",
                    description: Some(
                        "opamp_out connected to OPAMP_VINM input (Follower mode)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GPIO",
                    description: Some(
                        "GPIO connected to OPAMPx_VINP",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DACOUT",
                    description: Some(
                        "dac_outx connected to OPAMPx_VINP",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                