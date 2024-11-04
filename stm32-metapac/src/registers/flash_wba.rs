
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "Embedded memory",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "access control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nskeyr",
                    description: Some(
                        "key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "seckeyr",
                    description: Some(
                        "secure key register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "option key register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "pdkeyr",
                    description: Some(
                        "power-down key register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "nssr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secsr",
                    description: Some(
                        "secure status register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nscr1",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nscr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccr1",
                    description: Some(
                        "secure control register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccr",
                    description: Some(
                        "ECC register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "opsr",
                    description: Some(
                        "operation status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nscr2",
                    description: Some(
                        "control 2 register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nscr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccr2",
                    description: Some(
                        "secure control 2 register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optr",
                    description: Some(
                        "option register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsbootadd0r",
                    description: Some(
                        "boot address 0 register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsbootadd0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsbootadd1r",
                    description: Some(
                        "boot address 1 register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsbootadd1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbootadd0r",
                    description: Some(
                        "secure boot address 0 register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbootadd0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwmr1",
                    description: Some(
                        "secure watermark register 1",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwmr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwmr2",
                    description: Some(
                        "secure watermark register 2",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwmr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpar",
                    description: Some(
                        "WRP area A address register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpbr",
                    description: Some(
                        "WRP area B address register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oem1keyr1",
                    description: Some(
                        "OEM1 key register 1",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "oem1keyr2",
                    description: Some(
                        "OEM1 key register 2",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "oem2keyr1",
                    description: Some(
                        "OEM2 key register 1",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "oem2keyr2",
                    description: Some(
                        "OEM2 key register 2",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "secbbr",
                    description: Some(
                        "secure block based register 1",
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
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sechdpcr",
                    description: Some(
                        "secure HDP control register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sechdpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prifcfgr",
                    description: Some(
                        "privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Prifcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbbr",
                    description: Some(
                        "privilege block based register 1",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bbr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Latency\r These bits represent the ratio between the AHB hclk1 clock period and the memory access time.\r Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.\r ...\r Note: Before entering Stop 1 mode software must set wait state latency to at least 1.",
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
                    name: "prften",
                    description: Some(
                        "Prefetch enable\r This bit enables the prefetch buffer in the embedded memory.\r This bit can be protected against unprivileged access by NSPRIV.",
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
                    name: "lpm",
                    description: Some(
                        "Low-power read mode\r This bit puts the memory in low-power read mode.\r Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.\r This bit can’t be written when a program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a program or erase operation is busy (BSY = 1) is rejected.",
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
                    name: "pdreq",
                    description: Some(
                        "power-down mode request\r This bit requests to enter power-down mode. When enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked.\r This bit is write-protected with PDKEYR. \r Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.",
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
                    name: "sleep_pd",
                    description: Some(
                        "memory power-down mode during Sleep mode\r This bit determines whether the memory is in power-down mode or Idle mode when the device is in Sleep mode.\r Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV.\r The must not be put in power-down while a program or an erase operation is ongoing.",
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
            name: "Bbr",
            extends: None,
            description: Some(
                "block based register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "block",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccr",
            extends: None,
            description: Some(
                "ECC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC fail address\r This field indicates which address is concerned by the ECC error correction or by the double ECC error detection. The address is given relative to base address, from offset 0x0�0000 to 0xF�FFF0.\r Note that bit 19 is reserved on STM32WBAxEx devices.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_ecc",
                    description: Some(
                        "System memory ECC fail\r This bit indicates that the ECC error correction or double ECC error detection is located in the system memory.",
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
                    name: "eccie",
                    description: Some(
                        "ECC correction interrupt enable\r This bit enables the interrupt generation when the ECCC bit in the ECCR register is set.",
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
                    name: "eccc",
                    description: Some(
                        "ECC correction\r This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.",
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
                    name: "eccd",
                    description: Some(
                        "ECC detection\r This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.",
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
            name: "Nsbootadd0r",
            extends: None,
            description: Some(
                "boot address 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsbootadd0",
                    description: Some(
                        "Non-secure boot base address 0\r This address is only used when TZEN = 0.\r The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address [31:7]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state.\r Examples:\r NSBOOTADD0[24:0] = 0x0100000: Boot from memory (0x0800 0000)\r NSBOOTADD0[24:0] = 0x017F100: Boot from system memory bootloader (0x0BF8 8000)\r NSBOOTADD0[24:0] = 0x0400200: Boot from SRAM2 on S-Bus (0x2001 0000)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nsbootadd1r",
            extends: None,
            description: Some(
                "boot address 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsbootadd1",
                    description: Some(
                        "Non-secure boot address 1\r This address is only used when TZEN = 0.\r The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address [31:7]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. \r Examples:\r NSBOOTADD1[24:0] = 0x0100000: Boot from memory (0x0800 0000)\r NSBOOTADD1[24:0] = 0x017F100: Boot from system memory bootloader (0x0BF8 8000)\r NSBOOTADD1[24:0] = 0x0400200: Boot from SRAM2 (0x2001 0000)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nscr1",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Non-secure programming",
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
                    name: "per",
                    description: Some(
                        "Non-secure page erase",
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
                    name: "mer",
                    description: Some(
                        "Non-secure mass erase\r This bit triggers the non-secure mass erase (all user pages) when set.",
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
                    name: "pnb",
                    description: Some(
                        "Non-secure page number selection\r These bits select the page to erase.\r ...\r Note that bit 9 is reserved on STM32WBA5xEx devices.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bwr",
                    description: Some(
                        "Non-secure burst write programming mode\r When set, this bit selects the burst write programming mode.",
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
                    name: "strt",
                    description: Some(
                        "Non-secure operation start \r This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in NSSR is set (this condition is forbidden).\r This bit is set only by software and is cleared when the BSY bit is cleared in NSSR.",
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
                    name: "optstrt",
                    description: Some(
                        "Options modification start\r This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in NSSR.",
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
                    name: "eopie",
                    description: Some(
                        "Non-secure end of operation interrupt enable\r This bit enables the interrupt generation when the EOP bit in the NSSR is set to 1.",
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
                    name: "errie",
                    description: Some(
                        "Non-secure error interrupt enable\r This bit enables the interrupt generation when the OPERR bit in the NSSR is set to 1.",
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
                    name: "obl_launch",
                    description: Some(
                        "Force the option byte loading\r When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK.\r Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH.",
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
                    name: "optlock",
                    description: Some(
                        "Option lock\r This bit is set only. When set, the NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in OPTKEYR. The NSCR1.LOCK bit must be cleared before doing the OPTKEYR unlock sequence.\r In case of an unsuccessful unlock operation, this bit remains set until the next reset.",
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
                    name: "lock",
                    description: Some(
                        "Non-secure lock\r This bit is set only.\r When set, the NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in NSKEYR.\r In case of an unsuccessful unlock operation, this bit remains set until the next system reset.",
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
            name: "Nscr2",
            extends: None,
            description: Some(
                "control 2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ps",
                    description: Some(
                        "Program suspend request",
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
                    name: "es",
                    description: Some(
                        "Erase suspend request",
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
            name: "Nssr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some(
                        "Non-secure end of operation\r This bit is set by hardware when one or more memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in NSCR1). This bit is cleared by writing�1.",
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
                    name: "operr",
                    description: Some(
                        "Non-secure operation error\r This bit is set by hardware when a memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1.",
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
                    name: "progerr",
                    description: Some(
                        "Non-secure programming error\r This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.",
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
                    name: "wrperr",
                    description: Some(
                        "Non-secure write protection error\r This bit is set by hardware when a non-secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1.\r Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting.",
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
                    name: "pgaerr",
                    description: Some(
                        "Non-secure programming alignment error\r This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.",
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
                    name: "sizerr",
                    description: Some(
                        "Non-secure size error\r This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.",
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
                    name: "pgserr",
                    description: Some(
                        "Non-secure programming sequence error\r This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1.\r Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting.",
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
                    name: "optwerr",
                    description: Some(
                        "Option write error \r This bit is set by hardware when the options bytes are written with an invalid configuration or when modifying options in RDP level 2.. It is cleared by writing 1.\r Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting.",
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
                    name: "bsy",
                    description: Some(
                        "Non-secure busy\r This indicates that a memory secure or non-secure operation is in progress. This bit is set at the beginning of a operation and reset when the operation finishes or when an error occurs.",
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
                    name: "wdw",
                    description: Some(
                        "Non-secure wait data to write\r This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory.",
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
                    name: "oem1lock",
                    description: Some(
                        "OEM1 key RDP lock\r This bit indicates that the OEM1 key read during the OBL is not virgin. When set, the OEM1 key RDP lock mechanism is active.",
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
                    name: "oem2lock",
                    description: Some(
                        "OEM2 key RDP lock\r This bit indicates that the OEM2 key read during the OBL is not virgin. When set, the OEM2 key RDP lock mechanism is active.",
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
                    name: "pd",
                    description: Some(
                        "in power-down mode\r This bit indicates that the memory is in power-down state. It is reset when is in normal mode or being awaken.",
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
            name: "Opsr",
            extends: None,
            description: Some(
                "operation status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_op",
                    description: Some(
                        "Interrupted operation address\r This field indicates which address in the memory was accessed when reset occurred. The address is given relative to the base address, from offset 0x0�0000 to 0xF�FFF0.\r Note that bit 19 is reserved on STM32WBAxEx devices.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_op",
                    description: Some(
                        "Operation in system memory interrupted\r This bit indicates that the reset occurred during an operation in the system memory.",
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
                    name: "code_op",
                    description: Some(
                        "memory operation code\r This field indicates which memory operation has been interrupted by a system reset:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CodeOp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some(
                "option register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdp",
                    description: Some(
                        "Readout protection level\r Others: Level 1 (memories readout protection active)\r Note: Refer to Section�7.6.2: Readout protection (RDP) for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Rdp",
                    ),
                },
                Field {
                    name: "bor_lev",
                    description: Some(
                        "BOR reset level\r These bits contain the V<sub>DD</sub> supply level threshold that activates/releases the reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "BorLev",
                    ),
                },
                Field {
                    name: "nrst_stop",
                    description: Some(
                        "Reset generation in Stop mode",
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
                    name: "nrst_stdby",
                    description: Some(
                        "Reset generation in Standby mode",
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
                    name: "sram1_rst",
                    description: Some(
                        "SRAM1 erase upon system reset",
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
                    name: "iwdg_sw",
                    description: Some(
                        "Independent watchdog enable selection",
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
                    name: "iwdg_stop",
                    description: Some(
                        "Independent watchdog counter freeze in Stop mode",
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
                    name: "iwdg_stdby",
                    description: Some(
                        "Independent watchdog counter freeze in Standby mode",
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
                    name: "wwdg_sw",
                    description: Some(
                        "Window watchdog selection",
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
                    name: "sram2_pe",
                    description: Some(
                        "SRAM2 parity check enable",
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
                    name: "sram2_rst",
                    description: Some(
                        "SRAM2 erase when system reset",
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
                    name: "nswboot0",
                    description: Some(
                        "Software BOOT0",
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
                    name: "nboot0",
                    description: Some(
                        "NBOOT0 option bit",
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
                    name: "tzen",
                    description: Some(
                        "Global TrustZone security enable",
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
            name: "Prifcfgr",
            extends: None,
            description: Some(
                "privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "Privileged protection for secure registers\r This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN�=�1).",
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
                        "Privileged protection for non-secure registers",
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
            name: "Secbootadd0r",
            extends: None,
            description: Some(
                "secure boot address 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_lock",
                    description: Some(
                        "Boot lock\r This lock is only used when TZEN = 0.\r When set, the boot is always forced to base address value programmed in SECBOOTADD0[24:0] option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP regression level 1 to level 0.",
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
                    name: "secbootadd0",
                    description: Some(
                        "Secure boot base address 0\r This address is only used when TZEN = 1.\r The secure boot memory address can be programmed to any address in the valid address range (see Table�28: Boot space versus RDP protection) with a granularity of 128 bytes. This bits correspond to address [31:7] The SECBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. \r Examples:\r SECBOOTADD0[24:0] = 0x018 0000: Boot from secure user memory (0x0C00 0000)\r SECBOOTADD0[24:0] = 0x01F F000: Boot from RSS system memory (0x0FF8 0000)\r SECBOOTADD0[24:0] = 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccr1",
            extends: None,
            description: Some(
                "secure control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Secure programming",
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
                    name: "per",
                    description: Some(
                        "Secure page erase",
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
                    name: "mer",
                    description: Some(
                        "Secure mass erase\r This bit triggers the secure mass erase (all user pages) when set.",
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
                    name: "pnb",
                    description: Some(
                        "Secure page number selection\r These bits select the page to erase:\r ...\r Note that bit 9 is reserved on STM32WBA5xEx devices.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bwr",
                    description: Some(
                        "Secure burst write programming mode\r When set, this bit selects the burst write programming mode.",
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
                    name: "strt",
                    description: Some(
                        "Secure start \r This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the SECSR is set (this condition is forbidden).\r This bit is set only by software and is cleared when the BSY bit is cleared in SECSR.",
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
                    name: "eopie",
                    description: Some(
                        "Secure End of operation interrupt enable\r This bit enables the interrupt generation when the EOP bit in SECSR is set to 1.",
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
                    name: "errie",
                    description: Some(
                        "Secure error interrupt enable\r This bit enables the interrupt generation when the OPERR bit in SECSR is set to 1.",
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
                    name: "inv",
                    description: Some(
                        "memory security state invert\r This bit inverts the memory security state.",
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
                    name: "lock",
                    description: Some(
                        "Secure lock\r This bit is set only. When set, the SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in SECKEYR register.\r In case of an unsuccessful unlock operation, this bit remains set until the next system reset.",
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
            name: "Seccr2",
            extends: None,
            description: Some(
                "secure control 2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ps",
                    description: Some(
                        "Program suspend request",
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
                    name: "es",
                    description: Some(
                        "Erase suspend request",
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
            name: "Sechdpcr",
            extends: None,
            description: Some(
                "secure HDP control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp_accdis",
                    description: Some(
                        "Secure HDP area access disable \r When set, this bit is only cleared by a system reset.",
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
            name: "Secsr",
            extends: None,
            description: Some(
                "secure status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eop",
                    description: Some(
                        "Secure end of operation\r This bit is set by hardware when one or more memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in SECCR1). This bit is cleared by writing�1.",
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
                    name: "operr",
                    description: Some(
                        "Secure operation error\r This bit is set by hardware when a memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1.",
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
                    name: "progerr",
                    description: Some(
                        "Secure programming error\r This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.",
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
                    name: "wrperr",
                    description: Some(
                        "Secure write protection error\r This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1.\r Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting.",
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
                    name: "pgaerr",
                    description: Some(
                        "Secure programming alignment error\r This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1.",
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
                    name: "sizerr",
                    description: Some(
                        "Secure size error\r This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1.",
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
                    name: "pgserr",
                    description: Some(
                        "Secure programming sequence error\r This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1.\r Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting.",
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
                    name: "bsy",
                    description: Some(
                        "Secure busy\r This bit indicates that a memory secure or non-secure operation is in progress. This is set on the beginning of a operation and reset when the operation finishes or when an error occurs.",
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
                    name: "wdw",
                    description: Some(
                        "Secure wait data to write\r This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory.",
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
        FieldSet {
            name: "Secwmr1",
            extends: None,
            description: Some(
                "secure watermark register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secwm_pstrt",
                    description: Some(
                        "Start page of secure area\r This field contains the first page of the secure area.",
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
                    name: "secwm_pend",
                    description: Some(
                        "End page of secure area\r This field contains the last page of the secure area.",
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
            name: "Secwmr2",
            extends: None,
            description: Some(
                "secure watermark register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp_pend",
                    description: Some(
                        "End page of secure hide protection area\r This field contains the last page of the secure HDP area.",
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
                Field {
                    name: "hdpen",
                    description: Some(
                        "Secure Hide protection area enable",
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
            name: "Wrpar",
            extends: None,
            description: Some(
                "WRP area A address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpa_pstrt",
                    description: Some(
                        "WPR area A start page\r This field contains the first page of the WPR area A.\r Note that bit 6 is reserved on STM32WBAxEx devices.",
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
                    name: "wrpa_pend",
                    description: Some(
                        "WPR area A end page\r This field contains the last page of the WPR area A.\r Note that bit 22 is reserved on STM32WBAxEx devices.",
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
                Field {
                    name: "unlock",
                    description: Some(
                        "WPR area A unlock",
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
            name: "Wrpbr",
            extends: None,
            description: Some(
                "WRP area B address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpb_pstrt",
                    description: Some(
                        "WRP area B start page\r This field contains the first page of the WRP area B.\r Note that bit 6 is reserved on STM32WBAxEx devices.",
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
                    name: "wrpb_pend",
                    description: Some(
                        "WRP area B end page\r This field contains the last page of the WRP area B.\r Note that bit 22 is reserved on STM32WBAxEx devices.",
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
                Field {
                    name: "unlock",
                    description: Some(
                        "WPR area B unlock",
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
    ],
    enums: &[
        Enum {
            name: "BorLev",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "LEVEL0",
                    description: Some(
                        "BOR level 0 (reset level threshold around 1.7�V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LEVEL1",
                    description: Some(
                        "BOR level 1 (reset level threshold around 2.0�V)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LEVEL2",
                    description: Some(
                        "BOR level 2 (reset level threshold around 2.2�V)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LEVEL3",
                    description: Some(
                        "BOR level 3 (reset level threshold around 2.5�V)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LEVEL4",
                    description: Some(
                        "BOR level 4 (reset level threshold around 2.8�V)",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "CodeOp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No operation interrupted by previous reset",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Single write operation interrupted",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "Burst write operation interrupted",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "Page erase operation interrupted",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "Reserved",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "Mass erase operation interrupted",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "Option change operation interrupted",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "Reserved",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rdp",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X55",
                    description: Some(
                        "Level 0.5 (readout protection not active, only non-secure debug access is possible). Only available when TrustZone is active (TZEN=1)",
                    ),
                    value: 85,
                },
                EnumVariant {
                    name: "B_0XAA",
                    description: Some(
                        "Level 0 (readout protection not active)",
                    ),
                    value: 170,
                },
                EnumVariant {
                    name: "B_0XCC",
                    description: Some(
                        "Level 2 (chip readout protection active)",
                    ),
                    value: 204,
                },
            ],
        },
    ],
};
