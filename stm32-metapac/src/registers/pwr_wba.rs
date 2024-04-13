
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    byte_offset: 0x4,
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
                    name: "cr3",
                    description: Some(
                        "control register 3",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vosr",
                    description: Some(
                        "voltage scaling register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vosr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "svmcr",
                    description: Some(
                        "supply voltage monitoring control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr1",
                    description: Some(
                        "wakeup control register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr2",
                    description: Some(
                        "wakeup control register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr3",
                    description: Some(
                        "wakeup control register 3",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbpcr",
                    description: Some(
                        "disable Backup domain register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "security configuration register",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "privcfgr",
                    description: Some(
                        "privilege control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x38,
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
                    name: "svmsr",
                    description: Some(
                        "supply voltage monitoring status register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Svmsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wusr",
                    description: Some(
                        "wakeup status register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wusr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wuscr",
                    description: Some(
                        "wakeup status clear register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wuscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioretenr",
                    description: Some(
                        "port Standby IO retention enable register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioretenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioretra",
                    description: Some(
                        "port Standby IO retention status register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioretr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "radioscr",
                    description: Some(
                        "2.4 GHz RADIO status and control register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Radioscr",
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
                    name: "lpms",
                    description: Some(
                        "Low-power mode selection\r These bits select the low-power mode entered when the CPU enters the SleepDeep mode.\r 10x: Standby mode\r others reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpms",
                    ),
                },
                Field {
                    name: "r2rsb1",
                    description: Some(
                        "SRAM2 retention in Standby mode\r This bit is used to keep the SRAM2 content in Standby retention mode.",
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
                    name: "ulpmen",
                    description: Some(
                        "BOR0 ultra-low-power mode. \r This bit is used to reduce the consumption by configuring the BOR0 in discontinuous mode for Stop 1 and Standby modes. Discontinuous mode is only available when BOR levels 1 to 4 and PVD are disabled.\r Note: This bit must be set to reach the lowest power consumption in the low-power modes.\r Note: This bit must not be set together with autonomous peripherals using HSI as kernel clock.\r Note: When BOR level 1 to 4 or PVD is enabled continuous mode applies independent from ULPMEN.",
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
                    name: "radiorsb",
                    description: Some(
                        "2.4 GHz RADIO SRAMs (RXTXRAM and Sequence RAM) and Sleep clock retention in Standby mode.\r This bit is used to keep the 2.4 GHz RADIO SRAMs content in Standby retention mode and the 2.4 GHz RADIO sleep timer counter operational.",
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
                    name: "r1rsb1",
                    description: Some(
                        "SRAM1 retention in Standby mode\r This bit is used to keep the SRAM1 content in Standby retention mode.",
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
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1pds1",
                    description: Some(
                        "SRAM1 power-down in Stop modes (Stop 0, 1)\r Note: The SRAM1 retention in Standby mode is controlled by R1RSB1 bit in CR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampds",
                    ),
                },
                Field {
                    name: "sram2pds1",
                    description: Some(
                        "SRAM2 power-down in Stop modes (Stop 0, 1)\r Note: The SRAM2 retention in Standby mode is controlled by R2RSB1 bit in CR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Srampds",
                    ),
                },
                Field {
                    name: "icrampds",
                    description: Some(
                        "ICACHE SRAM power-down in Stop modes (Stop 0, 1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Icrampds",
                    ),
                },
                Field {
                    name: "flashfwu",
                    description: Some(
                        "Flash memory fast wakeup from Stop modes (Stop 0, 1)\r This bit is used to obtain the best trade-off between low-power consumption and wakeup time when exiting the Stop 0 or Stop 1 modes.\r When this bit is set, the Flash memory remains in normal mode in Stop 0 and Stop 1 modes, which offers a faster startup time with higher consumption.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Flashfwu",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsten",
                    description: Some(
                        "Fast soft start",
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
            name: "Dbpcr",
            extends: None,
            description: Some(
                "disable Backup domain register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable Backup domain write protection\r In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.",
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
            name: "Ioretenr",
            extends: None,
            description: Some(
                "port A Standby IO retention enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Port A Standby GPIO retention enable\r Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV.\r When set, each bit enables the Standby GPIO retention feature for PAy",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioretr",
            extends: None,
            description: Some(
                "port A Standby IO retention status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ret",
                    description: Some(
                        "Port A Standby GPIO retention active\r Access can be protected by GPIOA SECy, privilege protection is controlled by SPRIV or NSPRIV.",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "privilege control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "secure functions privilege configuration\r This bit is set and reset by software.\r It can be written only by a secure privileged access.",
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
                    name: "nspriv",
                    description: Some(
                        "non-secure functions privilege configuration\r This bit is set and reset by software.\r It can be written only by privileged access, secure or non-secure.",
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
            ],
        },
        FieldSet {
            name: "Radioscr",
            extends: None,
            description: Some(
                "2.4 GHz RADIO status and control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "2.4 GHz RADIO operating mode.\r 1x: 2.4 GHz RADIO active mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "phymode",
                    description: Some(
                        "2.4 GHz RADIO PHY operating mode",
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
                    name: "encmode",
                    description: Some(
                        "2.4 GHz RADIO encryption function operating mode",
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
                    name: "rfvddhpa",
                    description: Some(
                        "2.4 GHz RADIO VDDHPA control word.\r Bits [3:0] see Table 81: PA output power table format for definition.\r Bit [4] rf_event.",
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
                    name: "regpardyvddrfpa",
                    description: Some(
                        "Ready bit for V<sub>DDHPA</sub> voltage level when selecting VDDRFPA input.\r Note: REGPARDYVDDRFPA does not allow to detect correct V<sub>DDHPA</sub> voltage level when request to lower the level.",
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
            name: "Seccfgr",
            extends: None,
            description: Some(
                "security configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wup1sec",
                    description: Some(
                        "WUP1 secure protection",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lpmsec",
                    description: Some(
                        "Low-power modes secure protection",
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
                    name: "vdmsec",
                    description: Some(
                        "Voltage detection secure protection",
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
                    name: "vbsec",
                    description: Some(
                        "Backup domain secure protection",
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
                    name: "cssf",
                    description: Some(
                        "Clear Stop and Standby flags\r Access can be secured by LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.\r Writing 1 to this bit clears the STOPF and SBF flags.",
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
                    name: "stopf",
                    description: Some(
                        "Stop flag\r This bit is set by hardware when the device enters a Stop or Standby mode at the same time as the sysclk has been set by hardware to select HSI. It’s cleared by software by writing 1 to the CSSF bit and by hardware when SBF is set.",
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
                    name: "sbf",
                    description: Some(
                        "Standby flag\r This bit is set by hardware when the device enters the Standby mode and the CPU restart from its reset vector. It’s cleared by writing 1 to the CSSF bit, or by a power-on reset. It is not cleared by the system reset.",
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
            name: "Svmcr",
            extends: None,
            description: Some(
                "supply voltage monitoring control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "Programmable voltage detector enable",
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
                    name: "pvdls",
                    description: Some(
                        "Programmable voltage detector level selection\r These bits select the voltage threshold detected by the programmable voltage detector:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pvdls",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Svmsr",
            extends: None,
            description: Some(
                "supply voltage monitoring status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvdo",
                    description: Some(
                        "Programmable voltage detector output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pvdo",
                    ),
                },
                Field {
                    name: "actvosrdy",
                    description: Some(
                        "Voltage level ready for currently used VOS",
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
                    name: "actvos",
                    description: Some(
                        "VOS currently applied to V<sub>CORE</sub>\r This field provides the last VOS value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Actvos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vosr",
            extends: None,
            description: Some(
                "voltage scaling register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vosrdy",
                    description: Some(
                        "Ready bit for V<sub>CORE</sub> voltage scaling output selection\r Set and cleared by hardware. When decreasing the voltage scaling range, VOSRDY must be one before increasing the SYSCLK frequency.",
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
                    name: "vos",
                    description: Some(
                        "Voltage scaling range selection\r Set a and cleared by software.\r Cleared by hardware when entering Stop 1 mode.\r Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wucr1",
            extends: None,
            description: Some(
                "wakeup control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupen",
                    description: Some(
                        "Wakeup and interrupt pin WKUP1 enable\r Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wucr2",
            extends: None,
            description: Some(
                "wakeup control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupp",
                    description: Some(
                        "Wakeup pin WKUP1 polarity.\r This bit must be configured when WUPEN1 = 0.\r Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wupp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wucr3",
            extends: None,
            description: Some(
                "wakeup control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wusel1",
                    description: Some(
                        "Wakeup and interrupt pin WKUP1 selection\r This field must be configured when WUPEN1 = 0.\r Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel2",
                    description: Some(
                        "Wakeup and interrupt pin WKUP2 selection\r This field must be configured when WUPEN2 = 0.\r Access can be secured by WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel3",
                    description: Some(
                        "Wakeup and interrupt pin WKUP3 selection\r This field must be configured when WUPEN3 = 0.\r Access can be secured by WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel4",
                    description: Some(
                        "Wakeup and interrupt pin WKUP4 selection\r This field must be configured when WUPEN4 = 0.\r Access can be secured by WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel5",
                    description: Some(
                        "Wakeup and interrupt pin WKUP5 selection\r This field must be configured when WUPEN5 = 0.\r Access can be secured by WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel6",
                    description: Some(
                        "Wakeup and interrupt pin WKUP6 selection\r This field must be configured when WUPEN6 = 0.\r Access can be secured by WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel7",
                    description: Some(
                        "Wakeup and interrupt pin WKUP7 selection\r This field must be configured when WUPEN7 = 0.\r Access can be secured by WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
                Field {
                    name: "wusel8",
                    description: Some(
                        "Wakeup and interrupt pin WKUP8 selection\r This field must be configured when WUPEN8 = 0.\r Access can be secured by WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Wusel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wuscr",
            extends: None,
            description: Some(
                "wakeup status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some(
                        "Clear wakeup flag 1\r Access can be secured by WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.\r Writing 1 to this bit clears the WUF1 flag in WUSR.",
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
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wusr",
            extends: None,
            description: Some(
                "wakeup status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some(
                        "Wakeup and interrupt pending flag 1\r This bit is set when a wakeup event is detected on WKUP1 pin. This bit is cleared by writing 1 in the CWUF1 bit of WUSCR or by hardware when WUPEN1 = 0.",
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
                                len: 8,
                                stride: 1,
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
            name: "Actvos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Range 2 (lowest power)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Range 1 (highest frequency)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Flashfwu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOWPOWER",
                    description: Some(
                        "Flash memory enters low-power mode in Stop 0 and Stop 1 modes (lower-power consumption).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Flash memory remains in normal mode in Stop 0 and Stop 1 modes (faster wakeup time).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Icrampds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RETAINED",
                    description: Some(
                        "ICACHE SRAM content retained in Stop modes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOTRETAINED",
                    description: Some(
                        "ICACHE SRAM content lost in Stop modes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpms",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "STOP0",
                    description: Some(
                        "Stop 0 mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOP1",
                    description: Some(
                        "Stop 1 mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DEEPSLEEP",
                    description: Some(
                        "2.4 GHz RADIO deep sleep mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SLEEP",
                    description: Some(
                        "2.4 GHz RADIO sleep mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pvdls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "V20",
                    description: Some(
                        "VPVD0 around 2.0 V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V22",
                    description: Some(
                        "VPVD1 around 2.2 V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "V24",
                    description: Some(
                        "VPVD2 around 2.4 V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "V25",
                    description: Some(
                        "VPVD3 around 2.5 V",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "V26",
                    description: Some(
                        "VPVD4 around 2.6 V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V28",
                    description: Some(
                        "VPVD5 around 2.8 V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V29",
                    description: Some(
                        "VPVD6 around 2.9 V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PVD_IN",
                    description: Some(
                        "External input analog voltage PVD_IN (compared internally to VREFINT)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pvdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ABOVEOREQUAL",
                    description: Some(
                        "VDD is equal or above the PVD threshold selected by PVDLS[2:0].",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BELOW",
                    description: Some(
                        "VDD is below the PVD threshold selected by PVDLS[2:0].",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Srampds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "POWEREDON",
                    description: Some(
                        "SRAM1 content retained in Stop modes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POWEREDOFF",
                    description: Some(
                        "SRAM1 content lost in Stop modes",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Range 2 (lowest power)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Range 1 (highest frequency).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wupp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Detection on high level (rising edge)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Detection on low level (falling edge)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wusel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "reserved",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "WKUP3_1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "WKUP3_2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "reserved",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                