
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Opamp",
            extends: None,
            description: Some(
                "Operational amplifier",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "Control/status register",
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
                        "Offset trimming register in normal mode",
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
                        "Offset trimming register in low-power mode",
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
                "Control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opampen",
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
                    name: "force_vp",
                    description: Some(
                        "Force internal reference on VP (reserved for test)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vp_sel",
                    description: Some(
                        "Non-inverting input selection",
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
                        "Inverting input selection",
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
                        "High-speed mode enable",
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
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "PgaGain",
                    ),
                },
                Field {
                    name: "usertrim",
                    description: Some(
                        "User trimming enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tstref",
                    description: Some(
                        "Output the internal reference voltage",
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
                        "Calibration output",
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
            ],
        },
        FieldSet {
            name: "Hsotr",
            extends: None,
            description: Some(
                "Offset trimming register in low-power mode",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimlpoffsetn",
                    description: Some(
                        "Offset trimming value (NMOS)",
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
                        "Offset trimming value (PMOS)",
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
                "Offset trimming register in normal mode",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trimoffsetn",
                    description: Some(
                        "Offset trimming value (NMOS)",
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
                        "Offset trimming value (PMOS)",
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
            name: "Calsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Percent10",
                    description: Some(
                        "VREFOPAMP = 10% VDDA",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Percent33",
                    description: Some(
                        "VREFOPAMP = 3.3% VDDA",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Percent50",
                    description: Some(
                        "VREFOPAMP = 50% VDDA",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Percent90",
                    description: Some(
                        "VREFOPAMP = 90% VDDA",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PgaGain",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Gain16",
                    description: Some(
                        "Non-inverting internal gain 16, VREF- referenced",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Gain16FilteringVinm0",
                    description: Some(
                        "Non-inverting internal gain 8 with filtering on INM0, VREF- referenced",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Gain16invGainNeg15InputVinm0",
                    description: Some(
                        "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "Gain16invGainNeg15InputVinm0filteringVinm1",
                    description: Some(
                        "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "Gain2",
                    description: Some(
                        "Non-inverting internal gain 2, VREF- referenced",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Gain2FilteringVinm0",
                    description: Some(
                        "Non-inverting internal gain 2 with filtering on INM0, VREF- referenced",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Gain2invGainNeg1InputVinm0",
                    description: Some(
                        "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Gain2invGainNeg1InputVinm0filteringVinm1",
                    description: Some(
                        "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "Gain4",
                    description: Some(
                        "Non-inverting internal gain 4, VREF- referenced",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Gain4FilteringVinm0",
                    description: Some(
                        "Non-inverting internal gain 4 with filtering on INM0, VREF- referenced",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Gain4invGainNeg3InputVinm0",
                    description: Some(
                        "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "Gain4invGainNeg3InputVinm0filteringVinm1",
                    description: Some(
                        "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "Gain8",
                    description: Some(
                        "Non-inverting internal gain 8, VREF- referenced",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Gain8FilteringVinm0",
                    description: Some(
                        "Non-inverting internal gain 8 with filtering on INM0, VREF- referenced",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Gain8invGainNeg7InputVinm0",
                    description: Some(
                        "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "Gain8invGainNeg7InputVinm0filteringVinm1",
                    description: Some(
                        "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering",
                    ),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "VmSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Follower",
                    description: Some(
                        "opamp_out connected to OPAMP_VINM input (Follower mode)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Inm0",
                    description: Some(
                        "INM0 connected to OPAMP_VINM input",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Inm1",
                    description: Some(
                        "INM1 connected to OPAMP_VINM input",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Pga",
                    description: Some(
                        "Feedback resistor connected to the OPAMP_VINM input (PGA mode), Inverting input selection depends on the PGA_GAIN setting",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "VpSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DacOut",
                    description: Some(
                        "DAC connected to VINP",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "GpioInp0",
                    description: Some(
                        "GPIO INP0 connected to VINP",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GpioInp2",
                    description: Some(
                        "GPIO INP2 connected to OPAMP_VINP (not available on all chips)",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
