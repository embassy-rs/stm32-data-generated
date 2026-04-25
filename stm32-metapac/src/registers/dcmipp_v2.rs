
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dcmipp",
            extends: None,
            description: Some(
                "Digital camera interface pixel pipeline.",
            ),
            items: &[
                BlockItem {
                    name: "ipgr1",
                    description: Some(
                        "DCMIPP IPPLUG global register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipgr2",
                    description: Some(
                        "DCMIPP IPPLUG global register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipgr3",
                    description: Some(
                        "DCMIPP IPPLUG global register 3.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipgr8",
                    description: Some(
                        "DCMIPP IPPLUG identification register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipgr8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc1r1",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc1r1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc1r2",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 2.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc1r2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc1r3",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 3.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc1r3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc2r1",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 1.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc2r1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc2r2",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 2.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc2r2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc2r3",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 3.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc2r3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc3r1",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 1.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc3r1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc3r2",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 2.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc3r2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc3r3",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 3.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc3r3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc4r1",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 1.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc4r1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc4r2",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 2.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc4r2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc4r3",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 3.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc4r3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc5r1",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 1.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc5r1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc5r2",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 2.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc5r2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipc5r3",
                    description: Some(
                        "DCMIPP IPPLUG Clientx register 3.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipc5r3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prcr",
                    description: Some(
                        "DCMIPP parallel interface control register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prescr",
                    description: Some(
                        "DCMIPP parallel interface embedded synchronization code register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prescr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "presur",
                    description: Some(
                        "DCMIPP parallel interface embedded synchronization unmask register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Presur",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prier",
                    description: Some(
                        "DCMIPP parallel interface interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prsr",
                    description: Some(
                        "DCMIPP parallel interface status register.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prfcr",
                    description: Some(
                        "DCMIPP parallel interface interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmcr",
                    description: Some(
                        "DCMIPP common configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmfrcr",
                    description: Some(
                        "DCMIPP common frame counter register.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmfrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmier",
                    description: Some(
                        "DCMIPP common interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x3f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmsr1",
                    description: Some(
                        "DCMIPP common status register 1.",
                    ),
                    array: None,
                    byte_offset: 0x3f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmsr2",
                    description: Some(
                        "DCMIPP common status register 2.",
                    ),
                    array: None,
                    byte_offset: 0x3f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmfcr",
                    description: Some(
                        "DCMIPP common interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x3fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0fscr",
                    description: Some(
                        "DCMIPP Pipe0 flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0fscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0fctcr",
                    description: Some(
                        "DCMIPP Pipe0 flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0fctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0scstr",
                    description: Some(
                        "DCMIPP Pipe0 stat/crop start register.",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0scstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0scszr",
                    description: Some(
                        "DCMIPP Pipe0 stat/crop size register.",
                    ),
                    array: None,
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0scszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0dccntr",
                    description: Some(
                        "DCMIPP Pipe0 dump counter register.",
                    ),
                    array: None,
                    byte_offset: 0x5b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0dccntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0dclmtr",
                    description: Some(
                        "DCMIPP Pipe0 dump limit register.",
                    ),
                    array: None,
                    byte_offset: 0x5b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0dclmtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0ppcr",
                    description: Some(
                        "DCMIPP Pipe0 pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x5c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0ppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0ppm0ar1",
                    description: Some(
                        "DCMIPP Pipe0 pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x5c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0ppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0ppm0ar2",
                    description: Some(
                        "DCMIPP Pipe0 pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0x5c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0ppm0ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0stm0ar",
                    description: Some(
                        "DCMIPP Pipe0 status Memory0 address register.",
                    ),
                    array: None,
                    byte_offset: 0x5d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0stm0ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0ier",
                    description: Some(
                        "DCMIPP Pipe0 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x5f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0sr",
                    description: Some(
                        "DCMIPP Pipe0 status register.",
                    ),
                    array: None,
                    byte_offset: 0x5f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0fcr",
                    description: Some(
                        "DCMIPP Pipe0 interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x5fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cfscr",
                    description: Some(
                        "DCMIPP Pipe0 current flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x604,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cfscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cfctcr",
                    description: Some(
                        "DCMIPP Pipe0 current flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cfctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cscstr",
                    description: Some(
                        "DCMIPP Pipe0 current stat/crop start register.",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cscstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cscszr",
                    description: Some(
                        "DCMIPP Pipe0 current stat/crop size register.",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cscszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cppcr",
                    description: Some(
                        "DCMIPP Pipe0 current pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x7c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cppm0ar1",
                    description: Some(
                        "DCMIPP Pipe0 current pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x7c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p0cppm0ar2",
                    description: Some(
                        "DCMIPP Pipe0 current pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0x7c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P0cppm0ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1fscr",
                    description: Some(
                        "DCMIPP Pipe1 flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1fscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1srcr",
                    description: Some(
                        "DCMIPP Pipe1 stat removal configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x820,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1srcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1bprcr",
                    description: Some(
                        "DCMIPP Pipe1 bad pixel removal control register.",
                    ),
                    array: None,
                    byte_offset: 0x824,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1bprcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1bprsr",
                    description: Some(
                        "DCMIPP Pipe1 bad pixel removal status register.",
                    ),
                    array: None,
                    byte_offset: 0x828,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1bprsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1decr",
                    description: Some(
                        "DCMIPP Pipe1 decimation register.",
                    ),
                    array: None,
                    byte_offset: 0x830,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1decr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1blccr",
                    description: Some(
                        "DCMIPP Pipe1 black level calibration control register.",
                    ),
                    array: None,
                    byte_offset: 0x840,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1blccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1excr1",
                    description: Some(
                        "DCMIPP Pipe1 exposure control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x844,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1excr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1excr2",
                    description: Some(
                        "DCMIPP Pipe1 exposure control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x848,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1excr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st1cr",
                    description: Some(
                        "DCMIPP Pipe1 statistics1 control register.",
                    ),
                    array: None,
                    byte_offset: 0x850,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st1cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st2cr",
                    description: Some(
                        "DCMIPP Pipe1 statistics 2 control register.",
                    ),
                    array: None,
                    byte_offset: 0x854,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st3cr",
                    description: Some(
                        "DCMIPP Pipe1 statistics 3 control register.",
                    ),
                    array: None,
                    byte_offset: 0x858,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st3cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ststr",
                    description: Some(
                        "DCMIPP Pipe1 statistics window start register.",
                    ),
                    array: None,
                    byte_offset: 0x85c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ststr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1stszr",
                    description: Some(
                        "DCMIPP Pipe1 statistics window size register.",
                    ),
                    array: None,
                    byte_offset: 0x860,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1stszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st1sr",
                    description: Some(
                        "DCMIPP Pipe1 statistics 1 status register.",
                    ),
                    array: None,
                    byte_offset: 0x864,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st1sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st2sr",
                    description: Some(
                        "DCMIPP Pipe1 statistics 2 status register.",
                    ),
                    array: None,
                    byte_offset: 0x868,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st2sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1st3sr",
                    description: Some(
                        "DCMIPP Pipe1 statistics 3 status register.",
                    ),
                    array: None,
                    byte_offset: 0x86c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1st3sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1dmcr",
                    description: Some(
                        "DCMIPP Pipe1 demosaicing configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x870,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1dmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccr",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x880,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccrr1",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv red coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x884,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccrr2",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv red coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x888,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccgr1",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv green coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x88c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccgr2",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv green coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x890,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccbr1",
                    description: Some(
                        "DCMIPP Pipex ColorConv blue coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x894,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccbr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccbr2",
                    description: Some(
                        "DCMIPP Pipe1 ColorConv blue coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x898,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccbr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ctcr1",
                    description: Some(
                        "DCMIPP Pipe1 contrast control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x8a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ctcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ctcr2",
                    description: Some(
                        "DCMIPP Pipe1 contrast control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x8a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ctcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ctcr3",
                    description: Some(
                        "DCMIPP Pipe1 contrast control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x8a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ctcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1fctcr",
                    description: Some(
                        "DCMIPP Pipex flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1fctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1crstr",
                    description: Some(
                        "DCMIPP Pipex crop window start register.",
                    ),
                    array: None,
                    byte_offset: 0x904,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1crstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1crszr",
                    description: Some(
                        "DCMIPP Pipex crop window size register.",
                    ),
                    array: None,
                    byte_offset: 0x908,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1crszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1dccr",
                    description: Some(
                        "DCMIPP Pipex decimation register.",
                    ),
                    array: None,
                    byte_offset: 0x90c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1dccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1dscr",
                    description: Some(
                        "DCMIPP Pipex downsize configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x910,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1dscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1dsrtior",
                    description: Some(
                        "DCMIPP Pipex downsize ratio register.",
                    ),
                    array: None,
                    byte_offset: 0x914,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1dsrtior",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1dsszr",
                    description: Some(
                        "DCMIPP Pipex downsize destination size register.",
                    ),
                    array: None,
                    byte_offset: 0x918,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1dsszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cmricr",
                    description: Some(
                        "DCMIPP Pipex common ROI configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x920,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cmricr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri1cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI1 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x924,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri1cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri1cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI1 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x928,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri1cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri2cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI2 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x92c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri2cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri2cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI2 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x930,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri2cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri3cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI3 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x934,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri3cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri3cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI3 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x938,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri3cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri4cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI4 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x93c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri4cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri4cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI4 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x940,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri4cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri5cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI5 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x944,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri5cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri5cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI5 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x948,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri5cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri6cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI6 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x94c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri6cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri6cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI6 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x950,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri6cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri7cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI7 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x954,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri7cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri7cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI7 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x958,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri7cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri8cr1",
                    description: Some(
                        "DCMIPP Pipe1 ROI8 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0x95c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri8cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ri8cr2",
                    description: Some(
                        "DCMIPP Pipe1 ROI8 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x960,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ri8cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1gmcr",
                    description: Some(
                        "DCMIPP Pipex gamma configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x970,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1gmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvcr",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x980,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvrr1",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv red coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x984,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvrr2",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv red coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x988,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvgr1",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv green coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x98c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvgr2",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv green coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x990,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvbr1",
                    description: Some(
                        "DCMIPP Pipe1 YUVConv blue coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0x994,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvbr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1yuvbr2",
                    description: Some(
                        "DCMIPP Pipe1 YUV blue coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0x998,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1yuvbr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppcr",
                    description: Some(
                        "DCMIPP Pipe1 pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x9c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm0ar1",
                    description: Some(
                        "DCMIPP Pipe1 pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x9c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm0ar2",
                    description: Some(
                        "DCMIPP Pipe1 pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0x9c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm0ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm0pr",
                    description: Some(
                        "DCMIPP Pipex pixel packer Memory0 pitch register.",
                    ),
                    array: None,
                    byte_offset: 0x9cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm0pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1stm0ar",
                    description: Some(
                        "DCMIPP Pipex status Memory0 address register.",
                    ),
                    array: None,
                    byte_offset: 0x9d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1stm0ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm1ar1",
                    description: Some(
                        "DCMIPP Pipex pixel packer Memory1 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x9d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm1ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm1ar2",
                    description: Some(
                        "DCMIPP Pipex pixel packer Memory1 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0x9d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm1ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm1pr",
                    description: Some(
                        "DCMIPP Pipex pixel packer Memory1 pitch register.",
                    ),
                    array: None,
                    byte_offset: 0x9dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm1pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1stm1ar",
                    description: Some(
                        "DCMIPP Pipex status Memory1 address register.",
                    ),
                    array: None,
                    byte_offset: 0x9e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1stm1ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm2ar1",
                    description: Some(
                        "DCMIPP Pipex pixel packer memory2 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x9e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm2ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ppm2ar2",
                    description: Some(
                        "DCMIPP Pipex pixel packer memory2 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0x9e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ppm2ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1stm2ar",
                    description: Some(
                        "DCMIPP Pipex status Memory2 address register.",
                    ),
                    array: None,
                    byte_offset: 0x9f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1stm2ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ier",
                    description: Some(
                        "DCMIPP Pipe1 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x9f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1sr",
                    description: Some(
                        "DCMIPP Pipe1 status register.",
                    ),
                    array: None,
                    byte_offset: 0x9f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1fcr",
                    description: Some(
                        "DCMIPP Pipe1 interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x9fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cfscr",
                    description: Some(
                        "DCMIPP Pipe1 current flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xa04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cfscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cbprcr",
                    description: Some(
                        "DCMIPP Pipe1 current bad pixel removal register.",
                    ),
                    array: None,
                    byte_offset: 0xa24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cbprcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cblccr",
                    description: Some(
                        "DCMIPP Pipe1 current black level calibration control register.",
                    ),
                    array: None,
                    byte_offset: 0xa40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cblccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cexcr1",
                    description: Some(
                        "DCMIPP Pipe1 current exposure control register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cexcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cexcr2",
                    description: Some(
                        "DCMIPP Pipe1 current exposure control register 2.",
                    ),
                    array: None,
                    byte_offset: 0xa48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cexcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cst1cr",
                    description: Some(
                        "DCMIPP Pipe1 current statistics 1 control register.",
                    ),
                    array: None,
                    byte_offset: 0xa50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cst1cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cst2cr",
                    description: Some(
                        "DCMIPP Pipe1 current statistics 2 control register.",
                    ),
                    array: None,
                    byte_offset: 0xa54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cst2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cst3cr",
                    description: Some(
                        "DCMIPP Pipe1 current statistics 3 control register.",
                    ),
                    array: None,
                    byte_offset: 0xa58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cst3cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cststr",
                    description: Some(
                        "DCMIPP Pipe1 current statistics window start register.",
                    ),
                    array: None,
                    byte_offset: 0xa5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cststr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cstszr",
                    description: Some(
                        "DCMIPP Pipe1 current statistics window size register.",
                    ),
                    array: None,
                    byte_offset: 0xa60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cstszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccccr",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xa80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccrr1",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv red coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccrr2",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv red coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0xa88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccgr1",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv green coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccgr2",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv green coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0xa90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccbr1",
                    description: Some(
                        "DCMIPP Pipex current ColorConv blue coefficient register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccbr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cccbr2",
                    description: Some(
                        "DCMIPP Pipe1 current ColorConv blue coefficient register 2.",
                    ),
                    array: None,
                    byte_offset: 0xa98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cccbr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cctcr1",
                    description: Some(
                        "DCMIPP Pipe1 current contrast control register 1.",
                    ),
                    array: None,
                    byte_offset: 0xaa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cctcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cctcr2",
                    description: Some(
                        "DCMIPP Pipe1 current contrast control register 2.",
                    ),
                    array: None,
                    byte_offset: 0xaa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cctcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cctcr3",
                    description: Some(
                        "DCMIPP Pipe1 current contrast control register 3.",
                    ),
                    array: None,
                    byte_offset: 0xaa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cctcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cfctcr",
                    description: Some(
                        "DCMIPP Pipex current flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cfctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccrstr",
                    description: Some(
                        "DCMIPP Pipex current crop window start register.",
                    ),
                    array: None,
                    byte_offset: 0xb04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccrszr",
                    description: Some(
                        "DCMIPP Pipex current crop window size register.",
                    ),
                    array: None,
                    byte_offset: 0xb08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccrszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cdccr",
                    description: Some(
                        "DCMIPP Pipex current decimation register.",
                    ),
                    array: None,
                    byte_offset: 0xb0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cdccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cdscr",
                    description: Some(
                        "DCMIPP Pipex current downsize configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xb10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cdscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cdsrtior",
                    description: Some(
                        "DCMIPP Pipex current downsize ratio register.",
                    ),
                    array: None,
                    byte_offset: 0xb14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cdsrtior",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cdsszr",
                    description: Some(
                        "DCMIPP Pipex current downsize destination size register.",
                    ),
                    array: None,
                    byte_offset: 0xb18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cdsszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1ccmricr",
                    description: Some(
                        "DCMIPP Pipex current common ROI configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xb20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1ccmricr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri1cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI1 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri1cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri1cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI1 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri1cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri2cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI2 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri2cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri2cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI2 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri2cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri3cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI3 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri3cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri3cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI3 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri3cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri4cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI4 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri4cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri4cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI4 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri4cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri5cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI5 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri5cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri5cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI5 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri5cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri6cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI6 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri6cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri6cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI6 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri6cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri7cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI7 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri7cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri7cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI7 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri7cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri8cr1",
                    description: Some(
                        "DCMIPP Pipe1 current ROI8 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri8cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cri8cr2",
                    description: Some(
                        "DCMIPP Pipe1 current ROI8 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cri8cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppcr",
                    description: Some(
                        "DCMIPP Pipe1 current pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xbc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm0ar1",
                    description: Some(
                        "DCMIPP Pipe1 current pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xbc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm0ar2",
                    description: Some(
                        "DCMIPP Pipe1 current pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0xbc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm0ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm0pr",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory0 pitch register.",
                    ),
                    array: None,
                    byte_offset: 0xbcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm0pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm1ar1",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory1 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xbd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm1ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm1ar2",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory1 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0xbd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm1ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm1pr",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory1 pitch register.",
                    ),
                    array: None,
                    byte_offset: 0xbdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm1pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm2ar1",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory2 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xbe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm2ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p1cppm2ar2",
                    description: Some(
                        "DCMIPP Pipex current pixel packer Memory2 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xbe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P1cppm2ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2fscr",
                    description: Some(
                        "DCMIPP Pipe2 flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xc04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2fscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2fctcr",
                    description: Some(
                        "DCMIPP Pipex flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xd00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2fctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2crstr",
                    description: Some(
                        "DCMIPP Pipex crop window start register.",
                    ),
                    array: None,
                    byte_offset: 0xd04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2crstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2crszr",
                    description: Some(
                        "DCMIPP Pipex crop window size register.",
                    ),
                    array: None,
                    byte_offset: 0xd08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2crszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2dccr",
                    description: Some(
                        "DCMIPP Pipex decimation register.",
                    ),
                    array: None,
                    byte_offset: 0xd0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2dccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2dscr",
                    description: Some(
                        "DCMIPP Pipex downsize configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xd10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2dscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2dsrtior",
                    description: Some(
                        "DCMIPP Pipex downsize ratio register.",
                    ),
                    array: None,
                    byte_offset: 0xd14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2dsrtior",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2dsszr",
                    description: Some(
                        "DCMIPP Pipex downsize destination size register.",
                    ),
                    array: None,
                    byte_offset: 0xd18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2dsszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cmricr",
                    description: Some(
                        "DCMIPP Pipex common ROI configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xd20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cmricr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri1cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI1 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri1cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri1cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI1 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri1cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri2cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI2 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri2cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri2cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI2 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri2cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri3cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI3 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri3cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri3cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI3 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri3cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri4cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI4 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri4cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri4cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI4 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri4cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri5cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI5 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri5cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri5cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI5 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri5cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri6cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI6 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri6cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri6cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI6 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri6cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri7cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI7 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri7cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri7cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI7 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri7cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri8cr1",
                    description: Some(
                        "DCMIPP Pipe2 ROI8 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri8cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ri8cr2",
                    description: Some(
                        "DCMIPP Pipe2 ROI8 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xd60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ri8cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2gmcr",
                    description: Some(
                        "DCMIPP Pipex gamma configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xd70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2gmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ppcr",
                    description: Some(
                        "DCMIPP Pipe2 pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xdc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ppm0ar1",
                    description: Some(
                        "DCMIPP Pipe2 pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xdc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ppm0ar2",
                    description: Some(
                        "DCMIPP Pipe2 pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0xdc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ppm0ar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ppm0pr",
                    description: Some(
                        "DCMIPP Pipex pixel packer Memory0 pitch register.",
                    ),
                    array: None,
                    byte_offset: 0xdcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ppm0pr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2stm0ar",
                    description: Some(
                        "DCMIPP Pipex status Memory0 address register.",
                    ),
                    array: None,
                    byte_offset: 0xdd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2stm0ar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ier",
                    description: Some(
                        "DCMIPP Pipe2 interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xdf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2sr",
                    description: Some(
                        "DCMIPP Pipe2 status register.",
                    ),
                    array: None,
                    byte_offset: 0xdf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2fcr",
                    description: Some(
                        "DCMIPP Pipe2 interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0xdfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cfscr",
                    description: Some(
                        "DCMIPP Pipe2 current flow selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xe04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cfscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cfctcr",
                    description: Some(
                        "DCMIPP Pipex current flow control configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xf00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cfctcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ccrstr",
                    description: Some(
                        "DCMIPP Pipex current crop window start register.",
                    ),
                    array: None,
                    byte_offset: 0xf04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ccrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ccrszr",
                    description: Some(
                        "DCMIPP Pipex current crop window size register.",
                    ),
                    array: None,
                    byte_offset: 0xf08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ccrszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cdccr",
                    description: Some(
                        "DCMIPP Pipex current decimation register.",
                    ),
                    array: None,
                    byte_offset: 0xf0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cdccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cdscr",
                    description: Some(
                        "DCMIPP Pipex current downsize configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xf10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cdscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cdsrtior",
                    description: Some(
                        "DCMIPP Pipex current downsize ratio register.",
                    ),
                    array: None,
                    byte_offset: 0xf14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cdsrtior",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cdsszr",
                    description: Some(
                        "DCMIPP Pipex current downsize destination size register.",
                    ),
                    array: None,
                    byte_offset: 0xf18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cdsszr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2ccmricr",
                    description: Some(
                        "DCMIPP Pipex current common ROI configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xf20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2ccmricr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri1cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI1 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri1cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri1cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI1 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri1cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri2cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI2 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri2cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri2cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI2 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri2cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri3cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI3 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri3cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri3cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI3 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri3cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri4cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI4 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri4cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri4cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI4 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri4cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri5cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI5 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri5cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri5cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI5 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri5cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri6cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI6 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri6cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri6cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI6 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri6cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri7cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI7 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri7cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri7cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI7 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri7cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri8cr1",
                    description: Some(
                        "DCMIPP Pipe2 current ROI8 configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xf5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri8cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cri8cr2",
                    description: Some(
                        "DCMIPP Pipe2 current ROI8 configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0xf60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cri8cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cppcr",
                    description: Some(
                        "DCMIPP Pipe2 current pixel packer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0xfc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cppcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cppm0ar1",
                    description: Some(
                        "DCMIPP Pipe2 current pixel packer Memory0 address register 1.",
                    ),
                    array: None,
                    byte_offset: 0xfc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cppm0ar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2cppm0ar2",
                    description: Some(
                        "DCMIPP Pipe2 current pixel packer Memory0 address register 2.",
                    ),
                    array: None,
                    byte_offset: 0xfc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2cppm0ar2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmcr",
            extends: None,
            description: Some(
                "DCMIPP common configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel",
                    description: Some(
                        "input selection.",
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
                    name: "psfc",
                    description: Some(
                        "Pipe selection for the frame counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfc",
                    description: Some(
                        "Clear frame counter.",
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
                    name: "swaprb",
                    description: Some(
                        "Swap R/U and B/V.",
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
            name: "Cmfcr",
            extends: None,
            description: Some(
                "DCMIPP common interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "catxerrf",
                    description: Some(
                        "AXI transfer error interrupt status clear.",
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
                    name: "cprerrf",
                    description: Some(
                        "Synchronization error interrupt status clear.",
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
                    name: "cp0linef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cp0framef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cp0vsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "cp0limitf",
                    description: Some(
                        "limit interrupt status clear.",
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
                    name: "cp0ovrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
                    name: "cp1linef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cp1framef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cp1vsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "cp1ovrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
                    name: "cp2linef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cp2framef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cp2vsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "cp2ovrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
            name: "Cmfrcr",
            extends: None,
            description: Some(
                "DCMIPP common frame counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frmcnt",
                    description: Some(
                        "Frame counter, read-only, loops around.",
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
            name: "Cmier",
            extends: None,
            description: Some(
                "DCMIPP common interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "atxerrie",
                    description: Some(
                        "AXI transfer error interrupt enable for IPPLUG.",
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
                    name: "prerrie",
                    description: Some(
                        "Limit interrupt enable for the parallel Interface.",
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
                    name: "p0lineie",
                    description: Some(
                        "Multi-line capture complete interrupt enable for Pipe0.",
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
                    name: "p0frameie",
                    description: Some(
                        "Frame capture complete interrupt enable for Pipe0.",
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
                    name: "p0vsyncie",
                    description: Some(
                        "Vertical sync interrupt enable for Pipe0.",
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
                    name: "p0limitie",
                    description: Some(
                        "Limit interrupt enable for Pipe0.",
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
                    name: "p0ovrie",
                    description: Some(
                        "Overrun interrupt enable for Pipe0.",
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
                    name: "p1lineie",
                    description: Some(
                        "Multi-line capture complete interrupt status clear for Pipe1.",
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
                    name: "p1frameie",
                    description: Some(
                        "Frame capture complete interrupt enable for Pipe1.",
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
                    name: "p1vsyncie",
                    description: Some(
                        "Vertical sync interrupt enable for Pipe1.",
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
                    name: "p1ovrie",
                    description: Some(
                        "Overrun interrupt enable for Pipe1.",
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
                    name: "p2lineie",
                    description: Some(
                        "Multi-line capture complete interrupt enable for Pipe2.",
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
                    name: "p2frameie",
                    description: Some(
                        "Frame capture complete interrupt enable for Pipe2.",
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
                    name: "p2vsyncie",
                    description: Some(
                        "Vertical sync interrupt enable for Pipe2.",
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
                    name: "p2ovrie",
                    description: Some(
                        "Overrun interrupt status enable for Pipe2.",
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
            name: "Cmsr1",
            extends: None,
            description: Some(
                "DCMIPP common status register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prhsync",
                    description: Some(
                        "This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise.",
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
                    name: "prvsync",
                    description: Some(
                        "This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise.",
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
                    name: "p0lstline",
                    description: Some(
                        "Last line LSB bit, sampled at Frame capture complete event for Pipe0.",
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
                    name: "p0lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at Frame capture complete event for Pipe0.",
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
                    name: "p0cptact",
                    description: Some(
                        "Active frame capture (active from start-of-frame to frame complete) for Pipe0.",
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
                    name: "p1lstline",
                    description: Some(
                        "Last line LSB bit, sampled at Frame capture complete event for Pipe1.",
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
                    name: "p1lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at frame capture complete event for Pipe1.",
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
                    name: "p1cptact",
                    description: Some(
                        "Active frame capture (active from start-of-frame to frame complete) for Pipe1.",
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
                    name: "p2lstline",
                    description: Some(
                        "Last line LSB bit, sampled at frame capture complete event for Pipe2.",
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
                    name: "p2lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at frame capture complete event for Pipe2.",
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
                    name: "p2cptact",
                    description: Some(
                        "Active frame capture (active from start-of-frame to frame complete) for Pipe2.",
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
            name: "Cmsr2",
            extends: None,
            description: Some(
                "DCMIPP common status register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "atxerrf",
                    description: Some(
                        "AXI transfer error interrupt status flag for the IPPLUG.",
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
                    name: "prerrf",
                    description: Some(
                        "Synchronization error raw interrupt status for the parallel interface.",
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
                    name: "p0linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status for Pipe0.",
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
                    name: "p0framef",
                    description: Some(
                        "Frame capture completed raw interrupt status for Pipe0.",
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
                    name: "p0vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status for Pipe0.",
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
                    name: "p0limitf",
                    description: Some(
                        "Limit raw interrupt status for Pipe0.",
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
                    name: "p0ovrf",
                    description: Some(
                        "Overrun raw interrupt status for Pipe0.",
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
                    name: "p1linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status for Pipe1.",
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
                    name: "p1framef",
                    description: Some(
                        "Frame capture completed raw interrupt status for Pipe1.",
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
                    name: "p1vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status for Pipe1.",
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
                    name: "p1ovrf",
                    description: Some(
                        "Overrun raw interrupt status for Pipe1.",
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
                    name: "p2linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status for Pipe2.",
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
                    name: "p2framef",
                    description: Some(
                        "Frame capture completed raw interrupt status for Pipe2.",
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
                    name: "p2vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status for Pipe2.",
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
                    name: "p2ovrf",
                    description: Some(
                        "Overrun raw interrupt status for Pipe2.",
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
            name: "Ipc1r1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "traffic",
                    description: Some(
                        "Burst size as power of 2 of 8-byte units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otr",
                    description: Some(
                        "Maximum outstanding transactions.",
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
            ],
        },
        FieldSet {
            name: "Ipc1r2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svcmapping",
                    description: Some(
                        "Non-user, must be kept at reset value.",
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
                    name: "wlru",
                    description: Some(
                        "Ratio for WLRU[3:0] arbitration.",
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
            ],
        },
        FieldSet {
            name: "Ipc1r3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpregstart",
                    description: Some(
                        "Start word (AXI width = 64 bits) of the FIFO of Clientx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpregend",
                    description: Some(
                        "End word (AXI width = 64 bits) of the FIFO of Clientx.",
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
            ],
        },
        FieldSet {
            name: "Ipc2r1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "traffic",
                    description: Some(
                        "Burst size as power of 2 of 8-byte units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otr",
                    description: Some(
                        "Maximum outstanding transactions.",
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
            ],
        },
        FieldSet {
            name: "Ipc2r2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svcmapping",
                    description: Some(
                        "Non-user, must be kept at reset value.",
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
                    name: "wlru",
                    description: Some(
                        "Ratio for WLRU[3:0] arbitration.",
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
            ],
        },
        FieldSet {
            name: "Ipc2r3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpregstart",
                    description: Some(
                        "Start word (AXI width = 64 bits) of the FIFO of Clientx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpregend",
                    description: Some(
                        "End word (AXI width = 64 bits) of the FIFO of Clientx.",
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
            ],
        },
        FieldSet {
            name: "Ipc3r1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "traffic",
                    description: Some(
                        "Burst size as power of 2 of 8-byte units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otr",
                    description: Some(
                        "Maximum outstanding transactions.",
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
            ],
        },
        FieldSet {
            name: "Ipc3r2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svcmapping",
                    description: Some(
                        "Non-user, must be kept at reset value.",
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
                    name: "wlru",
                    description: Some(
                        "Ratio for WLRU[3:0] arbitration.",
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
            ],
        },
        FieldSet {
            name: "Ipc3r3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpregstart",
                    description: Some(
                        "Start word (AXI width = 64 bits) of the FIFO of Clientx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpregend",
                    description: Some(
                        "End word (AXI width = 64 bits) of the FIFO of Clientx.",
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
            ],
        },
        FieldSet {
            name: "Ipc4r1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "traffic",
                    description: Some(
                        "Burst size as power of 2 of 8-byte units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otr",
                    description: Some(
                        "Maximum outstanding transactions.",
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
            ],
        },
        FieldSet {
            name: "Ipc4r2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svcmapping",
                    description: Some(
                        "Non-user, must be kept at reset value.",
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
                    name: "wlru",
                    description: Some(
                        "Ratio for WLRU[3:0] arbitration.",
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
            ],
        },
        FieldSet {
            name: "Ipc4r3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpregstart",
                    description: Some(
                        "Start word (AXI width = 64 bits) of the FIFO of Clientx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpregend",
                    description: Some(
                        "End word (AXI width = 64 bits) of the FIFO of Clientx.",
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
            ],
        },
        FieldSet {
            name: "Ipc5r1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "traffic",
                    description: Some(
                        "Burst size as power of 2 of 8-byte units.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otr",
                    description: Some(
                        "Maximum outstanding transactions.",
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
            ],
        },
        FieldSet {
            name: "Ipc5r2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svcmapping",
                    description: Some(
                        "Non-user, must be kept at reset value.",
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
                    name: "wlru",
                    description: Some(
                        "Ratio for WLRU[3:0] arbitration.",
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
            ],
        },
        FieldSet {
            name: "Ipc5r3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG Clientx register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpregstart",
                    description: Some(
                        "Start word (AXI width = 64 bits) of the FIFO of Clientx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpregend",
                    description: Some(
                        "End word (AXI width = 64 bits) of the FIFO of Clientx.",
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
            ],
        },
        FieldSet {
            name: "Ipgr1",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG global register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "memorypage",
                    description: Some(
                        "Memory page size, as power of 2 of 64-byte units:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qos_mode",
                    description: Some(
                        "Quality of service.",
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
            name: "Ipgr2",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG global register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pstart",
                    description: Some(
                        "Request to lock the IP-Plug, to allow reconfiguration.",
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
            ],
        },
        FieldSet {
            name: "Ipgr3",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG global register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idle",
                    description: Some(
                        "Status of IP-Plug.",
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
            ],
        },
        FieldSet {
            name: "Ipgr8",
            extends: None,
            description: Some(
                "DCMIPP IPPLUG identification register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "did",
                    description: Some(
                        "Division identifier (0x14).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "revid",
                    description: Some(
                        "Revision identifier (0x03).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "archiid",
                    description: Some(
                        "Architecture identifier (0x04).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ippid",
                    description: Some(
                        "IP identifier (0xAA).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P0cfctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P0cfscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Current data type selection ID A.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtidb",
                    description: Some(
                        "Current data type selection ID B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtmode",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vc",
                    description: Some(
                        "Current flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Current activation of PipeN.",
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
            name: "P0cppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swapyuv",
                    description: Some(
                        "Swaps, within a 32-bit word, byte 0 vs. 1 and byte 2 vs. 3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.",
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
                    name: "pad",
                    description: Some(
                        "Current Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.",
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
                    name: "headeren",
                    description: Some(
                        "Current CSI header dump enable.",
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
                    name: "bsm",
                    description: Some(
                        "Current Byte select mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oebs",
                    description: Some(
                        "Current odd/even byte select (byte select start).",
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
                    name: "lsm",
                    description: Some(
                        "Current Line select mode.",
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
                    name: "oels",
                    description: Some(
                        "Current odd/even line select (ine select start).",
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
                    name: "linemult",
                    description: Some(
                        "Current amount of capture completed lines for LINE event and interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
            ],
        },
        FieldSet {
            name: "P0cppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P0cppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P0cscstr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current stat/crop start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 words wide.",
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
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
            name: "P0cscszr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 current stat/crop size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 word wide (data 32-bit).",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "posneg",
                    description: Some(
                        "Current value of the POSNEG bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Current value of the ENABLE bit.",
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
            name: "P0dccntr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 dump counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Number of data dumped during the frame.",
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
            name: "P0dclmtr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 dump limit register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit",
                    description: Some(
                        "Maximum number of 32-bit data that can be dumped during a frame, after the crop 2D operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
            name: "P0fcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clinef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cframef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cvsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "climitf",
                    description: Some(
                        "limit interrupt status clear.",
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
                    name: "covrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
            name: "P0fctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P0fscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Data type selection ID A.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtidb",
                    description: Some(
                        "Data type selection ID B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtmode",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vc",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Activation of PipeN.",
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
            name: "P0ier",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lineie",
                    description: Some(
                        "Multi-line capture completed interrupt enable.",
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
                    name: "frameie",
                    description: Some(
                        "Frame capture completed interrupt enable.",
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
                    name: "vsyncie",
                    description: Some(
                        "VSYNC interrupt enable.",
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
                    name: "limitie",
                    description: Some(
                        "Limit interrupt enable.",
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
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable.",
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
            name: "P0ppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swapyuv",
                    description: Some(
                        "Swaps, within a 32-bit word, byte 0-vs-1 and byte 2-vs-3. It corresponds, for YUV422 pixels formats, to swap between UYVY and YUYV.",
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
                    name: "pad",
                    description: Some(
                        "Pad mode for monochrome and raw Bayer 10/12/14 bpp: MSB vs. LSB alignment.",
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
                    name: "headeren",
                    description: Some(
                        "CSI header dump enable.",
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
                    name: "bsm",
                    description: Some(
                        "Byte select mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oebs",
                    description: Some(
                        "Odd/even byte select (byte select start).",
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
                    name: "lsm",
                    description: Some(
                        "Line select mode.",
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
                    name: "oels",
                    description: Some(
                        "Odd/even line select (line select start).",
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
                    name: "linemult",
                    description: Some(
                        "Amount of capture completed lines for LINE event and interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
            ],
        },
        FieldSet {
            name: "P0ppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P0ppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P0scstr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 stat/crop start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 words wide.",
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
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
            name: "P0scszr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 stat/crop size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 word wide (data 32-bit).",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "posneg",
                    description: Some(
                        "This bit is set and cleared by software. It has a meaning only if ENABLE bit is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "This bit is set and cleared by software.",
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
            name: "P0sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status.",
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
                    name: "framef",
                    description: Some(
                        "Frame capture completed raw interrupt status.",
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
                    name: "vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status.",
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
                    name: "limitf",
                    description: Some(
                        "Limit raw interrupt status.",
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
                    name: "ovrf",
                    description: Some(
                        "Overrun raw interrupt status.",
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
                    name: "lstline",
                    description: Some(
                        "Last line LSB bit, sampled at frame capture complete event.",
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
                    name: "lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number that can be delivered by the camera through the CSI2 interface.",
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
                    name: "cptact",
                    description: Some(
                        "Capture immediate status.",
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
            name: "P0stm0ar",
            extends: None,
            description: Some(
                "DCMIPP Pipe0 status Memory0 address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1blccr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 black level calibration control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Black level calibration.",
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
                    name: "blcb",
                    description: Some(
                        "Black level calibration - Blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blcg",
                    description: Some(
                        "Black level calibration - Green.",
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
                Field {
                    name: "blcr",
                    description: Some(
                        "Black level calibration - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1bprcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 bad pixel removal control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows.",
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
                    name: "strength",
                    description: Some(
                        "Strength (aggressiveness) of the bad pixel detection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1bprsr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 bad pixel removal status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "badcnt",
                    description: Some(
                        "Amount of detected bad pixels.",
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
            name: "P1cblccr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current black level calibration control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "For current black level calibration.",
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
                    name: "blcb",
                    description: Some(
                        "Current black level calibration - Blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blcg",
                    description: Some(
                        "Current black level calibration - Green.",
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
                Field {
                    name: "blcr",
                    description: Some(
                        "Current black level calibration - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cbprcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current bad pixel removal register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Current status of enable bit.",
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
                    name: "strength",
                    description: Some(
                        "Current strength (aggressiveness) of the bad pixel detection:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1ccbr1",
            extends: None,
            description: Some(
                "DCMIPP Pipex ColorConv blue coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "br",
                    description: Some(
                        "Coefficient row 3 column 1 of the matrix.",
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
                    name: "bg",
                    description: Some(
                        "Coefficient row 3 column 2 of the matrix.",
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
            name: "P1ccbr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv blue coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bb",
                    description: Some(
                        "Coefficient row 3 column 3 of the matrix.",
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
                    name: "ba",
                    description: Some(
                        "Coefficient row 3 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1cccbr1",
            extends: None,
            description: Some(
                "DCMIPP Pipex current ColorConv blue coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "br",
                    description: Some(
                        "Current coefficient row 3 column 1 of the matrix.",
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
                    name: "bg",
                    description: Some(
                        "Current coefficient row 3 column 2 of the matrix.",
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
            name: "P1cccbr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv blue coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bb",
                    description: Some(
                        "Current coefficient row 3 column 3 of the matrix.",
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
                    name: "ba",
                    description: Some(
                        "Current coefficient row 3 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1ccccr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Current value applied.",
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
                    name: "type_",
                    description: Some(
                        "Output samples type used while CLAMP is activated.",
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
                    name: "clamp",
                    description: Some(
                        "Clamp the output samples.",
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
            ],
        },
        FieldSet {
            name: "P1cccgr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv green coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gr",
                    description: Some(
                        "Current coefficient row 2 column 1 of the matrix.",
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
                    name: "gg",
                    description: Some(
                        "Current coefficient row 2 column 2 of the matrix.",
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
            name: "P1cccgr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv green coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gb",
                    description: Some(
                        "Current coefficient row 2 column 3 of the matrix.",
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
                    name: "ga",
                    description: Some(
                        "Current coefficient row 2 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1cccr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "type_",
                    description: Some(
                        "output samples type used while CLAMP is activated.",
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
                    name: "clamp",
                    description: Some(
                        "Clamp the output samples.",
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
            ],
        },
        FieldSet {
            name: "P1cccrr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv red coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rr",
                    description: Some(
                        "Current coefficient row 1 column 1 of the matrix.",
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
                    name: "rg",
                    description: Some(
                        "Current coefficient row 1 column 2 of the matrix.",
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
            name: "P1cccrr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ColorConv red coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rb",
                    description: Some(
                        "Current coefficient row 1 column 3 of the matrix.",
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
                    name: "ra",
                    description: Some(
                        "Current coefficient row 1 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1ccgr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv green coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gr",
                    description: Some(
                        "Coefficient row 2 column 1 of the matrix.",
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
                    name: "gg",
                    description: Some(
                        "Coefficient row 2 column 2 of the matrix.",
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
            name: "P1ccgr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv green coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gb",
                    description: Some(
                        "Coefficient row 2 column 3 of the matrix.",
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
                    name: "ga",
                    description: Some(
                        "Coefficient row 2 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1ccmricr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current common ROI configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roilsz",
                    description: Some(
                        "Current region of interest line size width.",
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
                    name: "roi1en",
                    description: Some(
                        "Current region of interest 1 enable.",
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
                    name: "roi2en",
                    description: Some(
                        "Current region of interest 2 enable.",
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
                    name: "roi3en",
                    description: Some(
                        "Current region of interest 3 enable.",
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
                    name: "roi4en",
                    description: Some(
                        "Current region of interest 4 enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "roi5en",
                    description: Some(
                        "Current region of interest 5 enable.",
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
                    name: "roi6en",
                    description: Some(
                        "Current region of interest 6 enable.",
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
                    name: "roi7en",
                    description: Some(
                        "Current region of interest 7 enable.",
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
                    name: "roi8en",
                    description: Some(
                        "Current region of interest 8 enable.",
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
            name: "P1ccrr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv red coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rr",
                    description: Some(
                        "Coefficient row 1 column 1 of the matrix.",
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
                    name: "rg",
                    description: Some(
                        "Coefficient row 1 column 2 of the matrix.",
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
            name: "P1ccrr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ColorConv red coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rb",
                    description: Some(
                        "Coefficient row 1 column 3 of the matrix.",
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
                    name: "ra",
                    description: Some(
                        "Coefficient row 1 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1ccrstr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current crop window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
            name: "P1ccrszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current crop window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "Current ENABLE bit value.",
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
            name: "P1cctcr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current contrast control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Current ENABLE bit value.",
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
                    name: "lum0",
                    description: Some(
                        "Current luminance increase for input luminance of 0 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cctcr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current contrast control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lum4",
                    description: Some(
                        "Current luminance increase for input luminance of 128 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum3",
                    description: Some(
                        "Current luminance increase for input luminance of 96 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum2",
                    description: Some(
                        "Current luminance increase for input luminance of 64 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum1",
                    description: Some(
                        "Current luminance increase for input luminance of 32 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cctcr3",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current contrast control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lum8",
                    description: Some(
                        "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum7",
                    description: Some(
                        "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum6",
                    description: Some(
                        "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum5",
                    description: Some(
                        "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cdccr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current decimation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "hdec",
                    description: Some(
                        "Horizontal decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdec",
                    description: Some(
                        "Vertical decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cdscr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdiv",
                    description: Some(
                        "Current horizontal division factor, from 128 (8x) to 1023 (1x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdiv",
                    description: Some(
                        "Current vertical division factor, from 128 (8x) to 1023 (1x).",
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
                    name: "enable",
                    description: Some(
                        "Current value of bit ENABLE.",
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
            name: "P1cdsrtior",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize ratio register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hratio",
                    description: Some(
                        "Current horizontal ratio, from 8192 (1x) to 65535 (8x).",
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
                Field {
                    name: "vratio",
                    description: Some(
                        "Current vertical ratio, from 8192 (1x) to 65535 (8x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cdsszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize destination size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cexcr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current exposure control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "for exposure control (multiplication and shift).",
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
                    name: "multr",
                    description: Some(
                        "Current exposure multiplier - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfr",
                    description: Some(
                        "Current exposure shift - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cexcr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current exposure control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "multb",
                    description: Some(
                        "Current exposure multiplier - Blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfb",
                    description: Some(
                        "Current exposure shift - Blue.",
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
                    name: "multg",
                    description: Some(
                        "Current exposure multiplier - Green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfg",
                    description: Some(
                        "Current exposure shift - Green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cfctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P1cfscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Current data type ID A.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtidb",
                    description: Some(
                        "Current data type ID B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtmode",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipediff",
                    description: Some(
                        "Current differentiates Pipe2 vs. Pipe1.",
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
                    name: "vc",
                    description: Some(
                        "Current flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtf",
                    description: Some(
                        "Current force data type format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtfen",
                    description: Some(
                        "Current force data type format enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Current activation of PipeN.",
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
            name: "P1cmricr",
            extends: None,
            description: Some(
                "DCMIPP Pipex common ROI configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roilsz",
                    description: Some(
                        "Region of interest line size width.",
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
                    name: "roi1en",
                    description: Some(
                        "Region of interest 1 enable.",
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
                    name: "roi2en",
                    description: Some(
                        "Region of interest 2 enable.",
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
                    name: "roi3en",
                    description: Some(
                        "Region of interest 3 enable.",
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
                    name: "roi4en",
                    description: Some(
                        "Region of interest 4 enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "roi5en",
                    description: Some(
                        "Region of interest 5 enable.",
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
                    name: "roi6en",
                    description: Some(
                        "Region of interest 6 enable.",
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
                    name: "roi7en",
                    description: Some(
                        "Region of interest 7 enable.",
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
                    name: "roi8en",
                    description: Some(
                        "Region of interest 8 enable.",
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
            name: "P1cppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "Memory format.",
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
                    name: "swaprb",
                    description: Some(
                        "Swaps R-vs-B components if RGB, and U-vs-V components if YUV.",
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
                    name: "linemult",
                    description: Some(
                        "Amount of capture completed lines for LINE Event and Interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
                    name: "lmawm",
                    description: Some(
                        "Line multi address wrapping modulo.",
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
                    name: "lmawe",
                    description: Some(
                        "Line multi address wrapping enable bit.",
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
            ],
        },
        FieldSet {
            name: "P1cppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1cppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1cppm0pr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory0 pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Current number of bytes between the address of two consecutive lines.",
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
            ],
        },
        FieldSet {
            name: "P1cppm1ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory1 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m1a",
                    description: Some(
                        "Memory1 address.",
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
            name: "P1cppm1ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory1 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m1a",
                    description: Some(
                        "Memory1 address.",
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
            name: "P1cppm1pr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory1 pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Current number of bytes between the address of two consecutive lines.",
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
            ],
        },
        FieldSet {
            name: "P1cppm2ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory2 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m2a",
                    description: Some(
                        "Memory 2 address.",
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
            name: "P1cppm2ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipex current pixel packer Memory2 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m2a",
                    description: Some(
                        "Memory 2 address.",
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
            name: "P1cri1cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI1 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri1cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI1 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri2cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI2 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri2cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI2 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri3cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI3 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri3cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI3 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri4cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI4 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri4cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI4 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri5cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI5 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri5cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI5 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri6cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI6 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri6cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI6 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri7cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI7 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri7cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI7 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1cri8cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI8 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P1cri8cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current ROI8 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P1crstr",
            extends: None,
            description: Some(
                "DCMIPP Pipex crop window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
            name: "P1crszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex crop window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
            name: "P1cst1cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current statistics 1 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Current enable bit value.",
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
                    name: "bins",
                    description: Some(
                        "Current bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Current source of statistics.",
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
                    name: "mode",
                    description: Some(
                        "Current statistics mode.",
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
                    name: "accu",
                    description: Some(
                        "Current accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cst2cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current statistics 2 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "bins",
                    description: Some(
                        "Bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Statistics source.",
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
                    name: "mode",
                    description: Some(
                        "Statistics mode.",
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
                    name: "accu",
                    description: Some(
                        "Accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cst3cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current statistics 3 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "bins",
                    description: Some(
                        "Current bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Statistics source.",
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
                    name: "mode",
                    description: Some(
                        "Statistics mode.",
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
                    name: "accu",
                    description: Some(
                        "Accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1cststr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current statistics window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
            name: "P1cstszr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 current statistics window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "cropen",
                    description: Some(
                        "Current CROPEN bit value.",
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
            name: "P1ctcr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 contrast control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "lum0",
                    description: Some(
                        "Luminance increase for input luminance of 0 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1ctcr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 contrast control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lum4",
                    description: Some(
                        "Luminance increase for input luminance of 128 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum3",
                    description: Some(
                        "Luminance increase for input luminance of 96 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum2",
                    description: Some(
                        "Luminance increase for input luminance of 64 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum1",
                    description: Some(
                        "Luminance increase for input luminance of 32 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1ctcr3",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 contrast control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lum8",
                    description: Some(
                        "Luminance increase for input luminance of 256 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum7",
                    description: Some(
                        "Luminance increase for input luminance of 224 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum6",
                    description: Some(
                        "Luminance increase for input luminance of 192 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lum5",
                    description: Some(
                        "Luminance increase for input luminance of 160 (increase is idle with LUMx = 16).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1dccr",
            extends: None,
            description: Some(
                "DCMIPP Pipex decimation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "hdec",
                    description: Some(
                        "Horizontal decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdec",
                    description: Some(
                        "Vertical decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1decr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 decimation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "hdec",
                    description: Some(
                        "Horizontal decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdec",
                    description: Some(
                        "Vertical decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1dmcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 demosaicing configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "type_",
                    description: Some(
                        "Raw Bayer type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "peak",
                    description: Some(
                        "Strength of the peak detection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linev",
                    description: Some(
                        "Strength of the vertical line detection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lineh",
                    description: Some(
                        "Strength of the horizontal line detection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "edge",
                    description: Some(
                        "Strength of the edge detection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1dscr",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdiv",
                    description: Some(
                        "Horizontal division factor, from 128 (8x) to 1023 (1x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdiv",
                    description: Some(
                        "Vertical division factor, from 128 (8x) to 1023 (1x).",
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
                    name: "enable",
                    description: Some(
                        "None.",
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
            name: "P1dsrtior",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize ratio register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hratio",
                    description: Some(
                        "Horizontal ratio, from 8192 (1x) to 65535 (8x).",
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
                Field {
                    name: "vratio",
                    description: Some(
                        "Vertical ratio, from 8192 (1x) to 65535 (8x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1dsszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize destination size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1excr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 exposure control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Exposure control (multiplication and shift) of all red, green and blue.",
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
                    name: "multr",
                    description: Some(
                        "Exposure multiplier - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfr",
                    description: Some(
                        "Exposure shift - Red.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1excr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 exposure control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "multb",
                    description: Some(
                        "Exposure multiplier - Blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfb",
                    description: Some(
                        "Exposure shift - Blue.",
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
                    name: "multg",
                    description: Some(
                        "Exposure multiplier - Green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shfg",
                    description: Some(
                        "Exposure shift - Green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1fcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clinef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cframef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cvsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "covrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
            name: "P1fctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P1fscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Data type selection ID A.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtidb",
                    description: Some(
                        "Data type selection ID B.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtmode",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipediff",
                    description: Some(
                        "Differentiates Pipe2 from Pipe1.",
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
                    name: "vc",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtf",
                    description: Some(
                        "Force Datatype format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtfen",
                    description: Some(
                        "Force Datatype format enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Activation of PipeN.",
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
            name: "P1gmcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex gamma configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
            ],
        },
        FieldSet {
            name: "P1ier",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lineie",
                    description: Some(
                        "Multi-line capture completed interrupt enable.",
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
                    name: "frameie",
                    description: Some(
                        "Frame capture completed interrupt enable.",
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
                    name: "vsyncie",
                    description: Some(
                        "VSYNC interrupt enable.",
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
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable.",
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
            name: "P1ppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "Memory format.",
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
                    name: "swaprb",
                    description: Some(
                        "Swaps R-vs-B components if RGB, and U-vs-V components if YUV.",
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
                    name: "linemult",
                    description: Some(
                        "Amount of capture completed lines for LINE Event and Interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
                    name: "lmawm",
                    description: Some(
                        "Line multi address wrapping modulo.",
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
                    name: "lmawe",
                    description: Some(
                        "Line multi address wrapping enable bit.",
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
            ],
        },
        FieldSet {
            name: "P1ppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1ppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1ppm0pr",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer Memory0 pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Number of bytes between the address of two consecutive lines.",
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
            ],
        },
        FieldSet {
            name: "P1ppm1ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer Memory1 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m1a",
                    description: Some(
                        "Memory1 address.",
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
            name: "P1ppm1ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer Memory1 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m1a",
                    description: Some(
                        "Memory1 address.",
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
            name: "P1ppm1pr",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer Memory1 pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Number of bytes between the address of two consecutive lines.",
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
            ],
        },
        FieldSet {
            name: "P1ppm2ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer memory2 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m2a",
                    description: Some(
                        "Memory 2 address.",
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
            name: "P1ppm2ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer memory2 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m2a",
                    description: Some(
                        "Memory 2 address.",
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
            name: "P1ri1cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI1 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri1cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI1 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri2cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI2 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri2cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI2 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri3cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI3 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri3cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI3 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri4cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI4 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri4cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI4 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri5cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI5 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri5cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI5 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri6cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI6 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri6cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI6 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri7cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI7 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri7cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI7 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1ri8cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI8 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P1ri8cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 ROI8 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P1sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status.",
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
                    name: "framef",
                    description: Some(
                        "Frame capture completed raw interrupt status.",
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
                    name: "vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status.",
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
                    name: "ovrf",
                    description: Some(
                        "Overrun raw interrupt status.",
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
                    name: "lstline",
                    description: Some(
                        "Last line LSB bit, sampled at frame capture complete event.",
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
                    name: "lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number, which can be delivered by the camera through the CSI2 interface.",
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
                    name: "cptact",
                    description: Some(
                        "Capture immediate status.",
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
            name: "P1srcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 stat removal configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lastline",
                    description: Some(
                        "Amount of following lines to keep when CROPEN = 1. If LASTLINE = 0 all pixels after FIRSTLINEDEL are fed through.",
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
                    name: "firstlinedel",
                    description: Some(
                        "Amount of first lines to delete when CROPEN = 1.",
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
                    name: "cropen",
                    description: Some(
                        "Crop line enable.",
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
            ],
        },
        FieldSet {
            name: "P1st1cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics1 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "bins",
                    description: Some(
                        "Current bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Statistics source.",
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
                    name: "mode",
                    description: Some(
                        "Statistics mode.",
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
            name: "P1st1sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics 1 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "accu",
                    description: Some(
                        "Accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1st2cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics 2 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "bins",
                    description: Some(
                        "Bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Statistics source.",
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
                    name: "mode",
                    description: Some(
                        "Statistics mode.",
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
            name: "P1st2sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics 2 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "accu",
                    description: Some(
                        "accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1st3cr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics 3 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "bins",
                    description: Some(
                        "Bin definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "src",
                    description: Some(
                        "Statistics source.",
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
                    name: "mode",
                    description: Some(
                        "Statistics mode.",
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
            name: "P1st3sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics 3 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "accu",
                    description: Some(
                        "accumulation result, divided by 256.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P1stm0ar",
            extends: None,
            description: Some(
                "DCMIPP Pipex status Memory0 address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P1stm1ar",
            extends: None,
            description: Some(
                "DCMIPP Pipex status Memory1 address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m1a",
                    description: Some(
                        "Memory1 address.",
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
            name: "P1stm2ar",
            extends: None,
            description: Some(
                "DCMIPP Pipex status Memory2 address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m2a",
                    description: Some(
                        "Memory2 address.",
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
            name: "P1ststr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
            name: "P1stszr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 statistics window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "cropen",
                    description: Some(
                        "None.",
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
            name: "P1yuvbr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv blue coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "br",
                    description: Some(
                        "Coefficient row 3 column 1 of the matrix.",
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
                    name: "bg",
                    description: Some(
                        "Coefficient row 3 column 2 of the matrix.",
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
            name: "P1yuvbr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUV blue coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bb",
                    description: Some(
                        "Coefficient row 3 column 3 of the matrix.",
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
                    name: "ba",
                    description: Some(
                        "Coefficient row 3 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1yuvcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "type_",
                    description: Some(
                        "Output samples type used while CLAMP is activated.",
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
                    name: "clamp",
                    description: Some(
                        "Clamp the output samples.",
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
            ],
        },
        FieldSet {
            name: "P1yuvgr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv green coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gr",
                    description: Some(
                        "Coefficient row 2 column 1 of the matrix.",
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
                    name: "gg",
                    description: Some(
                        "Coefficient row 2 column 2 of the matrix.",
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
            name: "P1yuvgr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv green coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gb",
                    description: Some(
                        "Coefficient row 2 column 3 of the matrix.",
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
                    name: "ga",
                    description: Some(
                        "Coefficient row 2 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P1yuvrr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv red coefficient register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rr",
                    description: Some(
                        "Coefficient row 1 column 1 of the matrix.",
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
                    name: "rg",
                    description: Some(
                        "Coefficient row 1 column 2 of the matrix.",
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
            name: "P1yuvrr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe1 YUVConv red coefficient register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rb",
                    description: Some(
                        "Coefficient row 1 column 3 of the matrix.",
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
                    name: "ra",
                    description: Some(
                        "Coefficient row 1 of the added column (signed integer value).",
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
            ],
        },
        FieldSet {
            name: "P2ccmricr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current common ROI configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roilsz",
                    description: Some(
                        "Current region of interest line size width.",
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
                    name: "roi1en",
                    description: Some(
                        "Current region of interest 1 enable.",
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
                    name: "roi2en",
                    description: Some(
                        "Current region of interest 2 enable.",
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
                    name: "roi3en",
                    description: Some(
                        "Current region of interest 3 enable.",
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
                    name: "roi4en",
                    description: Some(
                        "Current region of interest 4 enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "roi5en",
                    description: Some(
                        "Current region of interest 5 enable.",
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
                    name: "roi6en",
                    description: Some(
                        "Current region of interest 6 enable.",
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
                    name: "roi7en",
                    description: Some(
                        "Current region of interest 7 enable.",
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
                    name: "roi8en",
                    description: Some(
                        "Current region of interest 8 enable.",
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
            name: "P2ccrstr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current crop window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
            name: "P2ccrszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current crop window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "Current ENABLE bit value.",
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
            name: "P2cdccr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current decimation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "hdec",
                    description: Some(
                        "Horizontal decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdec",
                    description: Some(
                        "Vertical decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P2cdscr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdiv",
                    description: Some(
                        "Current horizontal division factor, from 128 (8x) to 1023 (1x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdiv",
                    description: Some(
                        "Current vertical division factor, from 128 (8x) to 1023 (1x).",
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
                    name: "enable",
                    description: Some(
                        "Current value of bit ENABLE.",
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
            name: "P2cdsrtior",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize ratio register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hratio",
                    description: Some(
                        "Current horizontal ratio, from 8192 (1x) to 65535 (8x).",
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
                Field {
                    name: "vratio",
                    description: Some(
                        "Current vertical ratio, from 8192 (1x) to 65535 (8x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P2cdsszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current downsize destination size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cfctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex current flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P2cfscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Current data type ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vc",
                    description: Some(
                        "Current flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtf",
                    description: Some(
                        "Current force data type format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtfen",
                    description: Some(
                        "Current force data type format enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Current activation of PipeN.",
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
            name: "P2cmricr",
            extends: None,
            description: Some(
                "DCMIPP Pipex common ROI configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roilsz",
                    description: Some(
                        "Region of interest line size width.",
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
                    name: "roi1en",
                    description: Some(
                        "Region of interest 1 enable.",
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
                    name: "roi2en",
                    description: Some(
                        "Region of interest 2 enable.",
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
                    name: "roi3en",
                    description: Some(
                        "Region of interest 3 enable.",
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
                    name: "roi4en",
                    description: Some(
                        "Region of interest 4 enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "roi5en",
                    description: Some(
                        "Region of interest 5 enable.",
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
                    name: "roi6en",
                    description: Some(
                        "Region of interest 6 enable.",
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
                    name: "roi7en",
                    description: Some(
                        "Region of interest 7 enable.",
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
                    name: "roi8en",
                    description: Some(
                        "Region of interest 8 enable.",
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
            name: "P2cppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "Memory format (only coplanar formats are supported in Pipe2).",
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
                    name: "swaprb",
                    description: Some(
                        "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components.",
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
                    name: "linemult",
                    description: Some(
                        "Amount of capture completed lines for LINE event and interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
                    name: "lmawm",
                    description: Some(
                        "Line multi address wrapping modulo.",
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
                    name: "lmawe",
                    description: Some(
                        "Line multi address wrapping enable bit.",
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
            ],
        },
        FieldSet {
            name: "P2cppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P2cppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P2cri1cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI1 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri1cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI1 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri2cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI2 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri2cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI2 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri3cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI3 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri3cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI3 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri4cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI4 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri4cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI4 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri5cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI5 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri5cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI5 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri6cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI6 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri6cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI6 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri7cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI7 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri7cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI7 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2cri8cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI8 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Current horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Current color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Current color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Current vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Current color line red.",
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
            ],
        },
        FieldSet {
            name: "P2cri8cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 current ROI8 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Current horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Current vertical size, from 0 to 4094 pixels high.",
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
            name: "P2crstr",
            extends: None,
            description: Some(
                "DCMIPP Pipex crop window start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
            name: "P2crszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex crop window size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide. If the value is maintained at 0 when enabling the crop by means of the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high. If the value is maintained at 0 when enabling the crop thanks to the ENABLE bit, the value is forced internally at 0xFFE, which is the maximum value.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
            name: "P2dccr",
            extends: None,
            description: Some(
                "DCMIPP Pipex decimation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
                    name: "hdec",
                    description: Some(
                        "Horizontal decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdec",
                    description: Some(
                        "Vertical decimation ratio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P2dscr",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdiv",
                    description: Some(
                        "Horizontal division factor, from 128 (8x) to 1023 (1x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vdiv",
                    description: Some(
                        "Vertical division factor, from 128 (8x) to 1023 (1x).",
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
                    name: "enable",
                    description: Some(
                        "None.",
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
            name: "P2dsrtior",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize ratio register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hratio",
                    description: Some(
                        "Horizontal ratio, from 8192 (1x) to 65535 (8x).",
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
                Field {
                    name: "vratio",
                    description: Some(
                        "Vertical ratio, from 8192 (1x) to 65535 (8x).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "P2dsszr",
            extends: None,
            description: Some(
                "DCMIPP Pipex downsize destination size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2fcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clinef",
                    description: Some(
                        "Multi-line capture complete interrupt status clear.",
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
                    name: "cframef",
                    description: Some(
                        "Frame capture complete interrupt status clear.",
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
                    name: "cvsyncf",
                    description: Some(
                        "Vertical synchronization interrupt status clear.",
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
                    name: "covrf",
                    description: Some(
                        "Overrun interrupt status clear.",
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
            name: "P2fctcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex flow control configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frate",
                    description: Some(
                        "Frame capture rate control.",
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
                    name: "cptmode",
                    description: Some(
                        "Capture mode.",
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
                    name: "cptreq",
                    description: Some(
                        "Capture requested.",
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
            ],
        },
        FieldSet {
            name: "P2fscr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 flow selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtida",
                    description: Some(
                        "Data type ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vc",
                    description: Some(
                        "Flow selection mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtf",
                    description: Some(
                        "Force data type format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdtfen",
                    description: Some(
                        "Force data type format enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pipen",
                    description: Some(
                        "Activation of PipeN.",
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
            name: "P2gmcr",
            extends: None,
            description: Some(
                "DCMIPP Pipex gamma configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "None.",
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
            ],
        },
        FieldSet {
            name: "P2ier",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lineie",
                    description: Some(
                        "Multi-line capture completed interrupt enable.",
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
                    name: "frameie",
                    description: Some(
                        "Frame capture completed interrupt enable.",
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
                    name: "vsyncie",
                    description: Some(
                        "VSYNC interrupt enable.",
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
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable.",
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
            name: "P2ppcr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 pixel packer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "Memory format (only coplanar formats are supported in Pipe2).",
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
                    name: "swaprb",
                    description: Some(
                        "Swaps R-vs-B components if RGB, and if YUV, swaps U-vs-V components.",
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
                    name: "linemult",
                    description: Some(
                        "Amount of capture completed lines for LINE event and interrupt.",
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
                    name: "dbm",
                    description: Some(
                        "Double buffer mode.",
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
                    name: "lmawm",
                    description: Some(
                        "Line multi address wrapping modulo.",
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
                    name: "lmawe",
                    description: Some(
                        "Line multi address wrapping enable bit.",
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
            ],
        },
        FieldSet {
            name: "P2ppm0ar1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 pixel packer Memory0 address register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P2ppm0ar2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 pixel packer Memory0 address register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "P2ppm0pr",
            extends: None,
            description: Some(
                "DCMIPP Pipex pixel packer Memory0 pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Number of bytes between the address of two consecutive lines.",
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
            ],
        },
        FieldSet {
            name: "P2ri1cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI1 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri1cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI1 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri2cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI2 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri2cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI2 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri3cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI3 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri3cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI3 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri4cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI4 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri4cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI4 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri5cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI5 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri5cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI5 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri6cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI6 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri6cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI6 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri7cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI7 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri7cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI7 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2ri8cr1",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI8 configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstart",
                    description: Some(
                        "Horizontal start, from 0 to 4094 pixels wide.",
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
                    name: "clb",
                    description: Some(
                        "Color line blue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clg",
                    description: Some(
                        "Color line green.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstart",
                    description: Some(
                        "Vertical start, from 0 to 4094 pixels high.",
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
                Field {
                    name: "clr",
                    description: Some(
                        "Color line red.",
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
            ],
        },
        FieldSet {
            name: "P2ri8cr2",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 ROI8 configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsize",
                    description: Some(
                        "Horizontal size, from 0 to 4094 pixels wide.",
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
                    name: "vsize",
                    description: Some(
                        "Vertical size, from 0 to 4094 pixels high.",
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
            name: "P2sr",
            extends: None,
            description: Some(
                "DCMIPP Pipe2 status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linef",
                    description: Some(
                        "Multi-line capture completed raw interrupt status.",
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
                    name: "framef",
                    description: Some(
                        "Frame capture completed raw interrupt status.",
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
                    name: "vsyncf",
                    description: Some(
                        "VSYNC raw interrupt status.",
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
                    name: "ovrf",
                    description: Some(
                        "Overrun raw interrupt status.",
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
                    name: "lstline",
                    description: Some(
                        "Last line LSB bit, sampled at frame capture complete event.",
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
                    name: "lstfrm",
                    description: Some(
                        "Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number which can be delivered by the camera through the CSI2 interface.",
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
                    name: "cptact",
                    description: Some(
                        "Capture immediate status.",
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
            name: "P2stm0ar",
            extends: None,
            description: Some(
                "DCMIPP Pipex status Memory0 address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m0a",
                    description: Some(
                        "Memory0 address.",
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
            name: "Prcr",
            extends: None,
            description: Some(
                "DCMIPP parallel interface control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ess",
                    description: Some(
                        "Embedded synchronization select.",
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
                    name: "pckpol",
                    description: Some(
                        "Pixel clock polarity.",
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
                    name: "hspol",
                    description: Some(
                        "Horizontal synchronization polarity.",
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
                    name: "vspol",
                    description: Some(
                        "Vertical synchronization polarity.",
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
                    name: "edm",
                    description: Some(
                        "Extended data mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Parallel interface enable.",
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
                    name: "format",
                    description: Some(
                        "Other values: data are captured and output as-is only through the data/dump pipeline (e.g. JPEG or byte input format).",
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
                Field {
                    name: "swapcycles",
                    description: Some(
                        "Swap data (cycle 0 vs. cycle 1) for pixels received on two cycles.",
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
                    name: "swapbits",
                    description: Some(
                        "Swap LSB vs. MSB within each received component.",
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
            ],
        },
        FieldSet {
            name: "Prescr",
            extends: None,
            description: Some(
                "DCMIPP parallel interface embedded synchronization code register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsc",
                    description: Some(
                        "Frame start delimiter code.",
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
                    name: "lsc",
                    description: Some(
                        "Line start delimiter code.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lec",
                    description: Some(
                        "Line end delimiter code.",
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
                Field {
                    name: "fec",
                    description: Some(
                        "Frame end delimiter code.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Presur",
            extends: None,
            description: Some(
                "DCMIPP parallel interface embedded synchronization unmask register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsu",
                    description: Some(
                        "Frame start delimiter unmask.",
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
                    name: "lsu",
                    description: Some(
                        "Line start delimiter unmask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "leu",
                    description: Some(
                        "Line end delimiter unmask.",
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
                Field {
                    name: "feu",
                    description: Some(
                        "Frame end delimiter unmask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Prfcr",
            extends: None,
            description: Some(
                "DCMIPP parallel interface interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cerrf",
                    description: Some(
                        "Synchronization error interrupt status clear.",
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
            ],
        },
        FieldSet {
            name: "Prier",
            extends: None,
            description: Some(
                "DCMIPP parallel interface interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errie",
                    description: Some(
                        "Synchronization error interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "Prsr",
            extends: None,
            description: Some(
                "DCMIPP parallel interface status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errf",
                    description: Some(
                        "Synchronization error raw interrupt status.",
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
                    name: "hsync",
                    description: Some(
                        "This bit gives the state of the HSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit, and cleared otherwise.",
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
                    name: "vsync",
                    description: Some(
                        "This bit gives the state of the VSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit, and cleared otherwise.",
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
            ],
        },
    ],
    enums: &[],
};
