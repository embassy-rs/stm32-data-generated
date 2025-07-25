
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "Real-time clock",
            ),
            items: &[
                BlockItem {
                    name: "tr",
                    description: Some(
                        "Time register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Date register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ssr",
                    description: Some(
                        "Sub second register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icsr",
                    description: Some(
                        "Initialization control and status register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prer",
                    description: Some(
                        "Prescaler register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpr",
                    description: Some(
                        "Write protection register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calr",
                    description: Some(
                        "Calibration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shiftr",
                    description: Some(
                        "Shift control register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Shiftr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tstr",
                    description: Some(
                        "Timestamp time register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsdr",
                    description: Some(
                        "Timestamp date register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsssr",
                    description: Some(
                        "Timestamp sub second register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrmr",
                    description: Some(
                        "Alarm register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrmssr",
                    description: Some(
                        "Alarm sub second register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrmssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                        "Masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 0x54,
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
                        "Status clear register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alrmr",
            extends: None,
            description: Some(
                "Alarm register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
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
                    name: "msk1",
                    description: Some(
                        "Alarm A seconds mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk2",
                    description: Some(
                        "Alarm A minutes mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrPm",
                    ),
                },
                Field {
                    name: "msk3",
                    description: Some(
                        "Alarm A hours mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
                Field {
                    name: "du",
                    description: Some(
                        "Date units or day in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdsel",
                    description: Some(
                        "Week day selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrWdsel",
                    ),
                },
                Field {
                    name: "msk4",
                    description: Some(
                        "Alarm A date mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Alrmssr",
            extends: None,
            description: Some(
                "Alarm sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Sub seconds value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "maskss",
                    description: Some(
                        "Mask the most-significant bits starting at this bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calr",
            extends: None,
            description: Some(
                "Calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calm",
                    description: Some(
                        "Calibration minus",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calw16",
                    description: Some(
                        "Use a 16-second calibration cycle period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calw16",
                    ),
                },
                Field {
                    name: "calw8",
                    description: Some(
                        "Use an 8-second calibration cycle period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calw8",
                    ),
                },
                Field {
                    name: "calp",
                    description: Some(
                        "Increase frequency of RTC by 488.5 ppm",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsedge",
                    description: Some(
                        "Timestamp event active edge",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsedge",
                    ),
                },
                Field {
                    name: "refckon",
                    description: Some(
                        "RTC_REFIN reference clock detection enable (50 or 60 Hz)",
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
                    name: "bypshad",
                    description: Some(
                        "Bypass the shadow registers",
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
                    name: "fmt",
                    description: Some(
                        "Hour format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmt",
                    ),
                },
                Field {
                    name: "alre",
                    description: Some(
                        "Alarm enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tse",
                    description: Some(
                        "Timestamp enable",
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
                    name: "alrie",
                    description: Some(
                        "Alarm interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tsie",
                    description: Some(
                        "Timestamp interrupt enable",
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
                    name: "add1h",
                    description: Some(
                        "Add 1 hour (summer time change)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sub1h",
                    description: Some(
                        "Subtract 1 hour (winter time change)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkp",
                    description: Some(
                        "Backup",
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
                    name: "cosel",
                    description: Some(
                        "Calibration output selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cosel",
                    ),
                },
                Field {
                    name: "pol",
                    description: Some(
                        "Output polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pol",
                    ),
                },
                Field {
                    name: "osel",
                    description: Some(
                        "Output selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Osel",
                    ),
                },
                Field {
                    name: "coe",
                    description: Some(
                        "Calibration output enable",
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
                    name: "tampalrm_pu",
                    description: Some(
                        "TAMPALRM pull-up enable",
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
                    name: "tampalrm_type",
                    description: Some(
                        "TAMPALRM output type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TampalrmType",
                    ),
                },
                Field {
                    name: "out2en",
                    description: Some(
                        "RTC_OUT2 output enable",
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
            name: "Dr",
            extends: None,
            description: Some(
                "Date register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdu",
                    description: Some(
                        "Week day units",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "yu",
                    description: Some(
                        "Year units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "yt",
                    description: Some(
                        "Year tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icsr",
            extends: None,
            description: Some(
                "Initialization control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrwf",
                    description: Some(
                        "Alarm write enabled",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "shpf",
                    description: Some(
                        "Shift operation pending",
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
                    name: "inits",
                    description: Some(
                        "Initialization status flag",
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
                    name: "rsf",
                    description: Some(
                        "Registers synchronization flag",
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
                    name: "initf",
                    description: Some(
                        "Initialization flag",
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
                    name: "init",
                    description: Some(
                        "Enter Initialization mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "recalpf",
                    description: Some(
                        "Recalibration pending Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Recalpf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Misr",
            extends: None,
            description: Some(
                "Masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrmf",
                    description: Some(
                        "Alarm masked flag",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Alrmf",
                    ),
                },
                Field {
                    name: "tsmf",
                    description: Some(
                        "Timestamp masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsmf",
                    ),
                },
                Field {
                    name: "tsovmf",
                    description: Some(
                        "Timestamp overflow masked flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovmf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Prer",
            extends: None,
            description: Some(
                "Prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prediv_s",
                    description: Some(
                        "Synchronous prescaler factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prediv_a",
                    description: Some(
                        "Asynchronous prescaler factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some(
                "Status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calrf",
                    description: Some(
                        "Clear alarm A flag",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "ctsf",
                    description: Some(
                        "Clear timestamp flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "ctsovf",
                    description: Some(
                        "Clear timestamp overflow flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Shiftr",
            extends: None,
            description: Some(
                "Shift control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subfs",
                    description: Some(
                        "Subtract a fraction of a second",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add1s",
                    description: Some(
                        "Add one second",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrf",
                    description: Some(
                        "Alarm flag",
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
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Alrf",
                    ),
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "Timestamp flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsf",
                    ),
                },
                Field {
                    name: "tsovf",
                    description: Some(
                        "Timestamp overflow flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ssr",
            extends: None,
            description: Some(
                "Sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous binary counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tr",
            extends: None,
            description: Some(
                "Time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
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
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ampm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Tsdr",
            extends: None,
            description: Some(
                "Timestamp date register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "du",
                    description: Some(
                        "Date units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdu",
                    description: Some(
                        "Week day units",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tsssr",
            extends: None,
            description: Some(
                "Timestamp sub second register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Sub second value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tstr",
            extends: None,
            description: Some(
                "Timestamp time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su",
                    description: Some(
                        "Second units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
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
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
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
            ],
        },
        FieldSet {
            name: "Wpr",
            extends: None,
            description: Some(
                "Write protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "Write protection key",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Key",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Alrf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MATCH",
                    description: Some(
                        "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Alrmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MATCH",
                    description: Some(
                        "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrMsk",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TO_MATCH",
                    description: Some(
                        "Alarm set if the date/day match",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOT_MATCH",
                    description: Some(
                        "Date/day don’t care in Alarm comparison",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrPm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AM",
                    description: Some(
                        "AM or 24-hour format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PM",
                    description: Some(
                        "PM",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmrWdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATE_UNITS",
                    description: Some(
                        "DU[3:0] represents the date units",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WEEK_DAY",
                    description: Some(
                        "DU[3:0] represents the week day. DT[1:0] is don’t care.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ampm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AM",
                    description: Some(
                        "AM or 24-hour format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PM",
                    description: Some(
                        "PM",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NO_CHANGE",
                    description: Some(
                        "No RTCCLK pulses are added",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREASE_FREQ",
                    description: Some(
                        "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calrf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clear interrupt flag by writing 1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calw16",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SIXTEEN_SECONDS",
                    description: Some(
                        "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Calw8",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EIGHT_SECONDS",
                    description: Some(
                        "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CAL_FREQ_512HZ",
                    description: Some(
                        "Calibration output is 512 Hz (with default prescaler setting)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CAL_FREQ_1HZ",
                    description: Some(
                        "Calibration output is 1 Hz (with default prescaler setting)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fmt",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TWENTY_FOUR_HOUR",
                    description: Some(
                        "24 hour/day format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AM_PM",
                    description: Some(
                        "AM/PM hour format",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Key",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "ACTIVATE",
                    description: Some(
                        "Activate write protection (any value that is not the keys)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEACTIVATE2",
                    description: Some(
                        "Key 2",
                    ),
                    value: 83,
                },
                EnumVariant {
                    name: "DEACTIVATE1",
                    description: Some(
                        "Key 1",
                    ),
                    value: 202,
                },
            ],
        },
        Enum {
            name: "Osel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Output disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALARM_A",
                    description: Some(
                        "Alarm A output enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALARM_B",
                    description: Some(
                        "Alarm B output enabled",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "WAKEUP",
                    description: Some(
                        "Wakeup output enabled",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0])",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Recalpf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PENDING",
                    description: Some(
                        "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TampalrmType",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PUSH_PULL",
                    description: Some(
                        "TAMPALRM is push-pull output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPEN_DRAIN",
                    description: Some(
                        "TAMPALRM is open-drain output",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsedge",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some(
                        "RTC_TS input rising edge generates a time-stamp event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some(
                        "RTC_TS input falling edge generates a time-stamp event",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMP_EVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsovf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OVERFLOW",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs while TSF is already set",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tsovmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OVERFLOW",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs while TSF is already set",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
