
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
                    name: "sr",
                    description: Some(
                        "Comparator status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icfr",
                    description: Some(
                        "Comparator interrupt clear flag register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "Comparator configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "Comparator configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "Comparator configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP-Channel1.",
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
                    name: "brgen",
                    description: Some(
                        "Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4-V<sub>REF_COMP</sub>, 1/2-V<sub>REF_COMP</sub> and 1/4-V<sub>REF_COMP</sub> levels, respectively.",
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
                    name: "scalen",
                    description: Some(
                        "Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "polarity",
                    description: Some(
                        "COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.",
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
                    name: "iten",
                    description: Some(
                        "COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hyst",
                    description: Some(
                        "COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hyst",
                    ),
                },
                Field {
                    name: "pwrmode",
                    description: Some(
                        "Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pwrmode",
                    ),
                },
                Field {
                    name: "inmsel",
                    description: Some(
                        "COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table-146: COMP1 inverting input assignment for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Inmsel",
                    ),
                },
                Field {
                    name: "inpsel1",
                    description: Some(
                        "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table-145: COMP1 noninverting input assignment for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inpsel2",
                    description: Some(
                        "COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table-145: COMP1 noninverting input assignment for more details.",
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
                    name: "blanking",
                    description: Some(
                        "COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Blanking",
                    ),
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1[31:0].",
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
            name: "Cfgr2",
            extends: None,
            description: Some(
                "Comparator configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inpsel0",
                    description: Some(
                        "COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table-145: COMP1 noninverting input assignment for more details.",
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
                    name: "lock",
                    description: Some(
                        "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2[31:0].",
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
            name: "Icfr",
            extends: None,
            description: Some(
                "Comparator interrupt clear flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccif",
                    description: Some(
                        "Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 0,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Comparator status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cval",
                    description: Some(
                        "COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 0,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cif",
                    description: Some(
                        "COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 0,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Blanking",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Lptim1ch2",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Lptim2ch2",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "NoBlanking",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "Tim1oc5",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "Tim2oc3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "Tim3oc3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "Tim3oc4",
                    description: None,
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Hyst",
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
            name: "Inmsel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Dac1out1",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "Inm1",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "Inm2",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "Inm3",
                    description: None,
                    value: 7,
                },
                EnumVariant {
                    name: "VBat1over4",
                    description: None,
                    value: 9,
                },
                EnumVariant {
                    name: "VRef",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "VRef1over2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "VRef1over4",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "VRef3over4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "VSense",
                    description: None,
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Pwrmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "High",
                    description: Some(
                        "High speed / full power",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Low",
                    description: Some(
                        "Ultra low power / ultra-low-power",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Medium",
                    description: Some(
                        "Medium speed / medium power",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MediumEither",
                    description: Some(
                        "Medium speed / medium power",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
