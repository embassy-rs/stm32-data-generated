
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Opamp",
        extends: None,
        description: Some("OPAMP register block."),
        items: &[
            BlockItem {
                name: "opamp_csr",
                description: Some("OPAMP control/status register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("OpampCsr"),
                }),
            },
            BlockItem {
                name: "opamp_tcmr",
                description: Some("OPAMP timer-controlled mode register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("OpampTcmr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "OpampCsr",
            extends: None,
            description: Some("OPAMP control/status register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opaen",
                    description: Some("Operational amplifier enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "force_vp",
                    description: Some("Force internal reference on noninverting input."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("ForceVp"),
                },
                Field {
                    name: "vp_sel",
                    description: Some("Noninverting input primary selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VpSel"),
                },
                Field {
                    name: "usertrim",
                    description: Some("User trimming enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Usertrim"),
                },
                Field {
                    name: "vm_sel",
                    description: Some("Inverting input primary selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VmSel"),
                },
                Field {
                    name: "opahsm",
                    description: Some("Operational amplifier high-speed mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Opahsm"),
                },
                Field {
                    name: "opaintoen",
                    description: Some("Operational amplifier internal output enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Opaintoen"),
                },
                Field {
                    name: "calon",
                    description: Some("Calibration mode enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calsel",
                    description: Some("Calibration selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Calsel"),
                },
                Field {
                    name: "pga_gain",
                    description: Some("Operational amplifier programmable gain and PGA flavor primary control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 5,
                    array: None,
                    enumm: Some("PgaGain"),
                },
                Field {
                    name: "trimoffsetp",
                    description: Some("Trim for PMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimoffsetn",
                    description: Some("Trim for NMOS differential pairs."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tstref",
                    description: Some("OPAMP calibration reference voltage output control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Tstref"),
                },
                Field {
                    name: "calout",
                    description: Some("Operational amplifier calibration output."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some("OPAMP_CSR register lock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("OpampCsrLock"),
                },
            ],
        },
        FieldSet {
            name: "OpampTcmr",
            extends: None,
            description: Some("OPAMP timer-controlled mode register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vms_sel",
                    description: Some("OPAMP inverting input secondary selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("VmsSel"),
                },
                Field {
                    name: "vps_sel",
                    description: Some("OPAMP noninverting input secondary selection."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("VpsSel"),
                },
                Field {
                    name: "timcm_sel",
                    description: Some("Timer toggle signal selection for operational amplifier input control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("TimcmSel"),
                },
                Field {
                    name: "pgas_gain",
                    description: Some("Operational amplifier programmable gain and PGA flavor secondary control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 5,
                    array: None,
                    enumm: Some("PgasGain"),
                },
                Field {
                    name: "timpga_sel",
                    description: Some("Timer toggle signal selection for programmable gain control."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("TimpgaSel"),
                },
                Field {
                    name: "lock",
                    description: Some("OPAMP_TCMR register lock."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("OpampTcmrLock"),
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
                    name: "B0x0",
                    description: Some("0."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("0."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("0."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("0."),
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
                    name: "B0x0",
                    description: Some("Do not force (normal operation)."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("Force."),
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
                    name: "B0x0",
                    description: Some("Normal speed."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("High speed."),
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
                    name: "B0x0",
                    description: Some("OPAMP_VOUT pin."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("ADC/COMP channel."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OpampCsrLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("OPAMP_CSR is read-write."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("OPAMP_CSR is read-only."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OpampTcmrLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("Read-write."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("Read-only (OPAMP_TCMR locked)."),
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
                    name: "B0x0",
                    description: Some("gain 2."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("gain 4."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x10",
                    description: Some("gain 2 with filtering on OPAMP_VINM0."),
                    value: 16,
                },
                EnumVariant {
                    name: "B0x11",
                    description: Some("gain 4 with filtering on OPAMP_VINM0."),
                    value: 17,
                },
                EnumVariant {
                    name: "B0x12",
                    description: Some("gain 8 with filtering on OPAMP_VINM0."),
                    value: 18,
                },
                EnumVariant {
                    name: "B0x13",
                    description: Some("gain 16 with filtering on OPAMP_VINM0."),
                    value: 19,
                },
                EnumVariant {
                    name: "B0x18",
                    description: Some("gain -1 / gain 2 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 24,
                },
                EnumVariant {
                    name: "B0x19",
                    description: Some("gain -3 / gain 4 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 25,
                },
                EnumVariant {
                    name: "B0x1a",
                    description: Some("gain -7 / gain 8 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 26,
                },
                EnumVariant {
                    name: "B0x1b",
                    description: Some("gain -15 / gain 16 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 27,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("gain 8."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("gain 16."),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x8",
                    description: Some("gain -1 / gain 2 with bias on OPAMP_VINM0."),
                    value: 8,
                },
                EnumVariant {
                    name: "B0x9",
                    description: Some("gain -3 / gain 4 with bias on OPAMP_VINM0."),
                    value: 9,
                },
                EnumVariant {
                    name: "B0xA",
                    description: Some("gain -7 / gain 8 with bias on OPAMP_VINM0."),
                    value: 10,
                },
                EnumVariant {
                    name: "B0xB",
                    description: Some("gain -15 / gain 16 with bias on OPAMP_VINM0."),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "PgasGain",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("gain 2."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("gain 4."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x10",
                    description: Some("gain 2 with filtering on OPAMP_VINM0."),
                    value: 16,
                },
                EnumVariant {
                    name: "B0x11",
                    description: Some("gain 4 with filtering on OPAMP_VINM0."),
                    value: 17,
                },
                EnumVariant {
                    name: "B0x12",
                    description: Some("gain 8 with filtering on OPAMP_VINM0."),
                    value: 18,
                },
                EnumVariant {
                    name: "B0x13",
                    description: Some("gain 16 with filtering on OPAMP_VINM0."),
                    value: 19,
                },
                EnumVariant {
                    name: "B0x18",
                    description: Some("gain -1 / gain 2 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 24,
                },
                EnumVariant {
                    name: "B0x19",
                    description: Some("gain -3 / gain 4 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 25,
                },
                EnumVariant {
                    name: "B0x1a",
                    description: Some("gain -7 / gain 8 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 26,
                },
                EnumVariant {
                    name: "B0x1b",
                    description: Some("gain -15 / gain of 16 with bias on OPAMP_VINM0 and filtering on OPAMP_VINM1."),
                    value: 27,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("gain 8."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("gain 16."),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x8",
                    description: Some("gain -1 / gain 2 with bias on OPAMP_VINM0."),
                    value: 8,
                },
                EnumVariant {
                    name: "B0x9",
                    description: Some("gain -3 / gain 4 with bias on OPAMP_VINM0."),
                    value: 9,
                },
                EnumVariant {
                    name: "B0xA",
                    description: Some("gain -7 / gain 8 with bias on OPAMP_VINM0."),
                    value: 10,
                },
                EnumVariant {
                    name: "B0xB",
                    description: Some("gain -15 / gain 16 with bias on OPAMP_VINM0."),
                    value: 11,
                },
            ],
        },
        Enum {
            name: "TimcmSel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some(
                        "None (input configuration permanently controlled through VP_SEL[1:0] and VM_SEL).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("opamp_tc1."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("opamp_tc2."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("opamp_tc3."),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x4",
                    description: Some("opamp_tc4."),
                    value: 4,
                },
                EnumVariant {
                    name: "B0x5",
                    description: Some("opamp_tc5."),
                    value: 5,
                },
                EnumVariant {
                    name: "B0x6",
                    description: Some("opamp_tc6."),
                    value: 6,
                },
                EnumVariant {
                    name: "B0x7",
                    description: Some("opamp_tc7."),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "TimpgaSel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("None (gain permanently controlled through PGA_GAIN[4:0] and VM_SEL[1:0])."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("opamp_tp1."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("opamp_tp2."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("opamp_tp3."),
                    value: 3,
                },
                EnumVariant {
                    name: "B0x4",
                    description: Some("opamp_tp4."),
                    value: 4,
                },
                EnumVariant {
                    name: "B0x5",
                    description: Some("opamp_tp5."),
                    value: 5,
                },
                EnumVariant {
                    name: "B0x6",
                    description: Some("opamp_tp6."),
                    value: 6,
                },
                EnumVariant {
                    name: "B0x7",
                    description: Some("opamp_tp7."),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Tstref",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("Does not output calibration reference voltage."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("Outputs calibration reference voltage."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usertrim",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B0x0",
                    description: Some("Disable (trimming by user not possible)."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("Enable (trimming by user possible)."),
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
                    name: "B0x0",
                    description: Some("OPAMP_VINM0 pin."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("OPAMP_VINM1 pin."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("Internal resistor divider middle point."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("Operational amplifier output."),
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
                    name: "B0x0",
                    description: Some("Internal resistor divider middle point (operation as PGA)."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("Operational amplifier output (operation as follower)."),
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
                    name: "B0x0",
                    description: Some("OPAMP_VINP0 pin."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("OPAMP_VINP1 pin."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("OPAMP_VINP2 pin."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("OPAMP_VINP3 pin or DAC output channel."),
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
                    name: "B0x0",
                    description: Some("OPAMP_VINP0."),
                    value: 0,
                },
                EnumVariant {
                    name: "B0x1",
                    description: Some("OPAMP_VINP1."),
                    value: 1,
                },
                EnumVariant {
                    name: "B0x2",
                    description: Some("OPAMP_VINP2."),
                    value: 2,
                },
                EnumVariant {
                    name: "B0x3",
                    description: Some("OPAMP_VINP3 pin or DAC output channel."),
                    value: 3,
                },
            ],
        },
    ],
};
