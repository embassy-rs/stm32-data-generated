
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
                    byte_offset: 0,
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
                    byte_offset: 4,
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
                    byte_offset: 8,
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
                    byte_offset: 12,
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
                    byte_offset: 16,
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
                    name: "wutr",
                    description: Some(
                        "Wakeup timer register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wutr",
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
                    byte_offset: 24,
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
                    name: "privcr",
                    description: Some(
                        "Privilege mode control register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "Secure mode control register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
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
                    byte_offset: 36,
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
                    byte_offset: 40,
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
                    byte_offset: 44,
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
                    byte_offset: 48,
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
                    byte_offset: 52,
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
                    byte_offset: 56,
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
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 64,
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
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 68,
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
                    byte_offset: 80,
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
                    byte_offset: 84,
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
                    name: "smisr",
                    description: Some(
                        "Secure masked interrupt status register",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Smisr",
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
                    byte_offset: 92,
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
                    name: "alrbinr",
                    description: Some(
                        "Alarm binary mode register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrbinr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
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
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
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
                    bit_offset: 0,
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "maskss",
                    description: Some(
                        "Mask the most-significant bits starting at this bit",
                    ),
                    bit_offset: 24,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssclr",
                    description: Some(
                        "Clear synchronous counter on alarm (Binary mode only)",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmssrSsclr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "Secure mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrasec",
                    description: Some(
                        "ALRASEC",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrbsec",
                    description: Some(
                        "ALRBSEC",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wutsec",
                    description: Some(
                        "WUTSEC",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tssec",
                    description: Some(
                        "TSSEC",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calsec",
                    description: Some(
                        "CALSEC",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "initsec",
                    description: Some(
                        "INITSEC",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sec",
                    description: Some(
                        "SEC",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
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
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ampm",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smisr",
            extends: None,
            description: Some(
                "Secure masked interrupt status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alramf",
                    description: Some(
                        "ALRAMF",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrbmf",
                    description: Some(
                        "ALRBMF",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wutmf",
                    description: Some(
                        "WUTMF",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsmf",
                    description: Some(
                        "TSMF",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsovmf",
                    description: Some(
                        "TSOVMF",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itsmf",
                    description: Some(
                        "ITSMF",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssrumf",
                    description: Some(
                        "SSRUMF",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
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
                    bit_offset: 0,
                    bit_size: 32,
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
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Key",
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
                    name: "wucksel",
                    description: Some(
                        "Wakeup clock selection",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Wucksel",
                    ),
                },
                Field {
                    name: "tsedge",
                    description: Some(
                        "Timestamp event active edge",
                    ),
                    bit_offset: 3,
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
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Refckon",
                    ),
                },
                Field {
                    name: "bypshad",
                    description: Some(
                        "Bypass the shadow registers",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmt",
                    description: Some(
                        "Hour format",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fmt",
                    ),
                },
                Field {
                    name: "ssruie",
                    description: Some(
                        "SSR underflow interrupt enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alre",
                    description: Some(
                        "Alarm enable",
                    ),
                    bit_offset: 8,
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
                    name: "wute",
                    description: Some(
                        "Wakeup timer enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tse",
                    description: Some(
                        "Timestamp enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alraie",
                    description: Some(
                        "Alarm interrupt enable",
                    ),
                    bit_offset: 12,
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
                    name: "wutie",
                    description: Some(
                        "Wakeup timer interrupt enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsie",
                    description: Some(
                        "Timestamp interrupt enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add1h",
                    description: Some(
                        "Add 1 hour (summer time change)",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sub1h",
                    description: Some(
                        "Subtract 1 hour (winter time change)",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkp",
                    description: Some(
                        "Backup",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cosel",
                    description: Some(
                        "Calibration output selection",
                    ),
                    bit_offset: 19,
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
                    bit_offset: 20,
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
                    bit_offset: 21,
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
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itse",
                    description: Some(
                        "Timestamp on internal event enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tampts",
                    description: Some(
                        "Activate timestamp on tamper detection event",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tampoe",
                    description: Some(
                        "Tamper detection output enable on TAMPALRM",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrfclr",
                    description: Some(
                        "ALRFCLR",
                    ),
                    bit_offset: 27,
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
                    name: "tampalrm_pu",
                    description: Some(
                        "TAMPALRM pull-up enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TampalrmPu",
                    ),
                },
                Field {
                    name: "tampalrm_type",
                    description: Some(
                        "TAMPALRM output type",
                    ),
                    bit_offset: 30,
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
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
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
                    bit_offset: 0,
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add1s",
                    description: Some(
                        "Add one second",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
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
                    enumm: Some(
                        "Alrmf",
                    ),
                },
                Field {
                    name: "wutmf",
                    description: Some(
                        "Wakeup timer masked flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wutmf",
                    ),
                },
                Field {
                    name: "tsmf",
                    description: Some(
                        "Timestamp masked flag",
                    ),
                    bit_offset: 3,
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
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovmf",
                    ),
                },
                Field {
                    name: "itsmf",
                    description: Some(
                        "Internal timestamp masked flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Itsmf",
                    ),
                },
                Field {
                    name: "ssrumf",
                    description: Some(
                        "SSR underflow masked flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssrumf",
                    ),
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
                    name: "wutwf",
                    description: Some(
                        "Wakeup timer write flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shpf",
                    description: Some(
                        "Shift operation pending",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inits",
                    description: Some(
                        "Initialization status flag",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsf",
                    description: Some(
                        "Registers synchronization flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "initf",
                    description: Some(
                        "Initialization flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "init",
                    description: Some(
                        "Initialization mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Init",
                    ),
                },
                Field {
                    name: "bin",
                    description: Some(
                        "Binary mode",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Bin",
                    ),
                },
                Field {
                    name: "bcdu",
                    description: Some(
                        "BCD update",
                    ),
                    bit_offset: 10,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Bcdu",
                    ),
                },
                Field {
                    name: "recalpf",
                    description: Some(
                        "Recalibration pending Flag",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Recalpf",
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
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdu",
                    description: Some(
                        "Week day units",
                    ),
                    bit_offset: 13,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Alrbinr",
            extends: None,
            description: Some(
                "RTC alarm A binary mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss",
                    description: Some(
                        "Synchronous counter alarm value in Binary mode",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
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
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk1",
                    description: Some(
                        "Alarm A seconds mask",
                    ),
                    bit_offset: 7,
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
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk2",
                    description: Some(
                        "Alarm A minutes mask",
                    ),
                    bit_offset: 15,
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
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
                    ),
                    bit_offset: 22,
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
                    bit_offset: 23,
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
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: 28,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdsel",
                    description: Some(
                        "Week day selection",
                    ),
                    bit_offset: 30,
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
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AlrmrMsk",
                    ),
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
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st",
                    description: Some(
                        "Second tens in BCD format",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnu",
                    description: Some(
                        "Minute units in BCD format",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mnt",
                    description: Some(
                        "Minute tens in BCD format",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hu",
                    description: Some(
                        "Hour units in BCD format",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht",
                    description: Some(
                        "Hour tens in BCD format",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "AM/PM notation",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
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
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpcal",
                    description: Some(
                        "Calibration low-power mode",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lpcal",
                    ),
                },
                Field {
                    name: "calw16",
                    description: Some(
                        "Use a 16-second calibration cycle period",
                    ),
                    bit_offset: 13,
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
                    bit_offset: 14,
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
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wutr",
            extends: None,
            description: Some(
                "Wakeup timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wut",
                    description: Some(
                        "Wakeup auto-reload value bits",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wutoclr",
                    description: Some(
                        "Wakeup auto-reload output clear value",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
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
                    bit_offset: 0,
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prediv_a",
                    description: Some(
                        "Asynchronous prescaler factor",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcr",
            extends: None,
            description: Some(
                "Privilege mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alrapriv",
                    description: Some(
                        "ALRAPRIV",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrbpriv",
                    description: Some(
                        "ALRBPRIV",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wutpriv",
                    description: Some(
                        "WUTPRIV",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tspriv",
                    description: Some(
                        "TSPRIV",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calpriv",
                    description: Some(
                        "CALPRIV",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "initpriv",
                    description: Some(
                        "INITPRIV",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "priv_",
                    description: Some(
                        "PRIV",
                    ),
                    bit_offset: 15,
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
                    enumm: Some(
                        "Alrf",
                    ),
                },
                Field {
                    name: "wutf",
                    description: Some(
                        "Wakeup timer flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wutf",
                    ),
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "Timestamp flag",
                    ),
                    bit_offset: 3,
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
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tsovf",
                    ),
                },
                Field {
                    name: "itsf",
                    description: Some(
                        "Internal timestamp flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Itsf",
                    ),
                },
                Field {
                    name: "ssruf",
                    description: Some(
                        "SSR underflow flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ssruf",
                    ),
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
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "cwutf",
                    description: Some(
                        "Clear wakeup timer flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "ctsf",
                    description: Some(
                        "Clear timestamp flag",
                    ),
                    bit_offset: 3,
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
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "citsf",
                    description: Some(
                        "Clear internal timestamp flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
                },
                Field {
                    name: "cssruf",
                    description: Some(
                        "Clear SSR underflow flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Calrf",
                    ),
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
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dt",
                    description: Some(
                        "Date tens in BCD format",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mu",
                    description: Some(
                        "Month units in BCD format",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mt",
                    description: Some(
                        "Month tens in BCD format",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdu",
                    description: Some(
                        "Week day units",
                    ),
                    bit_offset: 13,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "yu",
                    description: Some(
                        "Year units in BCD format",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "yt",
                    description: Some(
                        "Year tens in BCD format",
                    ),
                    bit_offset: 20,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "TampalrmType",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PUSHPULL",
                    description: Some(
                        "TAMPALRM is push-pull output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPENDRAIN",
                    description: Some(
                        "TAMPALRM is open-drain output",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wucksel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "RTC/16 clock is selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "RTC/8 clock is selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "RTC/4 clock is selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "RTC/2 clock is selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CLOCKSPARE",
                    description: Some(
                        "ck_spre (usually 1 Hz) clock is selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CLOCKSPAREWITHOFFSET",
                    description: Some(
                        "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Ssrumf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNDERFLOW",
                    description: Some(
                        "This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1",
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
                    name: "TIMESTAMPEVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
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
            name: "Bcdu",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BIT7",
                    description: Some(
                        "1s increment each time SS[7:0]=0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT8",
                    description: Some(
                        "1s increment each time SS[8:0]=0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIT9",
                    description: Some(
                        "1s increment each time SS[9:0]=0",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "1s increment each time SS[10:0]=0",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BIT11",
                    description: Some(
                        "1s increment each time SS[11:0]=0",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BIT12",
                    description: Some(
                        "1s increment each time SS[12:0]=0",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BIT13",
                    description: Some(
                        "1s increment each time SS[13:0]=0",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BIT14",
                    description: Some(
                        "1s increment each time SS[14:0]=0",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "AlrmrWdsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DATEUNITS",
                    description: Some(
                        "DU[3:0] represents the date units",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "WEEKDAY",
                    description: Some(
                        "DU[3:0] represents the week day. DT[1:0] is don’t care.",
                    ),
                    value: 1,
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
            name: "Tsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMPEVENT",
                    description: Some(
                        "This flag is set by hardware when a time-stamp event occurs",
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
            name: "Calp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOCHANGE",
                    description: Some(
                        "No RTCCLK pulses are added",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INCREASEFREQ",
                    description: Some(
                        "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)",
                    ),
                    value: 1,
                },
            ],
        },
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
            name: "Init",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FREERUNNINGMODE",
                    description: Some(
                        "Free running mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INITMODE",
                    description: Some(
                        "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "AlrmssrSsclr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FREERUNNING",
                    description: Some(
                        "The synchronous binary counter (SS[31:0] in RTC_SSR) is free-running",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALRMBINR",
                    description: Some(
                        "The synchronous binary counter (SS[31:0] in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS[31:0] value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS[31:0]",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TampalrmPu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOPULLUP",
                    description: Some(
                        "No pull-up is applied on TAMPALRM output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PULLUP",
                    description: Some(
                        "A pull-up is applied on TAMPALRM output",
                    ),
                    value: 1,
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
                    name: "ALARMA",
                    description: Some(
                        "Alarm A output enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALARMB",
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
            name: "Ssruf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNDERFLOW",
                    description: Some(
                        "This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1",
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
            name: "Wutmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ZERO",
                    description: Some(
                        "This flag is set by hardware when the wakeup auto-reload counter reaches 0",
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
                    name: "RISINGEDGE",
                    description: Some(
                        "RTC_TS input rising edge generates a time-stamp event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "RTC_TS input falling edge generates a time-stamp event",
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
                    name: "EIGHTSECONDS",
                    description: Some(
                        "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected",
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
                    name: "TWENTYFOURHOUR",
                    description: Some(
                        "24 hour/day format",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AMPM",
                    description: Some(
                        "AM/PM hour format",
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
        Enum {
            name: "Lpcal",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RTCCLK",
                    description: Some(
                        "Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CKAPRE",
                    description: Some(
                        "Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Itsf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMPEVENT",
                    description: Some(
                        "This flag is set by hardware when a timestamp on the internal event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Refckon",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "RTC_REFIN detection disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "RTC_REFIN detection enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bin",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BCD",
                    description: Some(
                        "Free running BCD calendar mode (Binary mode disabled)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BINARY",
                    description: Some(
                        "Free running Binary mode (BCD mode disabled)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BINBCD",
                    description: Some(
                        "Free running BCD calendar and Binary modes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BINBCD2",
                    description: Some(
                        "Free running BCD calendar and Binary modes",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Calw16",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SIXTEENSECONDS",
                    description: Some(
                        "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1",
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
                    name: "CALFREQ_512HZ",
                    description: Some(
                        "Calibration output is 512 Hz (with default prescaler setting)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CALFREQ_1HZ",
                    description: Some(
                        "Calibration output is 1 Hz (with default prescaler setting)",
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
            name: "Itsmf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TIMESTAMPEVENT",
                    description: Some(
                        "This flag is set by hardware when a timestamp on the internal event occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wutf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ZERO",
                    description: Some(
                        "This flag is set by hardware when the wakeup auto-reload counter reaches 0",
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
                    name: "MASK",
                    description: Some(
                        "Alarm set if the date/day match",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOTMASK",
                    description: Some(
                        "Date/day don’t care in Alarm comparison",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
