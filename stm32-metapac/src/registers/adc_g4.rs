
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
                        "sampling time register 1",
                    ),
                    array: None,
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
                    name: "smpr2",
                    description: Some(
                        "sampling time register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tr1",
                    description: Some(
                        "analog watchdog threshold register 1",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tr2",
                    description: Some(
                        "analog watchdog threshold register 2",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tr3",
                    description: Some(
                        "analog watchdog threshold register 3",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tr3",
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
                    name: "difsel",
                    description: Some(
                        "channel differential or single-ended mode selection register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
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
                    byte_offset: 0xb4,
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
                    name: "gcomp",
                    description: Some(
                        "Gain compensation register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcomp",
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
                        "analog watchdog 2 channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
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
                        "analog watchdog 3 channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    name: "dmacfg",
                    description: Some(
                        "Direct memory access configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dmacfg",
                    ),
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "Direct memory access enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dmaen",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "data resolution",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Res",
                    ),
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "external trigger selection for regular group",
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
                        "external trigger enable and polarity selection for regular channels",
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
                        "overrun mode",
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
                        "delayed conversion mode",
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
                    name: "align",
                    description: Some(
                        "data alignment",
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
                    name: "discen",
                    description: Some(
                        "discontinuous mode for regular channels",
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
                        "discontinuous mode channel count",
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
                        "discontinuous mode on injected channels",
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
                        "JSQR queue mode",
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
                        "enable the watchdog 1 on a single channel or on all channels",
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
                        "analog watchdog 1 enable on regular channels",
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
                        "analog watchdog 1 enable on injected channels",
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
                        "automatic injected group conversion",
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
                        "analog watchdog 1 channel selection",
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
                        "injected queue disable",
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
                        "Regular Oversampling Enable",
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
                        "Injected Oversampling Enable",
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
                    name: "ovsr",
                    description: Some(
                        "Oversampling ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovss",
                    description: Some(
                        "Oversampling shift",
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
                        "Triggered Regular Oversampling",
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
                    name: "gcomp",
                    description: Some(
                        "Gain compensation mode",
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
                    name: "swtrig",
                    description: Some(
                        "Software trigger bit for sampling time control trigger mode",
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
                    name: "bulb",
                    description: Some(
                        "Bulb sampling mode",
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
                    name: "smptrig",
                    description: Some(
                        "Sampling time control trigger mode",
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
                                len: 18,
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
            name: "Gcomp",
            extends: None,
            description: Some(
                "Gain compensation coefficient",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gcompcoeff",
                    description: Some(
                        "Gain compensation coefficient",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
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
            ],
        },
        FieldSet {
            name: "Jdr",
            extends: None,
            description: Some(
                "group injected sequencer rank 1-4 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "group injected sequencer rank conversion data",
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
                    name: "jsq",
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
            name: "Ofr",
            extends: None,
            description: Some(
                "offset number x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "data offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offsetpos",
                    description: Some(
                        "Positive offset",
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
                    name: "saten",
                    description: Some(
                        "Saturation enable",
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
                    name: "offset1_ch",
                    description: Some(
                        "Channel selection for the data offset",
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
                    name: "offset_en",
                    description: Some(
                        "Offset enable",
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
            name: "Smpr",
            extends: None,
            description: Some(
                "sampling time register",
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
                Field {
                    name: "smpplus",
                    description: Some(
                        "Addition of one clock cycle to the sampling time",
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
            name: "Smpr2",
            extends: None,
            description: Some(
                "sampling time register",
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
                                len: 9,
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
                        "L",
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
        FieldSet {
            name: "Tr1",
            extends: None,
            description: Some(
                "analog watchdog threshold register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt1",
                    description: Some(
                        "analog watchdog 1 lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdfilt",
                    description: Some(
                        "analog watchdog filtering parameter",
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
                    name: "ht1",
                    description: Some(
                        "analog watchdog 1 higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tr2",
            extends: None,
            description: Some(
                "analog watchdog threshold register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt2",
                    description: Some(
                        "analog watchdog 2 lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht2",
                    description: Some(
                        "analog watchdog 2 higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tr3",
            extends: None,
            description: Some(
                "analog watchdog threshold register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt3",
                    description: Some(
                        "analog watchdog 3 lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht3",
                    description: Some(
                        "analog watchdog 3 higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
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
            name: "Dmacfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONESHOT",
                    description: Some(
                        "DMA One Shot mode selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CIRCULAR",
                    description: Some(
                        "DMA Circular mode selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmaen",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "DMA disable",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLE",
                    description: Some(
                        "DMA enable",
                    ),
                    value: 1,
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
            name: "Res",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12-bit resolution",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit resolution",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some(
                        "6-bit resolution",
                    ),
                    value: 3,
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
                    name: "CYCLES2_5",
                    description: Some(
                        "2.5 clock cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES6_5",
                    description: Some(
                        "6.5 clock cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES12_5",
                    description: Some(
                        "12.5 clock cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES24_5",
                    description: Some(
                        "24.5 clock cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES47_5",
                    description: Some(
                        "47.5 clock cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES92_5",
                    description: Some(
                        "92.5 clock cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES247_5",
                    description: Some(
                        "247.5 clock cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES640_5",
                    description: Some(
                        "640.5 clock cycles",
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
                        "All oversampled conversions for a channel are done consecutively following a trigger",
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
                