
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Adc",
        extends: None,
        description: Some("analog to Digital Converter"),
        items: &[
            BlockItem {
                name: "isr",
                description: Some("interrupt and status register"),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Isr"),
                }),
            },
            BlockItem {
                name: "ier",
                description: Some("interrupt enable register"),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ier"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("control register"),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "cfgr1",
                description: Some("configuration register 1"),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgr1"),
                }),
            },
            BlockItem {
                name: "cfgr2",
                description: Some("configuration register 2"),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgr2"),
                }),
            },
            BlockItem {
                name: "smpr",
                description: Some("sampling time register"),
                array: None,
                byte_offset: 0x14,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Smpr"),
                }),
            },
            BlockItem {
                name: "awd1tr",
                description: Some("watchdog threshold register"),
                array: None,
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awd1tr"),
                }),
            },
            BlockItem {
                name: "awd2tr",
                description: Some("watchdog threshold register"),
                array: None,
                byte_offset: 0x24,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awd2tr"),
                }),
            },
            BlockItem {
                name: "chselr",
                description: Some("channel selection register CHSELRMOD = 0 in ADC_CFGR1"),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Chselr"),
                }),
            },
            BlockItem {
                name: "chselr_sq",
                description: Some("channel selection register CHSELRMOD = 1 (seqencer enabled) in ADC_CFGR1"),
                array: None,
                byte_offset: 0x28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("ChselrSq"),
                }),
            },
            BlockItem {
                name: "awd3tr",
                description: Some("watchdog threshold register"),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awd3tr"),
                }),
            },
            BlockItem {
                name: "dr",
                description: Some("data register"),
                array: None,
                byte_offset: 0x40,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Dr"),
                }),
            },
            BlockItem {
                name: "awd2cr",
                description: Some("analog Watchdog 2 Configuration register"),
                array: None,
                byte_offset: 0xa0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awd2cr"),
                }),
            },
            BlockItem {
                name: "awd3cr",
                description: Some("analog Watchdog 3 Configuration register"),
                array: None,
                byte_offset: 0xa4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Awd3cr"),
                }),
            },
            BlockItem {
                name: "calfact",
                description: Some("Calibration factor"),
                array: None,
                byte_offset: 0xb4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Calfact"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Awd1tr",
            extends: None,
            description: Some("watchdog threshold register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt1",
                    description: Some("analog watchdog 1 lower threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht1",
                    description: Some("analog watchdog 1 higher threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd2cr",
            extends: None,
            description: Some("analog Watchdog 2 Configuration register"),
            bit_size: 32,
            fields: &[Field {
                name: "awd2ch",
                description: Some("analog watchdog channel selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 23,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Awd2tr",
            extends: None,
            description: Some("watchdog threshold register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt2",
                    description: Some("analog watchdog 2 lower threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht2",
                    description: Some("analog watchdog 2 higher threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd3cr",
            extends: None,
            description: Some("analog Watchdog 3 Configuration register"),
            bit_size: 32,
            fields: &[Field {
                name: "awd3ch",
                description: Some("analog watchdog channel selection"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 23,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Awd3tr",
            extends: None,
            description: Some("watchdog threshold register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt3",
                    description: Some("analog watchdog 3lower threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ht3",
                    description: Some("analog watchdog 3 higher threshold"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact",
            extends: None,
            description: Some("Calibration factor"),
            bit_size: 32,
            fields: &[Field {
                name: "calfact",
                description: Some("calibration factor in single-ended mode"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 7,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some("configuration register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaen",
                    description: Some("direct memory access enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmacfg",
                    description: Some("direct memory access configuration"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dmacfg"),
                },
                Field {
                    name: "scandir",
                    description: Some("scan sequence direction"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Scandir"),
                },
                Field {
                    name: "res",
                    description: Some("data resolution"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Res"),
                },
                Field {
                    name: "align",
                    description: Some("data alignment"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Align"),
                },
                Field {
                    name: "extsel",
                    description: Some("external trigger selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exten",
                    description: Some("external trigger enable and polarity selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Exten"),
                },
                Field {
                    name: "ovrmod",
                    description: Some("overrun management mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ovrmod"),
                },
                Field {
                    name: "cont",
                    description: Some("single / continuous conversion mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wait",
                    description: Some("wait conversion mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "autoff",
                    description: Some("auto-off mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discen",
                    description: Some("discontinuous mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chselrmod",
                    description: Some("mode selection of the ADC_CHSELR register"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1sgl",
                    description: Some("enable the watchdog on a single channel or on all channels"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd1sgl"),
                },
                Field {
                    name: "awd1en",
                    description: Some("analog watchdog enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ch",
                    description: Some("analog watchdog channel selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some("configuration register 2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovse",
                    description: Some("oversampler enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovsr",
                    description: Some("oversampling ratio"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("Ovsr"),
                },
                Field {
                    name: "ovss",
                    description: Some("oversampling shift"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tovs",
                    description: Some("oversampling discontinuous mode (triggered mode) for ADC group regular"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lftrig",
                    description: Some("low frequency trigger mode enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckmode",
                    description: Some("clock mode"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ckmode"),
                },
            ],
        },
        FieldSet {
            name: "Chselr",
            extends: None,
            description: Some("channel selection register [alternate]"),
            bit_size: 32,
            fields: &[Field {
                name: "chsel",
                description: Some("ADC channel selection for canversion"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 22, stride: 0 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "ChselrSq",
            extends: None,
            description: Some("channel selection register CHSELRMOD = 1 in ADC_CFGR1"),
            bit_size: 32,
            fields: &[Field {
                name: "sq",
                description: Some("Conversion sequence definition"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 4,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 0 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aden",
                    description: Some("ADC enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addis",
                    description: Some("ADC disable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adstart",
                    description: Some("ADC group regular conversion start"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adstp",
                    description: Some("ADC group regular conversion stop"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Adstp"),
                },
                Field {
                    name: "advregen",
                    description: Some("ADC voltage regulator enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adcal",
                    description: Some("ADC calibration"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some("group regular conversion data register"),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("group regular conversion data"),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some("ADC interrupt enable register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdyie",
                    description: Some("ready interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmpie",
                    description: Some("ADC group regular end of sampling interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocie",
                    description: Some("ADC group regular end of unitary conversion interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosie",
                    description: Some("ADC group regular end of sequence conversions interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovrie",
                    description: Some("ADC group regular overrun interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ie",
                    description: Some("ADC analog watchdog 1 interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd2ie",
                    description: Some("ADC analog watchdog 2 interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd3ie",
                    description: Some("ADC analog watchdog 3 interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocalie",
                    description: Some("end of calibration interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccrdyie",
                    description: Some("channel configuration ready interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some("interrupt and status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy",
                    description: Some("ADC ready flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp",
                    description: Some("ADC group regular end of sampling flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some("ADC group regular end of unitary conversion flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eos",
                    description: Some("ADC group regular end of sequence conversions flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some("ADC group regular overrun flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1",
                    description: Some("ADC analog watchdog 1 flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd2",
                    description: Some("ADC analog watchdog 2 flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd3",
                    description: Some("ADC analog watchdog 3 flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocal",
                    description: Some("End Of Calibration flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccrdy",
                    description: Some("Channel Configuration Ready flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smpr",
            extends: None,
            description: Some("sampling time register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp1",
                    description: Some("sampling time selection 1"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("SampleTime"),
                },
                Field {
                    name: "smp2",
                    description: Some("sampling time selection 2"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 3,
                    array: None,
                    enumm: Some("SampleTime"),
                },
                Field {
                    name: "smpsel",
                    description: Some("channel sampling time selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 22, stride: 0 })),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adstp",
            description: None,
            bit_size: 1,
            variants: &[EnumVariant {
                name: "STOP",
                description: Some("Stop conversion of channel"),
                value: 1,
            }],
        },
        Enum {
            name: "Align",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RIGHT",
                    description: Some("Right alignment"),
                    value: 0,
                },
                EnumVariant {
                    name: "LEFT",
                    description: Some("Left alignment"),
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
                    description: Some("Analog watchdog 1 enabled on all channels"),
                    value: 0,
                },
                EnumVariant {
                    name: "SINGLE",
                    description: Some("Analog watchdog 1 enabled on single channel selected in AWD1CH"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ckmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some("SYSCLK or HSIKER clock"),
                    value: 0,
                },
                EnumVariant {
                    name: "PCLK_DIV_2",
                    description: Some("PCLK divided by 2"),
                    value: 1,
                },
                EnumVariant {
                    name: "PCLK_DIV_4",
                    description: Some("PCLK divided by 4"),
                    value: 2,
                },
                EnumVariant {
                    name: "PCLK",
                    description: Some("PCLK"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dmacfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DMA_ONE_SHOT",
                    description: Some("DMA One Shot Mode selected"),
                    value: 0,
                },
                EnumVariant {
                    name: "DMA_CIRCULAR",
                    description: Some("DMA Circular Mode selected"),
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
                    description: Some("Trigger detection disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING_EDGE",
                    description: Some("Trigger detection on the rising edge"),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLING_EDGE",
                    description: Some("Trigger detection on the falling edge"),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTH_EDGES",
                    description: Some("Trigger detection on both the rising and falling edges"),
                    value: 3,
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
                    description: Some("Preserve DR register when an overrun is detected"),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERWRITE",
                    description: Some("Overwrite DR register when an overrun is detected"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ovsr",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RATIO2X",
                    description: Some("2x"),
                    value: 0,
                },
                EnumVariant {
                    name: "RATIO4X",
                    description: Some("4x"),
                    value: 1,
                },
                EnumVariant {
                    name: "RATIO8X",
                    description: Some("8x"),
                    value: 2,
                },
                EnumVariant {
                    name: "RATIO16X",
                    description: Some("16x"),
                    value: 3,
                },
                EnumVariant {
                    name: "RATIO32X",
                    description: Some("32x"),
                    value: 4,
                },
                EnumVariant {
                    name: "RATIO64X",
                    description: Some("64x"),
                    value: 5,
                },
                EnumVariant {
                    name: "RATIO128X",
                    description: Some("128x"),
                    value: 6,
                },
                EnumVariant {
                    name: "RATIO256X",
                    description: Some("256x"),
                    value: 7,
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
                    description: Some("12-bit resolution"),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some("10-bit resolution"),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some("8-bit resolution"),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some("6-bit resolution"),
                    value: 3,
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
                    description: Some("2.5 clock cycles"),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES6_5",
                    description: Some("6.5 clock cycles"),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES12_5",
                    description: Some("12.5 clock cycles"),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES24_5",
                    description: Some("24.5 clock cycles"),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES47_5",
                    description: Some("47.5 clock cycles"),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES92_5",
                    description: Some("92.5 clock cycles"),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES247_5",
                    description: Some("247.5 clock cycles"),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES640_5",
                    description: Some("640.5 clock cycles"),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Scandir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UP",
                    description: Some("Upward scan (from CHSEL0 to CHSEL22)."),
                    value: 0,
                },
                EnumVariant {
                    name: "BACK",
                    description: Some("Backward scan (from CHSEL22 to CHSEL0)."),
                    value: 1,
                },
            ],
        },
    ],
};
