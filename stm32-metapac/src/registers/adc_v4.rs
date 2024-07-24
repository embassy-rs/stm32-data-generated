
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog to Digital Converter",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "interrupt and status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "cfgr",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
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
                BlockItem {
                    name: "smpr",
                    description: Some(
                        "sampling time register 1-2",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcsel",
                    description: Some(
                        "pre channel selection register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr1",
                    description: Some(
                        "analog watchdog 1 threshold register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr1",
                    description: Some(
                        "analog watchdog 2 threshold register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr1",
                    description: Some(
                        "group regular sequencer ranks register 1",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr2",
                    description: Some(
                        "group regular sequencer ranks register 2",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr3",
                    description: Some(
                        "group regular sequencer ranks register 3",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr4",
                    description: Some(
                        "group regular sequencer ranks register 4",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "group regular conversion data register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jsqr",
                    description: Some(
                        "group injected sequencer register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jsqr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ofr",
                    description: Some(
                        "offset number 1-4 register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr",
                    description: Some(
                        "group injected sequencer rank 1-4 register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd2cr",
                    description: Some(
                        "analog watchdog 2 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd3cr",
                    description: Some(
                        "analog watchdog 3 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd3cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr2",
                    description: Some(
                        "watchdog lower threshold register 2",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr2",
                    description: Some(
                        "watchdog higher threshold register 2",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr3",
                    description: Some(
                        "watchdog lower threshold register 3",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr3",
                    description: Some(
                        "watchdog higher threshold register 3",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "difsel",
                    description: Some(
                        "channel differential or single-ended mode selection register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Difsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact",
                    description: Some(
                        "calibration factors register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfact",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact2",
                    description: Some(
                        "Calibration Factor register 2",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfact2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Awd2cr",
            extends: None,
            description: Some(
                "analog watchdog 2 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd2ch",
                    description: Some(
                        "analog watchdog 2 monitored channel selection",
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
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd3cr",
            extends: None,
            description: Some(
                "analog watchdog 3 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd3ch",
                    description: Some(
                        "analog watchdog 3 monitored channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact",
            extends: None,
            description: Some(
                "calibration factors register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calfact_s",
                    description: Some(
                        "calibration factor in single-ended mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calfact_d",
                    description: Some(
                        "calibration factor in differential mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact2",
            extends: None,
            description: Some(
                "Calibration Factor register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lincalfact",
                    description: Some(
                        "Linearity Calibration Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmngt",
                    description: Some(
                        "DMA transfer enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dmngt",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "data resolution",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Res",
                    ),
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "group regular external trigger source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exten",
                    description: Some(
                        "group regular external trigger polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Exten",
                    ),
                },
                Field {
                    name: "ovrmod",
                    description: Some(
                        "group regular overrun configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ovrmod",
                    ),
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Continuous conversion",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "autdly",
                    description: Some(
                        "low power auto wait",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discen",
                    description: Some(
                        "group regular sequencer discontinuous mode",
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
                    name: "discnum",
                    description: Some(
                        "group regular sequencer discontinuous number of ranks",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdiscen",
                    description: Some(
                        "group injected sequencer discontinuous mode",
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
                    name: "jqm",
                    description: Some(
                        "group injected contexts queue mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Jqm",
                    ),
                },
                Field {
                    name: "awd1sgl",
                    description: Some(
                        "analog watchdog 1 monitoring a single channel or all channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Awd1sgl",
                    ),
                },
                Field {
                    name: "awd1en",
                    description: Some(
                        "analog watchdog 1 enable on scope group regular",
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
                    name: "jawd1en",
                    description: Some(
                        "analog watchdog 1 enable on scope group injected",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jauto",
                    description: Some(
                        "group injected automatic trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ch",
                    description: Some(
                        "analog watchdog 1 monitored channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jqdis",
                    description: Some(
                        "group injected contexts queue disable",
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
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rovse",
                    description: Some(
                        "oversampler enable on scope group regular",
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
                    name: "jovse",
                    description: Some(
                        "oversampler enable on scope group injected",
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
                    name: "ovss",
                    description: Some(
                        "oversampling shift",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trovs",
                    description: Some(
                        "oversampling discontinuous mode (triggered mode) for group regular",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Trovs",
                    ),
                },
                Field {
                    name: "rovsm",
                    description: Some(
                        "Regular Oversampling mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rovsm",
                    ),
                },
                Field {
                    name: "rshift1",
                    description: Some(
                        "Right-shift data after Offset 1 correction",
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
                    name: "rshift2",
                    description: Some(
                        "Right-shift data after Offset 2 correction",
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
                    name: "rshift3",
                    description: Some(
                        "Right-shift data after Offset 3 correction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rshift4",
                    description: Some(
                        "Right-shift data after Offset 4 correction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osvr",
                    description: Some(
                        "Oversampling ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lshift",
                    description: Some(
                        "Left shift factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aden",
                    description: Some(
                        "enable",
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
                    name: "addis",
                    description: Some(
                        "disable",
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
                    name: "adstart",
                    description: Some(
                        "group regular conversion start",
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
                    name: "jadstart",
                    description: Some(
                        "group injected conversion start",
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
                    name: "adstp",
                    description: Some(
                        "group regular conversion stop",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adstp",
                    ),
                },
                Field {
                    name: "jadstp",
                    description: Some(
                        "group injected conversion stop",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adstp",
                    ),
                },
                Field {
                    name: "boost",
                    description: Some(
                        "Boost mode control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Boost",
                    ),
                },
                Field {
                    name: "adcallin",
                    description: Some(
                        "Linearity calibration",
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
                    name: "lincalrdyw1",
                    description: Some(
                        "Linearity calibration ready Word 1",
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
                    name: "lincalrdyw2",
                    description: Some(
                        "Linearity calibration ready Word 2",
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
                    name: "lincalrdyw3",
                    description: Some(
                        "Linearity calibration ready Word 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lincalrdyw4",
                    description: Some(
                        "Linearity calibration ready Word 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lincalrdyw5",
                    description: Some(
                        "Linearity calibration ready Word 5",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lincalrdyw6",
                    description: Some(
                        "Linearity calibration ready Word 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "advregen",
                    description: Some(
                        "voltage regulator enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "deeppwd",
                    description: Some(
                        "deep power down enable",
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
                    name: "adcaldif",
                    description: Some(
                        "differential mode for calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adcaldif",
                    ),
                },
                Field {
                    name: "adcal",
                    description: Some(
                        "calibration",
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
            name: "Difsel",
            extends: None,
            description: Some(
                "channel differential or single-ended mode selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "difsel",
                    description: Some(
                        "channel differential or single-ended mode for channel",
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
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Difsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "group regular conversion data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "group regular conversion data",
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
            name: "Htr1",
            extends: None,
            description: Some(
                "analog watchdog 2 threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr1",
                    description: Some(
                        "analog watchdog 2 threshold low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr2",
            extends: None,
            description: Some(
                "watchdog higher threshold register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr2",
                    description: Some(
                        "Analog watchdog 2 higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr3",
            extends: None,
            description: Some(
                "watchdog higher threshold register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr3",
                    description: Some(
                        "Analog watchdog 3 higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdyie",
                    description: Some(
                        "ready interrupt",
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
                    name: "eosmpie",
                    description: Some(
                        "group regular end of sampling interrupt",
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
                    name: "eocie",
                    description: Some(
                        "group regular end of unitary conversion interrupt",
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
                    name: "eosie",
                    description: Some(
                        "group regular end of sequence conversions interrupt",
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
                    name: "ovrie",
                    description: Some(
                        "group regular overrun interrupt",
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
                    name: "jeocie",
                    description: Some(
                        "group injected end of unitary conversion interrupt",
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
                    name: "jeosie",
                    description: Some(
                        "group injected end of sequence conversions interrupt",
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
                    name: "awd1ie",
                    description: Some(
                        "analog watchdog 1 interrupt",
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
                    name: "awd2ie",
                    description: Some(
                        "analog watchdog 2 interrupt",
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
                    name: "awd3ie",
                    description: Some(
                        "analog watchdog 3 interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jqovfie",
                    description: Some(
                        "group injected contexts queue overflow interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "interrupt and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy",
                    description: Some(
                        "ready flag",
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
                    name: "eosmp",
                    description: Some(
                        "group regular end of sampling flag",
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
                    name: "eoc",
                    description: Some(
                        "group regular end of unitary conversion flag",
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
                    name: "eos",
                    description: Some(
                        "group regular end of sequence conversions flag",
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
                    name: "ovr",
                    description: Some(
                        "group regular overrun flag",
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
                    name: "jeoc",
                    description: Some(
                        "group injected end of unitary conversion flag",
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
                    name: "jeos",
                    description: Some(
                        "group injected end of sequence conversions flag",
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
                    name: "awd1",
                    description: Some(
                        "analog watchdog 1 flag",
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
                    name: "awd2",
                    description: Some(
                        "analog watchdog 2 flag",
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
                    name: "awd3",
                    description: Some(
                        "analog watchdog 3 flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jqovf",
                    description: Some(
                        "group injected contexts queue overflow flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ldordy",
                    description: Some(
                        "ADC LDO output voltage ready (not always available)",
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
            ],
        },
        FieldSet {
            name: "Jdr",
            extends: None,
            description: Some(
                "group injected sequencer rank 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "group injected sequencer rank 1 conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jsqr",
            extends: None,
            description: Some(
                "group injected sequencer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jl",
                    description: Some(
                        "group injected sequencer scan length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jextsel",
                    description: Some(
                        "group injected external trigger source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jexten",
                    description: Some(
                        "group injected external trigger polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Jexten",
                    ),
                },
                Field {
                    name: "jsq1",
                    description: Some(
                        "group injected sequencer rank 1-4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr1",
            extends: None,
            description: Some(
                "analog watchdog 1 threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr1",
                    description: Some(
                        "analog watchdog 1 threshold low",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr2",
            extends: None,
            description: Some(
                "watchdog lower threshold register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr2",
                    description: Some(
                        "Analog watchdog 2 lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr3",
            extends: None,
            description: Some(
                "watchdog lower threshold register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr3",
                    description: Some(
                        "Analog watchdog 3 lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofr",
            extends: None,
            description: Some(
                "offset number x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset1",
                    description: Some(
                        "offset number x offset level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset1_ch",
                    description: Some(
                        "offset number x channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssate",
                    description: Some(
                        "Signed saturation enable",
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
            name: "Pcsel",
            extends: None,
            description: Some(
                "channel preselection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcsel",
                    description: Some(
                        "Channel x (VINP[i]) pre selection",
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
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr",
            extends: None,
            description: Some(
                "sampling time register n",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "channel n * 10 + x sampling time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sqr1",
            extends: None,
            description: Some(
                "group regular sequencer ranks register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l",
                    description: Some(
                        "L3",
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
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 1-4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr2",
            extends: None,
            description: Some(
                "group regular sequencer ranks register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 5-9",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr3",
            extends: None,
            description: Some(
                "group regular sequencer ranks register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 10-14",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr4",
            extends: None,
            description: Some(
                "group regular sequencer ranks register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 15-16",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 6,
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
            name: "Adcaldif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLEENDED",
                    description: Some(
                        "Calibration for single-ended mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIFFERENTIAL",
                    description: Some(
                        "Calibration for differential mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adstp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP",
                    description: Some(
                        "Stop conversion of channel",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Awd1sgl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL",
                    description: Some(
                        "Analog watchdog 1 enabled on all channels",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Analog watchdog 1 enabled on single channel selected in AWD1CH",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Boost",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LT6_25",
                    description: Some(
                        "Boost mode used when clock ≤ 6.25 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LT12_5",
                    description: Some(
                        "Boost mode used when 6.25 MHz < clock ≤ 12.5 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LT25",
                    description: Some(
                        "Boost mode used when 12.5 MHz < clock ≤ 25.0 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LT50",
                    description: Some(
                        "Boost mode used when 25.0 MHz < clock ≤ 50.0 MHz",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Difsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLEENDED",
                    description: Some(
                        "Input channel is configured in single-ended mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIFFERENTIAL",
                    description: Some(
                        "Input channel is configured in differential mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmngt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DR",
                    description: Some(
                        "Store output data in DR only",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DMA_ONESHOT",
                    description: Some(
                        "DMA One Shot Mode selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DFSDM",
                    description: Some(
                        "DFSDM mode selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DMA_CIRCULAR",
                    description: Some(
                        "DMA Circular Mode selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Exten",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Trigger detection disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISINGEDGE",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTHEDGES",
                    description: Some(
                        "Trigger detection on both the rising and falling edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Jexten",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Trigger detection disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISINGEDGE",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTHEDGES",
                    description: Some(
                        "Trigger detection on both the rising and falling edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Jqm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MODE0",
                    description: Some(
                        "JSQR Mode 0: Queue maintains the last written configuration into JSQR",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MODE1",
                    description: Some(
                        "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ovrmod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PRESERVE",
                    description: Some(
                        "Preserve DR register when an overrun is detected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERWRITE",
                    description: Some(
                        "Overwrite DR register when an overrun is detected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTPRESELECTED",
                    description: Some(
                        "Input channel x is not pre-selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRESELECTED",
                    description: Some(
                        "Pre-select input channel x",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Res",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "16-bit resolution",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS14",
                    description: Some(
                        "14-bit resolution in legacy mode (not optimized power consumption)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12-bit resolution in legacy mode (not optimized power consumption)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit resolution",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BITS14V",
                    description: Some(
                        "14-bit resolution",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BITS12V",
                    description: Some(
                        "12-bit resolution",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit resolution",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rovsm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONTINUED",
                    description: Some(
                        "Oversampling is temporary stopped and continued after injection sequence",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RESUMED",
                    description: Some(
                        "Oversampling is aborted and resumed from start after injection sequence",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES1_5",
                    description: Some(
                        "1.5 clock cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES2_5",
                    description: Some(
                        "2.5 clock cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES8_5",
                    description: Some(
                        "8.5 clock cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES16_5",
                    description: Some(
                        "16.5 clock cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES32_5",
                    description: Some(
                        "32.5 clock cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES64_5",
                    description: Some(
                        "64.5 clock cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES387_5",
                    description: Some(
                        "387.5 clock cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES810_5",
                    description: Some(
                        "810.5 clock cycles",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Trovs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AUTOMATIC",
                    description: Some(
                        "All oversampled conversions for a channel are run following a trigger",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRIGGERED",
                    description: Some(
                        "Each oversampled conversion for a channel needs a new trigger",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                