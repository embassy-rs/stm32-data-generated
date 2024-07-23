
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog-to-Digital Converter",
            ),
            items: &[
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
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
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    byte_offset: 0x8,
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
                    name: "smpr1",
                    description: Some(
                        "sample time register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr2",
                    description: Some(
                        "sample time register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "jofr1",
                    description: Some(
                        "injected channel data offset register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr2",
                    description: Some(
                        "injected channel data offset register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr3",
                    description: Some(
                        "injected channel data offset register 3",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr4",
                    description: Some(
                        "injected channel data offset register 4",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr",
                    description: Some(
                        "watchdog higher threshold register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr",
                    description: Some(
                        "watchdog lower threshold register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr1",
                    description: Some(
                        "regular sequence register 1",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                        "regular sequence register 2",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                        "regular sequence register 3",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                    name: "jsqr",
                    description: Some(
                        "injected sequence register",
                    ),
                    array: None,
                    byte_offset: 0x38,
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
                    name: "jdr1",
                    description: Some(
                        "injected data register 1",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr2",
                    description: Some(
                        "injected data register 2",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr3",
                    description: Some(
                        "injected data register 3",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr4",
                    description: Some(
                        "injected data register 4",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "regular data register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "analog watchdog channel select bits",
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
                    name: "eocie",
                    description: Some(
                        "interrupt enable for EOC",
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
                    name: "awdie",
                    description: Some(
                        "analog watchdog interrupt enable",
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
                    name: "jeocie",
                    description: Some(
                        "interrupt enable for injected channels",
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
                    name: "scan",
                    description: Some(
                        "scan mode",
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
                    name: "awdsgl",
                    description: Some(
                        "enable the watchdog on a single channel in scan mode",
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
                    name: "jauto",
                    description: Some(
                        "automatic injected group conversion",
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
                    name: "discen",
                    description: Some(
                        "discontinuous mode on regular channels",
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
                    name: "jdiscen",
                    description: Some(
                        "discontinuous mode on injected channels",
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
                    name: "discnum",
                    description: Some(
                        "discontinuous mode channel count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Discnum",
                    ),
                },
                Field {
                    name: "jawden",
                    description: Some(
                        "analog watchdog enable on injected channels",
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
                    name: "awden",
                    description: Some(
                        "analog watchdog enable on regular channels",
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
                    name: "adon",
                    description: Some(
                        "A/D converter ON / OFF",
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
                    name: "cont",
                    description: Some(
                        "Continuous conversion",
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
                    name: "cal",
                    description: Some(
                        "A/D calibration",
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
                    name: "rstcal",
                    description: Some(
                        "reset calibration",
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
                    name: "dma",
                    description: Some(
                        "DMA disable selection (for single ADC mode)",
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
                    name: "align",
                    description: Some(
                        "data alignment",
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
                    name: "jextsel",
                    description: Some(
                        "external event select for injected group",
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
                    name: "jexttrig",
                    description: Some(
                        "external trigger conversion mode for injected channels",
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
                    name: "extsel",
                    description: Some(
                        "external event select for regular group",
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
                    name: "exttrig",
                    description: Some(
                        "external trigger conversion mode for regular channels",
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
                    name: "jswstart",
                    description: Some(
                        "start conversion of injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swstart",
                    description: Some(
                        "start conversion of regular channels",
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
                    name: "tsvrefe",
                    description: Some(
                        "temperature sensor and VREFINT enable",
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
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "regular data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Regular data",
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
            name: "Htr",
            extends: None,
            description: Some(
                "watchdog higher threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ht",
                    description: Some(
                        "Analog watchdog high threshold",
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
            ],
        },
        FieldSet {
            name: "Jdr1",
            extends: None,
            description: Some(
                "injected data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata1",
                    description: Some(
                        "Injected data",
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
            name: "Jdr2",
            extends: None,
            description: Some(
                "injected data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata2",
                    description: Some(
                        "Injected data",
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
            name: "Jdr3",
            extends: None,
            description: Some(
                "injected data register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata3",
                    description: Some(
                        "Injected data",
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
            name: "Jdr4",
            extends: None,
            description: Some(
                "injected data register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata4",
                    description: Some(
                        "Injected data",
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
            name: "Jofr1",
            extends: None,
            description: Some(
                "injected channel data offset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset1",
                    description: Some(
                        "data offset for injected channel 1",
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
            ],
        },
        FieldSet {
            name: "Jofr2",
            extends: None,
            description: Some(
                "injected channel data offset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset2",
                    description: Some(
                        "data offset for injected channel 2",
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
            ],
        },
        FieldSet {
            name: "Jofr3",
            extends: None,
            description: Some(
                "injected channel data offset register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset3",
                    description: Some(
                        "data offset for injected channel 3",
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
            ],
        },
        FieldSet {
            name: "Jofr4",
            extends: None,
            description: Some(
                "injected channel data offset register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset4",
                    description: Some(
                        "data offset for injected channel 4",
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
            ],
        },
        FieldSet {
            name: "Jsqr",
            extends: None,
            description: Some(
                "injected sequence register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jsq1",
                    description: Some(
                        "1st conversion in injected sequence",
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
                    name: "jsq2",
                    description: Some(
                        "2nd conversion in injected sequence",
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
                    name: "jsq3",
                    description: Some(
                        "3rd conversion in injected sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jsq4",
                    description: Some(
                        "4th conversion in injected sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jl",
                    description: Some(
                        "injected sequence length",
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
            ],
        },
        FieldSet {
            name: "Ltr",
            extends: None,
            description: Some(
                "watchdog lower threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt",
                    description: Some(
                        "Analog watchdog low threshold",
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
            ],
        },
        FieldSet {
            name: "Smpr1",
            extends: None,
            description: Some(
                "sample time register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp10",
                    description: Some(
                        "channel 10 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp11",
                    description: Some(
                        "channel 11 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp12",
                    description: Some(
                        "channel 12 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp13",
                    description: Some(
                        "channel 13 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp14",
                    description: Some(
                        "channel 14 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp15",
                    description: Some(
                        "channel 15 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp16",
                    description: Some(
                        "channel 16 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp17",
                    description: Some(
                        "channel 17 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp18",
                    description: Some(
                        "channel 18 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr2",
            extends: None,
            description: Some(
                "sample time register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp0",
                    description: Some(
                        "channel 0 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp1",
                    description: Some(
                        "channel 1 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp2",
                    description: Some(
                        "channel 2 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp3",
                    description: Some(
                        "channel 3 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp4",
                    description: Some(
                        "channel 4 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp5",
                    description: Some(
                        "channel 5 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp6",
                    description: Some(
                        "channel 6 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp7",
                    description: Some(
                        "channel 7 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp8",
                    description: Some(
                        "channel 8 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "SampleTime",
                    ),
                },
                Field {
                    name: "smp9",
                    description: Some(
                        "channel 9 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 3,
                    array: None,
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
                "regular sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq13",
                    description: Some(
                        "13th conversion in regular sequence",
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
                    name: "sq14",
                    description: Some(
                        "14th conversion in regular sequence",
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
                    name: "sq15",
                    description: Some(
                        "15th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq16",
                    description: Some(
                        "16th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "l",
                    description: Some(
                        "regular channel sequence length",
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
            name: "Sqr2",
            extends: None,
            description: Some(
                "regular sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq7",
                    description: Some(
                        "7th conversion in regular sequence",
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
                    name: "sq8",
                    description: Some(
                        "8th conversion in regular sequence",
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
                    name: "sq9",
                    description: Some(
                        "9th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq10",
                    description: Some(
                        "10th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq11",
                    description: Some(
                        "11th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq12",
                    description: Some(
                        "12th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr3",
            extends: None,
            description: Some(
                "regular sequence register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq1",
                    description: Some(
                        "1st conversion in regular sequence",
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
                    name: "sq2",
                    description: Some(
                        "2nd conversion in regular sequence",
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
                    name: "sq3",
                    description: Some(
                        "3rd conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq4",
                    description: Some(
                        "4th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq5",
                    description: Some(
                        "5th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq6",
                    description: Some(
                        "6th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd",
                    description: Some(
                        "analog watchdog flag",
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
                    name: "eoc",
                    description: Some(
                        "end of conversion",
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
                    name: "jeoc",
                    description: Some(
                        "injected channel end of conversion",
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
                    name: "jstrt",
                    description: Some(
                        "injected channel start flag",
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
                    name: "strt",
                    description: Some(
                        "regular channel start flag",
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
                    name: "ovr",
                    description: Some(
                        "overrun",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Discnum",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISCNUM_1",
                    description: Some(
                        "1 conversions are discontinued and the conversion is carried out on one channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISCNUM_2",
                    description: Some(
                        "2 conversion is discontinued and the conversions are carried out on 2 channels",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DISCNUM_3",
                    description: Some(
                        "3 conversions are discontinued and the conversions are carried out on 3 channels",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DISCNUM_4",
                    description: Some(
                        "4 conversions are discontinued and the conversions are carried out on 4 channels",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DISCNUM_5",
                    description: Some(
                        "5 conversions are discontinued and the conversions are carried out on 5 channels",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DISCNUM_6",
                    description: Some(
                        "6 conversions are discontinued and the conversions are carried out on 6 channels",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DISCNUM_7",
                    description: Some(
                        "7 conversions are discontinued and the conversions are carried out on 7 channels",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DISCNUM_8",
                    description: Some(
                        "8 conversions are discontinued and the conversions are carried out on 8 channels",
                    ),
                    value: 7,
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
                        "1.5 ADC clock cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES7_5",
                    description: Some(
                        "7.5 ADC clock cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES13_5",
                    description: Some(
                        "13.5 ADC clock cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES28_5",
                    description: Some(
                        "28.5 ADC clock cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES41_5",
                    description: Some(
                        "41.5 ADC clock cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES55_5",
                    description: Some(
                        "55.5 ADC clock cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES71_5",
                    description: Some(
                        "71.5 ADC clock cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES239_5",
                    description: Some(
                        "239.5 ADC clock cycles",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
                