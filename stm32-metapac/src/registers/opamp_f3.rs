
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Opamp",
        extends: None,
        description: Some("Operational Amplifier"),
        items: &[BlockItem {
            name: "csr",
            description: Some("OPAMP control/status register"),
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
        description: Some("OPAMP control/status register"),
        bit_size: 32,
        fields: &[
            Field {
                name: "opampen",
                description: Some("OPAMP enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "force_vp",
                description: Some(
                    "Forces a calibration reference voltage on non-inverting input and disables external connections.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: Some("ForceVp"),
            },
            Field {
                name: "vp_sel",
                description: Some("OPAMP Non inverting input selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                bit_size: 2,
                array: None,
                enumm: Some("VpSel"),
            },
            Field {
                name: "vm_sel",
                description: Some("OPAMP inverting input selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                bit_size: 2,
                array: None,
                enumm: Some("VmSel"),
            },
            Field {
                name: "tcm_en",
                description: Some("Timer controlled Mux mode enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "vms_sel",
                description: Some("OPAMP inverting input secondary selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                bit_size: 1,
                array: None,
                enumm: Some("VmsSel"),
            },
            Field {
                name: "vps_sel",
                description: Some("OPAMP Non inverting input secondary selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                bit_size: 2,
                array: None,
                enumm: Some("VpsSel"),
            },
            Field {
                name: "calon",
                description: Some("Calibration mode enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "calsel",
                description: Some("Calibration selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                bit_size: 2,
                array: None,
                enumm: Some("Calsel"),
            },
            Field {
                name: "pga_gain",
                description: Some("Gain in PGA mode"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                bit_size: 4,
                array: None,
                enumm: Some("PgaGain"),
            },
            Field {
                name: "user_trim",
                description: Some("User trimming enable"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "trimoffsetp",
                description: Some("Offset trimming value (PMOS)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                bit_size: 5,
                array: None,
                enumm: None,
            },
            Field {
                name: "trimoffsetn",
                description: Some("Offset trimming value (NMOS)"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                bit_size: 5,
                array: None,
                enumm: None,
            },
            Field {
                name: "tstref",
                description: Some("Output the internal reference voltage"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "outcal",
                description: Some("OPAMP ouput status flag"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                bit_size: 1,
                array: None,
                enumm: Some("Outcal"),
            },
            Field {
                name: "lock",
                description: Some("OPAMP lock"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Calsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PERCENT3_3",
                    description: Some("VREFOPAMP=3.3% VDDA"),
                    value: 0,
                },
                EnumVariant {
                    name: "PERCENT10",
                    description: Some("VREFOPAMP=10% VDDA"),
                    value: 1,
                },
                EnumVariant {
                    name: "PERCENT50",
                    description: Some("VREFOPAMP=50% VDDA"),
                    value: 2,
                },
                EnumVariant {
                    name: "PERCENT90",
                    description: Some("VREFOPAMP=90% VDDA"),
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
                    description: Some("Normal operating mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "CALIBRATION",
                    description: Some("Calibration mode. Non-inverting input connected to calibration reference"),
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
                    description: Some("Non-inverting < inverting"),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some("Non-inverting > inverting"),
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
                    description: Some("Gain 2"),
                    value: 0,
                },
                EnumVariant {
                    name: "GAIN4",
                    description: Some("Gain 4"),
                    value: 1,
                },
                EnumVariant {
                    name: "GAIN8",
                    description: Some("Gain 8"),
                    value: 2,
                },
                EnumVariant {
                    name: "GAIN16",
                    description: Some("Gain 16"),
                    value: 4,
                },
                EnumVariant {
                    name: "GAIN2_VM0",
                    description: Some("Gain 2, feedback connected to VM0"),
                    value: 8,
                },
                EnumVariant {
                    name: "GAIN4_VM0",
                    description: Some("Gain 4, feedback connected to VM0"),
                    value: 9,
                },
                EnumVariant {
                    name: "GAIN8_VM0",
                    description: Some("Gain 8, feedback connected to VM0"),
                    value: 10,
                },
                EnumVariant {
                    name: "GAIN16_VM0",
                    description: Some("Gain 16, feedback connected to VM0"),
                    value: 11,
                },
                EnumVariant {
                    name: "GAIN2_VM1",
                    description: Some("Gain 2, feedback connected to VM1"),
                    value: 12,
                },
                EnumVariant {
                    name: "GAIN4_VM1",
                    description: Some("Gain 4, feedback connected to VM1"),
                    value: 13,
                },
                EnumVariant {
                    name: "GAIN8_VM1",
                    description: Some("Gain 8, feedback connected to VM1"),
                    value: 14,
                },
                EnumVariant {
                    name: "GAIN16_VM1",
                    description: Some("Gain 16, feedback connected to VM1"),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "VmSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PC5",
                    description: Some("PC5 (VM0) used as OPAMP2 inverting input"),
                    value: 0,
                },
                EnumVariant {
                    name: "PA5",
                    description: Some("PA5 (VM1) used as OPAMP2 inverting input"),
                    value: 1,
                },
                EnumVariant {
                    name: "PGA",
                    description: Some("Resistor feedback output (PGA mode)"),
                    value: 2,
                },
                EnumVariant {
                    name: "FOLLOWER",
                    description: Some("Follower mode"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "VmsSel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PC5",
                    description: Some("PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1"),
                    value: 0,
                },
                EnumVariant {
                    name: "PA5",
                    description: Some("PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PB14",
                    description: Some("PB14 used as OPAMP2 non-inverting input"),
                    value: 1,
                },
                EnumVariant {
                    name: "PB0",
                    description: Some("PB0 used as OPAMP2 non-inverting input"),
                    value: 2,
                },
                EnumVariant {
                    name: "PA7",
                    description: Some("PA7 used as OPAMP2 non-inverting input"),
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
                    name: "PB14",
                    description: Some("PB14 used as OPAMP2 non-inverting input when TCM_EN=1"),
                    value: 1,
                },
                EnumVariant {
                    name: "PB0",
                    description: Some("PB0 used as OPAMP2 non-inverting input when TCM_EN=1"),
                    value: 2,
                },
                EnumVariant {
                    name: "PA7",
                    description: Some("PA7 used as OPAMP2 non-inverting input when TCM_EN=1"),
                    value: 3,
                },
            ],
        },
    ],
};
