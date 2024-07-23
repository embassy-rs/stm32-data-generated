
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog-to-digital converter",
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
                    name: "cfgr1",
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
                                "Cfgr1",
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
                        "sampling time register",
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
                    name: "tr",
                    description: Some(
                        "watchdog threshold register",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "chselr",
                    description: Some(
                        "channel selection register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chselr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "data register",
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
                    name: "ccr",
                    description: Some(
                        "common configuration register",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "common configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vrefen",
                    description: Some(
                        "Temperature sensor and VREFINT enable",
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
                    name: "tsen",
                    description: Some(
                        "Temperature sensor enable",
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
                    name: "vbaten",
                    description: Some(
                        "VBAT enable",
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
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
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
                    enumm: None,
                },
                Field {
                    name: "dmacfg",
                    description: Some(
                        "Direct memory access configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dmacfg",
                    ),
                },
                Field {
                    name: "scandir",
                    description: Some(
                        "Scan sequence direction",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Scandir",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "Data resolution",
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
                    name: "align",
                    description: Some(
                        "Data alignment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Align",
                    ),
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "External trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exten",
                    description: Some(
                        "External trigger enable and polarity selection",
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
                        "Overrun management mode",
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
                    name: "wait",
                    description: Some(
                        "Wait conversion mode",
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
                    name: "autoff",
                    description: Some(
                        "Auto-off mode",
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
                        "Discontinuous mode",
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
                    name: "awdsgl",
                    description: Some(
                        "Enable the watchdog on a single channel or on all channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Awdsgl",
                    ),
                },
                Field {
                    name: "awden",
                    description: Some(
                        "Analog watchdog enable",
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
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel selection",
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
                    name: "ckmode",
                    description: Some(
                        "ADC clock mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckmode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Chselr",
            extends: None,
            description: Some(
                "channel selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chselx",
                    description: Some(
                        "Channel-x selection",
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
                                len: 19,
                                stride: 1,
                            },
                        ),
                    ),
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
                        "ADC enable command",
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
                        "ADC disable command",
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
                        "ADC start conversion command",
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
                    name: "adstp",
                    description: Some(
                        "ADC stop conversion command",
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
                    name: "adcal",
                    description: Some(
                        "ADC calibration",
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
                "data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Converted data",
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
                        "ADC ready interrupt enable",
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
                        "End of sampling flag interrupt enable",
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
                        "End of conversion interrupt enable",
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
                    name: "eoseqie",
                    description: Some(
                        "End of conversion sequence interrupt enable",
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
                        "Overrun interrupt enable",
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
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable",
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
                        "ADC ready",
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
                        "End of sampling flag",
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
                        "End of conversion flag",
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
                    name: "eoseq",
                    description: Some(
                        "End of sequence flag",
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
                        "ADC overrun",
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
                    name: "awd",
                    description: Some(
                        "Analog watchdog flag",
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
                        "Sampling time selection",
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
            ],
        },
        FieldSet {
            name: "Tr",
            extends: None,
            description: Some(
                "watchdog threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt",
                    description: Some(
                        "Analog watchdog lower threshold",
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
                    name: "ht",
                    description: Some(
                        "Analog watchdog higher threshold",
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
    ],
    enums: &[
        Enum {
            name: "Align",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RIGHT",
                    description: Some(
                        "Right alignment",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEFT",
                    description: Some(
                        "Left alignment",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Awdsgl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALLCHANNELS",
                    description: Some(
                        "Analog watchdog enabled on all channels",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SINGLECHANNEL",
                    description: Some(
                        "Analog watchdog enabled on a single channel",
                    ),
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
                    name: "ADCCLK",
                    description: Some(
                        "Asynchronous clock mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PCLK_DIV2",
                    description: Some(
                        "Synchronous clock mode (PCLK/2)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PCLK_DIV4",
                    description: Some(
                        "Sychronous clock mode (PCLK/4)",
                    ),
                    value: 2,
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
            name: "Ovrmod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PRESERVED",
                    description: Some(
                        "ADC_DR register is preserved with the old data when an overrun is detected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERWRITTEN",
                    description: Some(
                        "ADC_DR register is overwritten with the last conversion result when an overrun is detected",
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
                        "12-bit (14 ADCCLK cycles)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit (13 ADCCLK cycles)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit (11 ADCCLK cycles)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some(
                        "6-bit (9 ADCCLK cycles)",
                    ),
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
                    name: "CYCLES1_5",
                    description: Some(
                        "1.5 cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES7_5",
                    description: Some(
                        "7.5 cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES13_5",
                    description: Some(
                        "13.5 cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES28_5",
                    description: Some(
                        "28.5 cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES41_5",
                    description: Some(
                        "41.5 cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES55_5",
                    description: Some(
                        "55.5 cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES71_5",
                    description: Some(
                        "71.5 cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES239_5",
                    description: Some(
                        "239.5 cycles",
                    ),
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
                    name: "UPWARD",
                    description: Some(
                        "Upward scan (from CHSEL0 to CHSEL18)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BACKWARD",
                    description: Some(
                        "Backward scan (from CHSEL18 to CHSEL0)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                