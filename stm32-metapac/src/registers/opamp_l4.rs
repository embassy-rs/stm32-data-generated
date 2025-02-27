
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Opamp",
        extends: None,
        description: Some("Operational amplifiers."),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("OPAMP control/status register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "otr",
                description: Some("OPAMP offset trimming register in normal mode."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Otr"),
                }),
            },
            BlockItem {
                name: "lpotr",
                description: Some("OPAMP offset trimming register in low-power mode."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Lpotr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("OPAMP control/status register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
                    description: Some("Operational amplifier Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opalpm",
                    description: Some("Operational amplifier Low Power Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Opalpm"),
                },
                Field {
                    name: "opamode",
                    description: Some("Operational amplifier PGA mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Opamode"),
                },
                Field {
                    name: "pga_gain",
                    description: Some("Operational amplifier Programmable amplifier gain value."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("PgaGain"),
                },
                Field {
                    name: "vm_sel",
                    description: Some("Inverting input selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VmSel"),
                },
                Field {
                    name: "vp_sel",
                    description: Some("Non inverted input selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("VpSel"),
                },
                Field {
                    name: "calon",
                    description: Some("Calibration mode enabled."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calsel",
                    description: Some("Calibration selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Calsel"),
                },
                Field {
                    name: "usertrim",
                    description: Some("allows to switch from AOP offset trimmed values to AOP offset."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Usertrim"),
                },
                Field {
                    name: "calout",
                    description: Some("Operational amplifier calibration output."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opa_range",
                    description: Some("Operational amplifier power supply range for stability."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("OpaRange"),
                },
            ],
        },
        FieldSet {
            name: "Lpotr",
            extends: None,
            description: Some("OPAMP offset trimming register in low-power mode."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimlpoffsetn",
                    description: Some("Trim for NMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimlpoffsetp",
                    description: Some("Trim for PMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Otr",
            extends: None,
            description: Some("OPAMP offset trimming register in normal mode."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimoffsetn",
                    description: Some("Trim for NMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimoffsetp",
                    description: Some("Trim for PMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 5,
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
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NMOS",
                    description: Some("0.2V applied to OPAMP inputs during calibration."),
                    value: 0,
                },
                EnumVariant {
                    name: "PMOS",
                    description: Some("VDDA-0.2V applied to OPAMP inputs during calibration\"."),
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
                    description: Some("low range (VDDA < 2.4V."),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("low range (VDDA >2.4V."),
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
                    description: Some("OpAmp in normal mode."),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some("OpAmp in low power mode."),
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
                    name: "PGA_DISABLED",
                    description: Some("internal PGA diabled."),
                    value: 0,
                },
                EnumVariant {
                    name: "PGA_ENABLED",
                    description: Some("internal PGA enabled, gain programmed in PGA_GAIN."),
                    value: 2,
                },
                EnumVariant {
                    name: "FOLLOWER",
                    description: Some("internal follower."),
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
                    description: Some("Gain 2."),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some("Gain 4."),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some("Gain 8."),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some("Gain 16."),
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
                    description: Some("Factory trim used."),
                    value: 0,
                },
                EnumVariant {
                    name: "USER",
                    description: Some("User trim used."),
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
                    name: "GPIO",
                    description: Some("GPIO connectet to VINM."),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW_LEAKAGE",
                    description: Some("Low leakage inputs connecte (only available in certen BGA cases."),
                    value: 1,
                },
                EnumVariant {
                    name: "PGA_MODE",
                    description: Some("OPAMP in PGA mode."),
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
                    name: "GPIO",
                    description: Some("GPIO connectet to VINP."),
                    value: 0,
                },
                EnumVariant {
                    name: "DAC",
                    description: Some("DAC connected to VPINP."),
                    value: 1,
                },
            ],
        },
    ],
};
