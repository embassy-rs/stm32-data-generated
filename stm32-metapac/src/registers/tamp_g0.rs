
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tamp",
            extends: None,
            description: Some(
                "Tamper and backup registers",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltcr",
                    description: Some(
                        "TAMP filter control register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "TAMP interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "TAMP status register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misr",
                    description: Some(
                        "TAMP masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scr",
                    description: Some(
                        "TAMP status clear register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bkpr",
                    description: Some(
                        "TAMP backup register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 256,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bkpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hwcfgr2",
                    description: Some(
                        "TAMP hardware configuration register 2",
                    ),
                    array: None,
                    byte_offset: 1004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hwcfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hwcfgr1",
                    description: Some(
                        "TAMP hardware configuration register 1",
                    ),
                    array: None,
                    byte_offset: 1008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hwcfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "verr",
                    description: Some(
                        "EXTI IP Version register",
                    ),
                    array: None,
                    byte_offset: 1012,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Verr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipidr",
                    description: Some(
                        "EXTI Identification register",
                    ),
                    array: None,
                    byte_offset: 1016,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sidr",
                    description: Some(
                        "EXTI Size ID register",
                    ),
                    array: None,
                    byte_offset: 1020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sidr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Misr",
            extends: None,
            description: Some(
                "TAMP masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampmf",
                    description: Some(
                        "Tamper X interrupt masked flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampmf",
                    description: Some(
                        "Internal tamper X interrupt masked flag",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampnoer",
                    description: Some(
                        "Tamper X no erase",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tampmsk",
                    description: Some(
                        "Tamper X mask",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tamptrg",
                    description: Some(
                        "Active level for tamper X input",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hwcfgr2",
            extends: None,
            description: Some(
                "TAMP hardware configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptionreg_out",
                    description: Some(
                        "PTIONREG_OUT",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trust_zone",
                    description: Some(
                        "TRUST_ZONE",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bkpr",
            extends: None,
            description: Some(
                "TAMP backup register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkp",
                    description: Some(
                        "BKP",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hwcfgr1",
            extends: None,
            description: Some(
                "TAMP hardware configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "backup_regs",
                    description: Some(
                        "BACKUP_REGS",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tamper",
                    description: Some(
                        "TAMPER",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "active_tamper",
                    description: Some(
                        "ACTIVE_TAMPER",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "int_tamper",
                    description: Some(
                        "INT_TAMPER",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some(
                "TAMP status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctampf",
                    description: Some(
                        "Clear tamper X detection flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "citampf",
                    description: Some(
                        "Clear internal tamper X detection flag",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipidr",
            extends: None,
            description: Some(
                "EXTI Identification register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipid",
                    description: Some(
                        "IP Identification",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fltcr",
            extends: None,
            description: Some(
                "TAMP filter control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampfreq",
                    description: Some(
                        "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled.",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tampflt",
                    description: Some(
                        "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs.",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tampprch",
                    description: Some(
                        "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs.",
                    ),
                    bit_offset: 5,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tamppudis",
                    description: Some(
                        "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampe",
                    description: Some(
                        "Tamper detection on IN X enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampe",
                    description: Some(
                        "Internal tamper X enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Verr",
            extends: None,
            description: Some(
                "EXTI IP Version register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "minrev",
                    description: Some(
                        "Minor Revision number",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "majrev",
                    description: Some(
                        "Major Revision number",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "TAMP status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampf",
                    description: Some(
                        "Tamper X detection flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampf",
                    description: Some(
                        "Internal tamper X detection flag",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "TAMP interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tampie",
                    description: Some(
                        "Tamper X interrupt enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "itampie",
                    description: Some(
                        "Internal tamper X interrupt enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sidr",
            extends: None,
            description: Some(
                "EXTI Size ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sid",
                    description: Some(
                        "Size Identification",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
