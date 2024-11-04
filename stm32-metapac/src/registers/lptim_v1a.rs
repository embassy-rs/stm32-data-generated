
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lptim",
            extends: None,
            description: Some(
                "Low power timer with Output Compare",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "LPTIM interrupt and status register.",
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
                    name: "icr",
                    description: Some(
                        "LPTIM interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "LPTIM interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "cfgr",
                    description: Some(
                        "LPTIM configuration register.",
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
                    name: "cr",
                    description: Some(
                        "LPTIM control register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "cmp",
                    description: Some(
                        "LPTIM compare register 1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "arr",
                    description: Some(
                        "LPTIM autoreload register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Arr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "LPTIM counter register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "LPTIM option register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Arr",
            extends: None,
            description: Some(
                "LPTIM autoreload register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arr",
                    description: Some(
                        "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx[15:0] value.",
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
            name: "Cfgr",
            extends: None,
            description: Some(
                "LPTIM configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cksel",
                    description: Some(
                        "Clock selector The CKSEL bit selects which clock source the LPTIM uses:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClockSource",
                    ),
                },
                Field {
                    name: "ckpol",
                    description: Some(
                        "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckpol",
                    ),
                },
                Field {
                    name: "ckflt",
                    description: Some(
                        "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Filter",
                    ),
                },
                Field {
                    name: "trgflt",
                    description: Some(
                        "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Filter",
                    ),
                },
                Field {
                    name: "presc",
                    description: Some(
                        "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Presc",
                    ),
                },
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details.",
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
                    name: "trigen",
                    description: Some(
                        "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Trigen",
                    ),
                },
                Field {
                    name: "timout",
                    description: Some(
                        "Timeout enable The TIMOUT bit controls the Timeout feature.",
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
                    name: "wave",
                    description: Some(
                        "Waveform shape The WAVE bit controls the output shape.",
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
                    name: "wavpol",
                    description: Some(
                        "Waveform shape polarity The WAVEPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wavpol",
                    ),
                },
                Field {
                    name: "preload",
                    description: Some(
                        "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality.",
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
                    name: "countmode",
                    description: Some(
                        "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ClockSource",
                    ),
                },
                Field {
                    name: "enc",
                    description: Some(
                        "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
            name: "Cmp",
            extends: None,
            description: Some(
                "LPTIM compare register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output.",
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
            name: "Cnt",
            extends: None,
            description: Some(
                "LPTIM counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "LPTIM control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "LPTIM enable The ENABLE bit is set and cleared by software.",
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
                    name: "sngstrt",
                    description: Some(
                        "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware.",
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
                    name: "cntstrt",
                    description: Some(
                        "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN[1:0] = ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN[1:0] different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware.",
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
            name: "Icr",
            extends: None,
            description: Some(
                "LPTIM interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cccf",
                    description: Some(
                        "Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.",
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
                                stride: 9,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrmcf",
                    description: Some(
                        "Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register.",
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
                    name: "exttrigcf",
                    description: Some(
                        "External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register.",
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
                    name: "cmpokcf",
                    description: Some(
                        "Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrokcf",
                    description: Some(
                        "Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register.",
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
                    name: "upcf",
                    description: Some(
                        "Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
                    name: "downcf",
                    description: Some(
                        "Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
            name: "Ier",
            extends: None,
            description: Some(
                "LPTIM interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccie",
                    description: Some(
                        "Capture/compare 1 interrupt enable.",
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
                                stride: 9,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrmie",
                    description: Some(
                        "Autoreload match Interrupt Enable.",
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
                    name: "exttrigie",
                    description: Some(
                        "External trigger valid edge Interrupt Enable.",
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
                    name: "cmpokie",
                    description: Some(
                        "Compare register 1 update OK interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrokie",
                    description: Some(
                        "Autoreload register update OK Interrupt Enable.",
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
                    name: "upie",
                    description: Some(
                        "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
                    name: "downie",
                    description: Some(
                        "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
            name: "Isr",
            extends: None,
            description: Some(
                "LPTIM interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccif",
                    description: Some(
                        "Compare 1 interrupt flag The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register.",
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
                                stride: 9,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrm",
                    description: Some(
                        "Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.",
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
                    name: "exttrig",
                    description: Some(
                        "External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.",
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
                    name: "cmpok",
                    description: Some(
                        "Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "arrok",
                    description: Some(
                        "Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.",
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
                    name: "up",
                    description: Some(
                        "Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
                    name: "down",
                    description: Some(
                        "Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to.",
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
    ],
    enums: &[
        Enum {
            name: "Ckpol",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RISING",
                    description: Some(
                        "the rising edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLING",
                    description: Some(
                        "the falling edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BOTH",
                    description: Some(
                        "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "ClockSource",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "INTERNAL",
                    description: Some(
                        "clocked by internal clock source (APB clock or any of the embedded oscillators)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "clocked by an external clock source through the LPTIM external Input1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Filter",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "COUNT1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "COUNT2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "COUNT4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "COUNT8",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Presc",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "DIV32",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "DIV64",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "DIV128",
                    description: None,
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Trigen",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "software trigger (counting start is initiated by software)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISINGEDGE",
                    description: Some(
                        "rising edge is the active edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "falling edge is the active edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTHEDGE",
                    description: Some(
                        "both edges are active edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Wavpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "POSITIVE",
                    description: Some(
                        "The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NEGATIVE",
                    description: Some(
                        "The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
