
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ch",
            extends: None,
            description: Some("Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some("DFSDM channel y configuration register."),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cfgr1"),
                    }),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some("DFSDM channel y configuration register."),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cfgr2"),
                    }),
                },
                BlockItem {
                    name: "awscdr",
                    description: Some("DFSDM channel y analog watchdog and short-circuit detector register."),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awscdr"),
                    }),
                },
                BlockItem {
                    name: "wdatr",
                    description: Some("DFSDM channel y watchdog filter data register."),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Wdatr"),
                    }),
                },
                BlockItem {
                    name: "datinr",
                    description: Some("DFSDM channel y data input register."),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Datinr"),
                    }),
                },
            ],
        },
        Block {
            name: "ChDly",
            extends: Some("CH"),
            description: Some("Basic timers"),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some("DFSDM channel y configuration register."),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cfgr1"),
                    }),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some("DFSDM channel y configuration register."),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cfgr2"),
                    }),
                },
                BlockItem {
                    name: "awscdr",
                    description: Some("DFSDM channel y analog watchdog and short-circuit detector register."),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awscdr"),
                    }),
                },
                BlockItem {
                    name: "wdatr",
                    description: Some("DFSDM channel y watchdog filter data register."),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Wdatr"),
                    }),
                },
                BlockItem {
                    name: "datinr",
                    description: Some("DFSDM channel y data input register."),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Datinr"),
                    }),
                },
                BlockItem {
                    name: "dlyr",
                    description: None,
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Dlyr"),
                    }),
                },
            ],
        },
        Block {
            name: "Dfsdm2ch1fltTrg5",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 1, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm4ch2fltDlyTrg3",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm4ch2fltDlyTrg5Adc",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm4ch2fltDlyTrg5AdcHwid",
            extends: Some("DFSDM_4CH_2FLT_DLY_TRG5_ADC"),
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
                BlockItem {
                    name: "hwid",
                    description: Some("Cluster HWID, containing version registers"),
                    array: None,
                    byte_offset: 0x7f0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Hwid" }),
                },
            ],
        },
        Block {
            name: "Dfsdm4ch2fltTrg3",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 2, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm4ch4fltDlyTrg5Adc",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltDlyTrg3",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltDlyTrg5Adc",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltTrg3",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltTrg3Adc",
            extends: Some("DFSDM_8CH_4FLT_TRG3"),
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltTrg5",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch4fltTrg5Adc",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Ch" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch6fltDlyTrg5AdcHwid",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
                BlockItem {
                    name: "hwid",
                    description: Some("Cluster HWID, containing version registers"),
                    array: None,
                    byte_offset: 0x7f0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Hwid" }),
                },
            ],
        },
        Block {
            name: "Dfsdm8ch8fltDlyTrg5Adc",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
            ],
        },
        Block {
            name: "DfsdmSuperset",
            extends: None,
            description: Some("Digital filter for sigma delta modulators."),
            items: &[
                BlockItem {
                    name: "ch",
                    description: Some(
                        "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 8, stride: 32 })),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "ChDly" }),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
                    ),
                    array: Some(Array::Regular(RegularArray { len: 6, stride: 128 })),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Flt" }),
                },
                BlockItem {
                    name: "hwid",
                    description: Some("Cluster HWID, containing version registers"),
                    array: None,
                    byte_offset: 0x7f0,
                    inner: BlockItemInner::Block(BlockItemBlock { block: "Hwid" }),
                },
            ],
        },
        Block {
            name: "Flt",
            extends: None,
            description: Some(
                "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR.",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: None,
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cr1"),
                    }),
                },
                BlockItem {
                    name: "cr2",
                    description: None,
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cr2"),
                    }),
                },
                BlockItem {
                    name: "isr",
                    description: None,
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Isr"),
                    }),
                },
                BlockItem {
                    name: "icr",
                    description: None,
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Icr"),
                    }),
                },
                BlockItem {
                    name: "jchgr",
                    description: None,
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Jchgr"),
                    }),
                },
                BlockItem {
                    name: "fcr",
                    description: None,
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Fcr"),
                    }),
                },
                BlockItem {
                    name: "jdatar",
                    description: None,
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Jdatar"),
                    }),
                },
                BlockItem {
                    name: "rdatar",
                    description: None,
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Rdatar"),
                    }),
                },
                BlockItem {
                    name: "awhtr",
                    description: None,
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awhtr"),
                    }),
                },
                BlockItem {
                    name: "awltr",
                    description: None,
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awltr"),
                    }),
                },
                BlockItem {
                    name: "awsr",
                    description: None,
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awsr"),
                    }),
                },
                BlockItem {
                    name: "awcfr",
                    description: None,
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Awcfr"),
                    }),
                },
                BlockItem {
                    name: "exmax",
                    description: None,
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Exmax"),
                    }),
                },
                BlockItem {
                    name: "exmin",
                    description: None,
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Exmin"),
                    }),
                },
                BlockItem {
                    name: "cnvtimr",
                    description: None,
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(Register {
                        access: Access::ReadWrite,
                        bit_size: 32,
                        fieldset: Some("Cnvtimr"),
                    }),
                },
            ],
        },
        Block {
            name: "Hwid",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "hwcfgr",
                    description: Some("This register specifies the hardware configuration of DFSDM peripheral."),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Hwcfgr"),
                    }),
                },
                BlockItem {
                    name: "verr",
                    description: Some("This register specifies the version of DFSDM peripheral."),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Verr"),
                    }),
                },
                BlockItem {
                    name: "ipidr",
                    description: Some("This register specifies the identification of DFSDM peripheral."),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Ipidr"),
                    }),
                },
                BlockItem {
                    name: "sidr",
                    description: Some("This register specifies the size allocated to DFSDM registers."),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(Register {
                        access: Access::Read,
                        bit_size: 32,
                        fieldset: Some("Sidr"),
                    }),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Awcfr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrawltf",
                    description: Some(
                        "Clear the analog watchdog low threshold flag CLRAWLTF[y]=0: Writing '0' has no effect CLRAWLTF[y]=1: Writing '1' to position y clears the corresponding AWLTF[y] bit in the DFSDM_FLTxAWSR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrawhtf",
                    description: Some(
                        "Clear the analog watchdog high threshold flag CLRAWHTF[y]=0: Writing '0' has no effect CLRAWHTF[y]=1: Writing '1' to position y clears the corresponding AWHTF[y] bit in the DFSDM_FLTxAWSR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awhtr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawh",
                    description: Some(
                        "Break signal assignment to analog watchdog high threshold event BKAWH[i] = 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH[i] = 1: Break i signal is assigned to an analog watchdog high threshold event.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awht",
                    description: Some(
                        "Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT[23:8]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT[7:0] are not taken into comparison in this case.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awltr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "bkawl",
                    description: Some(
                        "Break signal assignment to analog watchdog low threshold event BKAWL[i] = 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL[i] = 1: Break i signal is assigned to an analog watchdog low threshold event.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awlt",
                    description: Some(
                        "Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT[23:8]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT[7:0] are not taken into comparison in this case.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awscdr",
            extends: None,
            description: Some("DFSDM channel y analog watchdog and short-circuit detector register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scdt",
                    description: Some(
                        "Short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkscd",
                    description: Some(
                        "Break signal assignment for short-circuit detector on channel y BKSCD[i] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD[i] = 1: Break i signal assigned to short-circuit detector on channel y.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awfosr",
                    description: Some(
                        "Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awford",
                    description: Some(
                        "Analog watchdog Sinc filter order on channel y. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awsr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "awltf",
                    description: Some(
                        "Analog watchdog low threshold flag AWLTF[y]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF[y] bit in the DFSDM_FLTxAWCFR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awhtf",
                    description: Some(
                        "Analog watchdog high threshold flag AWHTF[y]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF[y] bit in the DFSDM_FLTxAWCFR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some("DFSDM channel y configuration register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sitp",
                    description: Some(
                        "Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spicksel",
                    description: Some(
                        "SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scden",
                    description: Some("Short-circuit detector enable on channel y."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckaben",
                    description: Some("Clock absence detector enable on channel y."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chen",
                    description: Some(
                        "Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chinsel",
                    description: Some(
                        "Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datmpx",
                    description: Some(
                        "Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK[1:0] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datpack",
                    description: Some(
                        "Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0[15:0] (assigned to channel y) second sample INDAT1[15:0] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0[15:0] part is read as first sample and then INDAT1[15:0] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0[15:0] (assigned to channel y) second sample INDAT1[15:0] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK[1:0]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutdiv",
                    description: Some(
                        "Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2 - 256.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutsrc",
                    description: Some(
                        "Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdmen",
                    description: Some(
                        "Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some("DFSDM channel y configuration register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrbs",
                    description: Some(
                        "Data right bit-shift for channel y will be performed to have final results. Bit-shift is performed before offset correction. The data shift is rounding the result to nearest integer value. The sign of shifted result is maintained (to have valid 24-bit signed format of result data). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). 0-31: Defines the shift of the data result coming from the integrator - how many bit shifts to the right.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset",
                    description: Some(
                        "24-bit calibration offset for channel y For channel y, OFFSET is applied to the results of each conversion from this channel. This value is set by software.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cnvtimr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[Field {
                name: "cnvcnt",
                description: Some(
                    "28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR[9:0] = 0) is the conversion time measurement stopped and CNVCNT[27:0] = 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = [FOSR * (IOSR-1 + FORD) + FORD] / fCKIN ..... for Sincx filters t = [FOSR * (IOSR-1 + 4) + 2] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = [FOSR * IOSR] / fCKIN in case if FOSR = FOSR[9:0]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 28,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfen",
                    description: Some(
                        "DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jswstart",
                    description: Some(
                        "Start a conversion of the injected group of channels This bit is always read as '0'.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jsync",
                    description: Some(
                        "Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jscan",
                    description: Some(
                        "Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jextsel",
                    description: Some(
                        "Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jexten",
                    description: Some(
                        "Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rswstart",
                    description: Some(
                        "Software start of a conversion on the regular channel This bit is always read as '0'.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 17 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcont",
                    description: Some(
                        "Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 18 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsync",
                    description: Some(
                        "Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 19 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rch",
                    description: Some(
                        "Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = [FOSR * (IOSR-1 + FORD) + FORD] / fCKIN ..... for Sincx filters t = [FOSR * (IOSR-1 + 4) + 2] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = [FOSR * IOSR] / fCKIN in case if FOSR = FOSR[9:0]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awfsel",
                    description: Some("Analog watchdog fast mode select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocie",
                    description: Some(
                        "Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reocie",
                    description: Some(
                        "Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jovrie",
                    description: Some(
                        "Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rovrie",
                    description: Some(
                        "Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scdie",
                    description: Some(
                        "Short-circuit detector interrupt enable Please see the explanation of SCDF[7:0] in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckabie",
                    description: Some(
                        "Clock absence interrupt enable Please see the explanation of CKABF[7:0] in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exch",
                    description: Some(
                        "Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH[y] = 0: Extremes detector does not accept data from channel y EXCH[y] = 1: Extremes detector accepts data from channel y.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH[y] = 0: Analog watchdog is disabled on channel y AWDCH[y] = 1: Analog watchdog is enabled on channel y.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Datinr",
            extends: None,
            description: Some("DFSDM channel y data input register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indat0",
                    description: Some(
                        "Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX[1:0]=1 or DATMPX[1:0]=2. Data can be written by CPU/DMA (if DATMPX[1:0]=2) or directly by internal ADC (if DATMPX[1:0]=1). If DATPACK[1:0]=0 (standard mode) Channel y data sample is stored into INDAT0[15:0]. If DATPACK[1:0]=1 (interleaved mode) First channel y data sample is stored into INDAT0[15:0]. Second channel y data sample is stored into INDAT1[15:0]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK[1:0]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0[15:0]. For odd y channels: INDAT0[15:0] is write protected. See for more details. INDAT0[15:0] is in the16-bit signed format.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "indat1",
                    description: Some(
                        "Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX[1:0]=1 or DATMPX[1:0]=2. Data can be written by CPU/DMA (if DATMPX[1:0]=2) or directly by internal ADC (if DATMPX[1:0]=1). If DATPACK[1:0]=0 (standard mode) INDAT0[15:0] is write protected (not used for input sample). If DATPACK[1:0]=1 (interleaved mode) Second channel y data sample is stored into INDAT1[15:0]. First channel y data sample is stored into INDAT0[15:0]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK[1:0]=2 (dual mode). For even y channels: sample in INDAT1[15:0] is automatically copied into INDAT0[15:0] of channel (y+1). For odd y channels: INDAT1[15:0] is write protected. See for more details. INDAT0[15:1] is in the16-bit signed format.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dlyr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[Field {
                name: "plsskp",
                description: Some(
                    "Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP[5:0] returns current value of pulses which will be skipped. If PLSSKP[5:0]=0 then all required data samples were already skipped. Note: User can update PLSSKP[5:0] also when PLSSKP[5:0] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 6,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Exmax",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "exmaxch",
                    description: Some(
                        "Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX[23:0]. Bits are cleared by reading of this register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exmax",
                    description: Some(
                        "Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX[23:0] bits are reset to value (0x800000) by reading of this register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exmin",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "exminch",
                    description: Some(
                        "Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN[23:0]. Bits are cleared by reading of this register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exmin",
                    description: Some(
                        "Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN[23:0] bits are reset to value (0x7FFFFF) by reading of this register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fcr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "iosr",
                    description: Some(
                        "Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fosr",
                    description: Some(
                        "Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR[9:0] +1). This.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ford",
                    description: Some(
                        "Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 29 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hwcfgr",
            extends: None,
            description: Some("This register specifies the hardware configuration of DFSDM peripheral."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nbt",
                    description: Some(
                        "Number of implemented transceivers. Defines how many transceivers (input channels) are implemented in DFSDM peripheral.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbf",
                    description: Some(
                        "Number of implemented filters. Defines how many filters are implemented in DFSDM peripheral.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrjovrf",
                    description: Some("Clear the injected conversion overrun flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrrovrf",
                    description: Some("Clear the regular conversion overrun flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrckabf",
                    description: Some(
                        "Clear the clock absence flag CLRCKABF[y]=0: Writing '0' has no effect CLRCKABF[y]=1: Writing '1' to position y clears the corresponding CKABF[y] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF[y]. Note: CLRCKABF[7:0] is present only in DFSDM_FLT0ICR register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrscdf",
                    description: Some(
                        "Clear the short-circuit detector flag CLRSCDF[y]=0: Writing '0' has no effect CLRSCDF[y]=1: Writing '1' to position y clears the corresponding SCDF[y] bit in the DFSDM_FLTxISR register Note: CLRSCDF[7:0] is present only in DFSDM_FLT0ICR register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipidr",
            extends: None,
            description: Some("This register specifies the identification of DFSDM peripheral."),
            bit_size: 32,
            fields: &[Field {
                name: "id",
                description: Some(
                    "Peripheral identifier.\n Bits [31:0]: these bits returns the DFSDM identifier ID[31:0] = 0x0011 0031",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "jeocf",
                    description: Some(
                        "End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reocf",
                    description: Some(
                        "End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jovrf",
                    description: Some(
                        "Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rovrf",
                    description: Some(
                        "Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdf",
                    description: Some(
                        "Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF[7:0] and AWLTF[7:0] in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jcip",
                    description: Some(
                        "Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcip",
                    description: Some(
                        "Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckabf",
                    description: Some(
                        "Clock absence flag CKABF[y]=0: Clock signal on channel y is present. CKABF[y]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF[y]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF[y]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF[y] bit in the DFSDM_FLTxICR register. Note: CKABF[7:0] is present only in DFSDM_FLT0ISR register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scdf",
                    description: Some(
                        "Short-circuit detector flag SDCF[y]=0: No short-circuit detector event occurred on channel y SDCF[y]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF[y] bit in the DFSDM_FLTxICR register. SCDF[y] is cleared also by hardware when CHEN[y] = 0 (given channel is disabled). Note: SCDF[7:0] is present only in DFSDM_FLT0ISR register (filter x=0).",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jchgr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[Field {
                name: "jchg",
                description: Some(
                    "Injected channel group selection JCHG[y]=0: channel y is not part of the injected group JCHG[y]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Jdatar",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdatach",
                    description: Some(
                        "Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH[2:0] is updated to indicate which channel was converted. Thus, JDATA[23:0] holds the data that corresponds to the channel indicated by JDATACH[2:0].",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdatar",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdatach",
                    description: Some(
                        "Regular channel most recently converted When each regular conversion finishes, RDATACH[2:0] is updated to indicate which channel was converted (because regular channel selection RCH[2:0] in DFSDM_FLTxCR1 register can be updated during regular conversion). Thus RDATA[23:0] holds the data that corresponds to the channel indicated by RDATACH[2:0].",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpend",
                    description: Some(
                        "Regular channel pending data Regular data in RDATA[23:0] was delayed due to an injected channel trigger during the conversion.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdata",
                    description: Some(
                        "Regular channel conversion data When each regular conversion finishes, its data is stored in this register. The data is valid when REOCF=1. Reading this register clears the corresponding REOCF.",
                    ),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sidr",
            extends: None,
            description: Some("This register specifies the size allocated to DFSDM registers."),
            bit_size: 32,
            fields: &[Field {
                name: "sid",
                description: Some(
                    "Bits [31:8]: fixed code = 0xA3C5DD.\n Bits [7:0]: these bits returns the size of the memory region allocated to DFSDM registers.\n 0x02: 2KB allocated by DFSDM peripheral (fixed value).",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Verr",
            extends: None,
            description: Some("This register specifies the version of DFSDM peripheral."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "minrev",
                    description: Some("Minor revision of the DFSDM peripheral."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "majrev",
                    description: Some("Major revision of the DFSDM peripheral."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdatr",
            extends: None,
            description: Some("DFSDM channel y watchdog filter data register."),
            bit_size: 32,
            fields: &[Field {
                name: "wdata",
                description: Some(
                    "Input channel y watchdog data Data converted by the analog watchdog filter for input channel y. This data is continuously converted (no trigger) for this channel, with a limited resolution (OSR=1..32/sinc order = 1..3).",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
