
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opamp",
            extends: None,
            description: Some(
                "Operational Amplifier",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "OPAMP control/status register",
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
                    name: "tcmr",
                    description: Some(
                        "OPAMP control/status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcmr",
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
                "OPAMP control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
                    description: Some(
                        "OPAMP enable",
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
                        "Forces a calibration reference voltage on non-inverting input and disables external connections.",
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
                        "VP_SEL",
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
                    name: "usertrim",
                    description: Some(
                        "USERTRIM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usertrim",
                    ),
                },
                Field {
                    name: "vm_sel",
                    description: Some(
                        "OPAMP inverting input selection",
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
                        "OPAHSM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Opahsm",
                    ),
                },
                Field {
                    name: "opaintoen",
                    description: Some(
                        "OPAINTOEN",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Opaintoen",
                    ),
                },
                Field {
                    name: "calon",
                    description: Some(
                        "Calibration mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calsel",
                    description: Some(
                        "Calibration selection",
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
                        "Gain in PGA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "PgaGain",
                    ),
                },
                Field {
                    name: "trimoffsetp",
                    description: Some(
                        "Offset trimming value (PMOS)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimoffsetn",
                    description: Some(
                        "Offset trimming value (NMOS)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "outcal",
                    description: Some(
                        "OPAMP ouput status flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Outcal",
                    ),
                },
                Field {
                    name: "lock",
                    description: Some(
                        "LOCK",
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
        FieldSet {
            name: "Tcmr",
            extends: None,
            description: Some(
                "OPAMP timer controlled mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vms_sel",
                    description: Some(
                        "VMS_SEL",
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
                    name: "vps_sel",
                    description: Some(
                        "VPS_SEL",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "VpsSel",
                    ),
                },
                Field {
                    name: "t1cm_en",
                    description: Some(
                        "T1CM_EN",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t8cm_en",
                    description: Some(
                        "T8CM_EN",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t20cm_en",
                    description: Some(
                        "T20CM_EN",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "TCMR LOCK",
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
            name: "Calsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PERCENT3_3",
                    description: Some(
                        "VREFOPAMP=3.3% VDDA",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PERCENT10",
                    description: Some(
                        "VREFOPAMP=10% VDDA",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PERCENT50",
                    description: Some(
                        "VREFOPAMP=50% VDDA",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PERCENT90",
                    description: Some(
                        "VREFOPAMP=90% VDDA",
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
                    name: "NORMAL",
                    description: Some(
                        "Normal operating mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CALIBRATION",
                    description: Some(
                        "Calibration mode. Non-inverting input connected to calibration reference",
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
                        "OpAmp in normal mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGHSPEED",
                    description: Some(
                        "OpAmp in high speed mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Opaintoen",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OUTPUTPIN",
                    description: Some(
                        "Output is connected to the output Pin",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ADCCHANNEL",
                    description: Some(
                        "Output is connected internally to ADC channel",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Outcal",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Non-inverting < inverting",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Non-inverting > inverting",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "GAIN2",
                    description: Some(
                        "Gain 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some(
                        "Gain 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some(
                        "Gain 8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some(
                        "Gain 16",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "GAIN32",
                    description: Some(
                        "Gain 32",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "GAIN64",
                    description: Some(
                        "Gain 64",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "GAIN2_INPUTVINM0",
                    description: Some(
                        "Gain 2, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "GAIN4_INPUTVINM0",
                    description: Some(
                        "Gain 4, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "GAIN8_INPUTVINM0",
                    description: Some(
                        "Gain 8, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "GAIN16_INPUTVINM0",
                    description: Some(
                        "Gain 16, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "GAIN32_INPUTVINM0",
                    description: Some(
                        "Gain 32, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "GAIN64_INPUTVINM0",
                    description: Some(
                        "Gain 64, input/bias connected to VINM0 or inverting gain",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "GAIN2_FILTERINGVINM0",
                    description: Some(
                        "Gain 2, with filtering on VINM0",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "GAIN4_FILTERINGVINM0",
                    description: Some(
                        "Gain 4, with filtering on VINM0",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "GAIN8_FILTERINGVINM0",
                    description: Some(
                        "Gain 8, with filtering on VINM0",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "GAIN16_FILTERINGVINM0",
                    description: Some(
                        "Gain 16, with filtering on VINM0",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "GAIN32_FILTERINGVINM0",
                    description: Some(
                        "Gain 32, with filtering on VINM0",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "GAIN64_FILTERINGVINM0",
                    description: Some(
                        "Gain 64, with filtering on VINM0",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "GAIN2_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "GAIN4_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "GAIN8_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "GAIN16_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "GAIN32_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "GAIN64_INPUTVINM0FILTERINGVINM1",
                    description: Some(
                        "Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain",
                    ),
                    value: 29,
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
                        "Factory trim used",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USER",
                    description: Some(
                        "User trim used",
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
                    name: "VINM0",
                    description: Some(
                        "VINM0 connected to VINM input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "VINM1",
                    description: Some(
                        "VINM1 connected to VINM input",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PGA",
                    description: Some(
                        "Feedback resistor connected to VINM (PGA mode)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "OpAmp output connected to VINM (Follower mode)",
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
                    name: "VINP0",
                    description: Some(
                        "VINP0 connected to VINP input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "VINP1",
                    description: Some(
                        "VINP1 connected to VINP input",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "VINP2",
                    description: Some(
                        "VINP2 connected to VINP input",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DAC3_CH1",
                    description: Some(
                        "DAC3_CH1 connected to VINP input",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VpsSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "VINP0",
                    description: Some(
                        "VINP0 connected to VINP input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "VINP1",
                    description: Some(
                        "VINP1 connected to VINP input",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "VINP2",
                    description: Some(
                        "VINP2 connected to VINP input",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DAC3_CH1",
                    description: Some(
                        "DAC3_CH1 connected to VINP input",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                