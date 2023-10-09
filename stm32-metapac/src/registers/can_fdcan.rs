
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fdcan",
            extends: None,
            description: Some(
                "FDCAN",
            ),
            items: &[
                BlockItem {
                    name: "crel",
                    description: Some(
                        "FDCAN Core Release Register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Crel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "endn",
                    description: Some(
                        "FDCAN Core Release Register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Endn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbtp",
                    description: Some(
                        "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) [DTSEG1 + DTSEG2 + 3] tq or (functional values) [Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2] tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test",
                    description: Some(
                        "Write access to the Test Register has to be enabled by setting bit CCCR[TEST] to 1 . All Test Register functions are set to their reset values when bit CCCR[TEST] is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Test",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwd",
                    description: Some(
                        "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD[WDC] bits. The counter is reloaded with RWD[WDC] bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR[WDI] bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "For details about setting and resetting of single bits see Software initialization.",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nbtp",
                    description: Some(
                        "FDCAN_NBTP",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscc",
                    description: Some(
                        "FDCAN Timestamp Counter Configuration Register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscv",
                    description: Some(
                        "FDCAN Timestamp Counter Value Register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocc",
                    description: Some(
                        "FDCAN Timeout Counter Configuration Register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocv",
                    description: Some(
                        "FDCAN Timeout Counter Value Register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "FDCAN Error Counter Register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psr",
                    description: Some(
                        "FDCAN Protocol Status Register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdcr",
                    description: Some(
                        "FDCAN Transmitter Delay Compensation Register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ir",
                    description: Some(
                        "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ie",
                    description: Some(
                        "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ils",
                    description: Some(
                        "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE[EINT0] and ILE[EINT1].",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ils",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ile",
                    description: Some(
                        "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.",
                    ),
                    array: None,
                    byte_offset: 92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ile",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxgfc",
                    description: Some(
                        "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxgfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xidam",
                    description: Some(
                        "FDCAN Extended ID and Mask Register",
                    ),
                    array: None,
                    byte_offset: 132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xidam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpms",
                    description: Some(
                        "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpms",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxfs",
                    description: Some(
                        "FDCAN Rx FIFO X Status Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxfs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxfa",
                    description: Some(
                        "CAN Rx FIFO 0 Acknowledge Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxfa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbc",
                    description: Some(
                        "FDCAN Tx Buffer Configuration Register",
                    ),
                    array: None,
                    byte_offset: 192,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txfqs",
                    description: Some(
                        "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).",
                    ),
                    array: None,
                    byte_offset: 196,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Txfqs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbrp",
                    description: Some(
                        "FDCAN Tx Buffer Request Pending Register",
                    ),
                    array: None,
                    byte_offset: 200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbar",
                    description: Some(
                        "FDCAN Tx Buffer Add Request Register",
                    ),
                    array: None,
                    byte_offset: 204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcr",
                    description: Some(
                        "FDCAN Tx Buffer Cancellation Request Register",
                    ),
                    array: None,
                    byte_offset: 208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbto",
                    description: Some(
                        "FDCAN Tx Buffer Transmission Occurred Register",
                    ),
                    array: None,
                    byte_offset: 212,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcf",
                    description: Some(
                        "FDCAN Tx Buffer Cancellation Finished Register",
                    ),
                    array: None,
                    byte_offset: 216,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbtie",
                    description: Some(
                        "FDCAN Tx Buffer Transmission Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbtie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcie",
                    description: Some(
                        "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefs",
                    description: Some(
                        "FDCAN Tx Event FIFO Status Register",
                    ),
                    array: None,
                    byte_offset: 228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefa",
                    description: Some(
                        "FDCAN Tx Event FIFO Acknowledge Register",
                    ),
                    array: None,
                    byte_offset: 232,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckdiv",
                    description: Some(
                        "FDCAN CFG clock divider register",
                    ),
                    array: None,
                    byte_offset: 256,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckdiv",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Crel",
            extends: None,
            description: Some(
                "FDCAN Core Release Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "day",
                    description: Some(
                        "DAY",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mon",
                    description: Some(
                        "MON",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "year",
                    description: Some(
                        "YEAR",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "substep",
                    description: Some(
                        "SUBSTEP",
                    ),
                    bit_offset: 20,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "step",
                    description: Some(
                        "STEP",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rel",
                    description: Some(
                        "REL",
                    ),
                    bit_offset: 28,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some(
                "For details about setting and resetting of single bits see Software initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "init",
                    description: Some(
                        "INIT",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cce",
                    description: Some(
                        "CCE",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asm",
                    description: Some(
                        "ASM",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csa",
                    description: Some(
                        "CSA",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csr",
                    description: Some(
                        "CSR",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mon",
                    description: Some(
                        "MON",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dar",
                    description: Some(
                        "DAR",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "test",
                    description: Some(
                        "TEST",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdoe",
                    description: Some(
                        "FDOE",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brse",
                    description: Some(
                        "BRSE",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pxhd",
                    description: Some(
                        "PXHD",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efbi",
                    description: Some(
                        "EFBI",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txp",
                    description: Some(
                        "TXP",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "niso",
                    description: Some(
                        "NISO",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ile",
            extends: None,
            description: Some(
                "Each of the two interrupt lines to the CPU can be enabled/disabled separately by programming bits EINT0 and EINT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eint0",
                    description: Some(
                        "EINT0",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eint1",
                    description: Some(
                        "EINT1",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Xidam",
            extends: None,
            description: Some(
                "FDCAN Extended ID and Mask Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eidm",
                    description: Some(
                        "EIDM",
                    ),
                    bit_offset: 0,
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txefa",
            extends: None,
            description: Some(
                "FDCAN Tx Event FIFO Acknowledge Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efai",
                    description: Some(
                        "EFAI",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dbtp",
            extends: None,
            description: Some(
                "This register is only writable if bits CCCR.CCE and CCCR.INIT are set. The CAN bit time may be programed in the range of 4 to 25 time quanta. The CAN time quantum may be programmed in the range of 1 to 1024 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock period. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (programmed values) [DTSEG1 + DTSEG2 + 3] tq or (functional values) [Sync_Seg + Prop_Seg + Phase_Seg1 + Phase_Seg2] tq. The Information Processing Time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsjw",
                    description: Some(
                        "DSJW",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtseg2",
                    description: Some(
                        "DTSEG2",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtseg1",
                    description: Some(
                        "DTSEG1",
                    ),
                    bit_offset: 8,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbrp",
                    description: Some(
                        "DBRP",
                    ),
                    bit_offset: 16,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdc",
                    description: Some(
                        "TDC",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tdcr",
            extends: None,
            description: Some(
                "FDCAN Transmitter Delay Compensation Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdcf",
                    description: Some(
                        "TDCF",
                    ),
                    bit_offset: 0,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdco",
                    description: Some(
                        "TDCO",
                    ),
                    bit_offset: 8,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbcf",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Cancellation Finished Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cf",
                    description: Some(
                        "CF",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tocv",
            extends: None,
            description: Some(
                "FDCAN Timeout Counter Value Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toc",
                    description: Some(
                        "TOC",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbc",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbsa",
                    description: Some(
                        "TBSA",
                    ),
                    bit_offset: 2,
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ndtb",
                    description: Some(
                        "NDTB",
                    ),
                    bit_offset: 16,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqs",
                    description: Some(
                        "TFQS",
                    ),
                    bit_offset: 24,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqm",
                    description: Some(
                        "TFQM",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxfa",
            extends: None,
            description: Some(
                "CAN Rx FIFO X Acknowledge Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fai",
                    description: Some(
                        "FAI",
                    ),
                    bit_offset: 0,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbrp",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Request Pending Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trp",
                    description: Some(
                        "TRP",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txfqs",
            extends: None,
            description: Some(
                "The Tx FIFO/Queue status is related to the pending Tx requests listed in register TXBRP. Therefore the effect of Add/Cancellation requests may be delayed due to a running Tx scan (TXBRP not yet updated).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tffl",
                    description: Some(
                        "TFFL",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfgi",
                    description: Some(
                        "TFGI",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqpi",
                    description: Some(
                        "TFQPI",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqf",
                    description: Some(
                        "TFQF",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ie",
            extends: None,
            description: Some(
                "The settings in the Interrupt Enable register determine which status changes in the Interrupt Register will be signaled on an interrupt line.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfne",
                    description: Some(
                        "Rx FIFO X new message enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rffe",
                    description: Some(
                        "Rx FIFO X full enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rfle",
                    description: Some(
                        "Rx FIFO X message lost enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hpme",
                    description: Some(
                        "High-priority message enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tce",
                    description: Some(
                        "Transmission completed enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcfe",
                    description: Some(
                        "Transmission cancellation finished enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfee",
                    description: Some(
                        "Tx FIFO empty enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefne",
                    description: Some(
                        "Tx even FIFO new entry enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teffe",
                    description: Some(
                        "Tx event FIFO full enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefle",
                    description: Some(
                        "Tx event FIFO element lost enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tswe",
                    description: Some(
                        "Timestamp wraparound enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrafe",
                    description: Some(
                        "Message RAM access failure enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tooe",
                    description: Some(
                        "Timeout occurred enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eloe",
                    description: Some(
                        "Error logging overflow enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epe",
                    description: Some(
                        "Error passive enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ewe",
                    description: Some(
                        "Warning status enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boe",
                    description: Some(
                        "Bus_off status enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdie",
                    description: Some(
                        "Watchdog interrupt enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "peae",
                    description: Some(
                        "Protocol error in arbitration phase enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pede",
                    description: Some(
                        "Protocol error in data phase enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "arae",
                    description: Some(
                        "Access to reserved address enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ils",
            extends: None,
            description: Some(
                "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE[EINT0] and ILE[EINT1].",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfifo",
                    description: Some(
                        "RX FIFO bit grouping the following interruption",
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
                    name: "smsg",
                    description: Some(
                        "Status message bit grouping the following interruption",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tferr",
                    description: Some(
                        "TX FIFO error grouping the following interruption",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "misc",
                    description: Some(
                        "Interrupt regrouping the following interruption",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "berr",
                    description: Some(
                        "Bit and line error grouping the following interruption",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "perr",
                    description: Some(
                        "Protocol error grouping the following interruption",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Psr",
            extends: None,
            description: Some(
                "FDCAN Protocol Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lec",
                    description: Some(
                        "LEC",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "act",
                    description: Some(
                        "ACT",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep",
                    description: Some(
                        "EP",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ew",
                    description: Some(
                        "EW",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bo",
                    description: Some(
                        "BO",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlec",
                    description: Some(
                        "DLEC",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "resi",
                    description: Some(
                        "RESI",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rbrs",
                    description: Some(
                        "RBRS",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "redl",
                    description: Some(
                        "REDL",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pxe",
                    description: Some(
                        "PXE",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdcv",
                    description: Some(
                        "TDCV",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbtie",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Transmission Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "TIE",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbar",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Add Request Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ar",
                    description: Some(
                        "AR",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecr",
            extends: None,
            description: Some(
                "FDCAN Error Counter Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tec",
                    description: Some(
                        "TEC",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rec",
                    description: Some(
                        "TREC",
                    ),
                    bit_offset: 8,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rp",
                    description: Some(
                        "RP",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cel",
                    description: Some(
                        "CEL",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hpms",
            extends: None,
            description: Some(
                "This register is updated every time a Message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidx",
                    description: Some(
                        "BIDX",
                    ),
                    bit_offset: 0,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msi",
                    description: Some(
                        "MSI",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fidx",
                    description: Some(
                        "FIDX",
                    ),
                    bit_offset: 8,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flst",
                    description: Some(
                        "FLST",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Test",
            extends: None,
            description: Some(
                "Write access to the Test Register has to be enabled by setting bit CCCR[TEST] to 1 . All Test Register functions are set to their reset values when bit CCCR[TEST] is reset. Loop Back mode and software control of Tx pin FDCANx_TX are hardware test modes. Programming TX differently from 00 may disturb the message transfer on the CAN bus.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lbck",
                    description: Some(
                        "LBCK",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx",
                    description: Some(
                        "TX",
                    ),
                    bit_offset: 5,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx",
                    description: Some(
                        "RX",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ckdiv",
            extends: None,
            description: Some(
                "FDCAN CFG clock divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdiv",
                    description: Some(
                        "input clock divider. the APB clock could be divided prior to be used by the CAN sub",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tocc",
            extends: None,
            description: Some(
                "FDCAN Timeout Counter Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etoc",
                    description: Some(
                        "ETOC",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tos",
                    description: Some(
                        "TOS",
                    ),
                    bit_offset: 1,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "top",
                    description: Some(
                        "TOP",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tscv",
            extends: None,
            description: Some(
                "FDCAN Timestamp Counter Value Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsc",
                    description: Some(
                        "TSC",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxgfc",
            extends: None,
            description: Some(
                "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rrfe",
                    description: Some(
                        "RRFE",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrfs",
                    description: Some(
                        "RRFS",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "anfe",
                    description: Some(
                        "ANFE",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "anfs",
                    description: Some(
                        "ANFS",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1om",
                    description: Some(
                        "FIFO 1 operation mode",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0om",
                    description: Some(
                        "FIFO 0 operation mode",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lss",
                    description: Some(
                        "List size standard",
                    ),
                    bit_offset: 16,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lse",
                    description: Some(
                        "List size extended",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tscc",
            extends: None,
            description: Some(
                "FDCAN Timestamp Counter Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tss",
                    description: Some(
                        "TSS",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcp",
                    description: Some(
                        "TCP",
                    ),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbcr",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Cancellation Request Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "CR",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwd",
            extends: None,
            description: Some(
                "The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD[WDC] bits. The counter is reloaded with RWD[WDC] bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR[WDI] bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdc",
                    description: Some(
                        "WDC",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdv",
                    description: Some(
                        "WDV",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txefs",
            extends: None,
            description: Some(
                "FDCAN Tx Event FIFO Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "effl",
                    description: Some(
                        "EFFL",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efgi",
                    description: Some(
                        "EFGI",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efpi",
                    description: Some(
                        "EFPI",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eff",
                    description: Some(
                        "EFF",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefl",
                    description: Some(
                        "TEFL",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ir",
            extends: None,
            description: Some(
                "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfn",
                    description: Some(
                        "Rx FIFO X new message",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rff",
                    description: Some(
                        "Rx FIFO X full",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rfl",
                    description: Some(
                        "Rx FIFO X message lost",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "hpm",
                    description: Some(
                        "High-priority message",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tc",
                    description: Some(
                        "Transmission completed",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcf",
                    description: Some(
                        "Transmission cancellation finished",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfe",
                    description: Some(
                        "Tx FIFO empty",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefn",
                    description: Some(
                        "Tx even FIFO new entry",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teff",
                    description: Some(
                        "Tx event FIFO full",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefl",
                    description: Some(
                        "Tx event FIFO element lost",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsw",
                    description: Some(
                        "Timestamp wraparound",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mraf",
                    description: Some(
                        "Message RAM access failure",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "too",
                    description: Some(
                        "Timeout occurred",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "elo",
                    description: Some(
                        "Error logging overflow",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep",
                    description: Some(
                        "Error passive",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ew",
                    description: Some(
                        "Warning status",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bo",
                    description: Some(
                        "Bus_off status",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdi",
                    description: Some(
                        "Watchdog interrupt",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pea",
                    description: Some(
                        "Protocol error in arbitration phase",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ped",
                    description: Some(
                        "Protocol error in data phase",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ara",
                    description: Some(
                        "Access to reserved address",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxfs",
            extends: None,
            description: Some(
                "FDCAN Rx FIFO X Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ffl",
                    description: Some(
                        "FFL",
                    ),
                    bit_offset: 0,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgi",
                    description: Some(
                        "FGI",
                    ),
                    bit_offset: 8,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpi",
                    description: Some(
                        "FPI",
                    ),
                    bit_offset: 16,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ff",
                    description: Some(
                        "FF",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfl",
                    description: Some(
                        "RFL",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Endn",
            extends: None,
            description: Some(
                "FDCAN Core Release Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etv",
                    description: Some(
                        "ETV",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nbtp",
            extends: None,
            description: Some(
                "FDCAN_NBTP",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ntseg2",
                    description: Some(
                        "TSEG2",
                    ),
                    bit_offset: 0,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ntseg1",
                    description: Some(
                        "NTSEG1",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbrp",
                    description: Some(
                        "NBRP",
                    ),
                    bit_offset: 16,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsjw",
                    description: Some(
                        "NSJW",
                    ),
                    bit_offset: 25,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbto",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Transmission Occurred Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to",
                    description: Some(
                        "TO",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txbcie",
            extends: None,
            description: Some(
                "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfie",
                    description: Some(
                        "CFIE",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
